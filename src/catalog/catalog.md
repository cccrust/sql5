# Catalog - 系統目錄理論

`src/catalog/`

## 系統目錄概念

系統目錄（System Catalog）是描述資料庫結構的中繼資料儲存。

```sql
-- 使用者看到的表格
CREATE TABLE users (id INT, name TEXT);

-- 系統儲存的描述
INSERT INTO sqlite_master VALUES ('table', 'users', 'users', 5, 'CREATE TABLE...');
```

## 典型的系統表

| 表名 | 描述 |
|------|------|
| `sqlite_master` | 主目錄表 |
| `sqlite_schema` | 結構描述 |
| `sqlite_sequence` | AUTOINCREMENT 計數器 |
| `sqlite_stat1` | 統計資訊 |

## SQLite 的 Catalog 設計

```sql
CREATE TABLE sqlite_master (
    type TEXT,      -- 'table', 'index', 'view', 'trigger'
    name TEXT,      -- 物件名稱
    tbl_name TEXT,  -- 關聯的表格名稱
    rootpage INT,   -- B+Tree 根頁面
    sql TEXT        -- 建立語句
);
```

## 結構描述 (Schema)

Schema 是資料庫的完整結構定義：

```sql
CREATE TABLE t (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    age INTEGER DEFAULT 18,
    FOREIGN KEY (manager_id) REFERENCES t(id)
);
```

## 中繼資料的存取

```sql
-- 查詢所有表格
SELECT name FROM sqlite_master WHERE type='table';

-- 查詢表格結構
PRAGMA table_info(users);

-- 查詢索引
PRAGMA index_list(users);
```

## 自引用外鍵

允許表格引用自身：
```sql
CREATE TABLE org_chart (
    id INT PRIMARY KEY,
    name TEXT,
    manager_id INT REFERENCES org_chart(id)
);
```

## 理論參考

- Database System Concepts, Chapter 3
- SQLite Documentation: Schema Table