# sql5 v1.1 版本說明

## 版本資訊
- **版本**：1.1
- **日期**：2026-05-01
- **名稱**：磁碟儲存 + WAL + LRU 快取

## 新增功能

### 1. DiskStorage 磁碟儲存
- 使用 `SharedStorage` 包裝類型（`Arc<Mutex<Box<dyn Storage>>>`）
- 支援記憶體資料庫（預設）和磁碟資料庫
- 啟動方式：`./target/debug/sql5 [db_path]`

### 2. WAL 預寫日誌
- 寫入時先寫入 WAL 檔案（`.sql5wal`）
- 關閉資料庫時執行 checkpoint，将 WAL 內容寫入主檔（`.sql5db`）

### 3. ROLLBACK 修復
- 交易中新建的表會在 ROLLBACK 時刪除
- snapshot 中的表會截斷多餘資料

### 4. 日期時間函式
- 完全可用：`date()`, `datetime()`, `julianday()`, `strftime()`
- 支援 modifier：`'+5 days'`, `'+1 month'`, `'start of month'` 等

### 5. LRU 快取
- `LruCacheStorage<S>` 包裝任何 Storage 後端
- 預設容量 256 頁，可自訂
- 追蹤 hits/misses 命中率統計
- write-through 策略：寫入時同步更新快取

## 使用方式

```bash
# 啟動記憶體資料庫（預設）
./target/debug/sql5

# 啟動磁碟資料庫
./target/debug/sql5 /tmp/mydb.db

# 測試
./test.sh
```

## 架構

```
┌─────────────────────────────────────────┐
│            REPL Interface               │
│         (src/interface/)                │
└─────────────────┬───────────────────────┘
                  │
┌─────────────────▼───────────────────────┐
│          Executor + Planner             │
│         (src/planner/)                  │
│  ┌─────────────────────────────────┐   │
│  │  SharedStorage (Arc<Mutex<>>)  │   │
│  └─────────────────────────────────┘   │
└─────────────────┬───────────────────────┘
                  │
    ┌─────────────┼─────────────┐
    ▼             ▼             ▼
┌────────┐  ┌────────┐  ┌────────────┐
│ B+Tree │  │Catalog │  │  Tables    │
│(src/btree)│(src/catalog)│(src/table)│
└────┬────┘  └────┬────┘  └─────┬──────┘
     │           │             │
     └───────────┴─────────────┘
                  ▼
┌─────────────────────────────────────────┐
│        SharedStorage (DynStorage)       │
│  ┌───────────────────────────────────┐  │
│  │  Box<dyn Storage> (Memory/Disk)  │  │
│  └───────────────────────────────────┘  │
│  ┌───────────────────────────────────┐  │
│  │  WAL (Write-Ahead Log)           │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
```

## 已知限制

1. **Catalog 未持久化**：重啟資料庫後，table schema 不會恢復
   - 資料有寫入磁碟（checkpoint 成功）
   - 但 table metadata（在 Catalog 中）沒有儲存/載入機制

2. **LRU 快取**：已實作但預設未啟用

## 測試結果
- 174 個測試全部通過
- 日期時間函式測試：4 個
- Transaction 測試：2 個（BEGIN/COMMIT, ROLLBACK）
- LRU 快取測試：4 個

## 下一步

1. **Catalog 持久化**：儲存/載入 table metadata
2. **啟用 LRU 快取**：在 DiskStorage 上啟用 LRU 快取
3. **WAL 自動 checkpoint**：超過閾值自動 checkpoint
4. **外鍵約束**：FOREIGN KEY 驗證
5. **AUTOINCREMENT**：sqlite_sequence 追蹤