# FTS5 - 全文檢索理論

`src/fts/`

## 全文檢索概念

全文檢索（Full-Text Search）支援自由文字的內容搜尋，区别於精確匹配。

```
關鍵字檢索：WHERE name = 'hello'     # 精確匹配
全文檢索：  WHERE t MATCH 'hello'   # 自由文字搜尋
```

## 倒排索引 (Inverted Index)

核心資料結構：

```
文件1: "hello world"
文件2: "hello"
文件3: "world wide web"

建立索引：
hello → [文件1, 文件2]
world → [文件1, 文件3]
wide  → [文件3]
web   → [文件3]
```

## FTS5 架構

```
文件 → [Tokenizer] → [Indexer] → [Inverted Index]
                                    ↓
查詢 → [Query Parser] → [Index Searcher] → 結果排名
```

## Tokenizer 分詞器

### 英文分詞
```
"Hello, World!" → ["hello", "world"]
```
- 小寫化 (case folding)
- 標點移除 (punctuation removal)
- 詞根還原 (stemming) - 可選

### 中文分詞
```
"你好世界" → ["你", "你好", "好", "世界"]
```
- 單字 + 二元組 (bigram) 混合

## Bigram 分詞策略

CJK 文字没有自然分界符，採用 bigram：

| 方法 | 範例 | 優缺點 |
|------|------|--------|
| 單字 | 你/好/世/界 | 召回率高，精確度低 |
| 全詞 | 你好/好世/世界 | 詞典依賴 |
| Bigram | 你好/好世 | 平衡方案 |
| 混合 | 單字 + Bigram | 最靈活 |

## 布林查詢

| 運算子 | 意義 |
|--------|------|
| AND | 兩詞都在文件中 |
| OR | 任一詞在文件中 |
| NOT | 排除含特定詞的文件 |

```
"rust AND memory" → 包含 rust 且包含 memory
"hello NOT world" → 包含 hello 但不包含 world
```

## 排名算法

| 方法 | 說明 |
|------|------|
| TF-IDF | 詞頻 × 逆向文檔頻率 |
| BM25 | 對長文件懲罰的機率模型 |
| 語言模型 | 基於機率統計 |

## 理論參考

- Zobel & Moffat, "Inverted Files for Text Search Engines"
- Robertson & Zaragoza, "The Probabilistic Relevance Framework: BM25 and Beyond"
- Unicode Standard Annex #29: Text Segmentation