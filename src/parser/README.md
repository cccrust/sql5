# Parser - SQL 解析器

將 SQL 字串轉換為 AST（抽象語法樹）。

## 檔案結構

| 檔案 | 說明 | Docs |
|------|------|------|
| [lexer.rs](lexer.rs) | 詞彙分析器 | [lexer.md](lexer.md) |
| [parser.rs](parser.rs) | 語法分析器 | [parser.md](parser.md) |
| [ast.rs](ast.rs) | AST 節點定義 | [ast.md](ast.md) |

## 執行流程

```
SQL String → Lexer → Tokens → Parser → AST
```

## Lexer (`lexer.md`)

將 SQL 字串分解為 tokens。詳見 [lexer.md](lexer.md)。

## Parser (`parser.md`)

遞迴下降_parser，產生 AST。詳見 [parser.md](parser.md)。

## AST (`ast.md`)

所有語法樹節點的定義。詳見 [ast.md](ast.md)。

## 支援的 SQL 語句

- `SELECT` - 查詢
- `INSERT` - 插入
- `UPDATE` - 更新
- `DELETE` - 刪除
- `CREATE/DROP TABLE/INDEX/VIEW/TRIGGER`
- `BEGIN/COMMIT/ROLLBACK` - 交易控制
- `PRAGMA` - 資料庫設定

## 測試

```bash
cargo test parser::lexer  # Lexer 測試
cargo test parser::parser  # Parser 測試
cargo test parser::ast    # AST 測試
```

## 參考

- [planner/](../planner/README.md) - Planner 消費 AST