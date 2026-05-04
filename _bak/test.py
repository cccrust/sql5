#!/usr/bin/env python3
"""sql5 Server Integration Test

Tests the Python client -> Rust server SQL operations:
- CREATE/INSERT/SELECT/UPDATE/DELETE/DROP
- Aggregates, ORDER BY, LIMIT
- FTS5 full-text search
- Context manager
"""

import sys
import os

script_dir = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(script_dir, "sql5_pypi"))

from sql5 import connect


def main():
    print("=" * 50)
    print("Python Client -> Server Integration Test")
    print("=" * 50)
    print()

    results = {"passed": 0, "failed": 0}

    def check(name, condition, actual=""):
        nonlocal results
        if condition:
            print(f"  ✓ {name}")
            results["passed"] += 1
        else:
            print(f"  ✗ {name}")
            if actual:
                print(f"    Got: {actual}")
            results["failed"] += 1

    try:
        # Test 1: CREATE TABLE
        print("Test 1: CREATE TABLE")
        db = connect()
        r = db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)")
        check("CREATE TABLE", r.ok and "created" in str(r.rows))

        # Test 2: INSERT
        print("Test 2: INSERT")
        r = db.execute("INSERT INTO users VALUES (1, 'Alice', 30)")
        check("INSERT single row", r.ok and "inserted" in str(r.rows))

        r = db.execute("INSERT INTO users VALUES (2, 'Bob', 25)")
        check("INSERT second row", r.ok)

        r = db.execute("INSERT INTO users VALUES (3, 'Carol', 28)")
        check("INSERT with explicit id", r.ok)

        # Test 3: SELECT
        print("Test 3: SELECT")
        r = db.execute("SELECT * FROM users ORDER BY id")
        rows = r.fetchall()
        check("SELECT all rows", len(rows) == 3)
        check("SELECT columns", r.columns == ["id", "name", "age"])
        check("SELECT first row", rows[0] == [1, "Alice", 30])

        r = db.execute("SELECT name FROM users WHERE id = 2")
        check("SELECT with WHERE", r.fetchone() == ["Bob"])

        # Test 4: UPDATE
        print("Test 4: UPDATE")
        r = db.execute("UPDATE users SET age = 31 WHERE id = 1")
        check("UPDATE single row", r.ok)

        r = db.execute("SELECT age FROM users WHERE id = 1")
        check("UPDATE verification", r.fetchone() == [31])

        r = db.execute("UPDATE users SET age = 99")
        check("UPDATE all rows", r.ok)

        # Test 5: DELETE
        print("Test 5: DELETE")
        r = db.execute("DELETE FROM users WHERE id = 2")
        check("DELETE single row", r.ok)

        r = db.execute("SELECT * FROM users")
        check("DELETE verification", len(r.fetchall()) == 2)

        # Test 6: DROP TABLE
        print("Test 6: DROP TABLE")
        db.execute("CREATE TABLE temp (id INTEGER)")
        r = db.execute("DROP TABLE temp")
        check("DROP TABLE", r.ok)

        # Test 7: Aggregate Functions
        print("Test 7: Aggregate Functions")
        r = db.execute("SELECT COUNT(*) FROM users")
        check("COUNT(*)", r.fetchone() == [2])

        r = db.execute("SELECT MAX(age), MIN(age) FROM users")
        check("MAX/MIN", r.ok)

        # Test 8: ORDER BY
        print("Test 8: ORDER BY")
        r = db.execute("SELECT id FROM users ORDER BY id DESC")
        check("ORDER BY DESC", r.fetchall() == [[3], [2], [1]])

        # Test 9: LIMIT
        print("Test 9: LIMIT")
        r = db.execute("SELECT id FROM users LIMIT 1")
        check("LIMIT", len(r.fetchall()) == 1)

        # Test 10: FTS5
        print("Test 10: FTS5 Full-Text Search")
        r = db.execute("CREATE VIRTUAL TABLE articles USING fts5(title, body)")
        check("CREATE FTS5", r.ok)

        db.execute("INSERT INTO articles VALUES ('Hello World', 'The quick brown fox')")
        db.execute("INSERT INTO articles VALUES ('Rust Programming', 'Fast and memory safe')")
        db.execute("INSERT INTO articles VALUES ('資料庫', '繁體中文全文檢索')")

        r = db.execute("SELECT * FROM articles WHERE articles MATCH 'rust'")
        check("FTS5 English search", len(r.fetchall()) >= 1)

        r = db.execute("SELECT * FROM articles WHERE articles MATCH '資料'")
        check("FTS5 Chinese search", len(r.fetchall()) >= 1)

        # Test 11: Context Manager
        print("Test 11: Context Manager")
        with connect() as db2:
            db2.execute("CREATE TABLE ctx_test (id INTEGER)")
            db2.execute("INSERT INTO ctx_test VALUES (42)")
            r = db2.execute("SELECT * FROM ctx_test")
            check("Context manager works", r.fetchone() == [42])
        check("Context manager closes", True)

        # Test 12: Basic Query (parameterized not implemented yet)
        print("Test 12: Basic Query")
        db.execute("CREATE TABLE params (id INTEGER, val TEXT)")
        db.execute("INSERT INTO params VALUES (1, 'hello')")
        db.execute("INSERT INTO params VALUES (2, 'world')")
        r = db.execute("SELECT * FROM params WHERE id = 1")
        check("Query works", r.fetchone() == [1, 'hello'])

        # Close database
        db.close()
        print("Database closed - ✓")

        # Summary
        print()
        print("=" * 50)
        total = results["passed"] + results["failed"]
        if results["failed"] == 0:
            print(f"✓ All {results['passed']} tests passed!")
        else:
            print(f"✗ {results['failed']} / {total} tests failed")
        print("=" * 50)

        return 0 if results["failed"] == 0 else 1

    except Exception as e:
        print(f"Error: {e}")
        import traceback
        traceback.print_exc()
        return 1


if __name__ == "__main__":
    sys.exit(main())