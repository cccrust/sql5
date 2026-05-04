# Query Planner - 查詢規劃理論

`src/planner/planner.rs`

## 查詢規劃的角色

Query Planner 是資料庫系統的核心元件，負責將 SQL 語句轉換為可執行的查詢计划：

```
SQL 字串 → [Parser] → AST → [Planner] → Plan → [Executor] → 結果
```

## 代數優化 (Algebraic Optimization)

### 查詢代數

SQL 可表示為關聯代數運算：

```sql
SELECT name FROM users WHERE age > 18
```

等价於：

```
π_name (σ_age>18 (users))
```

其中：
- π = Projection (選擇欄位)
- σ = Selection (過濾條件)

### 等價變換

相同的結果可能有多種表達：

```sql
σ_{A>5} (R ⨝ S) ≡ R ⨝ σ_{A>5} (S)  -- 條件提前
π_{X} (σ_{A>5} (R)) ≡ σ_{A>5} (π_{X,A} (R))  -- 投影提前
```

## 代價模型 (Cost Model)

評估不同计划的執行代價：

```
Cost = CPU_cost + I/O_cost × weight

典型估計：
- 全表掃描：O(n) 其中 n 為總頁數
- 索引掃描：O(log n) + 範圍頁數
- 巢狀迴圈 Join：O(n × m)
```

## 啟發式優化 (Heuristic Optimization)

本專案採用的簡化策略：

### 1. 條件下推 (Predicate Pushdown)

將過濾條件尽可能下推到資料來源：

```
較差：σ (π (R))          -- 先投影再過濾
較佳：π (σ (R))          -- 先過濾再投影
```

### 2. 主鍵等值 → 索引掃描

```sql
WHERE id = 42  -- 若 id 是主鍵
→ IndexScan(id = 42)  -- 直接索引查找
```

### 3. 其他過濾 → SeqScan + filter

```sql
WHERE name = 'Alice'  -- 非主鍵欄位
→ SeqScan(filter: name = 'Alice')  -- 全表掃描 + 記憶體過濾
```

## 查詢计划的生成流程

```
1. Parse SELECT 語句
2. 處理 FROM：建立基礎掃描
3. 處理 JOIN：追加 Join 節點
4. 處理 WHERE：條件下推或包裝 Filter
5. 處理 GROUP BY：包裝 Aggregate 節點
6. 處理 ORDER BY：包裝 Sort 節點
7. 處理 LIMIT：包裝 Limit 節點
8. 處理投影：包裝 Projection（或在 Aggregate 中處理）
```

## 邏輯計劃 vs 實體計劃

### 邏輯計劃（Logical Plan）

描述「做什麼」，與執行無關：

```
Plan::Aggregate { group_by: [city], ... }
 └─ Plan::SeqScan { table: "users", ... }
```

### 實體計劃（Physical Plan）

描述「如何做」：

```
HashJoin
 ├─ SeqScan(users)
 └─ IndexScan(orders, user_id)
```

## 理論參考

- Selinger et al., "Access Path Selection in a Relational Database Management System" (1979)
- Garcia-Molina, Ullman, Widom, "Database Systems: The Complete Book"
- Ioannidis, "Query Optimization" (ACM Computing Surveys, 1996)