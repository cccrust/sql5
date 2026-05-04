//! 邏輯計畫節點（Logical Plan）
//!
//! AST → Plan 之後，executor 依照計畫執行。
//! 每個節點代表一個關聯代數運算子。

use crate::parser::ast::{Expr, OrderItem, SelectItem};

/// 邏輯計畫樹（遞迴結構）
#[derive(Debug, Clone)]
pub enum Plan {
    /// 全表掃描
    SeqScan {
        table: String,
        alias: Option<String>,
        filter: Option<Expr>,          // 下推的 WHERE 條件
    },

    /// 主鍵 / 索引查詢（等值）
    IndexScan {
        table:  String,
        alias:  Option<String>,
        column: String,
        value:  Expr,
    },

    /// 投影：選取欄位 / 運算式
    Projection {
        input:   Box<Plan>,
        columns: Vec<SelectItem>,
    },

    /// 篩選（WHERE）
    Filter {
        input:  Box<Plan>,
        expr:   Expr,
    },

    /// 排序（ORDER BY）
    Sort {
        input:  Box<Plan>,
        keys:   Vec<OrderItem>,
    },

    /// 分頁（LIMIT / OFFSET）
    Limit {
        input:  Box<Plan>,
        limit:  Option<u64>,
        offset: u64,
    },

    /// Nested-loop join
    Join {
        left:      Box<Plan>,
        right:     Box<Plan>,
        condition: Option<Expr>,
        kind:      JoinKind,
    },

    /// 聚合（GROUP BY + 聚合函式）
    Aggregate {
        input:    Box<Plan>,
        group_by: Vec<Expr>,
        having:   Option<Expr>,
        outputs:  Vec<SelectItem>,
    },

    /// 去重（DISTINCT）
    Distinct {
        input: Box<Plan>,
    },

    /// INSERT
    Insert {
        table:   String,
        columns: Vec<String>,
        source:  InsertSource,
        default_values: bool,
    },

    /// UPDATE
    Update {
        table:  String,
        input:  Box<Plan>,          // 通常是 SeqScan + Filter
        sets:   Vec<(String, Expr)>,
    },

    /// DELETE
    Delete {
        table: String,
        input: Box<Plan>,
    },

    /// CREATE TABLE
    CreateTable {
        stmt: crate::parser::ast::CreateTableStmt,
    },

    /// DROP TABLE
    DropTable {
        name:      String,
        if_exists: bool,
    },

    /// CREATE INDEX
    CreateIndex {
        stmt: crate::parser::ast::CreateIndexStmt,
    },

    /// DROP INDEX
    DropIndex {
        name:      String,
        if_exists: bool,
    },

    /// ALTER TABLE
    AlterTable {
        stmt: crate::parser::ast::AlterTableStmt,
    },

    /// PRAGMA
    Pragma {
        name:  String,
        value: Option<Expr>,
    },

    /// EXPLAIN
    Explain {
        inner: Box<Plan>,
    },

    /// CREATE VIEW
    CreateView {
        stmt: crate::parser::ast::CreateViewStmt,
    },

    /// DROP VIEW
    DropView {
        name:      String,
        if_exists: bool,
    },

    /// CREATE TRIGGER
    CreateTrigger {
        stmt: crate::parser::ast::CreateTriggerStmt,
    },

    /// DROP TRIGGER
    DropTrigger {
        name:      String,
        if_exists: bool,
    },

    /// REINDEX
    Reindex {
        name: Option<String>,
    },

    /// ANALYZE
    Analyze {
        name: Option<String>,
    },

    /// 子查詢作為掃描來源（FROM (SELECT ...)）
    SubqueryScan {
        query:  Box<Plan>,
        alias:  String,
    },

    /// CTE 展開（WITH name AS (query) SELECT ...）
    Cte {
        definitions: Vec<(String, Box<Plan>)>,
        query:       Box<Plan>,
    },

    /// UNION / UNION ALL
    SetOperation {
        left:  Box<Plan>,
        right: Box<Plan>,
        op:    SetOp,
    },

    /// 空計畫（BEGIN / COMMIT / ROLLBACK）
    Transaction(TransactionOp),

    /// ATTACH DATABASE
    Attach {
        path:  String,
        alias: String,
    },

    /// DETACH DATABASE
    Detach {
        alias: String,
    },

    /// VACUUM（資料庫整理）
    Vacuum,
}

#[derive(Debug, Clone)]
pub enum InsertSource {
    Values(Vec<Vec<Expr>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum JoinKind {
    Inner, Left, Cross,
}

#[derive(Debug, Clone)]
pub enum TransactionOp {
    Begin, Commit, Rollback,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetOp {
    Union,
    UnionAll,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_source_values() {
        use crate::parser::ast::Expr;
        let values = vec![
            vec![Expr::LitInt(1), Expr::LitStr("a".to_string())],
            vec![Expr::LitInt(2), Expr::LitStr("b".to_string())],
        ];
        let source = InsertSource::Values(values);
        match source {
            InsertSource::Values(v) => assert_eq!(v.len(), 2),
        }
    }

    #[test]
    fn test_join_kind_inner() {
        assert!(matches!(JoinKind::Inner, JoinKind::Inner));
        assert!(!matches!(JoinKind::Left, JoinKind::Inner));
    }

    #[test]
    fn test_join_kind_left() {
        assert!(matches!(JoinKind::Left, JoinKind::Left));
    }

    #[test]
    fn test_join_kind_cross() {
        assert!(matches!(JoinKind::Cross, JoinKind::Cross));
    }

    #[test]
    fn test_transaction_op_begin() {
        assert!(matches!(TransactionOp::Begin, TransactionOp::Begin));
    }

    #[test]
    fn test_transaction_op_commit() {
        assert!(matches!(TransactionOp::Commit, TransactionOp::Commit));
    }

    #[test]
    fn test_transaction_op_rollback() {
        assert!(matches!(TransactionOp::Rollback, TransactionOp::Rollback));
    }

    #[test]
    fn test_set_op_union() {
        assert!(matches!(SetOp::Union, SetOp::Union));
    }

    #[test]
    fn test_set_op_union_all() {
        assert!(matches!(SetOp::UnionAll, SetOp::UnionAll));
    }

    #[test]
    fn test_plan_seq_scan() {
        let plan = Plan::SeqScan {
            table: "users".to_string(),
            alias: None,
            filter: None,
        };
        assert!(matches!(plan, Plan::SeqScan { .. }));
    }

    #[test]
    fn test_plan_index_scan() {
        use crate::parser::ast::Expr;
        let plan = Plan::IndexScan {
            table: "users".to_string(),
            alias: None,
            column: "id".to_string(),
            value: Expr::LitInt(42),
        };
        assert!(matches!(plan, Plan::IndexScan { .. }));
    }

    #[test]
    fn test_plan_projection() {
        use crate::parser::ast::{Expr, SelectItem};
        let inner = Plan::SeqScan {
            table: "users".to_string(),
            alias: None,
            filter: None,
        };
        let plan = Plan::Projection {
            input: Box::new(inner),
            columns: vec![SelectItem::Expr {
                expr: Expr::Column { table: None, name: "name".to_string() },
                alias: None,
            }],
        };
        assert!(matches!(plan, Plan::Projection { .. }));
    }

    #[test]
    fn test_plan_filter() {
        use crate::parser::ast::{Expr, BinOp};
        let inner = Plan::SeqScan {
            table: "users".to_string(),
            alias: None,
            filter: None,
        };
        let plan = Plan::Filter {
            input: Box::new(inner),
            expr: Expr::BinOp {
                left: Box::new(Expr::Column { table: None, name: "age".to_string() }),
                op: BinOp::Gt,
                right: Box::new(Expr::LitInt(18)),
            },
        };
        assert!(matches!(plan, Plan::Filter { .. }));
    }

    #[test]
    fn test_plan_insert() {
        use crate::parser::ast::Expr;
        let plan = Plan::Insert {
            table: "users".to_string(),
            columns: vec!["name".to_string()],
            source: InsertSource::Values(vec![vec![Expr::LitStr("Alice".to_string())]]),
            default_values: false,
        };
        assert!(matches!(plan, Plan::Insert { .. }));
    }

    #[test]
    fn test_plan_delete() {
        let inner = Plan::SeqScan {
            table: "users".to_string(),
            alias: None,
            filter: None,
        };
        let plan = Plan::Delete {
            table: "users".to_string(),
            input: Box::new(inner),
        };
        assert!(matches!(plan, Plan::Delete { .. }));
    }

    #[test]
    fn test_plan_create_table() {
        let stmt = crate::parser::ast::CreateTableStmt {
            if_not_exists: false,
            name: "users".to_string(),
            columns: vec![],
            constraints: vec![],
        };
        let plan = Plan::CreateTable { stmt };
        assert!(matches!(plan, Plan::CreateTable { .. }));
    }

    #[test]
    fn test_plan_drop_table() {
        let plan = Plan::DropTable {
            name: "users".to_string(),
            if_exists: true,
        };
        assert!(matches!(plan, Plan::DropTable { .. }));
    }

    #[test]
    fn test_plan_explain() {
        let inner = Plan::SeqScan {
            table: "users".to_string(),
            alias: None,
            filter: None,
        };
        let plan = Plan::Explain {
            inner: Box::new(inner),
        };
        assert!(matches!(plan, Plan::Explain { .. }));
    }

    #[test]
    fn test_plan_transaction() {
        let plan = Plan::Transaction(TransactionOp::Begin);
        assert!(matches!(plan, Plan::Transaction(TransactionOp::Begin)));
    }

    #[test]
    fn test_plan_limit() {
        let inner = Plan::SeqScan {
            table: "users".to_string(),
            alias: None,
            filter: None,
        };
        let plan = Plan::Limit {
            input: Box::new(inner),
            limit: Some(10),
            offset: 0,
        };
        assert!(matches!(plan, Plan::Limit { .. }));
    }

    #[test]
    fn test_plan_sort() {
        use crate::parser::ast::Expr;
        let inner = Plan::SeqScan {
            table: "users".to_string(),
            alias: None,
            filter: None,
        };
        let plan = Plan::Sort {
            input: Box::new(inner),
            keys: vec![OrderItem {
                expr: Expr::Column { table: None, name: "name".to_string() },
                asc: true,
            }],
        };
        assert!(matches!(plan, Plan::Sort { .. }));
    }

    #[test]
    fn test_plan_clone() {
        let plan = Plan::SeqScan {
            table: "users".to_string(),
            alias: None,
            filter: None,
        };
        let cloned = plan.clone();
        assert!(matches!(cloned, Plan::SeqScan { .. }));
    }
}
