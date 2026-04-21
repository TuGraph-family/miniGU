"""
Test cases for miniGU Python API.
"""

import pytest
import asyncio


class TestMiniGUBasic:
    """Test basic MiniGU functionality."""

    def test_create_graph(self):
        """Test creating a graph."""
        from minigu import MiniGU

        with MiniGU() as db:
            result = db.create_graph("test_graph")
            assert result is True

    def test_create_graph_with_data(self):
        """Test creating a graph with test data."""
        from minigu import MiniGU

        with MiniGU() as db:
            # Create graph with 5 test vertices (PERSON, COMPANY, CITY nodes)
            result = db.create_graph("test_graph_data", num_vertices=5)
            assert result is True

    def test_execute_query(self):
        """Test executing a GQL query."""
        from minigu import MiniGU

        with MiniGU() as db:
            # Create graph with test data
            db.create_graph("test_query_graph", num_vertices=5)
            # Query PERSON nodes (created by create_test_graph_data)
            result = db.execute("MATCH (n:PERSON) RETURN n.name, n.age LIMIT 10")
            assert hasattr(result, "schema")
            assert hasattr(result, "data")
            assert isinstance(result.schema, list)
            assert isinstance(result.data, list)

    def test_execute_query_with_relationship(self):
        """Test executing a GQL query with relationship traversal."""
        from minigu import MiniGU

        with MiniGU() as db:
            # Create graph with test data
            db.create_graph("test_rel_graph", num_vertices=10)
            # Query PERSON-WORKS_AT-COMPANY relationships
            result = db.execute("MATCH (p:PERSON)-[r:WORKS_AT]->(c:COMPANY) RETURN p.name, c.name LIMIT 10")
            assert hasattr(result, "schema")
            assert hasattr(result, "data")

    def test_context_manager(self):
        """Test context manager protocol."""
        from minigu import MiniGU

        db = MiniGU()
        assert not db.is_connected

        with db:
            assert db.is_connected

        assert not db.is_connected


class TestMiniGULoadSave:
    """Test data loading and saving functionality."""

    @pytest.mark.skip(reason="load() requires graph schema to be defined first; empty graph has no schema")
    def test_load_dict_list(self):
        """Test loading data from a list of dictionaries."""
        from minigu import MiniGU

        with MiniGU() as db:
            db.create_graph("test_load_graph")
            data = [
                {"label": "Person", "name": "Alice", "age": 30},
                {"label": "Person", "name": "Bob", "age": 25}
            ]
            result = db.load(data)
            assert result is True

    def test_load_invalid_data(self):
        """Test loading invalid data format."""
        from minigu import MiniGU, DataError

        with MiniGU() as db:
            db.create_graph("test_invalid_graph")
            with pytest.raises((DataError, Exception)):
                db.load("nonexistent_file.csv")


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
        # Don't call init()
        with pytest.raises(ConnectionError):
            db.execute("MATCH (n:PERSON) RETURN n")

    def test_graph_error(self):
        """Test graph error when creating duplicate graph."""
        from minigu import MiniGU, GraphError

        with MiniGU() as db:
            db.create_graph("test_duplicate_graph")
            # Creating the same graph again should raise an error
            with pytest.raises((GraphError, Exception)):
                db.create_graph("test_duplicate_graph")


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
    async def test_concurrent_queries(self):
        """Test concurrent query execution."""
        from minigu import AsyncMiniGU

        async with AsyncMiniGU() as db:
            await db.create_graph("concurrent_test", num_vertices=10)

            # Execute multiple queries concurrently
            results = await asyncio.gather(
                db.execute("MATCH (n:PERSON) RETURN n.name LIMIT 5"),
                db.execute("MATCH (n:COMPANY) RETURN n.name LIMIT 5"),
                db.execute("MATCH (n:CITY) RETURN n.name LIMIT 5")
            )

            assert len(results) == 3
            for result in results:
                assert hasattr(result, "schema")
                assert hasattr(result, "data")


if __name__ == "__main__":
    pytest.main([__file__, "-v"])