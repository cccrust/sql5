# Table - 表格儲存理論

`src/table/`

## 堆組織 (Heap Organization)

無特定順序的資料頁集合：

```
Page 1: [row1][row2][row3]...
Page 2: [row4][row5]...
```

優點：插入快
缺點：範圍查詢需掃描

## 行式儲存 vs 列式儲存

| 特性 | 行式儲存 (Row-store) | 列式儲存 (Column-store) |
|------|---------------------|------------------------|
| 讀取單列 | 需讀整行 | 直接讀取 |
| OLTP 適合度 | 高 | 低 |
| OLAP 適合度 | 低 | 高 |
| 壓縮 | 較差 | 較好 |

本專案採用行式儲存。

## Slot 目錄

每頁包含 slot 目錄用於可變長度記錄：

```
+--------+--------+----------+
| Header | Slot 0 | Slot 1   | Slot 2
+--------+--------+----------+
                          ↓
              +--------+-----+
              | record | ... |
              +--------+-----+
```

## NULL 值表示

| 方法 | 說明 |
|------|------|
| Null Bitmap | 每欄一位元組標記 |
| Null Column 省略 | 可變長欄位專用 |
| 特殊值 | 如 -1, '' 等 |

## MVCC (多版本併發控制)

支援快照隔離：
- 讀取不阻塞寫入
- 寫入不阻塞讀取
- 每筆資料有多個版本

## 理論參考

- Database System Concepts, Chapter 13
- Stonebraker, "The Design and Implementation of PostgreSQL"