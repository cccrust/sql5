# Parser - 語法分析理論

`src/parser/parser.rs`

## 語法分析器理論

語法分析（Parsing）是編譯器第二階段，將 token 流轉換為 AST。

```
Token 流 → [Parser] → AST → [Planner] → 執行计划
```

## 文法分類

### 上下文無關文法 (CFG)

SQL 語法可用上下文無關文法描述：

```
SELECT_STMT → SELECT COLUMNS FROM TABLE WHERE_CLAUSE
COLUMNS     → COLUMNS ',' COLUMN | '*' | COLUMN
```

### LL(1) 文法

本專案使用遞迴下降 parser，適用於 LL(1) 文法：
- **L**：從左到右掃描
- **L**：最左推導
- **(1)**：前瞻一個 token

## 遞迴下降解析

每個文法規則對應一個函數：

```rust
fn parse_select(&mut self) -> Result<SelectStmt> {
    self.expect(Token::Select)?;
    let columns = self.parse_columns()?;
    let from = self.parse_from()?;
    let where_ = self.parse_where()?;
    Ok(SelectStmt { columns, from, where_, ... })
}
```

## First 集與 Follow 集

### First(α)

文法符號 α 開頭可能出現的 token 集合。

### Follow(A)

文法符號 A 後面可能出現的 token 集合。

用於錯誤恢復和空規則處理。

## 語法錯誤處理

| 策略 | 說明 |
|------|------|
| Panic Mode | 跳過輸入直到同步點 |
| Phrase Level | 嘗試個別插入/刪除 |
| Error Production | 將常見錯誤納入文法 |

## 運算子優先級

透過文法層級實現：

```
EXPR → EXPR + EXPR | EXPR * EXPR | '(' EXPR ')' | PRIMARY
```

`*` 的文法層級低於 `+`，自然形成優先級。

## 左递归消除

左遞迴會導致無限遏迴：

```
EXPR → EXPR + TERM  // 左遞迴
```

改寫為：

```
EXPR → TERM EXPR'
EXPR' → + TERM EXPR' | ε
```

## 理論參考

- Aho, Lam, Sethi, Ullman - "Compilers"
- LL(1) parsing theory
- Recursive descent parsing
- Operator precedence parsing