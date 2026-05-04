# Tokenizer - 分詞演算法理論

`src/fts/tokenizer.rs`

## 分詞的必要性

自然語言需要切分為可索引的單位：

```
英文："Hello, World!" → ["hello", "world"]
中文："你好世界" → ["你", "你好", "好", "世界"]
```

## 英文分詞流程

```
1. Unicode Normalization (NFKC)
2. Case Folding (大小寫統一)
3. Tokenization (分詞)
4. Filtering (過濾停用詞，可選)
5. Stemming (詞幹提取，可選)
```

## Unicode 斷字 (Unicode Segmentation)

基於 Unicode 屬性：

```rust
// Grapheme Cluster Boundary
// Word Boundary
// Sentence Boundary
```

### Unicode Line Breaking Algorithm (UAX #14)

决定在何處換行/斷字：
```
"Hello"  → 可以在 o 後換行
"您好"    → 不可以在字中間換行
```

## Bigram 分詞 (CJK)

### 為何需要特殊處理？

中文、日文、韓文（CJK）沒有自然分界。

### Bigram 演算法

滑動窗口取兩個相鄰字：

```
輸入：你 好 世 界
窗口：[你好] [好世] [世界]
輸出：你/好/你好/好世/世界
```

### 單字 + Bigram 混合

```
輸入："你好"
輸出：
  - 單字：你、好
  - Bigram：你好
```

好處：召回率高，平衡精確度。

## 詞典分詞 (Dictionary-based)

最大匹配演算法（Maximum Matching）：

```
輸入："研究生物"
詞典：研究生、研究、生物

正向最大匹配：
  研究生物 → 研究 + 生物 ✓

反向最大匹配：
  研究生物 → 研究生 + 物 ✗

結果：研究生/生物
```

## 理論參考

- Unicode Standard Annex #29: Text Segmentation
- Unicode Standard Annex #14: Line Breaking
- Stanford NLP - Word Segmentation