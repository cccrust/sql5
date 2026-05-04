# Index Scan - 索引查詢理論

`src/planner/plan.rs`

## 索引掃描概念

索引掃描利用索引結構快速定位資料，避免全表掃描。

```
無索引：Table Scan → O(n)
有索引：Index Scan → O(log n) + 範圍讀取
```

## 索引掃描類型

### 唯一掃描 (Point Search)
精確值查詢：
```sql
WHERE id = 5
→ 直接通过 B+Tree 找到
```

### 範圍掃描 (Range Search)
區間查詢：
```sql
WHERE age > 18 AND age <= 30
→ B+Tree 找到起始點，順序讀取
```

### 多索引選擇
多條件查詢可能使用多個索引：
```sql
WHERE age = 25 AND city = 'Taipei'
→ 可用 age_idx 或 city_idx
```

## 索引代價估算

```
Cost = index_probe_cost + range_scan_cost
     = log(m) + (sel × n)
```

## Index Skip Scan

索引跳躍掃描，適用於：
```sql
WHERE last_name = 'Smith'
-- 但查詢包含 first_name
-- 可跳過 last_name 相同的區段
```

## 理論參考

- Database System Concepts, Chapter 15
- Chaudhuri, "An Overview of Query Optimization in Relational Systems"