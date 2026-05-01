# sql5 v1.2 版本說明

## 版本資訊
- **版本**：1.2
- **日期**：2026-05-01
- **名稱**：Catalog 持久化

## 新增功能

### 1. Catalog 持久化 ✅
- Table schemas 在重啟後會恢復
- 使用 `SharedStorage` (Arc<Mutex<>>) 讓 Catalog 與 Tables 共享儲存
- 解決了 Rust Mutex deadlock 問題

### 2. Row 資料持久化 ✅
- Table 的 row 資料在重啟後會恢復
- 使用 Table::open 載入已存在的 root page
- 修復了 table root page 追蹤問題

### 3. MemoryStorage 改進
- 內部使用 `Arc<Mutex<MemoryInner>>` 實現 Cloneable

### 4. DiskStorage 直接寫入
- 使用直接寫入主檔（`.sql5db`）

### 5. LRU 快取已啟用 ✅
- `SharedStorage::disk_with_cache(path, capacity)` 啟用 LRU
- 預設容量 256 頁

### 6. AUTOINCREMENT 語法支援 ⚠️
- 解析器已支援 `INTEGER PRIMARY KEY AUTOINCREMENT`
- Schema 已儲存 autoinc 標記
- 自動 ID 生成邏輯尚有問題，待修復

### 7. FOREIGN KEY 驗證 ⚠️
- 約束檢查邏輯已實作（check_row 已呼叫）
- 驗證時檢查父表是否存在對應記錄
- 解析器尚未支援 `REFERENCES` 關鍵字

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

1. **WAL 已停用**：目前直接寫入主檔，喪失交易 atomicity 保護

2. **FOREIGN KEY**：已解析但未實作驗證邏輯

3. **AUTOINCREMENT**：尚未實作

## 測試結果
- 173 個測試通過
- 1 個測試忽略

## 下一步

1. **重新啟用 WAL**：修復 macOS 檔案擴展問題
2. **實作 FOREIGN KEY 驗證**：ON DELETE / ON UPDATE
3. **修復 AUTOINCREMENT**：自動 ID 生成邏輯