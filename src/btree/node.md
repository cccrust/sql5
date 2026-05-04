# Node - B+Tree 節點

`src/btree/node.rs`

## BTreeNode

```rust
pub enum BTreeNode {
    Internal(InternalNode),
    Leaf(LeafNode),
}
```

## InternalNode

```rust
pub struct InternalNode {
    pub children: Vec<(Vec<u8>, PageId)>,
    pub next: Option<PageId>,
}
```

## LeafNode

```rust
pub struct LeafNode {
    pub entries: Vec<(Vec<u8>, Vec<u8>)>,
    pub next: Option<PageId>,
}
```

## 節點操作

```rust
impl BTreeNode {
    pub fn is_leaf(&self) -> bool;
    pub fn len(&self) -> usize;
    pub fn insert(&mut self, key: &[u8], value: &[u8]) -> Result<()>;
    pub fn search(&self, key: &[u8]) -> Option<&Vec<u8>>;
}
```

## 分裂閾值

當節點大小超過 `order * 2` 時觸發分裂。

## 測試

```bash
cargo test node
```