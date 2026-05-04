#![allow(dead_code, unused)]

mod btree;
mod catalog;
mod fts;
mod interface;
mod pager;
mod parser;
mod planner;
mod table;

use interface::{Repl, Server, WsServer};
use std::env::{self, Args};
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for --websocket flag
    if let Some(idx) = args.iter().position(|s| s == "--websocket") {
        let port: u16 = args.get(idx + 1)
            .and_then(|s| s.parse().ok())
            .unwrap_or(8080);
        let db_path = args.get(idx + 2).map(|s| s.as_str());

        let mut server = if let Some(path) = db_path {
            eprintln!("Starting WebSocket server on port {} with database: {}", port, path);
            match WsServer::open(path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to open database: {}", e);
                    std::process::exit(1);
                }
            }
        } else {
            eprintln!("Starting WebSocket server on port {} (memory mode)", port);
            WsServer::new()
        };

        let runtime = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
        runtime.block_on(async {
            server.run(port).await.expect("WebSocket server error");
        });
        server.shutdown();
        return;
    }

    // Check for --server flag
    if args.contains(&"--server".to_string()) {
        let db_path = args.iter().skip_while(|s| *s != "--server").nth(1);
        let mut server = if let Some(path) = db_path {
            eprintln!("Starting server with database: {}", path);
            match Server::open(&path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to open database: {}", e);
                    std::process::exit(1);
                }
            }
        } else {
            eprintln!("Starting server (memory mode)");
            Server::new()
        };
        server.run();
        server.close();
        return;
    }

    let mut repl = if args.len() > 1 {
        let db_path = &args[1];
        println!("Opening database: {}", db_path);
        match Repl::open(db_path) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Failed to open database: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        Repl::new()
    };

    repl.run();
    repl.close();
}
