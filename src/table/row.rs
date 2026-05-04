//! Row：一筆資料列，由多個 Value 組成

use super::schema::Schema;

/// 單一欄位的值
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Text(String),
    Boolean(bool),
    Float(f64),
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(v) => write!(f, "{}", v),
            Value::Text(s)    => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Float(v)   => write!(f, "{}", v),
            Value::Null       => write!(f, "NULL"),
        }
    }
}

/// 一筆資料列
#[derive(Debug, Clone)]
pub struct Row {
    pub values: Vec<Value>,
}

impl Row {
    pub fn new(values: Vec<Value>) -> Self {
        Row { values }
    }

    /// 以欄位名稱取值（需要 schema 對應）
    pub fn get_by_name<'a>(&'a self, schema: &Schema, name: &str) -> Option<&'a Value> {
        schema.index_of(name).map(|i| &self.values[i])
    }

    /// 以欄位索引取值
    pub fn get(&self, idx: usize) -> Option<&Value> {
        self.values.get(idx)
    }

    /// 印出一列（以 | 分隔）
    pub fn display(&self, schema: &Schema) -> String {
        self.values
            .iter()
            .enumerate()
            .map(|(i, v)| format!("{}: {}", schema.columns[i].name, v))
            .collect::<Vec<_>>()
            .join(" | ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table::schema::{Column, DataType};

    #[test]
    fn test_value_integer() {
        let v = Value::Integer(42);
        assert_eq!(v.to_string(), "42");
        assert_eq!(v, Value::Integer(42));
        assert_ne!(v, Value::Integer(43));
    }

    #[test]
    fn test_value_text() {
        let v = Value::Text("hello".to_string());
        assert_eq!(v.to_string(), "hello");
        assert_eq!(v, Value::Text("hello".to_string()));
        assert_ne!(v, Value::Text("world".to_string()));
    }

    #[test]
    fn test_value_boolean() {
        let t = Value::Boolean(true);
        let f = Value::Boolean(false);
        assert_eq!(t.to_string(), "true");
        assert_eq!(f.to_string(), "false");
        assert_eq!(t, Value::Boolean(true));
        assert_ne!(t, f);
    }

    #[test]
    fn test_value_float() {
        let v = Value::Float(3.14159);
        assert_eq!(v.to_string(), "3.14159");
        assert_eq!(v, Value::Float(3.14159));
        assert_ne!(v, Value::Float(2.71828));
    }

    #[test]
    fn test_value_null() {
        let v = Value::Null;
        assert_eq!(v.to_string(), "NULL");
        assert_eq!(v, Value::Null);
    }

    #[test]
    fn test_value_clone() {
        let v = Value::Integer(100);
        let cloned = v.clone();
        assert_eq!(v, cloned);
    }

    #[test]
    fn test_value_debug() {
        let v = Value::Text("test".to_string());
        let debug = format!("{:?}", v);
        assert!(debug.contains("test"));
    }

    #[test]
    fn test_row_new() {
        let values = vec![Value::Integer(1), Value::Text("hello".to_string())];
        let row = Row::new(values.clone());
        assert_eq!(row.values, values);
    }

    #[test]
    fn test_row_get() {
        let values = vec![Value::Integer(1), Value::Text("hello".to_string()), Value::Null];
        let row = Row::new(values);

        assert_eq!(row.get(0), Some(&Value::Integer(1)));
        assert_eq!(row.get(1), Some(&Value::Text("hello".to_string())));
        assert_eq!(row.get(2), Some(&Value::Null));
        assert_eq!(row.get(3), None);
        assert_eq!(row.get(usize::MAX), None);
    }

    #[test]
    fn test_row_get_by_name() {
        let schema = Schema::new(vec![
            Column::new("id", DataType::Integer),
            Column::new("name", DataType::Text),
        ]);

        let values = vec![Value::Integer(1), Value::Text("Alice".to_string())];
        let row = Row::new(values);

        assert_eq!(row.get_by_name(&schema, "id"), Some(&Value::Integer(1)));
        assert_eq!(row.get_by_name(&schema, "name"), Some(&Value::Text("Alice".to_string())));
        assert_eq!(row.get_by_name(&schema, "nonexistent"), None);
    }

    #[test]
    fn test_row_display() {
        let schema = Schema::new(vec![
            Column::new("id", DataType::Integer),
            Column::new("name", DataType::Text),
        ]);

        let values = vec![Value::Integer(42), Value::Text("Bob".to_string())];
        let row = Row::new(values);

        let display = row.display(&schema);
        assert!(display.contains("id: 42"));
        assert!(display.contains("name: Bob"));
    }

    #[test]
    fn test_row_clone() {
        let values = vec![Value::Integer(1), Value::Float(2.5)];
        let row = Row::new(values.clone());
        let cloned = row.clone();
        assert_eq!(row.values, cloned.values);
    }
}
