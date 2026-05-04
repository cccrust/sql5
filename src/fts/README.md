# FTS5 - 全文檢索

`src/fts/`

## 模組結構

| 檔案 | 說明 |
|------|------|
| `mod.rs` | 模組入口 |
| `fts_table.rs` | FTS5 虛擬表格 |
| `tokenizer.rs` | 分詞器 (含 CJK 支援) |
| `index.rs` | 倒排索引 |

## FtsTable FTS5 表格

```rust
pub struct FtsTable {
    name: String,
    tokenizer: Tokenizer,
    index: FtsIndex,
}
```

### 建立 FTS5 表格

```sql
CREATE VIRTUAL TABLE articles USING fts5(title, content, tokenize='unicode61');
```

### 插入文件

```sql
INSERT INTO articles (title, content) VALUES ('Hello World', 'This is a test document.');
```

### 全文檢索

```sql
SELECT * FROM articles WHERE articles MATCH 'hello';
```

### 搭配 MATCH 的 WHERE 子句

```sql
SELECT * FROM articles WHERE articles MATCH 'test' AND id > 10;
```

## Tokenizer 分詞器

```rust
pub trait Tokenizer {
    fn tokenize(&self, text: &str) -> Vec<Token>;
}
```

### 内建分詞器

| 分詞器 | 說明 |
|--------|------|
| `unicode61` | Unicode 標準分詞 (預設) |
| `porter` | Porter 詞幹提取 |
| `cjk` | CJK 支援 (bigram + unicode) |

## CJK 分詞策略

1. **Unicode 斷字** - 依據 Unicode 屬性
2. **Bigram 分詞** - 鄰近中文字為一個詞
3. **混合模式** - 英文/數字按字，中文按 bigram

### 示例

```
輸入: "Hello 你好 World"
輸出: ["hello", "你", "你好", "好", "world"]
```

## FtsIndex 倒排索引

```rust
pub struct FtsIndex {
    postings: BTreeMap<String, Vec<DocumentId>>,
}
```

### 索引結構

```text
詞 -> [文件ID列表]
hello -> [1, 5, 9]
world -> [3, 7]
```

## 測試

```bash
cargo test fts
```