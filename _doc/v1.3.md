# sql5 v1.3 版本說明

## 版本資訊
- **版本**：1.3
- **日期**：2026-05-03
- **名稱**：SQLite 相容性強化

## 新增功能

### 1. CREATE INDEX / DROP INDEX ✅
- 完整支援 `CREATE INDEX` 語法
- 支援 `CREATE UNIQUE INDEX`
- 支援 `DROP INDEX`
- Index 可被查詢規劃器使用

### 2. PRAGMA 語句支援 ✅
- `PRAGMA journal_mode` - 查詢/設定 journal 模式
- `PRAGMA cache_size` - 查詢/設定快取大小
- `PRAGMA page_size` - 查詢 page size
- `PRAGMA freelist_count` - 查詢空閒頁面數

### 3. ALTER TABLE 支援 ✅
- `ALTER TABLE ... RENAME TO` - 重新命名表
- `ALTER TABLE ... ADD COLUMN` - 新增欄位

### 4. EXPLAIN 命令 ✅
- `EXPLAIN SELECT/UPDATE/DELETE/INSERT`
- 顯示查詢執行計畫

### 5. 增強 dot commands ✅
- `.indices` - 列出所有索引
- `.databases` - 列出附連的資料庫
- `.trace` - 追蹤 SQL 執行

## 使用方式

```bash
# 啟動磁碟資料庫
./target/release/sql5 /tmp/mydb.db

# 建立索引
CREATE INDEX idx_users_name ON users(name);

# 查詢執行計畫
EXPLAIN SELECT * FROM users WHERE name = 'Alice';

# PRAGMA 查詢
PRAGMA journal_mode;
PRAGMA cache_size;

# 重新命名表
ALTER TABLE users RENAME TO users_old;

# 新增欄位
ALTER TABLE users ADD COLUMN email TEXT;

# 離開
.quit
```

## 架構

```
┌─────────────────────────────────────────┐
│            REPL Interface               │
│         (.indices / .trace)             │
└─────────────────┬───────────────────────┘
                   │
┌─────────────────▼───────────────────────┐
│          Executor + Planner              │
│  ┌─────────────────────────────────┐   │
│  │  SharedStorage (Arc<Mutex<>>)  │   │
│  │  - MemoryStorage (cloneable)   │   │
│  │  - DiskStorage (direct write)  │   │
│  │  - LRU Cache (256 pages)       │   │
│  └─────────────────────────────────┘   │
└─────────────────┬───────────────────────┘
                   │
     ┌─────────────┼─────────────┐
     ▼             ▼             ▼
┌────────┐  ┌────────┐  ┌────────────┐
│ B+Tree │  │Catalog │  │  Tables    │
│(page 0)│ (page 1)│  │ (page 2+)   │
└────────┘  └────────┘  └────────────┘
                   │
                   ▼
          ┌───────────────┐
          │ Indexes (new) │
          └───────────────┘
```

## 已知限制

1. **VIEWs 尚未支援** - `CREATE VIEW` / `DROP VIEW`
2. **TRIGGERs 尚未支援** - 觸發器
3. **ATTACH/DETACH** - 多資料庫尚未支援
4. **VACUUM** - 資料庫壓縮尚未支援
5. **REINDEX** - 尚未支援
6. **ANALYZE** - 統計資訊收集尚未支援

## 下一步

1. **VIEWs 支援** - 虛擬表
2. **TRIGGERs 支援** - 觸發器
3. **ATTACH DATABASE** - 多資料庫
4. **VACUUM** - 資料庫壓縮
5. **REINDEX / ANALYZE** - 索引維護

## 測試結果
- **Cargo 單元測試**：189 個測試通過
- **CLI 系統測試**：79 個測試通過

## 對應 SQLite 相容性

| SQLite 功能 | sql5 v1.3 狀態 |
|------------|---------------|
| DDL (CREATE/DROP TABLE) | ✅ 完成 |
| DML (INSERT/UPDATE/DELETE) | ✅ 完成 |
| SELECT with WHERE, JOIN | ✅ 完成 |
| Aggregate (COUNT/SUM/AVG/MIN/MAX) | ✅ 完成 |
| Transactions | ✅ 完成 |
| AUTOINCREMENT | ✅ 完成 |
| FOREIGN KEY | ✅ 完成 |
| FTS5 (CJK) | ✅ 完成 |
| **CREATE/DROP INDEX** | ✅ 新增 |
| **PRAGMA** | ✅ 新增 |
| **ALTER TABLE** | ✅ 新增 |
| **EXPLAIN** | ✅ 新增 |
| VIEWs | ❌ 待支援 |
| TRIGGERs | ❌ 待支援 |
| ATTACH | ❌ 待支援 |
| VACUUM | ❌ 待支援 |