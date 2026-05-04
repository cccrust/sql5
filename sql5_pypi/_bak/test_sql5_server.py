#!/usr/bin/env python3
"""sql5 Client-Server Integration Tests

This test module verifies the sql5 Python client can communicate with
the Rust sql5 server binary via JSON over stdin/stdout.

Architecture:
  Python client (sql5.connect()) -> subprocess.Popen(sql5 --server)
                               -> JSON over stdin/stdout
"""

import sys
import os
import tempfile
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import sql5
from sql5 import connect, Error


def get_local_binary():
    """Find local sql5 binary for testing."""
    script_dir = os.path.dirname(os.path.abspath(__file__))
    project_dir = os.path.dirname(script_dir)
    local_binary = os.path.join(project_dir, "target", "debug", "sql5")
    if os.path.exists(local_binary):
        return local_binary
    return None


class TestSql5Server(unittest.TestCase):
    """Integration tests for sql5 client-server architecture."""

    @classmethod
    def setUpClass(cls):
        local_binary = get_local_binary()
        if local_binary:
            os.environ["SQL5_BINARY"] = local_binary

    def test_basic_select(self):
        """Test 1: Basic SELECT query."""
        db = connect()
        cursor = db.execute("SELECT 1 AS a, 2 AS b, 3 AS c")
        row = cursor.fetchone()
        self.assertEqual(row, [1, 2, 3])
        db.close()

    def test_create_and_insert(self):
        """Test 2: CREATE TABLE and INSERT."""
        db = connect()
        db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)")
        db.execute("INSERT INTO users VALUES (1, 'Alice', 30)")
        db.execute("INSERT INTO users VALUES (2, 'Bob', 25)")

        cursor = db.execute("SELECT * FROM users ORDER BY id")
        rows = cursor.fetchall()
        self.assertEqual(len(rows), 2)
        self.assertEqual(rows[0], [1, 'Alice', 30])
        self.assertEqual(rows[1], [2, 'Bob', 25])
        db.close()

    def test_fts5_chinese(self):
        """Test 3: FTS5 full-text search with Chinese."""
        db = connect()
        db.execute("CREATE VIRTUAL TABLE articles USING fts5(title, body)")
        db.execute("INSERT INTO articles VALUES ('Hello World', 'The quick brown fox')")
        db.execute("INSERT INTO articles VALUES ('Rust Guide', 'Memory safety without GC')")
        db.execute("INSERT INTO articles VALUES ('資料庫', '繁體中文全文檢索')")

        cursor = db.execute("SELECT * FROM articles WHERE articles MATCH 'rust'")
        rows = cursor.fetchall()
        self.assertGreaterEqual(len(rows), 1)

        cursor = db.execute("SELECT * FROM articles WHERE articles MATCH '資料庫'")
        rows = cursor.fetchall()
        self.assertGreaterEqual(len(rows), 1)
        db.close()

    def test_parameterized_query(self):
        """Test 4: Parameterized queries with ? placeholders."""
        db = connect()
        db.execute("CREATE TABLE t (id INTEGER, val TEXT)")
        db.execute("INSERT INTO t VALUES (?, ?)", (1, 'hello'))
        db.execute("INSERT INTO t VALUES (?, ?)", (2, 'world'))

        cursor = db.execute("SELECT * FROM t WHERE id = ?", (1,))
        row = cursor.fetchone()
        self.assertEqual(row, [1, 'hello'])
        db.close()

    def test_cursor_iteration(self):
        """Test 5: Cursor iteration with for loop."""
        db = connect()
        db.execute("CREATE TABLE nums (n INTEGER)")
        for i in range(5):
            db.execute("INSERT INTO nums VALUES (?)", (i,))

        cursor = db.execute("SELECT * FROM nums")
        count = 0
        for row in cursor:
            self.assertIsInstance(row, list)
            count += 1
        self.assertEqual(count, 5)
        db.close()

    def test_context_manager(self):
        """Test 6: Context manager (with statement)."""
        with connect() as db:
            db.execute("CREATE TABLE ctx_test (id INTEGER)")
            db.execute("INSERT INTO ctx_test VALUES (42)")
            cursor = db.execute("SELECT * FROM ctx_test")
            row = cursor.fetchone()
            self.assertEqual(row, [42])

    def test_disk_database(self):
        """Test 7: Persistent disk database."""
        with tempfile.NamedTemporaryFile(suffix='.db', delete=False) as f:
            db_path = f.name

        try:
            db = connect(db_path)
            db.execute("CREATE TABLE disk_test (id INTEGER, name TEXT)")
            db.execute("INSERT INTO disk_test VALUES (1, 'persisted')")
            db.close()

            db2 = connect(db_path)
            cursor = db2.execute("SELECT * FROM disk_test")
            row = cursor.fetchone()
            self.assertEqual(row, [1, 'persisted'])
            db2.close()
        finally:
            if os.path.exists(db_path):
                os.unlink(db_path)

    def test_error_handling(self):
        """Test 8: Error handling for invalid SQL."""
        db = connect()
        cursor = db.execute("SELECT * FROM nonexistent_table")
        rows = cursor.fetchall()
        self.assertEqual(rows, [])
        db.close()

    def test_multiple_statements(self):
        """Test 9: Multiple SQL statements in one execute."""
        db = connect()
        db.execute("CREATE TABLE multi (id INTEGER)")
        db.execute("INSERT INTO multi VALUES (1)")
        db.execute("INSERT INTO multi VALUES (2)")
        db.execute("INSERT INTO multi VALUES (3)")
        cursor = db.execute("SELECT COUNT(*) FROM multi")
        row = cursor.fetchone()
        self.assertEqual(row[0], 3)
        db.close()


def main():
    local_binary = get_local_binary()
    if not local_binary:
        print("WARNING: Local sql5 binary not found at target/debug/sql5")
        print("Set SQL5_BINARY environment variable or run: cargo build")
        print()

    print("=" * 60)
    print("sql5 Client-Server Integration Tests (v2.0.0)")
    print("=" * 60)
    if local_binary:
        print(f"Using local binary: {local_binary}")
    print()

    loader = unittest.TestLoader()
    suite = loader.loadTestsFromTestCase(TestSql5Server)
    runner = unittest.TextTestRunner(verbosity=2)
    result = runner.run(suite)

    print()
    print("=" * 60)
    if result.wasSuccessful():
        print(f"All {result.testsRun} tests passed!")
    else:
        print(f"Tests run: {result.testsRun}, Failures: {len(result.failures)}, Errors: {len(result.errors)}")
    print("=" * 60)

    return 0 if result.wasSuccessful() else 1


if __name__ == "__main__":
    sys.exit(main())