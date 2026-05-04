# sql5

SQLite-compatible database with CJK FTS5 support.

## Installation

```bash
pip install sql5
```

## Usage

```bash
# Run the REPL
sql5

# Open a database file
sql5 /path/to/database.db

# Or use it as a module
python -m sql5
```

## Features

- Full SQL support (SELECT, INSERT, UPDATE, DELETE)
- ACID transactions
- WAL mode
- Foreign keys
- Views
- Triggers
- Full-text search (FTS5) with CJK support
- Multiple database attachment (ATTACH DATABASE)
- And more!

## Requirements

- Python 3.8+

## License

MIT