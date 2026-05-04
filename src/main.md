# Main - CLI 入口

`src/main.rs`

## 功能

命令列介面主入口，處理三種執行模式：

| 模式 | 說明 |
|------|------|
| REPL | 互動式命令列 (無參數) |
| stdio server | JSON RPC (--server) |
| WebSocket server | 多客戶端支援 (--websocket) |

## 命令列參數

```bash
sql5 [OPTIONS] [database]
```

### Options

| 參數 | 說明 |
|------|------|
| `-s, --server [db]` | 啟動 stdio server 模式 |
| `-w, --websocket <port> [db]` | 啟動 WebSocket server |
| `-v, --version` | 顯示版本 |
| `-h, --help` | 顯示幫助 |

## 環境變數

| 變數 | 說明 |
|------|------|
| `SQL5_BINARY` | Python client 使用的二進位檔路徑 |

## 流程

```text
main()
  ├─ parse_args()
  ├─ setup_logging()
  ├─ select_mode()
  │   ├─ REPL mode → run_repl()
  │   ├─ Server mode → run_server()
  │   └─ WebSocket mode → run_websocket()
  └─ cleanup()
```

## 測試

```bash
cargo test --test main
```