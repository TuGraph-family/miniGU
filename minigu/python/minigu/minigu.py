"""
miniGU Python API

This module provides Python bindings for the miniGU graph database.
"""

import asyncio
from typing import Any, Dict, Iterator, List, Optional, Union


# Exception hierarchy
class MiniGUError(Exception):
    """Base exception class for miniGU database"""
    pass


class ConnectionError(MiniGUError):
    """Database connection error"""
    pass


class QueryError(MiniGUError):
    """Base query execution error"""
    pass


class QuerySyntaxError(QueryError):
    """Query syntax error"""
    pass


class QueryExecutionError(QueryError):
    """Query execution error"""
    pass


class QueryTimeoutError(QueryError):
    """Query timeout error"""
    pass


class DataError(MiniGUError):
    """Data loading/saving error"""
    pass


class GraphError(MiniGUError):
    """Graph creation/manipulation error"""
    pass


class TransactionError(MiniGUError):
    """Transaction error"""
    pass


def _handle_exception(e: Exception) -> None:
    """
    Handle exceptions from the Rust backend and convert them to appropriate Python exceptions.
    """
    error_msg = str(e)
    error_lower = error_msg.lower()

    # Syntax errors
    if ("syntax" in error_lower and "error" in error_lower) or \
       "unexpected" in error_lower or \
       ("invalid" in error_lower and "syntax" in error_lower):
        raise QuerySyntaxError(f"Invalid query syntax: {error_msg}")

    # Timeout errors
    elif "timeout" in error_lower:
        raise QueryTimeoutError(f"Query execution timed out: {error_msg}")

    # Transaction errors
    elif "transaction" in error_lower or \
         "txn" in error_lower or \
         "commit" in error_lower or \
         "rollback" in error_lower:
        raise TransactionError(f"Transaction operation failed: {error_msg}")

    # Not implemented errors
    elif "not implemented" in error_lower or \
         "not yet implemented" in error_lower:
        raise MiniGUError(f"Requested feature is not yet implemented: {error_msg}")

    # Connection errors
    elif "session" in error_lower or "database" in error_lower or "connect" in error_lower:
        raise ConnectionError(f"Database connection error: {error_msg}")

    # Graph errors
    elif "graph" in error_lower:
        raise GraphError(f"Graph operation failed: {error_msg}")

    # Data errors
    elif "data" in error_lower or "file" in error_lower or "load" in error_lower:
        raise DataError(f"Data operation failed: {error_msg}")

    # General execution errors
    else:
        raise QueryExecutionError(f"Query execution failed: {error_msg}")


class QueryResult:
    """Query result wrapper."""

    def __init__(self, schema: List[Dict], data: List[List], metrics: Dict[str, Any]):
        self.schema = schema
        self.data = data
        self.metrics = metrics

    def __iter__(self) -> Iterator[List]:
        return iter(self.data)

    def __len__(self) -> int:
        return len(self.data)

    def __getitem__(self, index: int) -> List:
        return self.data[index]

    def __repr__(self) -> str:
        return f"QueryResult(schema={self.schema}, rows={len(self.data)})"


class MiniGU:
    """
    Synchronous miniGU database connection.

    Example:
        >>> with MiniGU() as db:
        ...     db.create_graph("my_graph")
        ...     result = db.execute("MATCH (n) RETURN n LIMIT 10")
        ...     print(result.data)
    """

    def __init__(self, db_path: Optional[str] = None):
        """
        Initialize MiniGU instance.

        Args:
            db_path: Database path (not currently used, uses in-memory database)
        """
        self._db_path = db_path
        self._db = None
        self._connected = False

        # Import Rust bindings
        try:
            from . import minigu_python
            self._PyMiniGU = minigu_python.PyMiniGU
            self._is_syntax_error = minigu_python.is_syntax_error
            self._is_timeout_error = minigu_python.is_timeout_error
            self._is_transaction_error = minigu_python.is_transaction_error
        except ImportError:
            raise ImportError(
                "Rust bindings not available. miniGU requires Rust bindings to function. "
                "Please install with: pip install maturin && maturin develop"
            )

    def init(self) -> None:
        """Initialize the database connection."""
        if self._db is None:
            self._db = self._PyMiniGU()
        self._db.init()
        self._connected = True

    @property
    def is_connected(self) -> bool:
        """Check if database is connected."""
        return self._connected and self._db is not None and self._db.is_connected

    def create_graph(self, name: str, num_vertices: Optional[int] = None) -> bool:
        """
        Create a new graph.

        Args:
            name: Graph name (alphanumeric and underscore only)
            num_vertices: Optional number of test vertices to create.
                          If specified, creates a graph with test data (PERSON, COMPANY, CITY nodes).
                          If None, creates an empty graph.

        Returns:
            True if graph was created successfully

        Raises:
            GraphError: If graph already exists or name is invalid
            ConnectionError: If database is not initialized
        """
        if not self.is_connected:
            raise ConnectionError("Database not initialized. Call init() first.")

        try:
            return self._db.create_graph(name, num_vertices)
        except Exception as e:
            _handle_exception(e)

    def execute(self, query: str) -> QueryResult:
        """
        Execute a GQL query.

        Args:
            query: GQL query string

        Returns:
            QueryResult object containing schema and data

        Raises:
            QuerySyntaxError: If query has invalid syntax
            QueryExecutionError: If query execution fails
            ConnectionError: If database is not initialized
        """
        if not self.is_connected:
            raise ConnectionError("Database not initialized. Call init() first.")

        try:
            result = self._db.execute(query)
            return QueryResult(
                schema=result.get("schema", []),
                data=result.get("data", []),
                metrics=result.get("metrics", {})
            )
        except Exception as e:
            _handle_exception(e)

    def load(self, data: Union[List[Dict[str, Any]], str]) -> bool:
        """
        Load data into the database.

        Args:
             List of dictionaries or file path (CSV/JSON)

        Returns:
            True if data was loaded successfully

        Raises:
            DataError: If data format is invalid or file not found
            ConnectionError: If database is not initialized
        """
        if not self.is_connected:
            raise ConnectionError("Database not initialized. Call init() first.")

        try:
            return self._db.load(data)
        except Exception as e:
            _handle_exception(e)

    def save(self, path: str) -> bool:
        """
        Save database to a file.

        Args:
            path: File path to save to

        Returns:
            True if database was saved successfully

        Raises:
            DataError: If path is invalid or write permission denied
            ConnectionError: If database is not initialized
        """
        if not self.is_connected:
            raise ConnectionError("Database not initialized. Call init() first.")

        try:
            return self._db.save(path)
        except Exception as e:
            _handle_exception(e)

    def close(self) -> None:
        """Close the database connection."""
        if self._db is not None:
            self._db.close()
        self._connected = False

    def __enter__(self) -> "MiniGU":
        """Enter context manager, auto-initialize connection."""
        self.init()
        return self

    def __exit__(self, exc_type, exc_val, exc_tb) -> None:
        """Exit context manager, auto-cleanup resources."""
        self.close()


class AsyncMiniGU:
    """
    Asynchronous miniGU database connection.

    Example:
        >>> async with AsyncMiniGU() as db:
        ...     await db.create_graph("my_graph")
        ...     result = await db.execute("MATCH (n) RETURN n LIMIT 10")
        ...     print(result.data)
    """

    def __init__(self, db_path: Optional[str] = None):
        """
        Initialize AsyncMiniGU instance.

        Args:
            db_path: Database path (not currently used, uses in-memory database)
        """
        self._sync_db = MiniGU(db_path)

    async def init(self) -> None:
        """Initialize the database connection asynchronously."""
        loop = asyncio.get_running_loop()
        await loop.run_in_executor(None, self._sync_db.init)

    @property
    def is_connected(self) -> bool:
        """Check if database is connected."""
        return self._sync_db.is_connected

    async def create_graph(self, name: str, num_vertices: Optional[int] = None) -> bool:
        """Create a new graph asynchronously."""
        loop = asyncio.get_running_loop()
        return await loop.run_in_executor(None, lambda: self._sync_db.create_graph(name, num_vertices))

    async def execute(self, query: str) -> QueryResult:
        """Execute a GQL query asynchronously."""
        loop = asyncio.get_running_loop()
        return await loop.run_in_executor(None, self._sync_db.execute, query)

    async def load(self, data: Union[List[Dict[str, Any]], str]) -> bool:
        """Load data into the database asynchronously."""
        loop = asyncio.get_running_loop()
        return await loop.run_in_executor(None, self._sync_db.load, data)

    async def save(self, path: str) -> bool:
        """Save database to a file asynchronously."""
        loop = asyncio.get_running_loop()
        return await loop.run_in_executor(None, self._sync_db.save, path)

    async def close(self) -> None:
        """Close the database connection asynchronously."""
        loop = asyncio.get_running_loop()
        await loop.run_in_executor(None, self._sync_db.close)

    async def __aenter__(self) -> "AsyncMiniGU":
        """Enter async context manager, auto-initialize connection."""
        await self.init()
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb) -> None:
        """Exit async context manager, auto-cleanup resources."""
        await self.close()