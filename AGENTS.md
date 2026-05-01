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

## Version Documentation
每次完成重要功能後，必須在 `_doc/sql5_vX.X.md` 建立版本文件，內容包含：
- 版本號與日期
- 新增功能列表
- 架構變化說明
- 使用方式
- 已知限制
- 下一步工作
- 測試結果

## Notes
- Some temp files in tests use `/tmp/sql5_*.db` pattern
- REPL prompt shows "sql5> "
- Version banner: "sql5 v0.1.0 — SQLite-compatible database with FTS"