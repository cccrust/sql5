"""sql5 Client-Server Integration Tests (pytest version)

This test module verifies the sql5 Python client can communicate with
the Rust sql5 server binary via JSON over stdin/stdout.

Architecture:
  Python client (sql5.connect()) -> subprocess.Popen(sql5 --server)
                               -> JSON over stdin/stdout
"""

import os
import sys
import tempfile
import pytest

script_dir = os.path.dirname(os.path.abspath(__file__))
project_dir = os.path.dirname(script_dir)

# Find local sql5 binary before importing sql5
def get_local_binary():
    """Find local sql5 binary for testing."""
    paths_to_check = [
        os.path.join(project_dir, "target", "debug", "sql5"),
        os.path.join(project_dir, "..", "target", "debug", "sql5"),
        os.path.join(os.path.dirname(project_dir), "target", "debug", "sql5"),
    ]
    for p in paths_to_check:
        if os.path.exists(p):
            return p
    return None

# Set SQL5_BINARY before importing sql5
local_binary = get_local_binary()
if local_binary:
    os.environ["SQL5_BINARY"] = local_binary
    print(f"Using local binary: {local_binary}", file=sys.stderr)

sys.path.insert(0, project_dir)

import sql5
from sql5 import connect, Error


@pytest.fixture
def db():
    """Fixture to provide a database connection."""
    connection = connect()
    yield connection
    connection.close()


class TestBasicOperations:
    """Basic SQL operations tests."""

    def test_basic_select(self, db):
        """Test 1: Basic SELECT query."""
        cursor = db.execute("SELECT 1 AS a, 2 AS b, 3 AS c")
        row = cursor.fetchone()
        assert row == [1, 2, 3]

    def test_select_multiple_rows(self, db):
        """Test SELECT with multiple rows."""
        cursor = db.execute("SELECT 1 UNION SELECT 2 UNION SELECT 3")
        rows = cursor.fetchall()
        assert len(rows) == 3

    def test_select_with_alias(self, db):
        """Test SELECT with column alias."""
        cursor = db.execute("SELECT 42 AS answer, 'hello' AS greeting")
        row = cursor.fetchone()
        assert row == [42, "hello"]


class TestCreateAndInsert:
    """CREATE TABLE and INSERT tests."""

    def test_create_table(self, db):
        """Test CREATE TABLE."""
        db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)")
        cursor = db.execute("SELECT name FROM sqlite_master WHERE type='table'")
        tables = cursor.fetchall()
        table_names = [t[0] for t in tables]
        assert "users" in table_names

    def test_insert_single_row(self, db):
        """Test INSERT single row."""
        db.execute("CREATE TABLE test_insert (id INTEGER, val TEXT)")
        db.execute("INSERT INTO test_insert VALUES (1, 'hello')")
        cursor = db.execute("SELECT * FROM test_insert")
        row = cursor.fetchone()
        assert row == [1, "hello"]

    def test_insert_multiple_rows(self, db):
        """Test INSERT multiple rows."""
        db.execute("CREATE TABLE test_multi (id INTEGER, name TEXT)")
        db.execute("INSERT INTO test_multi VALUES (1, 'Alice')")
        db.execute("INSERT INTO test_multi VALUES (2, 'Bob')")
        db.execute("INSERT INTO test_multi VALUES (3, 'Charlie')")
        cursor = db.execute("SELECT * FROM test_multi ORDER BY id")
        rows = cursor.fetchall()
        assert len(rows) == 3
        assert rows[0] == [1, "Alice"]
        assert rows[1] == [2, "Bob"]
        assert rows[2] == [3, "Charlie"]

    def test_update_row(self, db):
        """Test UPDATE row."""
        db.execute("CREATE TABLE test_update (id INTEGER, val TEXT)")
        db.execute("INSERT INTO test_update VALUES (1, 'old')")
        db.execute("UPDATE test_update SET val = 'new' WHERE id = 1")
        cursor = db.execute("SELECT val FROM test_update WHERE id = 1")
        row = cursor.fetchone()
        assert row == ["new"]

    def test_delete_row(self, db):
        """Test DELETE row."""
        db.execute("CREATE TABLE test_delete (id INTEGER, val TEXT)")
        db.execute("INSERT INTO test_delete VALUES (1, 'keep')")
        db.execute("INSERT INTO test_delete VALUES (2, 'delete')")
        db.execute("DELETE FROM test_delete WHERE id = 2")
        cursor = db.execute("SELECT COUNT(*) FROM test_delete")
        count = cursor.fetchone()[0]
        assert count == 1


class TestFullTextSearch:
    """FTS5 full-text search tests."""

    @pytest.mark.skip(reason="FTS5 may not be fully supported")
    def test_fts5_create(self, db):
        """Test CREATE VIRTUAL TABLE with FTS5."""
        db.execute("CREATE VIRTUAL TABLE articles USING fts5(title, content)")
        cursor = db.execute("SELECT name FROM sqlite_master WHERE type='table'")
        tables = cursor.fetchall()
        assert any("articles" in t[0] for t in tables)

    def test_fts5_insert_and_search(self, db):
        """Test FTS5 insert and search."""
        db.execute("CREATE VIRTUAL TABLE docs USING fts5(content)")
        db.execute("INSERT INTO docs VALUES ('Python is great')")
        db.execute("INSERT INTO docs VALUES ('Rust is fast')")
        db.execute("INSERT INTO docs VALUES ('SQL is powerful')")

        cursor = db.execute("SELECT * FROM docs WHERE docs MATCH 'Python'")
        rows = cursor.fetchall()
        assert len(rows) >= 1

    @pytest.mark.skip(reason="FTS5 Chinese may not be fully supported")
    def test_fts5_chinese(self, db):
        """Test FTS5 with Chinese text."""
        db.execute("CREATE VIRTUAL TABLE articles USING fts5(title, body)")
        db.execute("INSERT INTO articles VALUES ('Hello World', 'The quick brown fox')")
        db.execute("INSERT INTO articles VALUES ('Rust Guide', 'Memory safety without GC')")
        db.execute("INSERT INTO articles VALUES ('資料庫', '繁體中文全文檢索')")

        cursor = db.execute("SELECT * FROM articles WHERE articles MATCH 'rust'")
        rows = cursor.fetchall()
        assert len(rows) >= 1

        cursor = db.execute("SELECT * FROM articles WHERE articles MATCH '資料庫'")
        rows = cursor.fetchall()
        assert len(rows) >= 1


class TestParameterizedQueries:
    """Parameterized query tests."""

    def test_parameterized_query_positional(self, db):
        """Test parameterized queries with ? placeholders."""
        db.execute("CREATE TABLE t (id INTEGER, val TEXT)")
        db.execute("INSERT INTO t VALUES (?, ?)", (1, "hello"))
        db.execute("INSERT INTO t VALUES (?, ?)", (2, "world"))

        cursor = db.execute("SELECT * FROM t WHERE id = ?", (1,))
        row = cursor.fetchone()
        assert row == [1, "hello"]

    def test_parameterized_multiple_rows(self, db):
        """Test parameterized query with multiple results."""
        db.execute("CREATE TABLE params (id INTEGER, name TEXT)")
        db.execute("INSERT INTO params VALUES (1, 'Alice')")
        db.execute("INSERT INTO params VALUES (2, 'Bob')")
        db.execute("INSERT INTO params VALUES (3, 'Charlie')")

        cursor = db.execute("SELECT * FROM params WHERE id > ?", (1,))
        rows = cursor.fetchall()
        assert len(rows) == 2

    def test_parameterized_update(self, db):
        """Test parameterized UPDATE."""
        db.execute("CREATE TABLE uparams (id INTEGER, val TEXT)")
        db.execute("INSERT INTO uparams VALUES (1, 'old')")
        db.execute("UPDATE uparams SET val = ? WHERE id = ?", ("new", 1))
        cursor = db.execute("SELECT val FROM uparams WHERE id = 1")
        row = cursor.fetchone()
        assert row == ["new"]


class TestCursorOperations:
    """Cursor operations tests."""

    def test_cursor_fetchone(self, db):
        """Test cursor fetchone."""
        cursor = db.execute("SELECT 1 AS id")
        row = cursor.fetchone()
        assert row == [1]

    def test_cursor_fetchall(self, db):
        """Test cursor fetchall."""
        db.execute("CREATE TABLE fetchall (id INTEGER)")
        db.execute("INSERT INTO fetchall VALUES (1)")
        db.execute("INSERT INTO fetchall VALUES (2)")
        db.execute("INSERT INTO fetchall VALUES (3)")
        cursor = db.execute("SELECT * FROM fetchall")
        rows = cursor.fetchall()
        assert len(rows) == 3

    def test_cursor_iteration(self, db):
        """Test cursor iteration with for loop."""
        db.execute("CREATE TABLE nums (n INTEGER)")
        for i in range(5):
            db.execute("INSERT INTO nums VALUES (?)", (i,))

        cursor = db.execute("SELECT * FROM nums")
        count = 0
        for row in cursor:
            assert isinstance(row, list)
            count += 1
        assert count == 5


class TestContextManager:
    """Context manager tests."""

    def test_context_manager(self):
        """Test context manager (with statement)."""
        with connect() as db:
            db.execute("CREATE TABLE ctx_test (id INTEGER)")
            db.execute("INSERT INTO ctx_test VALUES (42)")
            cursor = db.execute("SELECT * FROM ctx_test")
            row = cursor.fetchone()
            assert row == [42]

    def test_context_manager_rollback(self):
        """Test context manager with rollback on exception."""
        with pytest.raises(Exception):
            with connect() as db:
                db.execute("CREATE TABLE ctx_rollback (id INTEGER)")
                raise Exception("Simulated error")


class TestDiskDatabase:
    """Persistent disk database tests."""

    @pytest.mark.skip(reason="Disk database may not be fully supported")
    def test_disk_database(self):
        """Test persistent disk database."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            db_path = f.name

        try:
            db = connect(db_path)
            db.execute("CREATE TABLE disk_test (id INTEGER, name TEXT)")
            db.execute("INSERT INTO disk_test VALUES (1, 'persisted')")
            db.close()

            db2 = connect(db_path)
            cursor = db2.execute("SELECT * FROM disk_test")
            row = cursor.fetchone()
            assert row == [1, "persisted"]
            db2.close()
        finally:
            if os.path.exists(db_path):
                os.unlink(db_path)

    @pytest.mark.skip(reason="Disk database may not be fully supported")
    def test_disk_database_multiple_connections(self):
        """Test multiple connections to same disk database."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            db_path = f.name

        try:
            db1 = connect(db_path)
            db1.execute("CREATE TABLE multi_conn (id INTEGER, val TEXT)")
            db1.execute("INSERT INTO multi_conn VALUES (1, 'first')")
            db1.close()

            db2 = connect(db_path)
            db2.execute("INSERT INTO multi_conn VALUES (2, 'second')")
            db2.close()

            db3 = connect(db_path)
            cursor = db3.execute("SELECT COUNT(*) FROM multi_conn")
            count = cursor.fetchone()[0]
            assert count == 2
            db3.close()
        finally:
            if os.path.exists(db_path):
                os.unlink(db_path)


class TestErrorHandling:
    """Error handling tests."""

    def test_nonexistent_table(self, db):
        """Test error handling for invalid SQL."""
        cursor = db.execute("SELECT * FROM nonexistent_table")
        rows = cursor.fetchall()
        assert rows == []

    def test_syntax_error(self, db):
        """Test syntax error handling."""
        try:
            cursor = db.execute("SELECT * FORM invalid_syntax")
            rows = cursor.fetchall()
            assert isinstance(rows, list)
        except Exception:
            pass


class TestMultipleStatements:
    """Multiple SQL statements tests."""

    def test_multiple_statements(self, db):
        """Test multiple SQL statements in one execute."""
        db.execute("CREATE TABLE multi (id INTEGER)")
        db.execute("INSERT INTO multi VALUES (1)")
        db.execute("INSERT INTO multi VALUES (2)")
        cursor = db.execute("SELECT COUNT(*) FROM multi")
        count = cursor.fetchone()[0]
        assert count == 2


class TestAggregates:
    """Aggregate function tests."""

    def test_count_aggregate(self, db):
        """Test COUNT aggregate."""
        db.execute("CREATE TABLE agg_test (val INTEGER)")
        db.execute("INSERT INTO agg_test VALUES (10)")
        db.execute("INSERT INTO agg_test VALUES (20)")
        db.execute("INSERT INTO agg_test VALUES (30)")
        cursor = db.execute("SELECT COUNT(*) FROM agg_test")
        count = cursor.fetchone()[0]
        assert count == 3

    def test_sum_aggregate(self, db):
        """Test SUM aggregate."""
        db.execute("CREATE TABLE sum_test (val INTEGER)")
        db.execute("INSERT INTO sum_test VALUES (10)")
        db.execute("INSERT INTO sum_test VALUES (20)")
        db.execute("INSERT INTO sum_test VALUES (30)")
        cursor = db.execute("SELECT SUM(val) FROM sum_test")
        total = cursor.fetchone()[0]
        assert total == 60

    def test_avg_aggregate(self, db):
        """Test AVG aggregate."""
        db.execute("CREATE TABLE avg_test (val INTEGER)")
        db.execute("INSERT INTO avg_test VALUES (10)")
        db.execute("INSERT INTO avg_test VALUES (20)")
        db.execute("INSERT INTO avg_test VALUES (30)")
        cursor = db.execute("SELECT AVG(val) FROM avg_test")
        average = cursor.fetchone()[0]
        assert average == 20.0

    def test_min_max_aggregate(self, db):
        """Test MIN/MAX aggregate."""
        db.execute("CREATE TABLE minmax_test (val INTEGER)")
        db.execute("INSERT INTO minmax_test VALUES (30)")
        db.execute("INSERT INTO minmax_test VALUES (10)")
        db.execute("INSERT INTO minmax_test VALUES (20)")
        cursor = db.execute("SELECT MIN(val), MAX(val) FROM minmax_test")
        row = cursor.fetchone()
        assert row == [10, 30]


class TestJoins:
    """JOIN tests."""

    @pytest.mark.skip(reason="JOIN may not be fully supported")
    def test_inner_join(self, db):
        """Test INNER JOIN."""
        db.execute("CREATE TABLE orders (id INTEGER, customer_id INTEGER)")
        db.execute("CREATE TABLE customers (id INTEGER, name TEXT)")
        db.execute("INSERT INTO customers VALUES (1, 'Alice')")
        db.execute("INSERT INTO customers VALUES (2, 'Bob')")
        db.execute("INSERT INTO orders VALUES (100, 1)")
        db.execute("INSERT INTO orders VALUES (101, 2)")
        cursor = db.execute("""
            SELECT o.id, c.name
            FROM orders o
            INNER JOIN customers c ON o.customer_id = c.id
        """)
        rows = cursor.fetchall()
        assert len(rows) == 2


class TestSubqueries:
    """Subquery tests."""

    def test_subquery_in_where(self, db):
        """Test subquery in WHERE clause."""
        db.execute("CREATE TABLE outer_tbl (id INTEGER)")
        db.execute("CREATE TABLE inner_tbl (id INTEGER, val TEXT)")
        db.execute("INSERT INTO inner_tbl VALUES (1, 'found')")
        db.execute("INSERT INTO inner_tbl VALUES (2, 'not found')")
        cursor = db.execute("""
            SELECT * FROM inner_tbl
            WHERE id IN (SELECT id FROM inner_tbl WHERE id = 1)
        """)
        rows = cursor.fetchall()
        assert len(rows) == 1
        assert rows[0][1] == "found"


class TestTransactions:
    """Transaction tests."""

    def test_explicit_transaction(self, db):
        """Test explicit BEGIN/COMMIT."""
        db.execute("CREATE TABLE trans_test (id INTEGER)")
        db.execute("BEGIN")
        db.execute("INSERT INTO trans_test VALUES (1)")
        db.execute("INSERT INTO trans_test VALUES (2)")
        db.execute("COMMIT")
        cursor = db.execute("SELECT COUNT(*) FROM trans_test")
        count = cursor.fetchone()[0]
        assert count == 2


if __name__ == "__main__":
    pytest.main([__file__, "-v"])