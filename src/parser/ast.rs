//! AST：SQL 語句的抽象語法樹節點定義

// ── 頂層語句 ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Select(SelectStmt),
    Insert(InsertStmt),
    Update(UpdateStmt),
    Delete(DeleteStmt),
    CreateTable(CreateTableStmt),
    DropTable(DropTableStmt),
    CreateIndex(CreateIndexStmt),
    DropIndex(DropIndexStmt),
    AlterTable(AlterTableStmt),
    Pragma(PragmaStmt),
    Explain(ExplainStmt),
    CreateView(CreateViewStmt),
    DropView(DropViewStmt),
    CreateTrigger(CreateTriggerStmt),
    DropTrigger(DropTriggerStmt),
    Reindex(ReindexStmt),
    Analyze(AnalyzeStmt),
    Attach { path: String, alias: String },
    Detach { alias: String },
    Vacuum,
    Begin,
    Commit,
    Rollback,
}

// ── SELECT ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct SelectStmt {
    pub with:      Vec<Cte>,            // WITH ... AS (...)
    pub distinct:  bool,
    pub columns:   Vec<SelectItem>,
    pub from:      Option<FromItem>,    // table name 或子查詢
    pub joins:     Vec<Join>,
    pub where_:    Option<Expr>,
    pub group_by:  Vec<Expr>,
    pub having:    Option<Expr>,
    pub order_by:  Vec<OrderItem>,
    pub limit:     Option<Expr>,
    pub offset:    Option<Expr>,
    pub union_with: Option<Box<(SelectStmt, bool)>>,  // (select_stmt, is_all)
}

#[derive(Debug, Clone, PartialEq)]
pub enum SelectItem {
    Star,                          // *
    TableStar(String),             // table.*
    Expr { expr: Expr, alias: Option<String> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableRef {
    pub name:  String,
    pub alias: Option<String>,
}

/// FROM 子句可以是資料表名稱或子查詢
#[derive(Debug, Clone, PartialEq)]
pub enum FromItem {
    Table(TableRef),
    Subquery { query: Box<SelectStmt>, alias: String },
}

/// CTE（Common Table Expression）定義：WITH name AS (query)
#[derive(Debug, Clone, PartialEq)]
pub struct Cte {
    pub name:  String,
    pub query: Box<SelectStmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Join {
    pub kind:      JoinKind,
    pub table:     TableRef,   // 暫保留 TableRef，子查詢 JOIN 後續擴充
    pub condition: JoinCondition,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JoinKind {
    Inner, Left, Right, Full, Cross, Natural,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JoinCondition {
    On(Expr),
    Using(Vec<String>),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrderItem {
    pub expr: Expr,
    pub asc:  bool,
}

// ── INSERT ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct InsertStmt {
    pub table:   String,
    pub columns: Vec<String>,   // 空表示不指定欄位
    pub values:  Vec<Vec<Expr>>,
    pub default_values: bool,  // true for INSERT DEFAULT VALUES
    pub on_conflict: Option<OnConflict>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OnConflict {
    DoNothing,
    DoUpdate { column: String, value: Expr },
}

// ── UPDATE ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateStmt {
    pub table:   String,
    pub sets:    Vec<(String, Expr)>,
    pub where_:  Option<Expr>,
}

// ── DELETE ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteStmt {
    pub table:  String,
    pub where_: Option<Expr>,
}

// ── CREATE TABLE ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct CreateTableStmt {
    pub if_not_exists: bool,
    pub name:          String,
    pub columns:       Vec<ColumnDef>,
    pub constraints:   Vec<TableConstraint>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColumnDef {
    pub name:        String,
    pub data_type:   SqlType,
    pub constraints: Vec<ColumnConstraint>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SqlType {
    Integer, Real, Text, Blob, Boolean, Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnConstraint {
    NotNull,
    PrimaryKey { autoincrement: bool },
    Unique,
    Default(Expr),
    Check(Expr),
    References { table: String, column: Option<String> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TableConstraint {
    PrimaryKey(Vec<String>),
    Unique(Vec<String>),
}

// ── DROP TABLE ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct DropTableStmt {
    pub if_exists: bool,
    pub name:      String,
}

// ── CREATE INDEX ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct CreateIndexStmt {
    pub unique:    bool,
    pub name:      String,
    pub table:     String,
    pub columns:   Vec<String>,
}

// ── DROP INDEX ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct DropIndexStmt {
    pub if_exists: bool,
    pub name:      String,
}

// ── ALTER TABLE ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum AlterTableOp {
    RenameTo(String),
    AddColumn { name: String, data_type: SqlType },
}

#[derive(Debug, Clone, PartialEq)]
pub struct AlterTableStmt {
    pub table: String,
    pub op:    AlterTableOp,
}

// ── PRAGMA ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct PragmaStmt {
    pub name:  String,
    pub value: Option<Expr>,
}

// ── EXPLAIN ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct ExplainStmt {
    pub inner: Box<Statement>,
}

// ── CREATE VIEW ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct CreateViewStmt {
    pub if_not_exists: bool,
    pub temp:         bool,
    pub name:         String,
    pub query:        Box<SelectStmt>,
}

// ── DROP VIEW ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct DropViewStmt {
    pub if_exists: bool,
    pub name:      String,
}

// ── TRIGGER ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct CreateTriggerStmt {
    pub if_not_exists: bool,
    pub name:          String,
    pub table:         String,
    pub timing:        TriggerTiming,
    pub event:         TriggerEvent,
    pub for_each_row:  bool,
    pub when:          Option<Box<Expr>>,
    pub body:          String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TriggerTiming {
    Before,
    After,
    InsteadOf,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TriggerEvent {
    Delete,
    Insert,
    Update(Option<Vec<String>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct DropTriggerStmt {
    pub if_exists: bool,
    pub name:      String,
}

// ── REINDEX ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct ReindexStmt {
    pub name: Option<String>,
}

// ── ANALYZE ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyzeStmt {
    pub name: Option<String>,
}

// ── 運算式 ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // 字面值
    LitInt(i64),
    LitFloat(f64),
    LitStr(String),
    LitBool(bool),
    LitNull,

    // 欄位參照（可帶 table prefix）
    Column { table: Option<String>, name: String },

    // 函式呼叫
    Function { name: String, args: Vec<Expr>, distinct: bool },

    // 二元運算
    BinOp { left: Box<Expr>, op: BinOp, right: Box<Expr> },

    // 一元運算
    UnaryOp { op: UnaryOp, expr: Box<Expr> },

    // IS NULL / IS NOT NULL
    IsNull  { expr: Box<Expr>, negated: bool },

    // BETWEEN
    Between { expr: Box<Expr>, low: Box<Expr>, high: Box<Expr>, negated: bool },

    // IN (...)
    InList  { expr: Box<Expr>, list: Vec<Expr>, negated: bool },

    // IN (SELECT ...)
    InSubquery { expr: Box<Expr>, query: Box<SelectStmt>, negated: bool },

    // EXISTS (SELECT ...)
    Exists { query: Box<SelectStmt>, negated: bool },

    // 純量子查詢 (SELECT ...)
    ScalarSubquery(Box<SelectStmt>),

    // LIKE / GLOB
    Like    { expr: Box<Expr>, pattern: Box<Expr>, negated: bool },
    Glob    { expr: Box<Expr>, pattern: Box<Expr>, negated: bool },

    // 子查詢（留待後續實作）
    Subquery(Box<SelectStmt>),

    // CAST(expr AS type)
    Cast { expr: Box<Expr>, to: SqlType },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Eq, NotEq, Lt, LtEq, Gt, GtEq,
    And, Or,
    Add, Sub, Mul, Div, Mod,
    Concat,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,    // -
    Not,    // NOT
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_lit_int() {
        let e = Expr::LitInt(42);
        assert!(matches!(e, Expr::LitInt(42)));
    }

    #[test]
    fn test_expr_lit_float() {
        let e = Expr::LitFloat(3.14);
        assert!(matches!(e, Expr::LitFloat(3.14)));
    }

    #[test]
    fn test_expr_lit_str() {
        let e = Expr::LitStr("hello".to_string());
        assert!(matches!(e, Expr::LitStr(s) if s == "hello"));
    }

    #[test]
    fn test_expr_lit_bool() {
        let t = Expr::LitBool(true);
        let f = Expr::LitBool(false);
        assert!(matches!(t, Expr::LitBool(true)));
        assert!(matches!(f, Expr::LitBool(false)));
    }

    #[test]
    fn test_expr_lit_null() {
        let e = Expr::LitNull;
        assert!(matches!(e, Expr::LitNull));
    }

    #[test]
    fn test_expr_column() {
        let e = Expr::Column { table: None, name: "id".to_string() };
        assert!(matches!(e, Expr::Column { table: None, name } if name == "id"));
    }

    #[test]
    fn test_expr_column_with_table() {
        let e = Expr::Column { table: Some("users".to_string()), name: "id".to_string() };
        assert!(matches!(e, Expr::Column { table: Some(t), name } if t == "users" && name == "id"));
    }

    #[test]
    fn test_expr_function() {
        let e = Expr::Function { name: "COUNT".to_string(), args: vec![Expr::Column { table: None, name: "*".to_string() }], distinct: false };
        assert!(matches!(e, Expr::Function { name, .. } if name == "COUNT"));
    }

    #[test]
    fn test_expr_binop() {
        use BinOp::*;
        let e = Expr::BinOp {
            left: Box::new(Expr::LitInt(1)),
            op: Add,
            right: Box::new(Expr::LitInt(2)),
        };
        assert!(matches!(e, Expr::BinOp { op: Add, .. }));
    }

    #[test]
    fn test_expr_unary_op() {
        let e = Expr::UnaryOp { op: UnaryOp::Neg, expr: Box::new(Expr::LitInt(5)) };
        assert!(matches!(e, Expr::UnaryOp { op: UnaryOp::Neg, .. }));
    }

    #[test]
    fn test_expr_is_null() {
        let e = Expr::IsNull { expr: Box::new(Expr::Column { table: None, name: "x".to_string() }), negated: false };
        assert!(matches!(e, Expr::IsNull { negated: false, .. }));
    }

    #[test]
    fn test_expr_is_not_null() {
        let e = Expr::IsNull { expr: Box::new(Expr::Column { table: None, name: "x".to_string() }), negated: true };
        assert!(matches!(e, Expr::IsNull { negated: true, .. }));
    }

    #[test]
    fn test_binop_equality() {
        let eq = BinOp::Eq;
        let ne = BinOp::NotEq;
        assert!(matches!(eq, BinOp::Eq));
        assert!(matches!(ne, BinOp::NotEq));
    }

    #[test]
    fn test_binop_comparison() {
        assert!(matches!(BinOp::Lt, BinOp::Lt));
        assert!(matches!(BinOp::LtEq, BinOp::LtEq));
        assert!(matches!(BinOp::Gt, BinOp::Gt));
        assert!(matches!(BinOp::GtEq, BinOp::GtEq));
    }

    #[test]
    fn test_binop_logical() {
        assert!(matches!(BinOp::And, BinOp::And));
        assert!(matches!(BinOp::Or, BinOp::Or));
    }

    #[test]
    fn test_binop_arithmetic() {
        assert!(matches!(BinOp::Add, BinOp::Add));
        assert!(matches!(BinOp::Sub, BinOp::Sub));
        assert!(matches!(BinOp::Mul, BinOp::Mul));
        assert!(matches!(BinOp::Div, BinOp::Div));
        assert!(matches!(BinOp::Mod, BinOp::Mod));
    }

    #[test]
    fn test_unary_op() {
        assert!(matches!(UnaryOp::Neg, UnaryOp::Neg));
        assert!(matches!(UnaryOp::Not, UnaryOp::Not));
    }

    #[test]
    fn test_select_item_star() {
        assert!(matches!(SelectItem::Star, SelectItem::Star));
    }

    #[test]
    fn test_select_item_table_star() {
        let si = SelectItem::TableStar("users".to_string());
        match si {
            SelectItem::TableStar(s) => assert_eq!(s, "users"),
            _ => panic!("not TableStar"),
        }
    }

    #[test]
    fn test_select_item_expr() {
        let si = SelectItem::Expr { expr: Expr::LitInt(1), alias: Some("one".to_string()) };
        assert!(matches!(si, SelectItem::Expr { alias: Some(a), .. } if a == "one"));
    }

    #[test]
    fn test_table_ref() {
        let t = TableRef { name: "users".to_string(), alias: Some("u".to_string()) };
        assert_eq!(t.name, "users");
        assert_eq!(t.alias, Some("u".to_string()));
    }

    #[test]
    fn test_from_item_table() {
        let fi = FromItem::Table(TableRef { name: "users".to_string(), alias: None });
        assert!(matches!(fi, FromItem::Table(t) if t.name == "users"));
    }

    #[test]
    fn test_order_item() {
        let oi = OrderItem { expr: Expr::LitInt(1), asc: true };
        assert!(oi.asc);
        assert!(matches!(oi.expr, Expr::LitInt(1)));
    }

    #[test]
    fn test_join_kind() {
        assert!(matches!(JoinKind::Inner, JoinKind::Inner));
        assert!(matches!(JoinKind::Left, JoinKind::Left));
        assert!(matches!(JoinKind::Right, JoinKind::Right));
        assert!(matches!(JoinKind::Full, JoinKind::Full));
        assert!(matches!(JoinKind::Cross, JoinKind::Cross));
        assert!(matches!(JoinKind::Natural, JoinKind::Natural));
    }

    #[test]
    fn test_join_condition() {
        let on = JoinCondition::On(Expr::LitInt(1));
        let using = JoinCondition::Using(vec!["id".to_string()]);
        let none = JoinCondition::None;
        assert!(matches!(on, JoinCondition::On(_)));
        assert!(matches!(using, JoinCondition::Using(_)));
        assert!(matches!(none, JoinCondition::None));
    }

    #[test]
    fn test_insert_stmt() {
        let stmt = InsertStmt {
            table: "users".to_string(),
            columns: vec!["name".to_string()],
            values: vec![vec![Expr::LitStr("Alice".to_string())]],
            default_values: false,
            on_conflict: None,
        };
        assert_eq!(stmt.table, "users");
        assert_eq!(stmt.columns.len(), 1);
    }

    #[test]
    fn test_update_stmt() {
        let stmt = UpdateStmt {
            table: "users".to_string(),
            sets: vec![("age".to_string(), Expr::LitInt(30))],
            where_: None,
        };
        assert_eq!(stmt.table, "users");
        assert_eq!(stmt.sets.len(), 1);
    }

    #[test]
    fn test_delete_stmt() {
        let stmt = DeleteStmt { table: "users".to_string(), where_: None };
        assert_eq!(stmt.table, "users");
    }

    #[test]
    fn test_create_table_stmt() {
        let stmt = CreateTableStmt {
            if_not_exists: true,
            name: "users".to_string(),
            columns: vec![],
            constraints: vec![],
        };
        assert!(stmt.if_not_exists);
        assert_eq!(stmt.name, "users");
    }

    #[test]
    fn test_drop_table_stmt() {
        let stmt = DropTableStmt { if_exists: true, name: "users".to_string() };
        assert!(stmt.if_exists);
        assert_eq!(stmt.name, "users");
    }

    #[test]
    fn test_sql_type() {
        assert!(matches!(SqlType::Integer, SqlType::Integer));
        assert!(matches!(SqlType::Real, SqlType::Real));
        assert!(matches!(SqlType::Text, SqlType::Text));
        assert!(matches!(SqlType::Blob, SqlType::Blob));
        assert!(matches!(SqlType::Boolean, SqlType::Boolean));
        assert!(matches!(SqlType::Null, SqlType::Null));
    }

    #[test]
    fn test_column_constraint() {
        let not_null = ColumnConstraint::NotNull;
        let pk = ColumnConstraint::PrimaryKey { autoincrement: false };
        let unique = ColumnConstraint::Unique;
        assert!(matches!(not_null, ColumnConstraint::NotNull));
        assert!(matches!(pk, ColumnConstraint::PrimaryKey { autoincrement: false }));
        assert!(matches!(unique, ColumnConstraint::Unique));
    }

    #[test]
    fn test_trigger_timing() {
        assert!(matches!(TriggerTiming::Before, TriggerTiming::Before));
        assert!(matches!(TriggerTiming::After, TriggerTiming::After));
        assert!(matches!(TriggerTiming::InsteadOf, TriggerTiming::InsteadOf));
    }

    #[test]
    fn test_trigger_event() {
        assert!(matches!(TriggerEvent::Delete, TriggerEvent::Delete));
        assert!(matches!(TriggerEvent::Insert, TriggerEvent::Insert));
        assert!(matches!(TriggerEvent::Update(None), TriggerEvent::Update(None)));
        if let TriggerEvent::Update(Some(cols)) = TriggerEvent::Update(Some(vec!["id".to_string()])) {
            assert_eq!(cols.len(), 1);
        } else {
            panic!("expected Update with Some");
        }
    }

    #[test]
    fn test_statement() {
        let s = Statement::Begin;
        assert!(matches!(s, Statement::Begin));

        let s = Statement::Commit;
        assert!(matches!(s, Statement::Commit));

        let s = Statement::Rollback;
        assert!(matches!(s, Statement::Rollback));

        let s = Statement::Vacuum;
        assert!(matches!(s, Statement::Vacuum));
    }

    #[test]
    fn test_expr_clone() {
        let e = Expr::LitInt(42);
        let cloned = e.clone();
        assert_eq!(e, cloned);
    }

    #[test]
    fn test_statement_clone() {
        let stmt = Statement::Begin;
        let cloned = stmt.clone();
        assert_eq!(stmt, cloned);
    }
}
