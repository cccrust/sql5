# Client-Server Architecture - 用戶端/伺服器架構理論

`src/interface/server.rs`, `src/interface/websocket.rs`

## 為何需要 Client-Server？

| 模式 | 優點 | 缺點 |
|------|------|------|
| 嵌入式 (in-process) | 簡單，低延遲 | 只能單一連線 |
| Client-Server | 支援多用戶 | 網路延遲 |
| Shared nothing | 高擴展性 | 複雜度最高 |

## 程序間通訊 (IPC)

本專案兩種 IPC 機制：

### 標準輸入/輸出 (stdio)
```
Python → [JSON over stdin] → Rust Server
       ← [JSON over stdout] ←
```

適合單一客戶端，本機通訊。

### WebSocket
```
Python ←→ [TCP + WebSocket] ←→ Rust Server
                    ↑
              多客戶端支援
```

適合網路/跨程序，多客戶端。

## WebSocket 協定

建立在 TCP 之上的雙向通訊：

```
HTTP 升級請求 → WebSocket 連接 → 雙向訊息
```

優點：
- 持久連接
- 伺服器可主動推送
- 低協定負載

## 連線池 (Connection Pool)

管理資料庫連線：

```
應用程式 → [連線池] → 實際連線
           ↓
     [conn1] [conn2] [conn3]
```

好處：
- 減少連線建立開銷
- 控制併發數

## 理論參考

- Tanenbaum, "Distributed Systems"
- RFC 6455: The WebSocket Protocol
- Bernstein & Schenk, "Client-Server Architecture"