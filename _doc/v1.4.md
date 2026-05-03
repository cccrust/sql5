# sql5 v1.4 版本說明

## 版本資訊
- **版本**：1.4
- **日期**：2026-05-03
- **名稱**：WAL 交易 atomicity 恢復

## 新增功能

### 1. WAL 寫入恢復 ✅
- 恢復 `DiskStorage::write_node` 經過 WAL 寫入
- 每次寫入前先將頁面內容寫入 WAL
- 確保崩潰後可恢復

### 2. WAL Rollback 實作 ✅
- 實作 `rollback_txn` 機制
- 交易回滾時丟棄 dirty pages，恢復原始頁面
- 恢復 `#[ignore]` 的 `disk_rollback` 測試

### 3. WAL Checkpoint ✅
- `flush()` 時自動 checkpoint
- 將已提交頁面寫回主檔並截斷 WAL

### 4. PRAGMA journal_mode ✅
- `PRAGMA journal_mode` 回傳 `wal`（WAL 模式啟用）
- `DiskStorage::is_wal()` 回傳 `true`

## 使用方式

```bash
# 啟動磁碟資料庫（WAL 預設啟用）
./target/release/sql5 /tmp/mydb.db

# 測試交易 atomicity
BEGIN;
INSERT INTO users VALUES (1, 'Alice');
COMMIT;
SELECT * FROM users;  -- 應該顯示 Alice

# 手動 checkpoint
PRAGMA wal_checkpoint;

# 檢查 WAL 狀態
PRAGMA journal_mode;  -- 回傳 wal
PRAGMA page_size;    -- 回傳 4096
```

## 架構

```
┌─────────────────────────────────────────┐
│            REPL Interface               │
└─────────────────┬───────────────────────┘
                   │
┌─────────────────▼───────────────────────┐
│          Executor + Planner              │
│  ┌─────────────────────────────────┐   │
│  │  SharedStorage (Arc<Mutex<>>)  │   │
│  └─────────────────────────────────┘   │
└─────────────────┬───────────────────────┘
                   │
┌─────────────────▼───────────────────────┐
│           DiskStorage                   │
│  ┌─────────────────────────────────┐   │
│  │  WAL (Write-Ahead Log)         │   │ ← 完整功能
│  │  - write_page (交易寫入)        │   │
│  │  - commit (寫入並標記提交)       │   │
│  │  - rollback (恢復原始)          │   │
│  │  - checkpoint (刷回主檔)        │   │
│  │  - pre_image (原始頁面追蹤)     │   │
│  └─────────────────────────────────┘   │
└─────────────────┬───────────────────────┘
                   │
         ┌─────────┴─────────┐
         ▼                   ▼
   ┌──────────┐      ┌──────────┐
   │ Main DB  │      │ WAL File │
   │(*.sql5db)│      │(*.sql5wal)│
   └──────────┘      └──────────┘
```

## 已知限制

1. **B+Tree 快取未清除** - UPDATE 同一列後 ROLLBACK，B+Tree 可能仍返回修改後的值
   - 原因：B+Tree 在記憶體中快取頁面，ROLLBACK 只清除 WAL dirty/comitted，不清除 B+Tree 快取
   - 影響：使用同一連線執行 UPDATE + ROLLBACK 後，SELECT 可能仍看到修改後的值
   - 解決方式：使用不同的連線或重新開啟資料庫
   - 或使用 INSERT/DELETE 而非 UPDATE 同一列

2. **VIEWs 尚未支援** - `CREATE VIEW` / `DROP VIEW`
3. **TRIGGERs 尚未支援** - 觸發器
4. **ATTACH/DETACH** - 多資料庫尚未支援
5. **VACUUM** - 資料庫壓縮尚未支援

## 測試結果
- **Cargo 單元測試**：205 個測試通過
- **CLI 系統測試**：82 個測試通過

## 對應 SQLite 相容性

| SQLite 功能 | sql5 v1.4 狀態 |
|------------|---------------|
| DDL (CREATE/DROP TABLE) | ✅ 完成 |
| DML (INSERT/UPDATE/DELETE) | ✅ 完成 |
| SELECT with WHERE, JOIN | ✅ 完成 |
| Aggregate (COUNT/SUM/AVG/MIN/MAX) | ✅ 完成 |
| Transactions | ✅ 完成 |
| AUTOINCREMENT | ✅ 完成 |
| FOREIGN KEY | ✅ 完成 |
| FTS5 (CJK) | ✅ 完成 |
| CREATE/DROP INDEX | ✅ 完成 (v1.3) |
| PRAGMA | ✅ 完成 (v1.3) |
| ALTER TABLE | ✅ 完成 (v1.3) |
| EXPLAIN | ✅ 完成 (v1.3) |
| **WAL Mode** | ✅ 恢復 (v1.4) |
| VIEWs | ❌ 待支援 |
| TRIGGERs | ❌ 待支援 |
| ATTACH | ❌ 待支援 |
| VACUUM | ❌ 待支援 |