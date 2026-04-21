"""
miniGU Python API

This module provides Python bindings for the miniGU graph database.
"""

from .minigu import (
    MiniGU,
    AsyncMiniGU,
    QueryResult,
    MiniGUError,
    ConnectionError,
    QueryError,
    QuerySyntaxError,
    QueryExecutionError,
    QueryTimeoutError,
    GraphError,
    DataError,
    TransactionError,
)

try:
    from . import minigu_python
    from .minigu_python import PyMiniGU, is_syntax_error, is_timeout_error, is_transaction_error, is_not_implemented_error
    HAS_RUST_BINDINGS = True
except ImportError:
    HAS_RUST_BINDINGS = False
    PyMiniGU = None

__all__ = [
    "MiniGU",
    "AsyncMiniGU",
    "QueryResult",
    "MiniGUError",
    "ConnectionError",
    "QueryError",
    "QuerySyntaxError",
    "QueryExecutionError",
    "QueryTimeoutError",
    "GraphError",
    "DataError",
    "TransactionError",
    "PyMiniGU",
    "HAS_RUST_BINDINGS",
    "is_syntax_error",
    "is_timeout_error",
    "is_transaction_error",
    "is_not_implemented_error",
]

__version__ = "0.1.0"