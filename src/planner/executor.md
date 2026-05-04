# Query Planner - 查詢規劃理論

`src/planner/planner.rs`

## 查詢規劃角色

Query Planner 將 SQL 語句轉換為可執行的運算元序列。

```
SQL → [Parser] → AST → [Planner] → Plan → [Executor] → 結果
```

## 邏輯 vs 實體計劃

### 邏輯計劃
表達「做什麼」，與執行無關：
```
SELECT * FROM users WHERE age > 18
↓
Project(*)
 └─ Select(age > 18)
     └─ Table(users)
```

### 實體計劃
表達「如何做」，考慮索引、join 順序：
```
Project(*)
 └─ IndexScan(users_age_idx, age > 18)
```

## 代價模型 (Cost Model)

評估不同計劃的執行代價：

```
Cost = CPU_cost + I/O_cost × w
     = rows × cpu_per_row + pages × disk_sequential
```

## 選擇率 (Selectivity)

估算過濾後剩餘的比例：

```sql
WHERE age > 18
-- 若 age 均勻分佈 0-100
-- 選擇率 ≈ (100-18)/100 = 0.82
```

## 等價轉換

SQL 可有多種等價表示：

```sql
SELECT * FROM a, b WHERE a.id = b.id
≡ SELECT * FROM a JOIN b ON a.id = b.id
≡ SELECT * FROM a INNER JOIN b USING(id)
```

## 常見執行策略

### Table Scan
全表掃描，O(n) 時間。

### Index Scan
使用索引，O(log n) 定位起始位置。

### Nested Loop Join
```
for each row in outer:
    for each row in inner:
        if match: output
```

### Hash Join
建立 hash 表後進行匹配。

## 理論參考

- Garcia-Molina, Ullman, Widom, "Database Systems: The Complete Book"
- Selinger et al., "Access Path Selection in a Relational Database Management System"
- Ioannidis, "Query Optimization"