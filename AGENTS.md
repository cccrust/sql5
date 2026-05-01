# AGENTS.md

## Project Type
- Rust project (Cargo-based)
- Package name: `sql5` (SQLite-compatible database with CJK FTS support)
- Uses Rust edition 2024

## Commands
- `cargo run` - Run the REPL
- `cargo build` - Build the project
- `cargo check` - Type-check without building
- `./test.sh` - Run CLI test suite (default: `./target/debug/sql5`)

## Architecture
SQL database engine with layered architecture:
- **parser** - SQL parsing and AST
- **planner** - Query planning and execution
- **btree** - B+Tree index implementation
- **table** - Table management (schema, rows, serialization)
- **pager** - Storage engine (memory + disk, WAL support)
- **catalog** - System catalog and metadata
- **fts** - Full-text search (FTS5 with CJK support)
- **interface** - REPL CLI

## Testing
Run `./test.sh` for CLI tests, or point to a specific binary: `./test.sh ./target/debug/sql5`

## Notes
- Some temp files in tests use `/tmp/sql5_*.db` pattern
- REPL prompt shows "sql5> " 
- Version banner: "sql5 v0.1.0 — SQLite-compatible database with FTS"