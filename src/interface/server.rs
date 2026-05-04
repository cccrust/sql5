//! SQL5 Server Mode：JSON over stdin/stdout
//!
//! Protocol:
//! - Read lines: {"method": "execute", "sql": "...", "params": [...]}
//! - Write lines: {"ok": true, "columns": [...], "rows": [...], "affected": N}
//!               {"ok": false, "error": "..."}
//! - Special: {"method": "close"} to shutdown server

use std::io::{self, BufRead, Write};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::fts::FtsTable;
use crate::parser::parse;
use crate::planner::planner::Planner;
use crate::planner::{Executor, ResultSet};
use crate::table::row::Value;

pub struct Server {
    executor: Arc<Mutex<Executor>>,
    fts_tables: Arc<Mutex<HashMap<String, FtsTable>>>,
    db_path: Option<String>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            executor: Arc::new(Mutex::new(Executor::new())),
            fts_tables: Arc::new(Mutex::new(HashMap::new())),
            db_path: None,
        }
    }

    pub fn open<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Self> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        let executor = Executor::with_disk(&path_str)?;
        Ok(Server {
            executor: Arc::new(Mutex::new(executor)),
            fts_tables: Arc::new(Mutex::new(HashMap::new())),
            db_path: Some(path_str),
        })
    }

    pub fn run(&mut self) {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut lines = stdin.lock().lines();

        // Send ready signal
        let _ = writeln!(stdout.lock(), "{{\"ok\":true,\"ready\":true}}");
        let _ = stdout.lock().flush();

        loop {
            let line = match lines.next() {
                Some(Ok(l)) => l,
                Some(Err(e)) => {
                    let _ = writeln!(stdout.lock(), "{{\"ok\":false,\"error\":\"read error: {}\"}}", e);
                    break;
                }
                None => break,
            };

            if let Some(response) = self.handle_line(&line) {
                let _ = writeln!(stdout.lock(), "{}", response);
                let _ = stdout.lock().flush();

                // Check if close command
                if line.contains("\"close\"") {
                    break;
                }
            } else {
                let _ = writeln!(stdout.lock(), "{{\"ok\":false,\"error\":\"invalid request\"}}");
                let _ = stdout.lock().flush();
            }
        }
    }

    fn handle_line(&mut self, line: &str) -> Option<String> {
        let request: serde_json::Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(e) => return Some(format!(r#"{{"ok":false,"error":"json parse error: {}"}}"#, e)),
        };

        let method = request.get("method")?.as_str()?;
        match method {
            "execute" => {
                let sql = request.get("sql")?.as_str()?;
                let params: Vec<serde_json::Value> = request.get("params")
                    .and_then(|v| v.as_array())
                    .cloned()
                    .unwrap_or_default();
                self.execute_sql(sql, params)
            }
            "close" => {
                self.close();
                Some(r#"{"ok":true}"#.to_string())
            }
            _ => Some(format!(r#"{{"ok":false,"error":"unknown method: {}"}}"#, method)),
        }
    }

    fn execute_sql(&mut self, sql: &str, _params: Vec<serde_json::Value>) -> Option<String> {
        // First try FTS handling
        if let Some(result) = self.try_handle_fts(sql) {
            return Some(result);
        }

        let stmts = match parse(sql) {
            Ok(s) => s,
            Err(e) => return Some(format!(r#"{{"ok":false,"error":"parse error: {}"}}"#, e)),
        };

        let mut last_result: Option<String> = None;
        for stmt in stmts {
            let mut executor = self.executor.lock().unwrap();
            let plan = match Planner::new(executor.catalog()).plan(stmt) {
                Ok(p) => p,
                Err(e) => return Some(format!(r#"{{"ok":false,"error":"plan error: {}"}}"#, e)),
            };

            match executor.execute(plan) {
                Ok(rs) => {
                    last_result = Some(self.resultset_to_json(&rs));
                }
                Err(e) => return Some(format!(r#"{{"ok":false,"error":"execution error: {}"}}"#, e)),
            }
        }

        last_result
    }

    fn resultset_to_json(&self, rs: &ResultSet) -> String {
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

    fn try_handle_fts(&mut self, sql: &str) -> Option<String> {
        let upper = sql.trim().to_uppercase();

        if upper.starts_with("CREATE VIRTUAL TABLE") && upper.contains("USING FTS5") {
            return Some(self.fts_create(sql));
        }

        if upper.starts_with("INSERT INTO") {
            if let Some(name) = extract_table_name_from_insert(sql) {
                if self.fts_tables.lock().unwrap().contains_key(&name) {
                    return Some(self.fts_insert(sql, &name));
                }
            }
        }

        if upper.contains("MATCH") {
            if let Some((name, query)) = extract_match_query(sql) {
                if self.fts_tables.lock().unwrap().contains_key(&name) {
                    return Some(self.fts_select(&name, &query));
                }
            }
        }

        None
    }

    fn fts_create(&mut self, sql: &str) -> String {
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

        if self.fts_tables.lock().unwrap().contains_key(&name) {
            return format!(r#"{{"ok":false,"error":"FTS table '{}' already exists"}}"#, name);
        }
        self.fts_tables.lock().unwrap().insert(name.clone(), FtsTable::new(&name, columns));
        format!(r#"{{"ok":true,"columns":[],"rows":[],"affected":1}}"#)
    }

    fn fts_insert(&mut self, sql: &str, table_name: &str) -> String {
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

        if let Some(tbl) = self.fts_tables.lock().unwrap().get_mut(table_name) {
            tbl.insert(values);
            format!(r#"{{"ok":true,"columns":[],"rows":[],"affected":1}}"#)
        } else {
            format!(r#"{{"ok":false,"error":"table '{}' not found"}}"#, table_name)
        }
    }

    fn fts_select(&mut self, table_name: &str, query: &str) -> String {
        let mut fts = self.fts_tables.lock().unwrap();
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

    pub fn close(&mut self) {
        if self.db_path.is_some() {
            self.executor.lock().unwrap().flush();
        }
    }
}

impl Default for Server {
    fn default() -> Self { Self::new() }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table::row::Value;

    #[test]
    fn test_value_to_json_null() {
        let v = Value::Null;
        let json = value_to_json(&v);
        assert_eq!(json, serde_json::Value::Null);
    }

    #[test]
    fn test_value_to_json_integer() {
        let v = Value::Integer(42);
        let json = value_to_json(&v);
        assert_eq!(json, serde_json::json!(42));
    }

    #[test]
    fn test_value_to_json_float() {
        let v = Value::Float(3.14);
        let json = value_to_json(&v);
        assert_eq!(json, serde_json::json!(3.14));
    }

    #[test]
    fn test_value_to_json_text() {
        let v = Value::Text("hello".to_string());
        let json = value_to_json(&v);
        assert_eq!(json, serde_json::json!("hello"));
    }

    #[test]
    fn test_value_to_json_boolean() {
        assert_eq!(value_to_json(&Value::Boolean(true)), serde_json::json!(true));
        assert_eq!(value_to_json(&Value::Boolean(false)), serde_json::json!(false));
    }

    #[test]
    fn test_extract_table_name_from_insert() {
        assert_eq!(extract_table_name_from_insert("INSERT INTO users VALUES (1, 'a')"), Some("users".to_string()));
        assert_eq!(extract_table_name_from_insert("INSERT INTO my_table (id) VALUES (1)"), Some("my_table".to_string()));
        assert_eq!(extract_table_name_from_insert("INSERT INTO users_db_123 VALUES (1)"), Some("users_db_123".to_string()));
        assert_eq!(extract_table_name_from_insert("INSERT INTO users (id, name) VALUES (1, 'a')"), Some("users".to_string()));
    }

    #[test]
    fn test_extract_table_name_from_insert_case_insensitive() {
        assert_eq!(extract_table_name_from_insert("insert into users values (1)"), Some("users".to_string()));
        assert_eq!(extract_table_name_from_insert("INSERT INTO USERS VALUES (1)"), Some("USERS".to_string()));
    }

    #[test]
    fn test_extract_table_name_from_insert_not_found() {
        assert_eq!(extract_table_name_from_insert("SELECT * FROM users"), None);
        assert_eq!(extract_table_name_from_insert("UPDATE users SET id = 1"), None);
    }

    #[test]
    fn test_extract_match_query() {
        let result = extract_match_query("SELECT * FROM articles WHERE articles MATCH 'rust'");
        assert!(result.is_some());
        let (table, query) = result.unwrap();
        assert_eq!(table, "articles");
        assert_eq!(query, "rust");
    }

    #[test]
    fn test_extract_match_query_with_quotes() {
        let result = extract_match_query("SELECT * FROM articles WHERE articles MATCH '中文'");
        assert!(result.is_some());
        let (_, query) = result.unwrap();
        assert_eq!(query, "中文");
    }

    #[test]
    fn test_extract_match_query_not_found() {
        assert_eq!(extract_match_query("SELECT * FROM users"), None);
        assert_eq!(extract_match_query("WHERE id = 1"), None);
    }

    #[test]
    fn test_split_sql_values_simple() {
        let result = split_sql_values("1, 2, 3");
        assert_eq!(result, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_split_sql_values_with_strings() {
        let result = split_sql_values("'a', 'b', 'c'");
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_split_sql_values_quoted_comma() {
        let result = split_sql_values("'hello, world', 'foo'");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_split_sql_values_empty() {
        let result = split_sql_values("");
        assert!(result.is_empty());
    }

    #[test]
    fn test_split_sql_values_single_value() {
        let result = split_sql_values("42");
        assert_eq!(result, vec!["42"]);
    }

    #[test]
    fn test_split_sql_values_with_spaces() {
        let result = split_sql_values("  1  ,  2  ,  3  ");
        assert_eq!(result, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_split_sql_values_double_quotes() {
        let result = split_sql_values("\"a\", \"b\"");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_ws_split_sql_values_double_quotes() {
        use super::*;
        let result = split_sql_values("\"a\", \"b\"");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_server_new() {
        let server = Server::new();
        assert!(server.db_path.is_none());
    }

    #[test]
    fn test_server_close() {
        let mut server = Server::new();
        server.close();
    }

    #[test]
    fn test_server_default() {
        let server = Server::default();
        assert!(server.db_path.is_none());
    }
}