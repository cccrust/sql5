# Catalog - 系統目錄

`src/catalog/`

## 模組結構

| 檔案 | 說明 |
|------|------|
| `mod.rs` | 模組入口 |
| `catalog.rs` | 目錄管理 |
| `meta.rs` | 中繼資料定義 |

## Catalog

```rust
pub struct Catalog {
    tables: HashMap<String, TableSchema>,
    indices: HashMap<String, IndexDef>,
}
```

### 主要方法

```rust
impl Catalog {
    pub fn new() -> Self;
    pub fn get_table(&self, name: &str) -> Option<&TableSchema>;
    pub fn add_table(&mut self, schema: TableSchema);
    pub fn drop_table(&mut self, name: &str);
    pub fn get_index(&self, name: &str) -> Option<&IndexDef>;
    pub fn add_index(&mut self, idx: IndexDef);
    pub fn drop_index(&mut self, name: &str);
}
```

## TableSchema 表格結構

```rust
pub struct TableSchema {
    pub name: String,
    pub columns: Vec<ColumnDef>,
    pub rowid_column: Option<String>,
    pub foreign_keys: Vec<ForeignKey>,
    pub constraints: Vec<TableConstraint>,
}
```

## ColumnDef 欄位定義

```rust
pub struct ColumnDef {
    pub name: String,
    pub data_type: SqlType,
    pub not_null: bool,
    pub primary_key: bool,
    pub default: Option<Expr>,
    pub autoincrement: bool,
}
```

## IndexDef 索引定義

```rust
pub struct IndexDef {
    pub name: String,
    pub table: String,
    pub columns: Vec<String>,
    pub unique: bool,
}
```

## 系統目錄表

SQLite 風格的系統表：

| 表名 | 說明 |
|------|------|
| `sqlite_master` | 主目錄表 |
| `sqlite_schema` | 結構描述資訊 |
| `sqlite_sequence` | AUTOINCREMENT 計數器 |

## 測試

```bash
cargo test catalog
```