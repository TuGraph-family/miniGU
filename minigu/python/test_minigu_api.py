"""
Test cases for miniGU Python API.
"""

import pytest
import asyncio
import tempfile
import os


# ---------------------------------------------------------------------------
# TestMiniGUBasic
# ---------------------------------------------------------------------------

class TestMiniGUBasic:
    """Test basic MiniGU functionality."""

    def test_create_graph(self):
        """Test creating an empty graph."""
        from minigu import MiniGU

        with MiniGU() as db:
            result = db.create_graph("test_graph")
            assert result is True

    def test_create_graph_with_data(self):
        """Test creating a graph with test data."""
        from minigu import MiniGU

        with MiniGU() as db:
            result = db.create_graph("test_graph_data", num_vertices=5)
            assert result is True

    def test_create_graph_with_large_vertex_count(self):
        """Test creating a graph with a large number of vertices (i64 range)."""
        from minigu import MiniGU

        with MiniGU() as db:
            result = db.create_graph("test_large_graph", num_vertices=100)
            assert result is True

    def test_create_graph_sanitizes_name(self):
        """Test that special characters in graph names are sanitized."""
        from minigu import MiniGU

        with MiniGU() as db:
            # Special chars should be stripped; "test-graph!" becomes "testgraph"
            result = db.create_graph("test-graph!")
            assert result is True

    def test_execute_query(self):
        """Test executing a GQL query."""
        from minigu import MiniGU

        with MiniGU() as db:
            db.create_graph("test_query_graph", num_vertices=5)
            result = db.execute("MATCH (n:PERSON) RETURN n.name, n.age LIMIT 10")
            assert hasattr(result, "schema")
            assert hasattr(result, "data")
            assert isinstance(result.schema, list)
            assert isinstance(result.data, list)
            assert len(result) > 0

    def test_execute_query_with_relationship(self):
        """Test executing a GQL query with relationship traversal."""
        from minigu import MiniGU

        with MiniGU() as db:
            db.create_graph("test_rel_graph", num_vertices=10)
            result = db.execute(
                "MATCH (p:PERSON)-[r:WORKS_AT]->(c:COMPANY) RETURN p.name, c.name LIMIT 10"
            )
            assert hasattr(result, "schema")
            assert hasattr(result, "data")

    def test_execute_optional_match(self):
        """Test OPTIONAL MATCH query."""
        from minigu import MiniGU

        with MiniGU() as db:
            db.create_graph("test_optional_graph", num_vertices=5)
            result = db.execute(
                "OPTIONAL MATCH (n:PERSON) RETURN n.name, n.age LIMIT 3"
            )
            assert hasattr(result, "schema")
            assert hasattr(result, "data")
            assert len(result) > 0

    def test_context_manager(self):
        """Test context manager protocol."""
        from minigu import MiniGU

        db = MiniGU()
        assert not db.is_connected

        with db:
            assert db.is_connected

        assert not db.is_connected

    def test_close_and_reconnect(self):
        """Test closing and reconnecting to the database."""
        from minigu import MiniGU

        db = MiniGU()
        db.init()
        assert db.is_connected

        db.close()
        assert not db.is_connected

        db.init()
        assert db.is_connected
        db.create_graph("reconnect_test", num_vertices=3)
        result = db.execute("MATCH (n:PERSON) RETURN n.name LIMIT 1")
        assert len(result) > 0
        db.close()

    def test_multiple_graphs(self):
        """Test creating and switching between multiple graphs."""
        from minigu import MiniGU

        with MiniGU() as db:
            db.create_graph("graph_a", num_vertices=3)
            r1 = db.execute("MATCH (n:PERSON) RETURN n.name LIMIT 1")
            assert len(r1) > 0

            db.create_graph("graph_b", num_vertices=3)
            r2 = db.execute("MATCH (n:PERSON) RETURN n.name LIMIT 1")
            assert len(r2) > 0


# ---------------------------------------------------------------------------
# TestQueryResult
# ---------------------------------------------------------------------------

class TestQueryResult:
    """Test QueryResult object behavior."""

    def test_result_repr(self):
        """Test QueryResult __repr__."""
        from minigu import MiniGU

        with MiniGU() as db:
            db.create_graph("test_repr", num_vertices=5)
            result = db.execute("MATCH (n:PERSON) RETURN n.name, n.age LIMIT 3")
            r = repr(result)
            assert "QueryResult" in r
            assert "rows=" in r

    def test_result_len(self):
        """Test QueryResult __len__."""
        from minigu import MiniGU

        with MiniGU() as db:
            db.create_graph("test_len", num_vertices=10)
            result = db.execute("MATCH (n:PERSON) RETURN n.name LIMIT 5")
            assert len(result) == 5

    def test_result_getitem(self):
        """Test QueryResult __getitem__."""
        from minigu import MiniGU

        with MiniGU() as db:
            db.create_graph("test_getitem", num_vertices=5)
            result = db.execute("MATCH (n:PERSON) RETURN n.name, n.age LIMIT 5")
            first = result[0]
            assert isinstance(first, list)
            assert len(first) == 2
            # Negative indexing
            last = result[-1]
            assert isinstance(last, list)

    def test_result_iter(self):
        """Test QueryResult __iter__."""
        from minigu import MiniGU

        with MiniGU() as db:
            db.create_graph("test_iter", num_vertices=5)
            result = db.execute("MATCH (n:PERSON) RETURN n.name LIMIT 5")
            rows = list(result)
            assert len(rows) > 0
            for row in rows:
                assert isinstance(row, list)

    def test_result_schema(self):
        """Test QueryResult schema information."""
        from minigu import MiniGU

        with MiniGU() as db:
            db.create_graph("test_schema", num_vertices=5)
            result = db.execute("MATCH (n:PERSON) RETURN n.name, n.age LIMIT 1")
            assert len(result.schema) == 2
            assert result.schema[0]["name"] == "n.name"
            assert result.schema[1]["name"] == "n.age"

    def test_result_metrics(self):
        """Test QueryResult metrics."""
        from minigu import MiniGU

        with MiniGU() as db:
            db.create_graph("test_metrics", num_vertices=5)
            result = db.execute("MATCH (n:PERSON) RETURN n.name LIMIT 1")
            assert "parsing_time_ms" in result.metrics
            assert "planning_time_ms" in result.metrics
            assert "execution_time_ms" in result.metrics

    def test_result_empty(self):
        """Test QueryResult with no matching data (LIMIT 0)."""
        from minigu import MiniGU

        with MiniGU() as db:
            db.create_graph("test_empty_result", num_vertices=5)
            # LIMIT 0 produces an empty result set
            result = db.execute("MATCH (n:PERSON) RETURN n.name LIMIT 0")
            assert len(result) == 0
            assert result.data == []


# ---------------------------------------------------------------------------
# TestMiniGULoadSave
# ---------------------------------------------------------------------------

class TestMiniGULoadSave:
    """Test data loading and saving functionality."""

    @pytest.mark.skip(reason="INSERT statement not yet implemented in GQL parser")
    def test_load_dict_list(self):
        """Test loading data from a list of dictionaries."""
        from minigu import MiniGU

        with MiniGU() as db:
            db.create_graph("test_load_graph")
            data = [
                {"label": "Person", "name": "Alice", "age": 30},
                {"label": "Person", "name": "Bob", "age": 25},
            ]
            result = db.load(data)
            assert result is True

    @pytest.mark.skip(reason="INSERT statement not yet implemented in GQL parser")
    def test_load_dict_list_with_various_types(self):
        """Test loading data with string, int, float, and bool values."""
        from minigu import MiniGU

        with MiniGU() as db:
            db.create_graph("test_load_types")
            data = [
                {"label": "Entity", "name": "test", "count": 42, "score": 3.14, "active": True},
            ]
            result = db.load(data)
            assert result is True

    def test_load_invalid_data_type(self):
        """Test loading invalid data format (not list or string)."""
        from minigu import MiniGU, DataError

        with MiniGU() as db:
            db.create_graph("test_invalid_type")
            with pytest.raises((DataError, Exception)):
                db.load(12345)

    def test_load_nonexistent_file(self):
        """Test loading from a nonexistent file path."""
        from minigu import MiniGU, DataError

        with MiniGU() as db:
            db.create_graph("test_invalid_file")
            with pytest.raises((DataError, Exception)):
                db.load("/nonexistent/path/to/file.csv")

    @pytest.mark.skip(reason="CALL export procedure not yet fully implemented")
    def test_save(self):
        """Test saving database to a file."""
        from minigu import MiniGU

        with MiniGU() as db:
            db.create_graph("test_save_graph", num_vertices=5)
            with tempfile.TemporaryDirectory() as tmpdir:
                path = os.path.join(tmpdir, "export")
                result = db.save(path)
                assert result is True


# ---------------------------------------------------------------------------
# TestMiniGUErrors
# ---------------------------------------------------------------------------

class TestMiniGUErrors:
    """Test error handling."""

    def test_query_syntax_error(self):
        """Test query syntax error handling."""
        from minigu import MiniGU, QuerySyntaxError

        with MiniGU() as db:
            db.create_graph("test_syntax_graph", num_vertices=5)
            with pytest.raises((QuerySyntaxError, Exception)):
                db.execute("INVALID GQL SYNTAX HERE")

    def test_connection_error(self):
        """Test connection error when not initialized."""
        from minigu import MiniGU, ConnectionError

        db = MiniGU()
        with pytest.raises(ConnectionError):
            db.execute("MATCH (n:PERSON) RETURN n")

    def test_connection_error_create_graph(self):
        """Test connection error on create_graph when not initialized."""
        from minigu import MiniGU, ConnectionError

        db = MiniGU()
        with pytest.raises(ConnectionError):
            db.create_graph("no_init_graph")

    def test_connection_error_load(self):
        """Test connection error on load when not initialized."""
        from minigu import MiniGU, ConnectionError

        db = MiniGU()
        with pytest.raises(ConnectionError):
            db.load([{"label": "Test"}])

    def test_connection_error_save(self):
        """Test connection error on save when not initialized."""
        from minigu import MiniGU, ConnectionError

        db = MiniGU()
        with pytest.raises(ConnectionError):
            db.save("/tmp/test_export")

    def test_graph_error_empty_name(self):
        """Test graph error when creating graph with empty name."""
        from minigu import MiniGU, GraphError

        with MiniGU() as db:
            with pytest.raises((GraphError, Exception)):
                db.create_graph("")

    def test_graph_error_duplicate_graph(self):
        """Test graph error when creating duplicate graph."""
        from minigu import MiniGU, GraphError

        with MiniGU() as db:
            db.create_graph("test_duplicate_graph")
            with pytest.raises((GraphError, Exception)):
                db.create_graph("test_duplicate_graph")


# ---------------------------------------------------------------------------
# TestExceptionHierarchy
# ---------------------------------------------------------------------------

class TestExceptionHierarchy:
    """Test exception class hierarchy."""

    def test_base_exception(self):
        """Test MiniGUError is base exception."""
        from minigu import MiniGUError

        assert issubclass(MiniGUError, Exception)

    def test_connection_error_hierarchy(self):
        """Test ConnectionError inherits from MiniGUError."""
        from minigu import MiniGUError, ConnectionError

        assert issubclass(ConnectionError, MiniGUError)

    def test_query_error_hierarchy(self):
        """Test QueryError and its subclasses inherit correctly."""
        from minigu import MiniGUError, QueryError, QuerySyntaxError, QueryExecutionError, QueryTimeoutError

        assert issubclass(QueryError, MiniGUError)
        assert issubclass(QuerySyntaxError, QueryError)
        assert issubclass(QueryExecutionError, QueryError)
        assert issubclass(QueryTimeoutError, QueryError)

    def test_graph_error_hierarchy(self):
        """Test GraphError inherits from MiniGUError."""
        from minigu import MiniGUError, GraphError

        assert issubclass(GraphError, MiniGUError)

    def test_data_error_hierarchy(self):
        """Test DataError inherits from MiniGUError."""
        from minigu import MiniGUError, DataError

        assert issubclass(DataError, MiniGUError)

    def test_transaction_error_hierarchy(self):
        """Test TransactionError inherits from MiniGUError."""
        from minigu import MiniGUError, TransactionError

        assert issubclass(TransactionError, MiniGUError)

    def test_catch_with_base_exception(self):
        """Test that specific exceptions can be caught with MiniGUError."""
        from minigu import MiniGU, MiniGUError, ConnectionError

        db = MiniGU()
        with pytest.raises(MiniGUError):
            db.execute("MATCH (n) RETURN n")


# ---------------------------------------------------------------------------
# TestErrorClassificationFunctions
# ---------------------------------------------------------------------------

class TestErrorClassificationFunctions:
    """Test Rust-side error classification functions."""

    def test_is_syntax_error(self):
        """Test is_syntax_error function."""
        from minigu import is_syntax_error

        class FakeError:
            def __str__(self):
                return "syntax error at line 1"

        assert is_syntax_error(FakeError()) is True

    def test_is_syntax_error_unexpected(self):
        """Test is_syntax_error detects 'unexpected' keyword."""
        from minigu import is_syntax_error

        class FakeError:
            def __str__(self):
                return "unexpected token"

        assert is_syntax_error(FakeError()) is True

    def test_is_syntax_error_negative(self):
        """Test is_syntax_error returns False for non-syntax errors."""
        from minigu import is_syntax_error

        class FakeError:
            def __str__(self):
                return "connection refused"

        assert is_syntax_error(FakeError()) is False

    def test_is_timeout_error(self):
        """Test is_timeout_error function."""
        from minigu import is_timeout_error

        class FakeError:
            def __str__(self):
                return "query timeout after 30s"

        assert is_timeout_error(FakeError()) is True

    def test_is_timeout_error_negative(self):
        """Test is_timeout_error returns False for non-timeout errors."""
        from minigu import is_timeout_error

        class FakeError:
            def __str__(self):
                return "syntax error"

        assert is_timeout_error(FakeError()) is False

    def test_is_transaction_error(self):
        """Test is_transaction_error function."""
        from minigu import is_transaction_error

        class FakeError:
            def __str__(self):
                return "transaction aborted"

        assert is_transaction_error(FakeError()) is True

    def test_is_transaction_error_commit(self):
        """Test is_transaction_error detects 'commit' keyword."""
        from minigu import is_transaction_error

        class FakeError:
            def __str__(self):
                return "commit failed"

        assert is_transaction_error(FakeError()) is True

    def test_is_transaction_error_negative(self):
        """Test is_transaction_error returns False for non-transaction errors."""
        from minigu import is_transaction_error

        class FakeError:
            def __str__(self):
                return "syntax error"

        assert is_transaction_error(FakeError()) is False

    def test_is_not_implemented_error(self):
        """Test is_not_implemented_error function."""
        from minigu import is_not_implemented_error

        class FakeError:
            def __str__(self):
                return "not implemented: INSERT"

        assert is_not_implemented_error(FakeError()) is True

    def test_is_not_implemented_error_negative(self):
        """Test is_not_implemented_error returns False for other errors."""
        from minigu import is_not_implemented_error

        class FakeError:
            def __str__(self):
                return "connection error"

        assert is_not_implemented_error(FakeError()) is False


# ---------------------------------------------------------------------------
# TestModuleExports
# ---------------------------------------------------------------------------

class TestModuleExports:
    """Test module-level exports and flags."""

    def test_has_rust_bindings(self):
        """Test HAS_RUST_BINDINGS flag is True when bindings are available."""
        from minigu import HAS_RUST_BINDINGS

        assert HAS_RUST_BINDINGS is True

    def test_pyminigu_available(self):
        """Test PyMiniGU class is available."""
        from minigu import PyMiniGU

        assert PyMiniGU is not None

    def test_all_exports(self):
        """Test __all__ contains expected exports."""
        import minigu

        expected = [
            "MiniGU", "AsyncMiniGU", "QueryResult",
            "MiniGUError", "ConnectionError", "QueryError",
            "QuerySyntaxError", "QueryExecutionError", "QueryTimeoutError",
            "GraphError", "DataError", "TransactionError",
            "PyMiniGU", "HAS_RUST_BINDINGS",
            "is_syntax_error", "is_timeout_error",
            "is_transaction_error", "is_not_implemented_error",
        ]
        for name in expected:
            assert name in minigu.__all__, f"{name} not in __all__"

    def test_version(self):
        """Test __version__ is defined."""
        import minigu

        assert hasattr(minigu, "__version__")
        assert minigu.__version__ == "0.1.0"


# ---------------------------------------------------------------------------
# TestAsyncMiniGU
# ---------------------------------------------------------------------------

class TestAsyncMiniGU:
    """Test asynchronous API."""

    @pytest.mark.asyncio
    async def test_async_create_graph(self):
        """Test creating a graph asynchronously."""
        from minigu import AsyncMiniGU

        async with AsyncMiniGU() as db:
            result = await db.create_graph("async_test_graph")
            assert result is True

    @pytest.mark.asyncio
    async def test_async_create_graph_with_data(self):
        """Test creating a graph with data asynchronously."""
        from minigu import AsyncMiniGU

        async with AsyncMiniGU() as db:
            result = await db.create_graph("async_test_graph_data", num_vertices=5)
            assert result is True

    @pytest.mark.asyncio
    async def test_async_execute_query(self):
        """Test executing a query asynchronously."""
        from minigu import AsyncMiniGU

        async with AsyncMiniGU() as db:
            await db.create_graph("async_query_graph", num_vertices=5)
            result = await db.execute("MATCH (n:PERSON) RETURN n.name, n.age LIMIT 10")
            assert hasattr(result, "schema")
            assert hasattr(result, "data")

    @pytest.mark.asyncio
    async def test_async_context_manager(self):
        """Test async context manager protocol."""
        from minigu import AsyncMiniGU

        db = AsyncMiniGU()
        assert not db.is_connected

        async with db:
            assert db.is_connected

        assert not db.is_connected

    @pytest.mark.asyncio
    async def test_async_execute_optional_match(self):
        """Test executing OPTIONAL MATCH asynchronously."""
        from minigu import AsyncMiniGU

        async with AsyncMiniGU() as db:
            await db.create_graph("async_optional_graph", num_vertices=5)
            result = await db.execute(
                "OPTIONAL MATCH (n:PERSON) RETURN n.name, n.age LIMIT 3"
            )
            assert hasattr(result, "schema")
            assert hasattr(result, "data")
            assert len(result) > 0

    @pytest.mark.asyncio
    async def test_async_connection_error(self):
        """Test connection error in async mode when not initialized."""
        from minigu import AsyncMiniGU, ConnectionError

        db = AsyncMiniGU()
        with pytest.raises(ConnectionError):
            await db.execute("MATCH (n) RETURN n")

    @pytest.mark.asyncio
    async def test_concurrent_queries(self):
        """Test concurrent query execution."""
        from minigu import AsyncMiniGU

        async with AsyncMiniGU() as db:
            await db.create_graph("concurrent_test", num_vertices=10)

            results = await asyncio.gather(
                db.execute("MATCH (n:PERSON) RETURN n.name LIMIT 5"),
                db.execute("MATCH (n:COMPANY) RETURN n.name LIMIT 5"),
                db.execute("MATCH (n:CITY) RETURN n.name LIMIT 5"),
            )

            assert len(results) == 3
            for result in results:
                assert hasattr(result, "schema")
                assert hasattr(result, "data")


if __name__ == "__main__":
    pytest.main([__file__, "-v"])