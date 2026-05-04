# AGENTS.md - sql5 開發規範

## 專案資訊
- **專案**: sql5 v2.0.0 (SQLite 相容資料庫，含 CJK FTS5 全文檢索)
- **Edition**: Rust 2024 (需要 nightly 或近期 stable)
- **架構**: Client-Server 模式 (Python client + Rust server)

## 快速開始

```bash
cd /Users/Shared/ccc/project/sql5

# 編譯
cargo build --release

# 執行全部測試 (推薦)
./test.sh
```

## 常用指令

| 指令 | 用途 |
|------|------|
| `cargo build` | 編譯 debug 版 (target/debug/sql5) |
| `cargo build --release` | 編譯 release 版 (target/release/sql5) |
| `cargo check` | 型別檢查 (不編譯) |
| `cargo test` | 執行所有單元測試 (205 tests) |
| `./test.sh` | **執行全部測試** (Rust + CLI + Python) |
| `./rutest.sh` | CLI 整合測試 (113 tests) |
| `./pytest.sh` | Python client 整合測試 |
| `./pub.sh <version> pypi` | 上傳到 PyPI（自動更新版本號） |
| `./pub.sh <version> github` | 建立 GitHub tag 觸發 CI 發布（自動更新版本號） |

## 測試說明

### test.sh (全部測試)
執行四個階段：
1. **Build** - 編譯 Rust release binary
2. **Rust unit tests** - `cargo test` (205 tests)
3. **CLI integration tests** - `./rutest.sh` (113 tests)
4. **Python pytest** - pytest (26 tests, 5 skipped)

### 測試結果
```
[PASS] Rust unit tests (cargo test)      — 205 passed
[PASS] CLI integration tests (rutest.sh) — 113 passed
[PASS] Python pytest tests               — 26 passed, 5 skipped
```

## 專案架構

```
src/
├── main.rs              # CLI + server 入口
├── parser/              # SQL 解析與 AST
│   ├── lexer.rs
│   ├── parser.rs
│   └── ast.rs
├── planner/             # 查詢規劃與執行
│   ├── planner.rs
│   ├── executor.rs
│   ├── plan.rs
│   ├── transaction.rs
│   └── datetime.rs
├── btree/               # B+Tree 索引實作
├── table/               # 表格管理
├── pager/               # 儲存引擎 (含 WAL)
├── catalog/             # 系統目錄
├── fts/                 # FTS5 全文檢索
│   ├── fts_table.rs
│   ├── tokenizer.rs     # CJK 分詞
│   └── index.rs
└── interface/
    ├── repl.rs          # REPL 模式
    └── server.rs        # Server 模式 (JSON stdio)
```

## Python Client

```
sql5_pypi/
├── sql5/
│   ├── __init__.py
│   ├── client.py        # Python DB-API client
│   ├── _binary.py       # Binary 下載
│   └── __main__.py
├── tests/
│   └── test_sql5.py     # pytest 測試 (31 tests)
└── dist/                # PyPI 發布目錄
```

### 執行 Python 測試
```bash
cd sql5_pypi
export SQL5_BINARY=../../target/release/sql5
python -m pytest tests/test_sql5.py -v
```

## v2.0 Client-Server Protocol

Server 透過 stdin/stdout 接收/輸出 JSON：

```json
// Request
{"method": "execute", "sql": "SELECT 1"}

// Response
{"ok": true, "columns": ["1"], "rows": [[1]], "affected": 0}
```

## REPL 用法

```bash
./target/release/sql5          # In-memory REPL
./target/release/sql5 my.db    # 開啟資料庫檔案

sql5> SELECT 1
sql5> .tables
sql5> .schema my_table
sql5> .quit
```

## 程式碼品質規範

依據 `ccc_code_skill.md`:

1. **模組化**: 程式超過 1000 行需強制分模組 (src/ 下 26 個模組)
2. **零警告**: 提交前確保 `cargo build` 零警告
3. **版本文件**: 每個重要功能完成後需在 `_doc/vX.X.md` 記錄

## 版本文件

每次發布新版本需在 `_doc/vX.X.md` 建立文件，參考現有範例 (v1.1–v2.0)。

## 檔案結構

```
sql5/
├── src/                  # Rust 原始碼 (26 modules)
├── tests/                # Integration tests
├── sql5_pypi/            # Python package
│   ├── sql5/             # Python module
│   ├── tests/            # pytest tests
│   └── _bak/             # 備份檔案
├── _doc/                 # 版本文件
├── _bak/                 # 備份 (test.sh, test.py, server.sh)
├── Cargo.toml
├── test.sh               # 全部測試 (主要指令)
├── rutest.sh             # CLI 整合測試
├── pytest.sh             # Python pytest
├── pub.sh                # 發布腳本 (pypi/github)
```

## 備份檔案 (_bak/)

已移至 _bak 的過時檔案：
- `server.sh` - 舊的簡單測試
- `test.sh` - 已被新的 test.sh 取代
- `test.py` - 已被 pytest 取代
- `sql5_pypi/test_sql5_server.py` - 已被 pytest 取代
- `sql5_pypi/test_server.sh` - 已被 pytest 取代