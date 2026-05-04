# Pager - 儲存引擎

`src/pager/`

## 檔案結構

| 檔案 | 說明 | Docs |
|------|------|------|
| [mod.rs](mod.rs) | 模組入口 | - |
| [storage.rs](storage.rs) | 儲存抽象介面 | [storage.md](storage.md) |
| [wal.rs](wal.rs) | WAL 日誌實作 | [wal.md](wal.md) |
| [codec.rs](codec.rs) | 頁面編碼/解碼 | [codec.md](codec.md) |

## Storage trait

```rust
pub trait Storage: Send + Sync {
    fn read_page(&self, page_id: PageId) -> Result<Page>;
    fn write_page(&mut self, page: &Page) -> Result<PageId>;
    fn allocate_page(&mut self) -> Result<PageId>;
    fn deallocate_page(&mut self, page_id: PageId) -> Result<()>;
    fn flush(&mut self) -> Result<()>;
}
```

## Page 頁面

```rust
pub struct Page {
    pub id: PageId,
    pub data: Vec<u8>,
    pub checksum: u32,
}
```

## WAL (Write-Ahead Log)

```rust
pub struct Wal {
    log_pages: Vec<Page>,
    committed: bool,
}
```

### WAL 運作流程

1. **記錄** - 修改前先寫 WAL
2. **刷寫** - WAL 刷到磁碟
3. ** checkpoint** - 將修改合併回主檔案

### WAL 模式特點

| 特性 | 說明 |
|------|------|
| 原子性 | 交易要么全部成功，要么全部失敗 |
| 快速復原 | 可從 WAL 恢復未刷寫的修改 |
| 讀寫不阻塞 | 讀操作不被寫入阻塞 |

## Codec 頁面編碼

```rust
pub fn encode_page(page: &Page) -> Vec<u8>;
pub fn decode_page(data: &[u8]) -> Result<Page>;
pub fn compute_checksum(data: &[u8]) -> u32;
```

## 測試

```bash
cargo test pager
```