# B+Tree 索引實作

`src/btree/`

## 檔案結構

| 檔案 | 說明 | Docs |
|------|------|------|
| [mod.rs](mod.rs) | 模組入口 | - |
| [tree.rs](tree.rs) | B+Tree 主體實現 | [tree.md](tree.md) |
| [node.rs](node.rs) | B+Tree 節點操作 | [node.md](node.md) |

## BTree

```rust
pub struct BTree<S: Storage> {
    root: Option<PageId>,
    storage: S,
}
```

### 主要方法

```rust
impl<S: Storage> BTree<S> {
    pub fn new(storage: S) -> Self;
    pub fn insert(&mut self, key: &[u8], value: &[u8]) -> Result<()>;
    pub fn search(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;
    pub fn range_scan(&self, start: &[u8], end: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>>;
    pub fn delete(&mut self, key: &[u8]) -> Result<()>;
}
```

## Node 節點結構

```rust
pub enum BTreeNode {
    Internal {
        children: Vec<(Vec<u8>, PageId)>,
    },
    Leaf {
        entries: Vec<(Vec<u8>, Vec<u8>)>,
        next: Option<PageId>,
    },
}
```

## 分頁大小

預設分頁大小為 4096 位元組。

## 測試

```bash
cargo test btree
```