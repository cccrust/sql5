# WAL - 預寫式日誌理論

`src/pager/wal.rs`

## Write-Ahead Logging 概念

WAL 的核心原則：「在將修改寫入磁碟前，先將操作記錄到日誌」。

```
日誌寫入 → 資料寫入 → 提交確認
    ↑___________|
        先完成
```

## 日誌序列號 (LSN)

每筆記錄有唯一遞增的 LSN：

```
LSN 1: BEGIN tx
LSN 2: UPDATE page 5
LSN 3: COMMIT
```

## 日誌記錄類型

| 類型 | 內容 |
|------|------|
| UPDATE | 修改前後值 (before/after image) |
| INSERT | 新插入的記錄 |
| DELETE | 被刪除的記錄 |
| BEGIN | 交易開始 |
| COMMIT | 交易提交 |
| ROLLBACK | 交易取消 |

## 恢復程序

### 分析階段
從 WAL 尾端回推，建立活躍交易列表。

### 重做階段
對已提交交易的修改進行重做（若未刷到磁碟）。

### 撤銷階段
對未提交交易的修改進行撤銷。

## WAL 優點

| 優點 | 說明 |
|------|------|
| 效能提升 | 讀寫不阻塞 |
| 原子性 | COMMIT 前可恢復 |
| 增量檢查點 | 不需寫完整資料庫 |

## ARIES 演算法

IBM 研究的經典 WAL 恢復演算法：

- **A**dditive **R**ecoverable **I**ncremental **E**nvironment with **S**afeness
- 支援細粒度鎖定
- 統計分析最少重做

## WAL 與 Shadow Paging

| 特性 | WAL | Shadow Paging |
|------|-----|---------------|
| 空間使用 | 持續增長 | 固定 |
| 寫入放大 | 較低 | 中等 |
| 實現複雜度 | 中等 | 簡單 |
| 併發效能 | 高 | 中等 |

## 理論參考

- Weikum & Vossen, "Transactional Information Systems"
- Mohan et al., "ARIES: A Transaction Recovery Method Supporting Fine-Grained Locking"
- Database System Concepts, Chapter 14: Transaction Processing