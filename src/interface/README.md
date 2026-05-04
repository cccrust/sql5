# Interface - 系統介面

`src/interface/`

## 模組結構

| 檔案 | 說明 |
|------|------|
| `mod.rs` | 模組入口 |
| `repl.rs` | REPL 互動模式 |
| `server.rs` | JSON RPC Server (stdio) |
| `websocket.rs` | WebSocket Server (v3.0) |

## REPL

互動式命令列解釋器：

```bash
./sql5
sql5> SELECT 1;
sql5> .tables
sql5> .schema my_table
sql5> .quit
```

### REPL 指令

| 指令 | 說明 |
|------|------|
| `.tables` | 列出所有表格 |
| `.schema` | 顯示表格結構 |
| `.indices` | 列出所有索引 |
| `.quit` | 離開 REPL |
| `.mode` | 設定輸出格式 |

## Server (JSON stdio)

基於 stdin/stdout 的 JSON RPC：

### Request

```json
{"method": "execute", "sql": "SELECT 1"}
```

### Response

```json
{"ok": true, "columns": ["1"], "rows": [[1]], "affected": 0}
```

## WebSocket Server (v3.0)

```bash
./sql5 --websocket 8080
```

### 連接方式

```python
from sql5 import connect
conn = connect("ws://localhost:8080", transport="websocket")
cursor = conn.cursor()
cursor.execute("SELECT 1")
print(cursor.fetchone())  # (1,)
```

### 協定

- 使用 tokio + tokio-tungstenite
- JSON 訊息格式與 stdio server 相同
- 支援多客戶端並發

### 錯誤回應

```json
{"ok": false, "error": "syntax error"}
```

## 測試

```bash
cargo test interface
```