# Row Model - 資料列模型理論

`src/table/row.rs`

## 關聯式資料模型

關聯式模型中，「列」（Row）是關係的基本單位：

```sql
users(id: INTEGER, name: TEXT, age: INTEGER)
         ↓
    ┌──────────────────────┐
    │ id=1, name='Alice', age=30 │
    │ id=2, name='Bob', age=25  │
    └──────────────────────┘
```

## 元組 (Tuple) vs 行 (Row)

| 術語 | 使用場景 |
|------|----------|
| Tuple | 數學/理論描述 |
| Row | SQL/實際實現 |

兩者本質相同，都是有序的值集合。

## 空值 (NULL) 的表示

SQL 中 NULL 表示「未知或不存在」，有三值邏輯：

| 運算 | 結果 |
|------|------|
| 5 + NULL | NULL |
| 5 = NULL | NULL (非 TRUE/FALSE) |
| NULL AND TRUE | NULL |
| NULL OR FALSE | NULL |

## 值的類型層次

SQL 支援弱類型，類型可以隐式轉換：

```
INTEGER → REAL → TEXT → BLOB
```

## 內嵌 vs 外部儲存

| 方式 | 優點 | 缺點 |
|------|------|------|
| 內嵌 (inline) | 讀取快 | 更新需重寫頁面 |
| 外部 (overflow) | 更新快 | 讀取需多一次 I/O |

本專案採用內嵌儲存。

## 理論參考

- Codd, "A Relational Model of Data for Large Shared Data Banks"
- Date, "The Relational Model"
- Database System Concepts, Chapter 3