# Metadata - 中繼資料理論

`src/catalog/meta.rs`

## 中繼資料 (Metadata) 概念

中繼資料是「關於資料的資料」：

```
資料：用戶 Alice
中繼資料：資料表名稱、欄位結構、建立時間...
```

## 自描述性 (Self-Describing)

SQLite 的特色是檔案本身包含其結構描述：

```sql
CREATE TABLE sqlite_master (
    type TEXT,      -- 'table', 'index', 'view', 'trigger'
    name TEXT,      -- 物件名稱
    tbl_name TEXT,  -- 關聯的表格名稱
    rootpage INT,   -- B+Tree 根頁面位址
    sql TEXT        -- CREATE 語句
);
```

## 中繼資料的層次

| 層次 | 內容 |
|------|------|
| 資料庫層 | 資料庫名稱、擁有者 |
| 表格層 | 表格名稱、列資訊 |
| 欄位層 | 欄位名稱、類型、預設值 |
| 索引層 | 索引欄位、唯一性 |
| 約束層 | 約束定義 |

## 系統目錄的設計模式

### 內部表法
將中繼資料儲存在普通資料表中（如 sqlite_master）。

### 專用結構法
使用專用的內部結構儲存中繼資料。

SQLite 兩者兼用：
- `sqlite_master` - 邏輯結構
- `sqlite_sequence` - 特殊用途

## 查詢優化中的中繼資料

中繼資料用於代價估算：

```sql
-- 假設有統計資訊
SELECT * FROM users WHERE age > 18;
-- 若均勻分佈，選擇率 ≈ 82%
```

## 自引用結構

允許表格引用自身：

```sql
CREATE TABLE org_chart (
    id INT,
    name TEXT,
    manager_id INT REFERENCES org_chart(id)
);
```

形成樹狀結構。

## 理論參考

- Stonebraker, "The Design and Implementation of PostgreSQL"
- SQLite Documentation: Schema Table
- Database System Concepts, Chapter 3