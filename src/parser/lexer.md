# Lexer - 詞彙分析理論

`src/parser/lexer.rs`

## 詞彙分析器理論

詞彙分析（Lexical Analysis）是編譯器的第一階段，將輸入字串轉換為 token 序列。

```
原始輸入 → [Lexer] → Token 流 → [Parser] → AST
```

## 有限自動機（Finite Automata）

Lexer 內部使用確定的有限自動機（DFA）來識別 token：

```
字元輸入 → DFA 狀態機 → Token 類型
```

每種 keyword/operator 都對應一個 DFA 狀態圖。

## Token 結構

```rust
pub enum Token {
    Keyword(Keyword),
    Ident(String),
    Number(i64),
    Float(f64),
    String(String),
    Operator(Op),
    Punctuation(Punct),
}
```

## 正規表達式 vs DFA

| 方法 | 特點 |
|------|------|
| 正規表達式 | 簡潔但需轉換為 NFA/DFA |
| 直接 DFA | 快速但複雜 |
| 本專案 | 混合：關鍵字用 DFA，一般識別用規則 |

## 識別策略

### 1. 關鍵字識別
```
IF → 關鍵字 token
IKE → 識別為 IDENTIFIER
```

### 2. 數值識別
```
123 → INTEGER token
3.14 → FLOAT token
0xFF → HEX INTEGER token
```

### 3. 字串識別
```
'hello' → STRING token (支援 escape)
"double" → STRING token
```

### 4. 運算子識別
```
=  → EQ
== → EQEQ
!= → NEQ
```

## 跳過空白

空白字元（space, tab, newline）被 lexer 自動跳過，不產生 token。

## 錯誤處理

無法識別的字元產生 `Token::Illegal` 並記錄位置。

## 理論參考

- Aho, Lam, Sethi, Ullman - "Compilers: Principles, Techniques, and Tools" (Dragon Book)
- 正規語言與有限自動機理論
- Thompson's construction (regex → NFA)
- Hopcroft's algorithm (NFA → DFA)