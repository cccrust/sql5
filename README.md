# sql5

SQLite-compatible database with native CJK FTS5 full-text search support. Built with Rust.

## Features

- **Full SQL Support** — SELECT, INSERT, UPDATE, DELETE, CREATE, DROP, ALTER
- **Transactions** — ACID with BEGIN/COMMIT/ROLLBACK
- **WAL Mode** — Write-Ahead Logging for performance
- **Indexes** — CREATE/DROP INDEX, UNIQUE index
- **Views** — CREATE/DROP VIEW
- **Triggers** — CREATE/DROP TRIGGER (BEFORE/AFTER, INSERT/UPDATE/DELETE)
- **Foreign Keys** — With constraint validation
- **Full-Text Search (FTS5)** — CJK bigram tokenization, AND/OR/NOT operators
- **Multiple Databases** — ATTACH DATABASE / DETACH DATABASE
- **VACUUM** — Database compaction
- **PRAGMA** — journal_mode, page_size, cache_size, etc.
- **CTE** — WITH ... AS subqueries
- **Window Functions** — ROW_NUMBER, RANK, LAG, LEAD, FIRST_VALUE, LAST_VALUE
- **String Functions** — UPPER, LOWER, LENGTH, SUBSTR, TRIM, REPLACE, INSTR, GROUP_CONCAT
- **Math Functions** — ABS, ROUND, RANDOM, POWER, SQRT, CEIL, FLOOR
- **Date/Time Functions** — DATE, TIME, DATETIME, JULIANDAY, STRFTIME
- **Aggregate Functions** — COUNT, SUM, AVG, MIN, MAX, TOTAL
- **JSON Functions** — JSON, JSON_INSERT, JSON_REPLACE, JSON_SET, JSON_REMOVE, JSON_TYPE, JSON_VALID

## Installation

### Via pip (recommended)

```bash
pip install sql5
sql5 --version
```

### From source

```bash
# Build
cargo build --release

# Run REPL
cargo run

# Or use directly
./target/release/sql5
```

## Usage

### REPL

```bash
$ sql5
sql5> CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);
sql5> INSERT INTO users VALUES (1, 'Alice');
sql5> INSERT INTO users VALUES (2, 'Bob');
sql5> SELECT * FROM users;
+----+-------+
| id | name  |
+----+-------+
| 1  | Alice |
| 2  | Bob   |
+----+-------+
(2 rows)
sql5> .quit
```

### With database file

```bash
sql5 /path/to/database.db
```

### Command line mode

```bash
echo "SELECT 1 + 1;" | sql5
```

## SQL Examples

### Basic CRUD

```sql
CREATE TABLE t (id INTEGER PRIMARY KEY, name TEXT, age INTEGER);
INSERT INTO t VALUES (1, 'Alice', 30);
INSERT INTO t (name, age) VALUES ('Bob', 25);
UPDATE t SET age = 31 WHERE id = 1;
DELETE FROM t WHERE id = 2;
SELECT * FROM t;
```

### Full-Text Search (FTS5)

```sql
CREATE VIRTUAL TABLE articles USING fts5(title, body);
INSERT INTO articles VALUES ('Hello World', 'The quick brown fox');
INSERT INTO articles VALUES ('Rust Guide', 'Memory safety without GC');

-- Simple search
SELECT * FROM articles WHERE articles MATCH 'rust';

-- Boolean operators
SELECT * FROM articles WHERE articles MATCH 'rust AND memory';
SELECT * FROM articles WHERE articles MATCH 'hello OR world';

-- CJK text
INSERT INTO articles VALUES ('中文測試', '繁體中文全文檢索');
SELECT * FROM articles WHERE articles MATCH '中文';
```

### Triggers

```sql
CREATE TABLE audit_log (id INTEGER, action TEXT, ts TEXT);
CREATE TRIGGER before_delete BEFORE DELETE ON users
BEGIN
  INSERT INTO audit_log VALUES (OLD.id, 'delete', datetime('now'));
END;
DELETE FROM users WHERE id = 1;  -- Triggers audit_log insert
```

### Transactions

```sql
BEGIN;
INSERT INTO t VALUES (1, 'a');
INSERT INTO t VALUES (2, 'b');
COMMIT;  -- or ROLLBACK;
```

### ATTACH DATABASE

```sql
ATTACH DATABASE 'other.db' AS other;
SELECT * FROM other.users;
DETACH DATABASE other;
```

### Window Functions

```sql
SELECT name, ROW_NUMBER() OVER (ORDER BY id) AS rn FROM users;
SELECT name, RANK() OVER (ORDER BY age) FROM users;
SELECT name, LAG(age) OVER (ORDER BY id) AS prev_age FROM users;
```

## Development

### Requirements

- Rust (stable, 1.70+)
- Python 3.8+ (for pip package development)

### Build from source

```bash
# Clone
git clone https://github.com/cccrust/sql5.git
cd sql5

# Build
cargo build --release

# Run tests
cargo test

# CLI integration tests
./test.sh
```

### Publish to PyPI

```bash
# Install version, build and publish
./publish.sh 1.22

# Or manually:
# 1. Update version in:
#    - Cargo.toml
#    - sql5_pypi/sql5/__init__.py
#    - sql5_pypi/pyproject.toml
# 2. Tag and push
git tag v1.22
git push origin v1.22
```

## Version History

| Version | Date | Features |
|---------|------|----------|
| v1.21 | 2026-05-04 | VACUUM (disk mode), Trigger persistence |
| v1.20 | 2026-05-04 | Trigger CRUD, fire_triggers() |
| v1.19 | 2026-05-04 | VACUUM (memory mode) |
| v1.18 | 2026-05-04 | ATTACH/DETACH DATABASE |
| v1.17 | 2026-05-04 | Trigger framework (parser/planner) |
| v1.16 | 2026-05-04 | TRIGGERs planning |
| v1.15 | 2026-05-04 | GLOB/LIKE fix, Table::scan() |
| v1.14 | 2026-05-03 | LIMIT n,m, CROSS/NATURAL JOIN, GLOB |
| v1.5-v1.13 | various | FTS5, WAL, Views, Indexes, etc. |

See [`_doc/`](_doc/) for detailed version notes.

## License

MIT