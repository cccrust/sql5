# sql5 v1.2 版本說明

## 版本資訊
- **版本**：1.2
- **日期**：2026-05-01
- **名稱**：Catalog 持久化

## 新增功能

### 1. Catalog 持久化 ✅
- Table schemas 在重啟後會恢復
- 使用 `SharedStorage` (Arc<Mutex<>>) 讓 Catalog 與 Tables 共享儲存
- 解決了 Rust Mutex deadlock 問題（釋放 lock 後再使用 storage）

### 2. Row 資料持久化 ✅
- Table 的 row 資料在重啟後會恢復
- 使用 Table::open 載入已存在的 root page
- 修復了 table root page 追蹤問題

### 3. MemoryStorage 改進
- 內部使用 `Arc<Mutex<MemoryInner>>` 實現 Cloneable
- 多個 B+Tree 可以安全共享同一個 MemoryStorage

### 4. DiskStorage 直接寫入
- 使用直接寫入主檔（`.sql5db`）而非 WAL
- 避免 macOS 上 WAL 檔案擴展問題

### 5. LRU 快取
- `LruCacheStorage<S>` 包裝任何 Storage 後端
- 預設容量 256 頁，可自訂

## 使用方式

```bash
# 啟動磁碟資料庫（catalog 會持久化）
./target/release/sql5 /tmp/mydb.db

# 建立表
CREATE TABLE users(id INT, name TEXT);

# 退出後重新開啟，表結構會恢復
./target/release/sql5 /tmp/mydb.db

# 驗證
.tables
```

## 架構

```
┌─────────────────────────────────────────┐
│            REPL Interface               │
└─────────────────┬───────────────────────┘
                  │
┌─────────────────▼───────────────────────┐
│          Executor + Planner             │
│  ┌─────────────────────────────────┐   │
│  │  SharedStorage (Arc<Mutex<>>)  │   │
│  │  - MemoryStorage (cloneable)   │   │
│  │  - DiskStorage (direct write)  │   │
│  └─────────────────────────────────┘   │
└─────────────────┬───────────────────────┘
                  │
    ┌─────────────┼─────────────┐
    ▼             ▼             ▼
┌────────┐  ┌────────┐  ┌────────────┐
│ B+Tree │  │Catalog │  │  Tables    │
│ (page 0)│ (page 1)│  │ (page 2+)   │
└────────┘  └────────┘  └────────────┘
```

## 已知限制

1. **WAL 已停用**：目前直接寫入主檔
   - 喪失交易 atomicity 保護
   - 未來可重新啟用 WAL（需修復 macOS 檔案擴展問題）

2. **LRU 快取**：已實作但預設未啟用

## 測試結果
- 173 個測試通過
- 1 個測試忽略（WAL transaction 相關）

## 下一步

1. **重新啟用 WAL**：修復 macOS 檔案擴展問題
2. **啟用 LRU 快取**：在 DiskStorage 上啟用
3. **外鍵約束**：FOREIGN KEY 驗證
4. **AUTOINCREMENT**：sqlite_sequence 追蹤