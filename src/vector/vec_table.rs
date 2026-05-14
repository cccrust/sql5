//! vec0 虛擬表實作
//!
//! 模擬 sqlite-vec 的使用介面：
//!
//! ```sql
//! -- 建立向量表
//! CREATE VIRTUAL TABLE items USING vec0(
//!   embedding float[768],
//!   id integer primary key,
//!   category text,
//!   +description text
//! );
//!
//! -- 插入向量
//! INSERT INTO items(rowid, embedding) VALUES (1, '[0.1, 0.2, ...]');
//!
//! -- KNN 搜尋
//! SELECT rowid, distance FROM items WHERE embedding match '[0.1, 0.2, ...]' LIMIT 10;
//! ```
//!
//! 支援：
//! - 向量欄位：float[N], int8[N], bit[N]
//! - 元資料欄位：text, integer, float, boolean
//! - 分區鍵：column_name type partition key
//! - 輔助欄位：+column_name type

use std::collections::HashMap;

use crate::vector::vector::{parse_dimension_type, VectorType};
use crate::vector::distance::{distance_l2, distance_cosine, distance_hamming};

/// 向量表欄位定義
#[derive(Debug, Clone)]
pub enum ColumnDef {
    /// 向量欄位：維度資訊
    Vector { name: String, dimension: usize, vec_type: String },
    /// 元資料欄位
    Metadata { name: String, col_type: String },
    /// 分區鍵
    PartitionKey { name: String, col_type: String },
    /// 輔助欄位
    Auxiliary { name: String, col_type: String },
}

/// 向量表結構
pub struct VecTable {
    pub name: String,
    pub columns: Vec<ColumnDef>,
    /// 向量欄位名稱
    vector_column: Option<String>,
    /// 分區鍵欄位
    partition_keys: Vec<String>,
    /// 向量儲存：rowid -> VectorType
    vectors: HashMap<u64, VectorType>,
    /// 元資料儲存：rowid -> (column_name -> value)
    metadata: HashMap<u64, HashMap<String, String>>,
    /// 輔助欄位儲存：rowid -> (column_name -> value)
    auxiliary: HashMap<u64, HashMap<String, String>>,
    /// 下一個 rowid
    next_rowid: u64,
}

impl VecTable {
    /// 建立新的向量表
    pub fn new(name: &str, columns: Vec<ColumnDef>) -> Self {
        let vector_column = columns.iter()
            .find_map(|c| {
                if let ColumnDef::Vector { name, .. } = c {
                    Some(name.clone())
                } else {
                    None
                }
            });

        let partition_keys = columns.iter()
            .filter_map(|c| {
                if let ColumnDef::PartitionKey { name, .. } = c {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect();

        VecTable {
            name: name.to_string(),
            columns,
            vector_column,
            partition_keys,
            vectors: HashMap::new(),
            metadata: HashMap::new(),
            auxiliary: HashMap::new(),
            next_rowid: 1,
        }
    }

    /// 插入向量
    pub fn insert(&mut self, rowid: Option<u64>, vector: VectorType, metadata: HashMap<String, String>, auxiliary: HashMap<String, String>) -> u64 {
        let id = rowid.unwrap_or(self.next_rowid);
        if id >= self.next_rowid {
            self.next_rowid = id + 1;
        }

        self.vectors.insert(id, vector);
        if !metadata.is_empty() {
            self.metadata.insert(id, metadata);
        }
        if !auxiliary.is_empty() {
            self.auxiliary.insert(id, auxiliary);
        }

        id
    }

    /// KNN 搜尋
    pub fn search(&self, query_vector: &VectorType, k: usize, filters: &HashMap<String, String>) -> Vec<(u64, f64)> {
        let mut results: Vec<(u64, f64)> = Vec::new();

        for (rowid, vector) in &self.vectors {
            // 檢查元資料篩選
            let mut pass = true;
            if let Some(meta) = self.metadata.get(rowid) {
                for (key, value) in filters {
                    if let Some(v) = meta.get(key) {
                        if v != value {
                            pass = false;
                            break;
                        }
                    }
                }
            }
            if !pass {
                continue;
            }

            // 計算距離
            if let Ok(distance) = distance_l2(query_vector, vector) {
                results.push((*rowid, distance));
            }
        }

        // 排序並取 top k
        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(k);
        results
    }

    /// 取得 rowid 的向量
    pub fn get_vector(&self, rowid: u64) -> Option<&VectorType> {
        self.vectors.get(&rowid)
    }

    /// 取得 rowid 的元資料
    pub fn get_metadata(&self, rowid: u64) -> Option<&HashMap<String, String>> {
        self.metadata.get(&rowid)
    }

    /// 取得 rowid 的輔助欄位
    pub fn get_auxiliary(&self, rowid: u64) -> Option<&HashMap<String, String>> {
        self.auxiliary.get(&rowid)
    }

    /// 取得所有 rowid
    pub fn rowids(&self) -> Vec<u64> {
        self.vectors.keys().cloned().collect()
    }

    /// 取得向量數量
    pub fn row_count(&self) -> usize {
        self.vectors.len()
    }

    /// 刪除向量
    pub fn delete(&mut self, rowid: u64) {
        self.vectors.remove(&rowid);
        self.metadata.remove(&rowid);
        self.auxiliary.remove(&rowid);
    }
}

/// 解析 CREATE VIRTUAL TABLE 的欄位定義
pub fn parse_columns(columns_str: &str) -> Result<Vec<ColumnDef>, String> {
    let mut columns = Vec::new();

    for col in columns_str.split(',') {
        let col = col.trim();
        if col.is_empty() {
            continue;
        }

        // 輔助欄位：+column_name type
        if col.starts_with('+') {
            let rest = col[1..].trim();
            if let Some(space) = rest.find(' ') {
                let name = rest[..space].to_string();
                let col_type = rest[space+1..].to_string();
                columns.push(ColumnDef::Auxiliary { name, col_type });
            } else {
                return Err("Auxiliary column needs type".to_string());
            }
            continue;
        }

        // 分區鍵：column_name type partition key
        if col.contains("partition key") {
            let parts: Vec<&str> = col.split_whitespace().collect();
            if parts.len() >= 3 {
                let name = parts[0].to_string();
                let col_type = parts[1].to_string();
                columns.push(ColumnDef::PartitionKey { name, col_type });
            } else {
                return Err("Invalid partition key format".to_string());
            }
            continue;
        }

        // 向量欄位：name float[N] 或 name int8[N]
        if col.contains('[') {
            let parts: Vec<&str> = col.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0].to_string();
                let type_with_dim = parts[1];

                if let Some(bracket) = type_with_dim.find('[') {
                    let base_type = &type_with_dim[..bracket];
                    if let Some(bracket_close) = type_with_dim.find(']') {
                        let dim_str = &type_with_dim[bracket+1..bracket_close];
                        if let Ok(dimension) = dim_str.parse::<usize>() {
                            columns.push(ColumnDef::Vector {
                                name,
                                dimension,
                                vec_type: base_type.to_string(),
                            });
                            continue;
                        }
                    }
                }
            }
            return Err(format!("Invalid vector column format: {}", col));
        }

        // 元資料欄位
        let parts: Vec<&str> = col.split_whitespace().collect();
        if parts.len() >= 2 {
            let name = parts[0].to_string();
            let col_type = parts[1].to_string();
            columns.push(ColumnDef::Metadata { name, col_type });
        } else {
            return Err(format!("Invalid column definition: {}", col));
        }
    }

    if columns.is_empty() {
        return Err("At least one column must be defined".to_string());
    }

    // 檢查是否有向量欄位
    let has_vector = columns.iter().any(|c| matches!(c, ColumnDef::Vector { .. }));
    if !has_vector {
        return Err("At least one vector column must be defined".to_string());
    }

    Ok(columns)
}

/// 解析 INSERT 語句中的向量值
pub fn parse_vector_value(value: &str) -> Result<VectorType, String> {
    let value = value.trim();

    // JSON 格式：[...]
    if value.starts_with('[') {
        // 自動偵測類型
        if value.contains('.') {
            return VectorType::from_json(value, "float32");
        } else {
            return VectorType::from_json(value, "int8");
        }
    }

    // BLOB 格式：X'...'
    if value.starts_with("X'") || value.starts_with("x'") {
        let hex = &value[2..value.len()-1];
        let bytes = hex::decode(hex).map_err(|e| format!("Invalid hex: {}", e))?;
        // 嘗試識別類型
        if bytes.len() % 4 == 0 {
            return VectorType::from_blob(&bytes, "float32");
        } else if bytes.len() <= 128 {
            return VectorType::from_blob(&bytes, "int8");
        } else {
            return VectorType::from_blob(&bytes, "bit");
        }
    }

    Err("Invalid vector format".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_columns() {
        let cols = parse_columns("embedding float[768], id integer, +content text").unwrap();
        assert_eq!(cols.len(), 3);
        if let ColumnDef::Vector { name, dimension, vec_type } = &cols[0] {
            assert_eq!(name, "embedding");
            assert_eq!(*dimension, 768);
            assert_eq!(vec_type, "float");
        }
    }

    #[test]
    fn test_vec_table_insert() {
        let cols = parse_columns("embedding float[3]").unwrap();
        let mut table = VecTable::new("test", cols);

        let vector = VectorType::Float32(vec![1.0, 2.0, 3.0]);
        let id = table.insert(None, vector, HashMap::new(), HashMap::new());
        assert_eq!(id, 1);
        assert_eq!(table.row_count(), 1);
    }

    #[test]
    fn test_vec_table_search() {
        let cols = parse_columns("embedding float[2]").unwrap();
        let mut table = VecTable::new("test", cols);

        table.insert(None, VectorType::Float32(vec![1.0, 1.0]), HashMap::new(), HashMap::new());
        table.insert(None, VectorType::Float32(vec![2.0, 2.0]), HashMap::new(), HashMap::new());
        table.insert(None, VectorType::Float32(vec![3.0, 3.0]), HashMap::new(), HashMap::new());

        let query = VectorType::Float32(vec![1.1, 1.1]);
        let results = table.search(&query, 2, &HashMap::new());
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, 1); // 最近的是第一個
    }

    #[test]
    fn test_parse_vector_json() {
        let v = parse_vector_value("[1.0, 2.0, 3.0]").unwrap();
        assert!(matches!(v, VectorType::Float32(_)));
    }

    #[test]
    fn test_partition_key() {
        let cols = parse_columns("user_id integer partition key, embedding float[3]").unwrap();
        assert!(matches!(&cols[0], ColumnDef::PartitionKey { .. }));
    }
}