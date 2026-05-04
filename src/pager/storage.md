# Storage - 儲存層理論

`src/pager/storage.rs`

## 儲存層角色

儲存層是資料庫與作業系統檔案之間的介面，負責：
- 分頁管理
- 緩衝區管理
- I/O 调度

```
SQL 查詢
    ↓
Storage 介面
    ↓
檔案系統/作業系統緩衝
    ↓
實體磁碟
```

## 分頁概念

### 為何使用分頁？

| 因素 | 說明 |
|------|------|
| 磁碟 I/O 單位 | OS 以區塊為單位讀寫 |
| 記憶體管理 | 分頁是虛擬記憶體單位 |
| 空間局部性 | 相關資料應在同一分頁 |

### 分頁大小選擇

- **太小**：I/O 次數增加
- **太大**：記憶體浪費，緩衝區效率降低
- 典型值：4KB - 16KB

本專案預設 4096 位元組。

## Buffer Pool

記憶體中的分頁緩衝區：

```
請求分頁 → Buffer Pool 有？→ 是 → 返回
                  ↓ 否
              讀取磁碟 → 返回並快取
```

## 置換策略

當緩衝區滿時需要置換：

| 策略 | 說明 |
|------|------|
| LRU | 最久未使用置換 |
| Clock | LRU 的近似實現 |
| LRU-K | 最近 K 次引用時間 |
| ARC | 自適應置換快取 |

## 預寫式日誌 (WAL)

Write-Ahead Logging 確保原子性：

```
T1: BEGIN
T1: 寫 WAL（記憶體）
T1: 修改資料頁
T1: WAL 刷到磁碟
T1: COMMIT
T1: 資料頁刷到磁碟
```

## 檢查點 (Checkpoint)

定時將髒頁刷到磁碟：

1. 寫入檢查點記號到 WAL
2. 刷寫所有髒頁
3. 截斷 WAL

## 理論參考

- Database System Concepts, Chapter 13: Storage and Indexing
- Operating System Concepts, Chapter 9: Virtual Memory
- Gray & Reuter, "Transaction Processing: Concepts and Techniques"