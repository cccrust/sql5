# Schema - 結構描述理論

`src/table/schema.rs`

## 結構描述 (Schema) 概念

Schema 是資料庫的邏輯結構定義，描述：
- 表格結構
- 欄位類型
- 約束條件

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE
);
```

## 類型系統 (Type System)

### 強類型 vs 弱類型

| 特性 | 強類型 | 弱類型 |
|------|--------|--------|
| 隱式轉換 | 不允許 | 允許 |
| 安全性 | 高 | 低 |
| 彈性 | 低 | 高 |

SQL 是弱類型語言，允許隱式轉換。

### 靜態 vs 動態類型

SQLite 是動態類型：
- 欄位可儲存任意類型
- 類型跟值走（manifest typing）

## 約束的理論基礎

### 鍵約束 (Key Constraint)

**超鍵 (Superkey)**：能唯一識別元組的属性集合
**候選鍵 (Candidate Key)**：最小的超鍵
**主鍵 (Primary Key)**：選定的候選鍵

```
{SSN}            → 超鍵（也是候選鍵）
{SSN, Name}      → 超鍵（非候選鍵）
```

### 函數依賴 (Functional Dependency)

```
A → B (A 函數決定 B)
```

若 A → B 且 B 不能由 A 的真子集決定，則 A 是候選鍵。

### 參照完整性 (Referential Integrity)

外鍵約束确保子表的值存在於父表：

```sql
FOREIGN KEY (dept_id) REFERENCES departments(id)
```

## 預設值 (Default)

DEFAULT 約束的理論意義：
- 封裝建立邏輯
- 支援封闭世界假設

## 理論參考

- Codd, "The Relational Model"
- Abiteboul, Hull, Vianu, "Foundations of Databases"
- Database System Concepts, Chapter 3