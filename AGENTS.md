# AGENTS.md

## Project
- Single Cargo package `sql5` v1.4.0 (SQLite-compatible database with CJK FTS support)
- Edition: Rust 2024
- No workspace — single package only

## Commands
- `cargo run` — Run the REPL
- `cargo build` — Build
- `cargo check` — Type-check without building
- `cargo test` — Run unit tests (200+ inline tests in src/)
- `cargo test <name>` — Run tests matching `<name>`
- `./test.sh` — CLI integration test suite (uses `./target/debug/sql5` by default)
- `./test.sh ./target/debug/sql5` — Run CLI tests against a specific binary

## Architecture
- **parser** — SQL parsing and AST
- **planner** — Query planning and execution
- **btree** — B+Tree index implementation
- **table** — Table management (schema, rows, serialization)
- **pager** — Storage engine (memory + disk, WAL support)
- **catalog** — System catalog and metadata
- **fts** — Full-text search (FTS5 with CJK support)
- **interface** — REPL CLI

## REPL Usage
- `sql5` — In-memory REPL (prompt: `sql5> `)
- `sql5 <path>` — Open or create a database file
- `.quit` — Exit REPL

## Version Documentation
每次完成重要功能後，必須在 `_doc/vX.X.md` 建立版本文件，內容包含：
- 版本號與日期
- 新增功能列表
- 架構變化說明
- 使用方式
- 已知限制
- 下一步工作
- 測試結果

## Notes
- Temp test files use `/tmp/sql5_*.db` pattern
- REPL hardcodes banner "sql5 v0.1.0" in `src/interface/repl.rs:375` (stale, not synced to Cargo.toml)