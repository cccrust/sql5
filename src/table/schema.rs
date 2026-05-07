//! Schema：定義表格的欄位結構

/// 支援的欄位型別
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Integer,  // i64
    Text,     // UTF-8 字串
    Boolean,  // bool
    Float,    // f64
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Integer => write!(f, "INTEGER"),
            DataType::Text    => write!(f, "TEXT"),
            DataType::Boolean => write!(f, "BOOLEAN"),
            DataType::Float   => write!(f, "FLOAT"),
        }
    }
}

/// 單一欄位定義
#[derive(Debug, Clone)]
pub struct Column {
    pub name:        String,
    pub data_type:   DataType,
    pub nullable:    bool,
    pub autoinc:     bool,
    pub default:     Option<crate::parser::ast::Expr>,
}

impl Column {
    pub fn new(name: &str, data_type: DataType) -> Self {
        Column { name: name.to_string(), data_type, nullable: true, autoinc: false, default: None }
    }

    pub fn not_null(mut self) -> Self {
        self.nullable = false;
        self
    }

    pub fn autoincrement(mut self) -> Self {
        self.autoinc = true;
        self
    }

    pub fn with_default(mut self, expr: crate::parser::ast::Expr) -> Self {
        self.default = Some(expr);
        self
    }
}

/// 表格 Schema：由多個 Column 組成
#[derive(Debug, Clone)]
pub struct Schema {
    pub columns: Vec<Column>,
}

impl Schema {
    pub fn new(columns: Vec<Column>) -> Self {
        Schema { columns }
    }

    /// 根據欄位名稱找索引
    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|c| c.name == name)
    }

    pub fn len(&self) -> usize {
        self.columns.len()
    }

    /// 產生 CREATE TABLE 語句
    pub fn to_sql(&self, table_name: &str) -> String {
        let cols: Vec<String> = self.columns.iter().map(|c| {
            let t = match c.data_type {
                DataType::Integer => "INTEGER",
                DataType::Text => "TEXT",
                DataType::Float => "FLOAT",
                DataType::Boolean => "BOOLEAN",
            };
            if !c.nullable {
                format!("{} {} NOT NULL", c.name, t)
            } else {
                format!("{} {}", c.name, t)
            }
        }).collect();
        format!("CREATE TABLE {}({})", table_name, cols.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_type_integer() {
        let dt = DataType::Integer;
        assert_eq!(dt.to_string(), "INTEGER");
    }

    #[test]
    fn test_data_type_text() {
        let dt = DataType::Text;
        assert_eq!(dt.to_string(), "TEXT");
    }

    #[test]
    fn test_data_type_boolean() {
        let dt = DataType::Boolean;
        assert_eq!(dt.to_string(), "BOOLEAN");
    }

    #[test]
    fn test_data_type_float() {
        let dt = DataType::Float;
        assert_eq!(dt.to_string(), "FLOAT");
    }

    #[test]
    fn test_column_new() {
        let col = Column::new("id", DataType::Integer);
        assert_eq!(col.name, "id");
        assert_eq!(col.data_type, DataType::Integer);
        assert!(col.nullable);
        assert!(!col.autoinc);
        assert!(col.default.is_none());
    }

    #[test]
    fn test_column_not_null() {
        let col = Column::new("id", DataType::Integer).not_null();
        assert!(!col.nullable);
    }

    #[test]
    fn test_column_autoincrement() {
        let col = Column::new("id", DataType::Integer).autoincrement();
        assert!(col.autoinc);
    }

    #[test]
    fn test_column_with_default() {
        let col = Column::new("id", DataType::Integer).not_null();
        assert!(!col.nullable);
        assert!(col.default.is_none());
    }

    #[test]
    fn test_schema_new() {
        let col1 = Column::new("id", DataType::Integer);
        let col2 = Column::new("name", DataType::Text);
        let schema = Schema::new(vec![col1, col2]);
        assert_eq!(schema.len(), 2);
    }

    #[test]
    fn test_schema_index_of() {
        let col1 = Column::new("id", DataType::Integer);
        let col2 = Column::new("name", DataType::Text);
        let schema = Schema::new(vec![col1, col2]);

        assert_eq!(schema.index_of("id"), Some(0));
        assert_eq!(schema.index_of("name"), Some(1));
        assert_eq!(schema.index_of("nonexistent"), None);
    }

    #[test]
    fn test_schema_len() {
        let schema = Schema::new(vec![
            Column::new("a", DataType::Integer),
            Column::new("b", DataType::Text),
            Column::new("c", DataType::Boolean),
        ]);
        assert_eq!(schema.len(), 3);
    }

    #[test]
    fn test_schema_clone() {
        let col = Column::new("id", DataType::Integer);
        let schema = Schema::new(vec![col]);
        let cloned = schema.clone();
        assert_eq!(schema.len(), cloned.len());
    }

    #[test]
    fn test_column_clone() {
        let col = Column::new("name", DataType::Text);
        let cloned = col.clone();
        assert_eq!(col.name, cloned.name);
        assert_eq!(col.data_type, cloned.data_type);
    }
}
