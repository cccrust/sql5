# Constraint - 完整性約束理論

`src/planner/constraints.rs`

## 完整性約束目的

確保資料庫狀態的正確性和有效性：

```sql
-- 約束違反
UPDATE account SET balance = -100 WHERE id = 1; -- 不允許餘額為負
```

## 約束類型

### 鍵約束 (Key Constraint)

確保記錄唯一性：
```sql
PRIMARY KEY (id)          -- 唯一，非空
UNIQUE (email)            -- 唯一（可為空）
```

### 實體完整性 (Entity Integrity)
每張表必須有主鍵。

### 參照完整性 (Referential Integrity)

外鍵約束：
```sql
FOREIGN KEY (dept_id) REFERENCES departments(id)
```

保證：
- 子表的外鍵值存在於父表
- 或為 NULL

### 域約束 (Domain Constraint)

值必須在允許範圍內：
```sql
CHECK (age >= 0 AND age <= 150)
CHECK (gender IN ('M', 'F'))
```

### 複合約束

多欄位約束：
```sql
UNIQUE (first_name, last_name, birth_date)
```

## 約束檢查時機

| 時機 | 說明 |
|------|------|
| IMMEDIATE | 語句結束時檢查 |
| DEFERRED | 交易提交時檢查 |

## 違反處理

約束違反時：
1. 交易回滾
2. 返回錯誤碼

## 理論參考

- Date, "The Relational Database Dictionary"
- Database System Concepts, Chapter 3