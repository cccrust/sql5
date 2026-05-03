//! Catalog：資料庫的「資料字典」
//!
//! 負責：
//!   - 記錄所有 Table 的 schema 與根頁號
//!   - 提供 create_table / drop_table / get_table
//!   - 自身也透過 B+Tree 持久化（系統表 `__catalog__`）
//!
//! 使用範例：
//! ```rust
//! use sql5::catalog::Catalog;
//! use sql5::table::schema::{Column, DataType, Schema};
//! use sql5::pager::MemoryStorage;
//!
//! let mut catalog = Catalog::new(MemoryStorage::new());
//! let schema = Schema::new(vec![
//!     Column::new("id",   DataType::Integer),
//!     Column::new("name", DataType::Text),
//! ]);
//! catalog.create_table("users", schema).unwrap();
//! assert!(catalog.get_table("users").is_some());
//! ```

use std::collections::HashMap;

use crate::btree::node::Key;
use crate::btree::tree::BPlusTree;
use crate::pager::storage::Storage;
use crate::table::schema::Schema;

use super::meta::{decode_meta, encode_meta, TableMeta, IndexMeta};
use crate::table::schema::Column;

pub struct Catalog<S: Storage> {
    /// 系統表：以表名為 key，儲存 TableMeta 的序列化結果
    sys_tree: BPlusTree<S>,
    /// 記憶體快取，避免每次都反序列化
    cache: HashMap<String, TableMeta>,
    /// 索引快取
    index_cache: HashMap<String, IndexMeta>,
}

impl<S: Storage> Catalog<S> {
    // ------------------------------------------------------------------ //
    //  建構                                                                //
    // ------------------------------------------------------------------ //

    /// 建立全新的 Catalog（全新資料庫）
    pub fn new(storage: S) -> Self {
        let sys_tree = BPlusTree::new(64, storage);
        Catalog { sys_tree, cache: HashMap::new(), index_cache: HashMap::new() }
    }

    /// 開啟已有的 Catalog（從磁碟重新載入）
    pub fn open(storage: S, root_page: usize) -> Self {
        let sys_tree = BPlusTree::open(64, storage, root_page, 0);
        let mut catalog = Catalog { sys_tree, cache: HashMap::new(), index_cache: HashMap::new() };
        catalog.load_all();
        catalog
    }

    // ------------------------------------------------------------------ //
    //  公開 API                                                            //
    // ------------------------------------------------------------------ //

    /// 建立新資料表；若表名已存在回傳 Err
    pub fn create_table(&mut self, name: &str, schema: Schema) -> Result<&TableMeta, String> {
        if self.cache.contains_key(name) {
            return Err(format!("table '{}' already exists", name));
        }

        // root_page = 0 是佔位值，實際由呼叫端在拿到 Table 後設定
        // 這裡先配置一個代表「尚未使用」的頁號 usize::MAX
        let meta = TableMeta::new(name, schema, usize::MAX);
        self.persist_meta(&meta);
        self.cache.insert(name.to_string(), meta);
        Ok(self.cache.get(name).unwrap())
    }

    /// 更新資料表的 root_page 與 row_count（Table 初始化後呼叫）
    pub fn update_table_meta(&mut self, name: &str, root_page: usize, row_count: usize) -> Result<(), String> {
        self.update_table_meta_full(name, root_page, row_count, None)
    }

    /// 更新資料表（含 autoinc_last）
    pub fn update_table_meta_full(&mut self, name: &str, root_page: usize, row_count: usize, autoinc_last: Option<u64>) -> Result<(), String> {
        let meta = self.cache.get_mut(name)
            .ok_or_else(|| format!("table '{}' not found", name))?;
        meta.root_page = root_page;
        meta.row_count = row_count;
        if let Some(v) = autoinc_last {
            meta.autoinc_last = v;
        }
        let meta_clone = meta.clone();
        self.persist_meta(&meta_clone);
        Ok(())
    }

    /// 查詢資料表定義；找不到回傳 None
    pub fn get_table(&self, name: &str) -> Option<&TableMeta> {
        self.cache.get(name)
    }

    /// 取得可變參照
    pub fn get_table_mut(&mut self, name: &str) -> Option<&mut TableMeta> {
        self.cache.get_mut(name)
    }

    /// 刪除資料表；找不到回傳 Err
    pub fn drop_table(&mut self, name: &str) -> Result<(), String> {
        if self.cache.remove(name).is_none() {
            return Err(format!("table '{}' not found", name));
        }
        self.sys_tree.delete(&Key::Text(name.to_string()));
        Ok(())
    }

    /// 列出所有資料表名稱
    pub fn table_names(&self) -> Vec<&str> {
        self.cache.keys().map(|s| s.as_str()).collect()
    }

    /// 資料表是否存在
    pub fn table_exists(&self, name: &str) -> bool {
        self.cache.contains_key(name)
    }

    /// 建立索引
    pub fn create_index(&mut self, name: &str, table: &str, columns: &[String], unique: bool) -> Result<(), String> {
        if self.index_cache.contains_key(name) {
            return Err(format!("index '{}' already exists", name));
        }
        if !self.table_exists(table) {
            return Err(format!("table '{}' does not exist", table));
        }
        let meta = IndexMeta::new(name, table, columns, unique);
        self.index_cache.insert(name.to_string(), meta);
        Ok(())
    }

    /// 刪除索引
    pub fn drop_index(&mut self, name: &str) -> Result<(), String> {
        if self.index_cache.remove(name).is_none() {
            return Err(format!("index '{}' not found", name));
        }
        Ok(())
    }

    /// 索引是否存在
    pub fn index_exists(&self, name: &str) -> bool {
        self.index_cache.contains_key(name)
    }

    /// 取得索引
    pub fn get_index(&self, name: &str) -> Option<&IndexMeta> {
        self.index_cache.get(name)
    }

    /// 列出所有索引名稱
    pub fn index_names(&self) -> Vec<&str> {
        self.index_cache.keys().map(|s| s.as_str()).collect()
    }

    /// 重新命名資料表
    pub fn rename_table(&mut self, old_name: &str, new_name: &str) -> Result<(), String> {
        let old_meta = self.cache.remove(old_name)
            .ok_or_else(|| format!("table '{}' not found", old_name))?;
        let mut new_meta = old_meta.clone();
        new_meta.name = new_name.to_string();
        self.persist_meta(&new_meta);
        self.cache.insert(new_name.to_string(), new_meta);
        Ok(())
    }

    /// 新增欄位
    pub fn add_column(&mut self, table: &str, _col_name: &str, col: Column) -> Result<(), String> {
        let meta = self.cache.get_mut(table)
            .ok_or_else(|| format!("table '{}' not found", table))?;
        meta.schema.columns.push(col);
        let meta_clone = meta.clone();
        drop(meta);
        self.persist_meta(&meta_clone);
        Ok(())
    }

    /// 系統表的根頁號（磁碟後端需要儲存此值以便重新開啟）
    pub fn root_page(&self) -> usize {
        self.sys_tree.root_page()
    }

    pub fn flush(&mut self) {
        self.sys_tree.flush();
    }

    /// 從磁碟重新載入所有 TableMeta（load_all 的公開版）
    pub fn reload(&mut self) {
        self.cache.clear();
        self.load_all();
    }

    // ------------------------------------------------------------------ //
    //  內部輔助                                                            //
    // ------------------------------------------------------------------ //

    fn persist_meta(&mut self, meta: &TableMeta) {
        let key = Key::Text(meta.name.clone());
        let bytes = encode_meta(meta);
        self.sys_tree.insert(key, bytes);
    }

    /// 啟動時從系統表讀入所有 TableMeta 到快取
    fn load_all(&mut self) {
        let min = Key::Text(String::new());
        let max = Key::Text("\u{10FFFF}".repeat(4));
        let records = self.sys_tree.range_search(&min, &max);
        for record in records {
            let meta = decode_meta(&record.value);
            self.cache.insert(meta.name.clone(), meta);
        }
    }
}

// ------------------------------------------------------------------ //
//  測試                                                                //
// ------------------------------------------------------------------ //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pager::storage::{DiskStorage, MemoryStorage};
    use crate::table::schema::{Column, DataType, Schema};

    fn users_schema() -> Schema {
        Schema::new(vec![
            Column::new("id",   DataType::Integer),
            Column::new("name", DataType::Text),
        ])
    }

    fn orders_schema() -> Schema {
        Schema::new(vec![
            Column::new("order_id", DataType::Integer),
            Column::new("amount",   DataType::Float),
        ])
    }

    #[test]
    fn create_and_get() {
        let mut cat = Catalog::new(MemoryStorage::new());
        cat.create_table("users", users_schema()).unwrap();
        let meta = cat.get_table("users").unwrap();
        assert_eq!(meta.name, "users");
        assert_eq!(meta.schema.columns.len(), 2);
    }

    #[test]
    fn create_duplicate_fails() {
        let mut cat = Catalog::new(MemoryStorage::new());
        cat.create_table("users", users_schema()).unwrap();
        assert!(cat.create_table("users", users_schema()).is_err());
    }

    #[test]
    fn drop_table() {
        let mut cat = Catalog::new(MemoryStorage::new());
        cat.create_table("users", users_schema()).unwrap();
        cat.drop_table("users").unwrap();
        assert!(cat.get_table("users").is_none());
        assert!(cat.drop_table("users").is_err());
    }

    #[test]
    fn multiple_tables() {
        let mut cat = Catalog::new(MemoryStorage::new());
        cat.create_table("users",  users_schema()).unwrap();
        cat.create_table("orders", orders_schema()).unwrap();
        assert_eq!(cat.table_names().len(), 2);
        assert!(cat.table_exists("users"));
        assert!(cat.table_exists("orders"));
        assert!(!cat.table_exists("missing"));
    }

    #[test]
    fn update_meta() {
        let mut cat = Catalog::new(MemoryStorage::new());
        cat.create_table("users", users_schema()).unwrap();
        cat.update_table_meta("users", 5, 100).unwrap();
        let meta = cat.get_table("users").unwrap();
        assert_eq!(meta.root_page, 5);
        assert_eq!(meta.row_count, 100);
    }

    #[test]
    fn disk_catalog_persist() {
        let path = "/tmp/sql5_catalog_test.sql5db";
        let _ = std::fs::remove_file(path);

        let root_page;
        {
            let mut cat = Catalog::new(DiskStorage::open(path).unwrap());
            cat.create_table("users",  users_schema()).unwrap();
            cat.create_table("orders", orders_schema()).unwrap();
            cat.update_table_meta("users", 3, 10).unwrap();
            root_page = cat.root_page();
            cat.flush();
        }

        {
            let cat = Catalog::open(DiskStorage::open(path).unwrap(), root_page);
            assert!(cat.table_exists("users"));
            assert!(cat.table_exists("orders"));
            let meta = cat.get_table("users").unwrap();
            assert_eq!(meta.root_page, 3);
            assert_eq!(meta.row_count, 10);
        }

        let _ = std::fs::remove_file(path);
    }
}
