"""
miniGU Python API Package

This package provides Python bindings for the miniGU graph database.
It offers both synchronous and asynchronous interfaces for interacting with
the graph database, including graph creation, data loading, querying, and
transaction management.

Stability:
    This API is currently in alpha state. Features may change in future versions.
    
Feature Status:
    - Graph operations: Implemented
    - Query execution: Implemented
    - Data loading/saving: Implemented
    - Transactions: Not yet implemented (planned)
    
Supported Python Versions:
    Python 3.7 and above
    
Examples:
    >>> from minigu import MiniGU
    >>> db = MiniGU()
    >>> success = db.create_graph("my_graph")
    >>> if success:
    ...     print("Graph created successfully")
    >>> data = [{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]
    >>> success = db.load(data)
    >>> if success:
    ...     print("Data loaded successfully")
    >>> success = db.save("/path/to/save/location")
    >>> if success:
    ...     print("Database saved successfully")
    >>> result = db.execute("MATCH (n) RETURN n LIMIT 10")
"""

# Main module initialization file
try:
    import minigu_python
    HAS_RUST_BINDINGS = True
    PyMiniGU = minigu_python.PyMiniGU
    # Try to import the error checking functions
    is_transaction_error = getattr(minigu_python, 'is_transaction_error', None)
    is_not_implemented_error = getattr(minigu_python, 'is_not_implemented_error', None)
except ImportError as e:
    # Fallback if the Rust extension is not available
    HAS_RUST_BINDINGS = False
    PyMiniGU = None
    is_transaction_error = None
    is_not_implemented_error = None
    # Print the actual error for debugging purposes
    print(f"Warning: Failed to import Rust extension: {e}")

# Import the main classes and functions to expose them publicly
from .minigu import (
    MiniGU,
    AsyncMiniGU,
    QueryResult,
    MiniGUError,
    ConnectionError,
    QuerySyntaxError,
    QueryExecutionError,
    QueryTimeoutError,
    GraphError,
    DataError,
    TransactionError,
)

__all__ = [
    "MiniGU",
    "AsyncMiniGU", 
    "QueryResult",
    "MiniGUError",
    "ConnectionError",
    "QuerySyntaxError",
    "QueryExecutionError",
    "QueryTimeoutError",
    "GraphError",
    "DataError",
    "TransactionError",
    "HAS_RUST_BINDINGS",
    "PyMiniGU",
    "is_transaction_error",
    "is_not_implemented_error",
]

__version__ = "0.1.0"
__author__ = "miniGU Team"