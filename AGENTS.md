# AGENTS.md - sql5 開發規範

## 專案資訊
- **版本**: v3.7.1 (SQLite 相容資料庫，含 CJK FTS5 全文檢索)
- **語言**: Rust 2024
- **架構**: Python client + Rust server (CLI/stdio/websocket)

## 常用指令

| 指令 | 用途 |
|------|------|
| `cargo build --release` | 編譯 release (target/release/sql5) |
| `cargo test` | Rust 單元測試 |
| `./test.sh` | 全部測試 (Rust + CLI + Python) |
| `./shtest.sh` | CLI 整合測試 (114 tests) |
| `./pub.sh <ver> pypi` | 上傳 PyPI |
| `./pub.sh <ver> github` | GitHub tag 觸發 CI |

## 測試

### Python 測試 (在 sql5_pypi/ 下執行)
```bash
cd sql5_pypi
export SQL5_BINARY=../../target/release/sql5
python -m pytest tests/ -v
```
**注意**: 必須設定 `SQL5_BINARY` 環境變數指向 release binary。

### 單一 Rust 測試
```bash
cargo test <test_name>
```

## 專案架構

| 目錄 | 說明 |
|------|------|
| `src/main.rs` | CLI + server 入口 |
| `src/parser/` | SQL 解析與 AST |
| `src/planner/` | 查詢規劃與執行 |
| `src/pager/` | 儲存引擎 (含 WAL) |
| `src/btree/` | B+Tree 索引 |
| `src/fts/` | FTS5 全文檢索 (CJK 分詞) |
| `src/interface/` | REPL + stdio server + WebSocket |
| `sql5_pypi/` | Python client 包 |

## Server 模式

| 模式 | 命令 |
|------|------|
| REPL | `./sql5` 或 `./sql5 db.db` |
| stdio server | `./sql5 --server [db]` (JSON stdio) |
| WebSocket | `./sql5 --websocket <port> [db]` |

## Client-Server Protocol (stdio)

Request:
```json
{"method": "execute", "sql": "SELECT 1"}
```

Response:
```json
{"ok": true, "columns": ["1"], "rows": [[1]], "affected": 0, "lastrowid": null}
```

## 品質規範

1. **零警告**: `cargo build` 需零警告
2. **版本文件**: 每次發布在 `_doc/vX.X.md` 記錄