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

### Python Client

```python
import sql5

# Subprocess mode (default, v2.0 compatible)
db = sql5.connect("mydb.db")

# WebSocket mode (v3.0 new, for multi-client)
db = sql5.connect(
    path="mydb.db",
    transport="websocket",
    host="127.0.0.1",
    port=8080
)

# Execute SQL
cursor = db.execute("SELECT * FROM users")
print(cursor.fetchall())

db.close()
```

### Command line mode

```bash
echo "SELECT 1 + 1;" | sql5
```

### Server Modes

sql5 supports three modes:

| Mode | Command | Description |
|------|---------|-------------|
| REPL | `sql5` or `sql5 db.db` | Interactive CLI |
| stdio server | `sql5 --server [db]` | JSON over stdin/stdout (for Python client) |
| WebSocket server | `sql5 --websocket <port> [db]` | WebSocket server (multi-client) |

#### WebSocket Server

```bash
# Start on default port 8080
sql5 --websocket 8080

# With database file
sql5 --websocket 8080 mydb.db

# Custom port
sql5 --websocket 9000
```

See [Python Client](#python-client) for WebSocket usage.

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
# Directly upload to PyPI (auto-updates version in all files)
./pub.sh 2.0.1 pypi

# Or push to GitHub (triggers CI to upload to PyPI)
./pub.sh 2.0.1 github
```

## Documentation

Full documentation available in `src/` directory:

| Module | Source | Docs |
|--------|--------|------|
| parser | [parser.rs](src/parser/) | [README](src/parser/README.md) |
| planner | [planner.rs](src/planner/) | [README](src/planner/README.md) |
| btree | [btree.rs](src/btree/) | [README](src/btree/README.md) |
| pager | [pager.rs](src/pager/) | [README](src/pager/README.md) |
| table | [table.rs](src/table/) | [README](src/table/README.md) |
| catalog | [catalog.rs](src/catalog/) | [README](src/catalog/README.md) |
| fts | [fts.rs](src/fts/) | [README](src/fts/README.md) |
| interface | [interface.rs](src/interface/) | [README](src/interface/README.md) |

Individual file documentation: each `.rs` file has a corresponding `.md` file in the same directory.

## Version History

| Version | Date | Features |
|---------|------|----------|
| v3.0.0 | 2026-05-04 | WebSocket server (multi-client support) |
| v2.4.2 | 2026-05-04 | CI/CD improvements, separate platform builds |
| v2.0.0 | 2026-05-04 | Client-server architecture |
| v1.22 | 2026-05-04 | Various fixes |
| v1.21 | 2026-05-04 | VACUUM (disk mode), Trigger persistence |

See [`_doc/`](_doc/) for detailed version notes.

## License

MIT