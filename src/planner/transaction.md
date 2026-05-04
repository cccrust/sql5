# Transaction - 交易理論

`src/planner/transaction.rs`

## 交易概念

交易（Transaction）是資料庫操作的基本單元，具有 ACID 特性。

```
BEGIN;
UPDATE accounts SET balance = balance - 100 WHERE id = 1;
UPDATE accounts SET balance = balance + 100 WHERE id = 2;
COMMIT;
```

## ACID 特性

### Atomicity（原子性）
交易要么全部成功，要么全部失敗。

### Consistency（一致性）
交易執行前後資料庫狀態都是有效的。

### Isolation（隔離性）
併發交易的執行結果與順序執行相同。

### Durability（持久性）
已提交的交易結果不會丢失。

## 並發控制問題

| 問題 | 說明 |
|------|------|
| Lost Update | 兩個交易同時修改，都被覆蓋 |
| Dirty Read | 讀取未提交交易的修改 |
| Non-repeatable Read | 兩次讀取結果不同 |
| Phantom Read | 新增/刪除記錄導致結果改變 |

## 隔離等級

| 等級 | Dirty Read | Non-repeatable | Phantom |
|------|------------|----------------|---------|
| READ UNCOMMITTED | 可能 | 可能 | 可能 |
| READ COMMITTED | 不可能 | 可能 | 可能 |
| REPEATABLE READ | 不可能 | 不可能 | 可能 |
| SERIALIZABLE | 不可能 | 不可能 | 不可能 |

本專案支援 READ COMMITTED。

## 兩段鎖定 (2PL)

確保可序列化：
```
擴展階段：取得鎖
收縮階段：釋放鎖
不得再獲取新鎖
```

## 死結 (Deadlock)

```
T1: lock(A) → wait(B)
T2: lock(B) → wait(A)
```

處理方式：
- 預防：層級順序鎖定
- 檢測：等待圖
- 超時：強制回滾

## 理論參考

- Gray & Reuter, "Transaction Processing: Concepts and Techniques"
- Bernstein, Hadzilacos, Goodman, "Concurrency Control and Recovery in Database Systems"
- Database System Concepts, Chapter 14