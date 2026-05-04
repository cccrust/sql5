# REPL - 互動式命令列理論

`src/interface/repl.rs`

## REPL 概念

REPL = **R**ead-**E**valuate-**P**rint **L**oop

```
┌─────────────────────────────────────┐
│  Read     │ 讀取使用者輸入           │
│  Evaluate │ 執行 SQL 語句           │
│  Print    │ 輸出結果                │
│  Loop     │ 返回等待下一個輸入       │
└─────────────────────────────────────┘
```

## REPL 的歷史

起源於 LISP (1960s)：

```lisp
> (+ 2 3)
5
> (defun factorial (n) ...)
FACTORIAL
```

## 為何資料庫需要 REPL？

| 用途 | 說明 |
|------|------|
| 快速查詢 | 即時執行 SQL |
| 偵錯 | 測試 SQL 語句 |
| 教育 | 學習 SQL 語法 |

## 回應式設計 (Responsive Design)

REPL 應即時回應：

```bash
sql5> SELECT 1;      -- 即時回應
+----+
| 1  |
+----+
| 1  |
+----+
sql5>                -- 立即返回提示
```

## 輸出格式化

### 表格視覺化

```
+----+-------+-------+
| id | name  | email |
+----+-------+-------+
| 1  | Alice | a@... |
| 2  | Bob   | b@... |
+----+-------+-------+
```

需計算：
- 最大欄寬
- 欄位對齊
- 邊界字元

### 對齊方式

| 類型 | 方式 |
|------|------|
| 數字 | 右對齊 |
| 文字 | 左對齊 |

## 歷史命令 (History)

常用功能：
- 上/下鍵瀏覽歷史
- `Ctrl+R` 增量搜尋
- `.history` 查看歷史

## 理論參考

- Graham, "ANSI Common Lisp"
- McCarthy, "History of Lisp"
- REPL design patterns in modern CLI tools