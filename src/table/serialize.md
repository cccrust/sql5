# Serialization - 資料序列化理論

`src/table/serialize.rs`

## 序列化概念

序列化（Serialization）將記憶體中的資料結構轉換為位元組流，以便儲存或傳輸。

```
記憶體中的 Row → [序列化] → 磁碟上的位元組
磁碟上的位元組 → [反序列化] → 記憶體中的 Row
```

## 為何需要序列化？

| 原因 | 說明 |
|------|------|
| 持久化 | 資料需儲存到磁碟 |
| 網路傳輸 | 資料需在程序間傳遞 |
| 程序重啟 | 資料結構重建 |

## 定長 vs 變長編碼

### 定長編碼
每種類型佔用固定位元組：
```
Integer: 8 位元組
Float: 8 位元組 (IEEE 754)
```

### 變長編碼 (Varint)
小數值用少位元組：
```
1     → 0x01 (1 byte)
127   → 0x7F (1 byte)
128   → 0x8001 (2 bytes)
16383 → 0xFF7F (2 bytes)
```

優點：節省空間

## SQLite 的資料格式

### NULL
```
0x00
```

### Integer
使用 Varint，小端序：
```
1-byte:  0x01-0xFB (1-251)
2-byte:  0xFC + 2 bytes
3-byte:  0xFD + 3 bytes
8-byte:  0xFE + 8 bytes
```

### Text/Blob
長度前綴 + 實際資料：
```
[length][UTF-8 bytes]
```

## 字节序 (Endianness)

| 類型 | 順序 |
|------|------|
| Little-endian | 低位元組在前（x86, ARM） |
| Big-endian | 高位元組在前（網路協定） |

SQLite 使用 Big-endian（網路序）儲存多位元組整數。

## 理論參考

- Oracle, "Technical Correspondence: Serialization"
- Gray & Reuter, "Transaction Processing" - 有關格式穩定性
- SQLite File Format: https://www.sqlite.org/fileformat.html