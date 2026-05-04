use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio_tungstenite::{accept_async, tungstenite::Message};

use crate::fts::FtsTable;
use crate::parser::parse;
use crate::planner::planner::Planner;
use crate::planner::{Executor, ResultSet};
use crate::table::row::Value;

pub struct WsServer {
    executor: Arc<Mutex<Executor>>,
    fts_tables: Arc<Mutex<HashMap<String, FtsTable>>>,
    db_path: Option<String>,
    shutdown: broadcast::Sender<()>,
}

impl WsServer {
    pub fn new() -> Self {
        let (shutdown, _) = broadcast::channel(1);
        WsServer {
            executor: Arc::new(Mutex::new(Executor::new())),
            fts_tables: Arc::new(Mutex::new(HashMap::new())),
            db_path: None,
            shutdown,
        }
    }

    pub fn open(path: &str) -> std::io::Result<Self> {
        let (shutdown, _) = broadcast::channel(1);
        let executor = Executor::with_disk(path)?;
        Ok(WsServer {
            executor: Arc::new(Mutex::new(executor)),
            fts_tables: Arc::new(Mutex::new(HashMap::new())),
            db_path: Some(path.to_string()),
            shutdown,
        })
    }

    pub async fn run(&mut self, port: u16) -> std::io::Result<()> {
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&addr).await?;
        println!("WebSocket server listening on ws://{}", addr);

        let mut shutdown_rx = self.shutdown.subscribe();

        loop {
            tokio::select! {
                result = listener.accept() => {
                    match result {
                        Ok((stream, addr)) => {
                            let executor = Arc::clone(&self.executor);
                            let fts_tables = Arc::clone(&self.fts_tables);
                            tokio::spawn(handle_connection(stream, addr, executor, fts_tables));
                        }
                        Err(e) => {
                            eprintln!("Accept error: {}", e);
                        }
                    }
                }
                _ = shutdown_rx.recv() => {
                    println!("Shutting down WebSocket server");
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn shutdown(&self) {
        let _ = self.shutdown.send(());
    }
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    executor: Arc<Mutex<Executor>>,
    fts_tables: Arc<Mutex<HashMap<String, FtsTable>>>,
) {
    println!("New WebSocket connection from: {}", addr);

    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("WebSocket handshake failed: {}", e);
            return;
        }
    };

    let (mut write, mut read) = ws_stream.split();

    let _ = write.send(Message::Text(r#"{"ok":true,"ready":true}"#.to_string())).await;

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let executor = Arc::clone(&executor);
                let fts_tables = Arc::clone(&fts_tables);
                let response = tokio::task::spawn_blocking(move || {
                    process_request(&text, &executor, &fts_tables)
                }).await.unwrap_or_else(|_| r#"{"ok":false,"error":"task error"}"#.to_string());
                let _ = write.send(Message::Text(response)).await;
            }
            Ok(Message::Close(_)) => {
                println!("Client {} disconnected", addr);
                break;
            }
            Err(e) => {
                eprintln!("Error reading from {}: {}", addr, e);
                break;
            }
            _ => {}
        }
    }
}

fn process_request(
    line: &str,
    executor: &Arc<Mutex<Executor>>,
    fts_tables: &Arc<Mutex<HashMap<String, FtsTable>>>,
) -> String {
    let request: serde_json::Value = match serde_json::from_str(line) {
        Ok(v) => v,
        Err(e) => return format!(r#"{{"ok":false,"error":"json parse error: {}"}}"#, e),
    };

    let method = match request.get("method").and_then(|v| v.as_str()) {
        Some(m) => m,
        None => return r#"{"ok":false,"error":"missing method"}"#.to_string(),
    };

    match method {
        "execute" => {
            let sql = match request.get("sql").and_then(|v| v.as_str()) {
                Some(s) => s,
                None => return r#"{"ok":false,"error":"missing sql"}"#.to_string(),
            };
            execute_sql(sql, executor, fts_tables)
        }
        "close" => {
            r#"{"ok":true}"#.to_string()
        }
        _ => format!(r#"{{"ok":false,"error":"unknown method: {}"}}"#, method),
    }
}

fn execute_sql(
    sql: &str,
    executor: &Arc<Mutex<Executor>>,
    fts_tables: &Arc<Mutex<HashMap<String, FtsTable>>>,
) -> String {
    let upper = sql.trim().to_uppercase();

    if upper.starts_with("CREATE VIRTUAL TABLE") && upper.contains("USING FTS5") {
        return fts_create(sql, fts_tables);
    }

    if let Some(name) = extract_table_name_from_insert(sql) {
        if fts_tables.lock().unwrap().contains_key(&name) {
            return fts_insert(sql, &name, fts_tables);
        }
    }

    if upper.contains("MATCH") {
        if let Some((name, query)) = extract_match_query(sql) {
            if fts_tables.lock().unwrap().contains_key(&name) {
                return fts_select(&name, &query, fts_tables);
            }
        }
    }

    let stmts = match parse(sql) {
        Ok(s) => s,
        Err(e) => return format!(r#"{{"ok":false,"error":"parse error: {}"}}"#, e),
    };

    let mut last_result: Option<String> = None;
    for stmt in stmts {
        let mut exec = executor.lock().unwrap();
        let plan = match Planner::new(exec.catalog()).plan(stmt) {
            Ok(p) => p,
            Err(e) => return format!(r#"{{"ok":false,"error":"plan error: {}"}}"#, e),
        };

        match exec.execute(plan) {
            Ok(rs) => {
                last_result = Some(resultset_to_json(&rs));
            }
            Err(e) => return format!(r#"{{"ok":false,"error":"execution error: {}"}}"#, e),
        }
    }

    last_result.unwrap_or_else(|| r#"{"ok":true,"columns":[],"rows":[],"affected":0}"#.to_string())
}

fn resultset_to_json(rs: &ResultSet) -> String {
    let columns: Vec<String> = rs.columns.clone();
    let rows: Vec<Vec<serde_json::Value>> = rs.rows.iter().map(|row| {
        row.iter().map(|v| value_to_json(v)).collect()
    }).collect();

    let json = serde_json::json!({
        "ok": true,
        "columns": columns,
        "rows": rows,
        "affected": 0
    });
    serde_json::to_string(&json).unwrap_or_else(|_| r#"{"ok":false,"error":"json serialization error"}"#.to_string())
}

fn value_to_json(v: &Value) -> serde_json::Value {
    match v {
        Value::Null => serde_json::Value::Null,
        Value::Integer(i) => serde_json::Value::Number((*i).into()),
        Value::Float(f) => serde_json::Number::from_f64(*f)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        Value::Text(s) => serde_json::Value::String(s.clone()),
        Value::Boolean(b) => serde_json::Value::Bool(*b),
    }
}

fn fts_create(sql: &str, fts_tables: &Arc<Mutex<HashMap<String, FtsTable>>>) -> String {
    let lower = sql.to_lowercase();
    let after_table = match lower.find("table") {
        Some(p) => p + 5,
        None => return r#"{"ok":false,"error":"parse error"}"#.to_string(),
    };
    let after_using = match lower.find("using") {
        Some(p) => p,
        None => return r#"{"ok":false,"error":"parse error"}"#.to_string(),
    };
    let name = sql[after_table..after_using].trim().to_string();

    let after_fts5 = match lower.find("fts5") {
        Some(p) => p + 4,
        None => return r#"{"ok":false,"error":"parse error"}"#.to_string(),
    };
    let lparen = match sql[after_fts5..].find('(') {
        Some(p) => p + after_fts5,
        None => return r#"{"ok":false,"error":"parse error"}"#.to_string(),
    };
    let rparen = match sql.rfind(')') {
        Some(p) => p,
        None => return r#"{"ok":false,"error":"parse error"}"#.to_string(),
    };
    let cols_str = &sql[lparen + 1..rparen];
    let columns: Vec<String> = cols_str.split(',')
        .map(|c| c.trim().to_string())
        .filter(|c| !c.is_empty())
        .collect();

    if fts_tables.lock().unwrap().contains_key(&name) {
        return format!(r#"{{"ok":false,"error":"FTS table '{}' already exists"}}"#, name);
    }
    fts_tables.lock().unwrap().insert(name.clone(), FtsTable::new(&name, columns));
    format!(r#"{{"ok":true,"columns":[],"rows":[],"affected":1}}"#)
}

fn fts_insert(sql: &str, table_name: &str, fts_tables: &Arc<Mutex<HashMap<String, FtsTable>>>) -> String {
    let lower = sql.to_lowercase();
    let after_values = match lower.find("values") {
        Some(p) => p + 6,
        None => return r#"{"ok":false,"error":"parse error"}"#.to_string(),
    };
    let lparen = match sql[after_values..].find('(') {
        Some(p) => p + after_values,
        None => return r#"{"ok":false,"error":"parse error"}"#.to_string(),
    };
    let rparen = match sql.rfind(')') {
        Some(p) => p,
        None => return r#"{"ok":false,"error":"parse error"}"#.to_string(),
    };
    let vals_str = &sql[lparen + 1..rparen];
    let values: Vec<String> = split_sql_values(vals_str);

    if let Some(tbl) = fts_tables.lock().unwrap().get_mut(table_name) {
        tbl.insert(values);
        format!(r#"{{"ok":true,"columns":[],"rows":[],"affected":1}}"#)
    } else {
        format!(r#"{{"ok":false,"error":"table '{}' not found"}}"#, table_name)
    }
}

fn fts_select(table_name: &str, query: &str, fts_tables: &Arc<Mutex<HashMap<String, FtsTable>>>) -> String {
    let mut fts = fts_tables.lock().unwrap();
    let tbl = match fts.get_mut(table_name) {
        Some(t) => t,
        None => return format!(r#"{{"ok":false,"error":"table '{}' not found"}}"#, table_name),
    };
    let results = tbl.search(query);
    let col_names = tbl.columns.clone();

    let mut out_cols = vec!["rowid".to_string(), "score".to_string()];
    out_cols.extend(col_names);

    let rows: Vec<Vec<serde_json::Value>> = results.into_iter().map(|(rowid, score, vals)| {
        let mut row = vec![serde_json::Value::Number(rowid.into()), serde_json::Number::from_f64(score).map(|n| serde_json::Value::Number(n)).unwrap_or(serde_json::Value::Null)];
        row.extend(vals.into_iter().map(|v| serde_json::Value::String(v)));
        row
    }).collect();

    let json = serde_json::json!({
        "ok": true,
        "columns": out_cols,
        "rows": rows,
        "affected": 0
    });
    serde_json::to_string(&json).unwrap_or_else(|_| r#"{"ok":false,"error":"json error"}"#.to_string())
}

fn extract_table_name_from_insert(sql: &str) -> Option<String> {
    let lower = sql.to_lowercase();
    let after_into = lower.find("into")? + 4;
    let rest = sql[after_into..].trim();
    let name: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
    if name.is_empty() { None } else { Some(name) }
}

fn extract_match_query(sql: &str) -> Option<(String, String)> {
    let lower = sql.to_lowercase();
    let match_pos = lower.find("match")?;
    let after_match = sql[match_pos + 5..].trim();

    let where_pos = lower.find("where")?;
    let between = sql[where_pos + 5..match_pos].trim();
    let table_name: String = between.chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();

    let query = after_match.trim_matches(|c| c == '\'' || c == '"' || c == ';').to_string();
    if table_name.is_empty() || query.is_empty() { None } else { Some((table_name, query)) }
}

fn split_sql_values(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut quote_char = ' ';

    for c in s.chars() {
        match c {
            '\'' | '"' if !in_quote => { in_quote = true; quote_char = c; }
            c if in_quote && c == quote_char => { in_quote = false; }
            ',' if !in_quote => {
                result.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(c),
        }
    }
    if !current.trim().is_empty() {
        result.push(current.trim().to_string());
    }
    result
}