"""
miniGU Python API

This module provides Python bindings for the miniGU graph database.
"""

import sys
from typing import Optional, List, Dict, Any, Union
from pathlib import Path
import json
import asyncio


# Try to import Rust bindings
try:
    from . import minigu_python
    from .minigu_python import PyMiniGU
    HAS_RUST_BINDINGS = True
except (ImportError, ModuleNotFoundError):
    try:
        # Try alternative import path
        import minigu_python
        from minigu_python import PyMiniGU
        HAS_RUST_BINDINGS = True
    except (ImportError, ModuleNotFoundError):
        # No longer provide simulated implementation warning, directly raise exception
        HAS_RUST_BINDINGS = False
        raise ImportError("Rust bindings not available. miniGU requires Rust bindings to function.")


class Node:
    """Graph node representation"""
    
    def __init__(self, label: str, properties: Optional[Dict[str, Any]] = None):
        """
        Initialize a node
        
        Args:
            label: Node label
            properties: Node properties as key-value pairs
        """
        self.label = label
        self.properties = properties or {}
    
    def __repr__(self) -> str:
        return f"Node(label='{self.label}', properties={self.properties})"


class Edge:
    """Graph edge representation"""
    
    def __init__(self, label: str, src: Union[Node, int], dst: Union[Node, int], 
                 properties: Optional[Dict[str, Any]] = None):
        """
        Initialize an edge
        
        Args:
            label: Edge label
            src: Source node or node ID
            dst: Destination node or node ID
            properties: Edge properties as key-value pairs
        """
        self.label = label
        self.src = src
        self.dst = dst
        self.properties = properties or {}
    
    def __repr__(self) -> str:
        return f"Edge(label='{self.label}', src={self.src}, dst={self.dst}, properties={self.properties})"


class Path:
    """Graph path representation"""
    
    def __init__(self, nodes: List[Node], edges: List[Edge]):
        """
        Initialize a path
        
        Args:
            nodes: List of nodes in the path
            edges: List of edges in the path
        """
        self.nodes = nodes
        self.edges = edges
    
    def __repr__(self) -> str:
        return f"Path(nodes={len(self.nodes)}, edges={len(self.edges)})"


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


class QueryResult:
    """
    Query result class
    """
    
    def __init__(self, schema: Optional[List[Dict[str, Any]]] = None, 
                 data: Optional[List[List[Any]]] = None,
                 metrics: Optional[Dict[str, float]] = None):
        self.schema = schema or []
        self.data = data or []
        self.metrics = metrics or {}
        self.row_count = len(self.data)
    
    def to_list(self) -> List[Dict[str, Any]]:
        """
        Convert the result to a list of dictionaries format
        
        Returns:
            List of dictionaries, with each row as a dictionary
        """
        if not self.schema or not self.data:
            return []
        
        column_names = [col["name"] for col in self.schema]
        return [dict(zip(column_names, row)) for row in self.data]
    
    def to_dict(self) -> Dict[str, Any]:
        """
        Convert the result to dictionary format
        
        Returns:
            Dictionary containing schema, data, and metrics
        """
        return {
            "schema": self.schema,
            "data": self.data,
            "metrics": self.metrics,
            "row_count": self.row_count
        }
    
    def __repr__(self) -> str:
        return f"QueryResult(rows={self.row_count}, columns={len(self.schema)})"


class AsyncMiniGU:
    """
    Async miniGU database connection class.
    
    This class provides an async interface for interacting with a miniGU database.
    It supports connecting to the database, executing queries, and managing graph data.
    
    Attributes:
        db_path (Optional[str]): Database file path
        is_connected (bool): Connection status
    """
    
    def __init__(self, db_path: Optional[str] = None, 
                 thread_count: int = 1, 
                 cache_size: int = 1000,
                 enable_logging: bool = False):
        """
        Initialize async miniGU database connection.
        
        Args:
            db_path: Database file path, if None creates an in-memory database
            thread_count: Number of threads for parallel execution
            cache_size: Size of the query result cache
            enable_logging: Whether to enable query execution logging
        """
        self.db_path = db_path
        self.thread_count = thread_count
        self.cache_size = cache_size
        self.enable_logging = enable_logging
        self._rust_instance = None
        self.is_connected = False
        self._stored_data = []
        self._connect()
    
    def _connect(self) -> None:
        """
        Establish database connection.
        
        Raises:
            ConnectionError: If connection fails
        """
        try:
            if HAS_RUST_BINDINGS:
                self._rust_instance = PyMiniGU()
                self._rust_instance.init()
            else:
                raise ConnectionError("Rust bindings not available")
            self.is_connected = True
            print("Database connected")
        except Exception as e:
            raise ConnectionError(f"Failed to connect to database: {str(e)}")
    
    async def execute(self, query: str) -> QueryResult:
        """
        Execute GQL query asynchronously.
        
        Args:
            query: GQL query statement
            
        Returns:
            Query result
            
        Raises:
            MiniGUError: Raised when database is not connected
            QuerySyntaxError: Raised when query has syntax errors
            QueryExecutionError: Raised when query execution fails
            QueryTimeoutError: Raised when query times out
        """
        
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
            try:
                result = self._rust_instance.execute(query)
                
                return QueryResult(
                    schema=result.get("schema", []),
                    data=result.get("data", []),
                    metrics=result.get("metrics", {})
                )
            except Exception as e:
                # 根据具体的错误类型抛出更精确的异常
                error_str = str(e).lower()
                if "syntax" in error_str or "unexpected" in error_str:
                    raise QuerySyntaxError(f"Query syntax error: {str(e)}")
                elif "timeout" in error_str:
                    raise QueryTimeoutError(f"Query timeout: {str(e)}")
                else:
                    raise QueryExecutionError(f"Query execution failed: {str(e)}")
        else:
            # When Rust bindings are not available, raise an error directly
            raise RuntimeError("Rust bindings required for database operations")
    
    async def load(self, data: Union[List[Dict], str, Path]) -> None:
        """
        Load data into the database asynchronously.
        
        Args:
            data: Data to load, can be a list of dictionaries or file path
            
        Raises:
            MiniGUError: Raised when database is not connected
            DataError: Raised when data loading fails
        """
        
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
    
            try:
                if isinstance(data, (str, Path)):
                    # Convert path to string and load from file
                    file_path = str(data)
                    if not Path(file_path).exists():
                        raise DataError(f"File not found: {file_path}")
                    
                    self._rust_instance.load_from_file(file_path)
                else:
                    # Validate input data format
                    if not isinstance(data, list):
                        raise DataError("Data must be a list of dictionaries")
                    
                    for item in data:
                        if not isinstance(item, dict):
                            raise DataError("Each item in data list must be a dictionary")
                    
                    self._rust_instance.load_data(data)
                    self._stored_data = data
                
                print(f"Data loaded successfully")
            except Exception as e:
                raise DataError(f"Data loading failed: {str(e)}")
        else:
            # When Rust bindings are not available, raise an error directly
            raise RuntimeError("Rust bindings required for database operations")
    
    async def save(self, path: str) -> None:
        """
        Save the database to the specified path asynchronously.
        
        Args:
            path: Target file path
            
        Raises:
            MiniGUError: Raised when database is not connected
            IOError: Raised when save operation fails
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
            try:
                self._rust_instance.save(path)
                print(f"Database saved to {path}")
            except Exception as e:
                raise IOError(f"Failed to save database: {str(e)}")
        else:
            # When Rust bindings are not available, raise an error directly
            raise RuntimeError("Rust bindings required for database operations")
    
    async def begin_transaction(self) -> None:
        """
        Begin a transaction asynchronously.
        
        Raises:
            MiniGUError: Raised when database is not connected
            TransactionError: Raised when transaction operations fail
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
            try:
                self._rust_instance.begin_transaction()
            except AttributeError:
                print("Transactions not yet implemented in Rust backend")
            except Exception as e:
                raise TransactionError(f"Failed to begin transaction: {str(e)}")
        else:
            raise RuntimeError("Rust bindings required for database operations")
    
    async def commit(self) -> None:
        """
        Commit the current transaction asynchronously.
        
        Raises:
            MiniGUError: Raised when database is not connected
            TransactionError: Raised when transaction operations fail
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
            try:
                self._rust_instance.commit()
            except AttributeError:
                print("Transactions not yet implemented in Rust backend")
            except Exception as e:
                raise TransactionError(f"Failed to commit transaction: {str(e)}")
        else:
            raise RuntimeError("Rust bindings required for database operations")
    
    async def rollback(self) -> None:
        """
        Rollback the current transaction asynchronously.
        
        Raises:
            MiniGUError: Raised when database is not connected
            TransactionError: Raised when transaction operations fail
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
            try:
                self._rust_instance.rollback()
            except AttributeError:
                print("Transactions not yet implemented in Rust backend")
            except Exception as e:
                raise TransactionError(f"Failed to rollback transaction: {str(e)}")
        else:
            raise RuntimeError("Rust bindings required for database operations")
    
    async def create_graph(self, name: str, schema: Optional[Dict] = None) -> None:
        """
        Create a graph database asynchronously.
        
        Args:
            name: Graph name
            schema: Graph schema definition (optional)
            
        Raises:
            MiniGUError: Raised when database is not connected
            GraphError: Raised when graph creation fails
        """
        
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
            try:
                self._rust_instance.create_graph(name, json.dumps(schema) if schema else None)
                print(f"Graph '{name}' created successfully")
            except Exception as e:
                raise GraphError(f"Graph creation failed: {str(e)}")
        else:
            raise RuntimeError("Rust bindings required for database operations")
    
    async def create_node(self, label: str, properties: Optional[Dict[str, Any]] = None) -> Node:
        """
        Create a node object asynchronously.
        
        Args:
            label: Node label
            properties: Node properties
            
        Returns:
            Node object
        """
        return Node(label, properties)
    
    async def create_edge(self, label: str, src: Union[Node, int], dst: Union[Node, int], 
                          properties: Optional[Dict[str, Any]] = None) -> Edge:
        """
        Create an edge object asynchronously.
        
        Args:
            label: Edge label
            src: Source node or node ID
            dst: Destination node or node ID
            properties: Edge properties
            
        Returns:
            Edge object
        """
        return Edge(label, src, dst, properties)
    
    async def create_path(self, nodes: List[Node], edges: List[Edge]) -> Path:
        """
        Create a path object asynchronously.
        
        Args:
            nodes: List of nodes
            edges: List of edges
            
        Returns:
            Path object
        """
        return Path(nodes, edges)
    
    async def __aenter__(self):
        return self
    
    async def close(self) -> None:
        """
        Close the database connection asynchronously.
        """
        if self.is_connected and HAS_RUST_BINDINGS and self._rust_instance:
            try:
                self._rust_instance.close()
            except:
                pass
        self.is_connected = False
    
    async def __aexit__(self, exc_type, exc_val, exc_tb):
        await self.close()
        return False


class MiniGU:
    """
    miniGU database connection class.
    
    This class provides the main interface for interacting with a miniGU database.
    It supports connecting to the database, executing queries, and managing graph data.
    
    Attributes:
        db_path (Optional[str]): Database file path
        is_connected (bool): Connection status
    """
    
    def __init__(self, db_path: Optional[str] = None, 
                 thread_count: int = 1, 
                 cache_size: int = 1000,
                 enable_logging: bool = False):
        """
        Initialize miniGU database connection.
        
        Args:
            db_path: Database file path, if None creates an in-memory database
            thread_count: Number of threads for parallel execution
            cache_size: Size of the query result cache
            enable_logging: Whether to enable query execution logging
        """
        self.db_path = db_path
        self.thread_count = thread_count
        self.cache_size = cache_size
        self.enable_logging = enable_logging
        self._rust_instance = None
        self.is_connected = False
        self._stored_data = []
        self._connect()
    
    def _connect(self) -> None:
        """
        Establish database connection
        """
        try:
            if HAS_RUST_BINDINGS:
                self._rust_instance = PyMiniGU()
                self._rust_instance.init()
            else:
                raise ConnectionError("Rust bindings not available")
            self.is_connected = True
            print("Database connected")
        except Exception as e:
            raise ConnectionError(f"Failed to connect to database: {str(e)}")
    
    def execute(self, query: str) -> QueryResult:
        """
        Execute GQL query
        
        Args:
            query: GQL query statement
            
        Returns:
            Query result
            
        Raises:
            MiniGUError: Raised when database is not connected
            QuerySyntaxError: Raised when query has syntax errors
            QueryExecutionError: Raised when query execution fails
            QueryTimeoutError: Raised when query times out
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
            # Execute query using Rust backend
            try:
                result_dict = self._rust_instance.execute(query)
                schema = result_dict.get("schema", [])
                data = result_dict.get("data", [])
                metrics = result_dict.get("metrics", {})
                return QueryResult(schema, data, metrics)
            except Exception as e:
                # 根据具体的错误类型抛出更精确的异常
                error_str = str(e).lower()
                if "syntax" in error_str or "unexpected" in error_str:
                    raise QuerySyntaxError(f"Query syntax error: {str(e)}")
                elif "timeout" in error_str:
                    raise QueryTimeoutError(f"Query timeout: {str(e)}")
                else:
                    raise QueryExecutionError(f"Query execution failed: {str(e)}")
        else:
            raise RuntimeError("Rust bindings required for database operations")
    
    def load(self, data: Union[List[Dict], str, Path]) -> None:
        """
        Load data into the database
        
        Args:
            data: Data to load, can be a list of dictionaries or file path
            
        Raises:
            MiniGUError: Raised when database is not connected
            DataError: Raised when data loading fails
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
            try:
                if isinstance(data, (str, Path)):
                    self._rust_instance.load_from_file(str(data))
                else:
                    self._rust_instance.load_data(data)
                print(f"Data loaded successfully")
            except Exception as e:
                raise DataError(f"Data loading failed: {str(e)}")
        else:
            raise RuntimeError("Rust bindings required for database operations")

    def save(self, path: str) -> None:
        """
        Save the database to the specified path
        
        Args:
            path: Save path
            
        Raises:
            MiniGUError: Raised when database is not connected
            DataError: Raised when save fails
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
            try:
                self._rust_instance.save_to_file(path)
                print(f"Database saved to {path}")
            except Exception as e:
                raise DataError(f"Database save failed: {str(e)}")
        else:
            raise RuntimeError("Rust bindings required for database operations")
    
    def create_graph(self, name: str, schema: Optional[Dict] = None) -> None:
        """
        Create a graph database
        
        Args:
            name: Graph name
            schema: Graph schema definition (optional)
            
        Raises:
            MiniGUError: Raised when database is not connected
            GraphError: Raised when graph creation fails
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
            try:
                self._rust_instance.create_graph(name, json.dumps(schema) if schema else None)
                print(f"Graph '{name}' created successfully")
            except Exception as e:
                raise GraphError(f"Graph creation failed: {str(e)}")
        else:
            raise RuntimeError("Rust bindings required for database operations")
    
    def begin_transaction(self) -> None:
        """
        Begin a transaction.
        
        Raises:
            MiniGUError: Raised when database is not connected
            TransactionError: Raised when transaction cannot be started
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
            try:
                # Check if the method exists before calling it
                if hasattr(self._rust_instance, 'begin_transaction'):
                    self._rust_instance.begin_transaction()
                else:
                    # For now, just print a message since the method doesn't exist in the Rust code yet
                    print("Transactions not yet implemented in Rust backend")
            except Exception as e:
                raise TransactionError(f"Failed to begin transaction: {str(e)}")
        else:
            raise RuntimeError("Rust bindings required for database operations")
    
    def commit(self) -> None:
        """
        Commit the current transaction.
        
        Raises:
            MiniGUError: Raised when database is not connected
            TransactionError: Raised when transaction cannot be committed
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
            try:
                # Check if the method exists before calling it
                if hasattr(self._rust_instance, 'commit'):
                    self._rust_instance.commit()
                else:
                    # For now, just print a message since the method doesn't exist in the Rust code yet
                    print("Transactions not yet implemented in Rust backend")
            except Exception as e:
                raise TransactionError(f"Failed to commit transaction: {str(e)}")
        else:
            raise RuntimeError("Rust bindings required for database operations")
    
    def rollback(self) -> None:
        """
        Rollback the current transaction.
        
        Raises:
            MiniGUError: Raised when database is not connected
            TransactionError: Raised when transaction cannot be rolled back
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
            try:
                # Check if the method exists before calling it
                if hasattr(self._rust_instance, 'rollback'):
                    self._rust_instance.rollback()
                else:
                    # For now, just print a message since the method doesn't exist in the Rust code yet
                    print("Transactions not yet implemented in Rust backend")
            except Exception as e:
                raise TransactionError(f"Failed to rollback transaction: {str(e)}")
        else:
            raise RuntimeError("Rust bindings required for database operations")

    def close(self) -> None:
        """
        Close the database connection.
        """
        if self.is_connected and HAS_RUST_BINDINGS and self._rust_instance:
            try:
                self._rust_instance.close()
            except:
                pass
        self.is_connected = False
    
    def __enter__(self):
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()
        return False


def connect(db_path: Optional[str] = None,
            thread_count: int = 1,
            cache_size: int = 1000,
            enable_logging: bool = False) -> MiniGU:
    """
    Create a connection to the miniGU database.
    
    Args:
        db_path: Database file path, if None creates an in-memory database
        thread_count: Number of threads for parallel execution
        cache_size: Size of the query result cache
        enable_logging: Whether to enable query execution logging
        
    Returns:
        MiniGU database connection object
    """
    return MiniGU(db_path, thread_count, cache_size, enable_logging)


async def async_connect(db_path: Optional[str] = None,
                        thread_count: int = 1,
                        cache_size: int = 1000,
                        enable_logging: bool = False) -> AsyncMiniGU:
    """
    Create an asynchronous connection to the miniGU database.
    
    Args:
        db_path: Database file path, if None creates an in-memory database
        thread_count: Number of threads for parallel execution
        cache_size: Size of the query result cache
        enable_logging: Whether to enable query execution logging
        
    Returns:
        AsyncMiniGU database connection object
    """
    return AsyncMiniGU(db_path, thread_count, cache_size, enable_logging)