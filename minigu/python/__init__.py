# Main module initialization file
try:
    import minigu_python
    HAS_RUST_BINDINGS = True
    PyMiniGU = minigu_python.PyMiniGU
except ImportError:
    # Fallback if the Rust extension is not available
    HAS_RUST_BINDINGS = False
    PyMiniGU = None

# Import the main classes and functions to expose them publicly
from .minigu import (
    MiniGU,
    AsyncMiniGU,
    connect,
    async_connect,
    MiniGUError,
    ConnectionError,
    QueryError,
    QuerySyntaxError,
    QueryExecutionError,
    QueryTimeoutError,
    DataError,
    GraphError,
    TransactionError,
    QueryResult
)

# Export everything
__all__ = [
    'MiniGU',
    'AsyncMiniGU',
    'connect',
    'async_connect',
    'MiniGUError',
    'ConnectionError',
    'QueryError',
    'QuerySyntaxError',
    'QueryExecutionError',
    'QueryTimeoutError',
    'DataError',
    'GraphError',
    'TransactionError',
    'QueryResult',
    'PyMiniGU',
    'HAS_RUST_BINDINGS'
]