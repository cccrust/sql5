#![allow(dead_code, unused)]

mod btree;
mod catalog;
mod fts;
mod interface;
mod pager;
mod parser;
mod planner;
mod table;

use interface::Repl;
use std::env::{self, Args};
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    
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
