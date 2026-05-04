# WebSocket - 雙向通訊理論

`src/interface/websocket.rs`

## 為何需要 WebSocket？

HTTP 是「請求-回應」模式，伺服器無法主動推送：

```
HTTP:  Client →→→ Request →→→ Server
             ←←← Response ←←←

問題：伺服器無法主動通知客戶端
```

## WebSocket 解決方案

在 TCP 之上建立持久雙向連接：

```
HTTP 握手 → WebSocket 連接 → 雙向訊息
```

## 握手過程

### 客戶端請求
```
GET / HTTP/1.1
Upgrade: websocket
Connection: Upgrade
Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==
```

### 伺服器回應
```
HTTP/1.1 101 Switching Protocols
Upgrade: websocket
Connection: Upgrade
Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYG3hZbA==
```

## 訊訊框結構

```
 +-+-+-+-+-------+-+-------------+-------------------------------+
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-------------+-------------+-----------------------------------+
|F|R|R|R|  opcode   |M| Payload len |    Extended payload length    |
|I|S|S|S|  (4bits)  |A|   (7bits)  |             (16/64bits)        |
|N|V|V|V|             |S|             |                                 |
| |1|2|3|             |K|             |                                 |
+-+-------------+-------------+-----------------------------------+
```

|欄位|說明|
|------|---|
| opcode | 0x0=continuation, 0x1=text, 0x2=binary |
| MASK | 客戶端→伺服器時為 1 |
| Payload len | 資料長度 |

## 事件驅動架構

```
┌────────────┐
│  Acceptor  │ 接收新連線
└─────┬──────┘
      │
┌─────▼──────┐
│   Dispatch  │ 分發到 handler
└─────┬──────┘
      │
┌─────▼──────┐
│   Handler   │ 處理訊息
└────────────┘
```

## 回到連線 (Backpressure)

當客戶端發送太快時：
- 、作業系統緩衝區滿
- 應用層需實現背壓控制

## 理論參考

- RFC 6455: The WebSocket Protocol
- Tanenbaum, "Distributed Systems" - 網路層次
- Nielsen, "Event-Driven Architecture"