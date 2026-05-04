# Inverted Index - 倒排索引理論

`src/fts/index.rs`

## 倒排索引概念

文件 → 詞的映射，支援快速全文搜尋：

```
正排索引：文件 → 內容
倒排索引：詞 → 文件列表
```

## 基本結構

```rust
struct InvertedIndex {
    // 詞 → [(doc_id, positions...)]
    postings: HashMap<String, Vec<Posting>>,
}

struct Posting {
    doc_id: i64,
    positions: Vec<usize>,
    // 可選：frequency, field...
}
```

## 文件範例

```
文檔1: "hello world"
文檔2: "hello"
文檔3: "hello world wide"
```

## 建立的倒排索引

```
hello → [doc1: [0], doc2: [0], doc3: [0]]
world → [doc1: [1], doc3: [1]]
wide  → [doc3: [2]]
```

## 布林查詢演算法

### AND 查詢
取交集：
```
"hello AND world"
hello: [doc1, doc2, doc3]
world: [doc1, doc3]
結果：[doc1, doc3] ∩ [doc1, doc3] = [doc1, doc3]
```

### OR 查詢
取聯集：
```
"hello OR world"
hello: [doc1, doc2, doc3]
world: [doc1, doc3]
結果：[doc1, doc2, doc3] ∪ [doc1, doc3] = [doc1, doc2, doc3]
```

### NOT 查詢
取差集：
```
"hello NOT world"
hello: [doc1, doc2, doc3]
world: [doc1, doc3]
結果：[doc1, doc2, doc3] - [doc1, doc3] = [doc2]
```

## 排名 (Ranking)

### TF (Term Frequency)
```
TF(t,d) = 詞 t 在文檔 d 出現次數
```

### IDF (Inverse Document Frequency)
```
IDF(t) = log(總文檔數 / 含 t 的文檔數)
```

### TF-IDF
```
score(t,d) = TF(t,d) × IDF(t)
```

文件分數 = 所有查詢詞分數總和。

## 壓縮

倒排索引可能很大，需壓縮：

| 方法 | 說明 |
|------|------|
| Variable-length encoding | 小數字用少位元組 |
| Front-coding | 前綴共享 |
| Bitmap | DocID 用 bitmap |

## 理論參考

- Zobel & Moffat, "Inverted Files for Text Search Engines"
- Manning, Raghavan, Schütze, "Introduction to Information Retrieval"
- Büttcher, Clarke, Cormack, "Information Retrieval"