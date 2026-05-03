# AGENTS.md

## Project
- Single Cargo package `sql5` v1.4.0 (SQLite-compatible database with CJK FTS5 support)
- Edition: Rust 2024 (requires nightly or recent stable)
- No workspace — single package only

## Commands
- `cargo build` — Build binary to `./target/debug/sql5`
- `cargo run` — Run the REPL (in-memory mode)
- `cargo check` — Type-check without building
- `cargo test` — Run all unit tests (200+ inline tests in `src/`)
- `cargo test <name>` — Run tests matching `<name>`
- `./test.sh` — CLI integration test suite
- `./test.sh /path/to/binary` — Run CLI tests against a specific binary

## Architecture
- **parser** — SQL parsing and AST
- **planner** — Query planning and execution (includes executor, datetime, string functions)
- **btree** — B+Tree index implementation
- **table** — Table management (schema, rows, serialization)
- **pager** — Storage engine (memory + disk, WAL support)
- **catalog** — System catalog and metadata
- **fts** — Full-text search (FTS5 with CJK character support and bigram tokenization)
- **interface** — REPL CLI (`src/main.rs` is the entrypoint)

## REPL Usage
- `sql5` — In-memory REPL (prompt: `sql5> `)
- `sql5 <path>` — Open or create a database file
- `.quit` — Exit REPL
- `.help` — List dot commands
- `.tables` — List tables
- `.schema [table]` — Show table/view DDL

## Version Documentation
每次完成重要功能後，必須在 `_doc/vX.X.md` 建立版本文件（見現有 `v1.1`–`v1.16` 範例）。

## Notes
- Temp test files use `/tmp/sql5_*.db` pattern
- REPL hardcodes banner "sql5 v0.1.0" at `src/interface/repl.rs:375` (stale; not synced to Cargo.toml)