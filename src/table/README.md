# Table - 表格管理

`src/table/`

## 模組結構

| 檔案 | 說明 |
|------|------|
| `mod.rs` | 模組入口 |
| `table.rs` | 表格操作 |
| `row.rs` | 列資料結構 |
| `schema.rs` | 表格結構定義 |
| `serialize.rs` | 序列化/反序列化 |

## TableHandle

```rust
pub struct TableHandle {
    pub name: String,
    pub page_id: PageId,
}
```

## 主要方法

```rust
impl Table {
    pub fn open(name: &str, catalog: &Catalog) -> Result<Option<Self>>;
    pub fn insert(&mut self, row: Row) -> Result<()>;
    pub fn update(&mut self, row: Row) -> Result<()>;
    pub fn delete(&mut self, rowid: i64) -> Result<()>;
    pub fn scan(&self) -> Result<TableScan>;
}
```

## TableScan

```rust
pub struct TableScan {
    table: TableHandle,
    cursor: BTreeCursor,
}
```

## Row 列結構

```rust
pub struct Row {
    pub rowid: i64,
    pub values: Vec<Value>,
}
```

## Schema 表格結構

```rust
pub struct TableSchema {
    pub name: String,
    pub columns: Vec<ColumnDef>,
    pub indices: Vec<IndexDef>,
    pub rowid_col: Option<String>,
}
```

## 測試

```bash
cargo test table
```