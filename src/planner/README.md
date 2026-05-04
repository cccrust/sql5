# Planner - 查詢規劃與執行

將 AST 轉換為可執行的查询计划。

## 檔案結構

| 檔案 | 說明 | Docs |
|------|------|------|
| [mod.rs](mod.rs) | 模組入口 | - |
| [planner.rs](planner.rs) | 查詢規劃主邏輯 | - |
| [executor.rs](executor.rs) | 執行引擎 | [executor.md](executor.md) |
| [plan.rs](plan.rs) | 執行计划節點定義 | [plan.md](plan.md) |
| [transaction.rs](transaction.rs) | 交易管理 | [transaction.md](transaction.md) |
| [constraints.rs](constraints.rs) | 約束驗證 | [constraints.md](constraints.md) |
| [datetime.rs](datetime.rs) | 日期時間函數 | [datetime.md](datetime.md) |

## Planner

```rust
pub struct Planner<S: Storage> {
    catalog: Catalog,
    storage: S,
}
```

Planner 負責將 AST 轉換為可執行的查询计划。

### 主要方法

```rust
impl<S: Storage> Planner<S> {
    pub fn new(storage: S) -> Self;
    pub fn plan(&mut self, stmt: Statement) -> Result<Box<dyn Executor>>;
    pub fn plan_select(&mut self, stmt: SelectStmt) -> Result<SelectPlan>;
}
```

## Executor trait

```rust
pub trait Executor {
    fn execute(&mut self) -> Result<QueryResult>;
}
```

## Plan 節點類型

```rust
pub enum PlanNode {
    Scan { table: String, filter: Option<Expr> },
    IndexScan { table: String, index: String, range: (Option<Expr>, Option<Expr>) },
    Insert { table: String, values: Vec<Vec<Expr>> },
    Update { table: String, set: Vec<(String, Expr)>, filter: Option<Expr> },
    Delete { table: String, filter: Option<Expr> },
    Aggregate { aggs: Vec<Expr>, group_by: Vec<Expr> },
    Sort { order_by: Vec<OrderItem> },
    Limit { limit: Expr, offset: Option<Expr> },
}
```

## 交易支援

```rust
pub enum TransactionState {
    Idle,
    Active,
    Committed,
    RolledBack,
}
```

### 交易語句

```sql
BEGIN
COMMIT
ROLLBACK
```

## 約束檢查

```rust
pub fn check_constraints(
    row: &Row,
    constraints: &[TableConstraint],
) -> Result<()>
```

## 日期時間函數

| 函數 | 說明 |
|------|------|
| `date()` | 解析日期 |
| `time()` | 解析時間 |
| `datetime()` | 解析日期時間 |
| `strftime()` | 格式化日期時間 |

## 測試

```bash
cargo test planner
```