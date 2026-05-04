# AST - 抽象語法樹理論

`src/parser/ast.rs`

## 抽象語法樹概念

AST 是程式的抽象結構表示，只保留語法結構中對語意分析重要的部分，忽略空白、註解等語法細節。

```
       SELECT
      / | \
   ...  ...  ...
```

## AST vs Parse Tree

| 特性 | Parse Tree | AST |
|------|-----------|-----|
| 完整度 | 包含所有節點 | 僅保留語意相關節點 |
| 大小 | 較大 | 較精簡 |
| 用途 | 語法驗證 | 語意分析 |

## 樹狀結構表示

AST 是一種樹狀資料結構：
- **根節點**：語句類型（SELECT, INSERT...）
- **內部節點**：運算式、條件
- **葉節點**：變數、 literal

## 表達式結構

### 階層式表達式

```
       BinOp(+)
      /         \
   Column(a)   Number(5)
```

### 邏輯表達式

```
       BinOp(AND)
      /           \
  IsNull(x)    BinOp(>)
              /      \
         Column(y)  Number(0)
```

## 遍歷方式

| 方式 | 順序 |
|------|------|
| 前序遍歷 | 根 → 左 → 右 |
| 後序遍歷 | 左 → 右 → 根 |
| 中序遍歷 | 左 → 根 → 右（不常用於 AST） |

## 表達式 vs 語句

### 表達式 (Expression)
- 有值
- 可巢狀
- 例：`a + b * c`, `CASE WHEN ... THEN ... END`

### 語句 (Statement)
- 執行副作用
- 例：`SELECT`, `INSERT`, `CREATE TABLE`

## SQL 特殊性

SQL 是宣告式語言，AST 結構與命令式語言不同：

1. **集合導向** - 不像傳統語言的變數賦值
2. **宣告式** - 描述「什麼」而非「如何」
3. **巢狀查詢** - 子查詢作為 table  expression

## 理論參考

- AST 作為語義分析的中介表示
- Tree traversal algorithms
- Recursive descent parsing theory