"""
miniGU Python API Enhanced Version

Adding missing functionality to the existing implementation
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
        HAS_RUST_BINDINGS = False
        # No longer provide simulated implementation warning, directly raise exception
        raise ImportError("Rust bindings not available. miniGU requires Rust bindings to function.")
        print("Warning: Rust bindings not available. Using simulated implementation.")


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
    """miniGU database exception class"""
    pass


class ConnectionError(MiniGUError):
    """Database connection error"""
    pass


class QueryError(MiniGUError):
    """Query execution error"""
    pass


class DataError(MiniGUError):
    """Data loading/saving error"""
    pass


class GraphError(MiniGUError):
    """Graph creation/manipulation error"""
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
    Asynchronous miniGU database connection class.
    
    This class provides an asynchronous interface for interacting with a miniGU database.
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
        Initialize asynchronous miniGU database connection.
        
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
                # Set configuration options (only in synchronous mode)
                if not asyncio.iscoroutinefunction(self.set_thread_count):
                    self.set_thread_count(self.thread_count)
                    self.set_cache_size(self.cache_size)
                    self.enable_query_logging(self.enable_logging)
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
            QueryError: Raised when query execution fails
        """
        
        await asyncio.sleep(0.01)
        
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
                raise QueryError(f"Query execution failed: {str(e)}")
        else:
            # Remove simulated implementation, only keep real functionality
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
        
        await asyncio.sleep(0.01)
        
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
            # Simulated implementation for when Rust bindings are not available
            if isinstance(data, (str, Path)):
                file_path = str(data)
                print(f"Loading data from file: {file_path}")
        
                if file_path.endswith('.json'):
                    try:
                        with open(file_path, 'r', encoding='utf-8') as f:
                            file_data = json.load(f)
                            
                            if not isinstance(file_data, list):
                                raise DataError("JSON file must contain a list of objects")
                            
                            for item in file_data:
                                if not isinstance(item, dict):
                                    raise DataError("JSON file must contain a list of objects")
                            
                            self._stored_data = file_data
                            print(f"Loaded {len(file_data)} records from JSON file")
                    except FileNotFoundError:
                        raise DataError(f"File not found: {file_path}")
                    except json.JSONDecodeError:
                        raise DataError(f"Invalid JSON format in file: {file_path}")
                    except Exception as e:
                        raise DataError(f"Could not load data from file: {str(e)}")
                else:
                    raise DataError(f"Unsupported file format: {Path(file_path).suffix[1:]}")
            else:
                if not isinstance(data, list):
                    raise DataError("Data must be a list of dictionaries")
                
                for item in data:
                    if not isinstance(item, dict):
                        raise DataError("Each item in data list must be a dictionary")
                
                self._stored_data = data
                print(f"Loading {len(data)} records into database")
            print("Data loaded successfully")
    
    async def save(self, path: str) -> None:
        """
        Save the database to the specified path asynchronously.
        
        Args:
            path: Save path
            
        Raises:
            MiniGUError: Raised when database is not connected
            DataError: Raised when save fails
        """

        await asyncio.sleep(0.01)
        
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
    
            try:
                self._rust_instance.save_to_file(path)
                print(f"Database saved to {path}")
            except Exception as e:
                raise DataError(f"Database save failed: {str(e)}")
        else:
            # Remove simulated implementation, only keep real functionality
            raise RuntimeError("Rust bindings required for database operations")
    
    async def create_graph(self, graph_name: str, schema: Optional[Dict] = None) -> None:
        """
        Create a graph database asynchronously.
        
        Args:
            graph_name: Graph name
            schema: Graph schema definition (optional)
            
        Raises:
            MiniGUError: Raised when database is not connected
            GraphError: Raised when graph creation fails
        """
        
        await asyncio.sleep(0.01)
        
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:

            try:
                if schema:
                    schema_str = self._format_schema(schema)
                    self._rust_instance.create_graph(graph_name, schema_str)
                else:
                    self._rust_instance.create_graph(graph_name, None)
                print(f"Graph '{graph_name}' created")
            except Exception as e:
                raise GraphError(f"Graph creation failed: {str(e)}")
        else:
            # Remove simulated implementation, only keep real functionality
            raise RuntimeError("Rust bindings required for database operations")
    
    def _format_schema(self, schema: Dict) -> str:
        """
        Format graph schema definition.
        
        Args:
            schema: Graph schema definition
            
        Returns:
            Formatted schema string
        """
        
        elements = []
        for label, properties in schema.items():
            props = ", ".join([f"{name} {ptype}" for name, ptype in properties.items()])
            elements.append(f"({label} :{label} {{{props}}})")
        return "; ".join(elements)
    
    async def insert(self, data: Union[List[Dict], str]) -> None:
        """
        Insert data into the current graph asynchronously.
        
        Args:
            data: List of data to insert or GQL INSERT statement
            
        Raises:
            MiniGUError: Raised when database is not connected
            DataError: Raised when data insertion fails
        """

        await asyncio.sleep(0.01)
        
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
        
            try:
                if isinstance(data, str):
                    
                    self._rust_instance.insert_data(data)
                else:
                    
                    gql_data = await self._format_insert_data(data)
                    self._rust_instance.insert_data(gql_data)
                print(f"Data inserted successfully")
            except Exception as e:
                raise DataError(f"Data insertion failed: {str(e)}")
        else:

            if isinstance(data, str):
                print(f"Executing INSERT statement: {data}")
            else:
                print(f"Inserting {len(data)} records")
                
                if isinstance(data, list):
                    self._stored_data.extend(data)
            print("Data inserted (simulated)")
    
    def _format_insert_data(self, data: List[Dict]) -> str:
        """
        Format data as GQL INSERT statement.
        
        Args:
            data: List of data to insert
            
        Returns:
            GQL INSERT statement fragment
        """
        # Based on the GQL examples, we should use :Label syntax instead of (Label)
        # and generate separate INSERT statements for each record
        statements = []
        for item in data:
            label = item.get("label", "Node")
            # Format properties correctly for GQL
            # Based on examples, we should not put quotes around all values
            props = []
            for k, v in item.items():
                if k != "label":
                    # Handle different data types appropriately
                    if isinstance(v, str):
                        props.append(f"{k}: '{v}'")
                    elif isinstance(v, (int, float)):
                        props.append(f"{k}: {v}")
                    else:
                        # For other types, convert to string and quote
                        props.append(f"{k}: '{str(v)}'")
            
            props_str = ", ".join(props)
            # Based on GQL examples, use :Label syntax and separate INSERT statements
            statement = f"INSERT :{label} {{ {props_str} }}"
            statements.append(statement)
            
        # Join statements with semicolon and space
        return "; ".join(statements)
    
    async def update(self, query: str) -> None:
        """
        Update data in the current graph using a GQL UPDATE statement asynchronously.
        
        Args:
            query: GQL UPDATE statement
            
        Raises:
            MiniGUError: Raised when database is not connected
            QueryError: Raised when query execution fails
        """

        await asyncio.sleep(0.01)
        
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
        
            try:
                self._rust_instance.update_data(query)
                print(f"Data updated successfully with query: {query}")
            except Exception as e:
                raise QueryError(f"Data update failed: {str(e)}")
        else:
    
            print(f"Executing UPDATE statement: {query}")
            print("Data updated (simulated)")
    
    async def delete(self, query: str) -> None:
        """
        Delete data from the current graph using a GQL DELETE statement asynchronously.
        
        Args:
            query: GQL DELETE statement
            
        Raises:
            MiniGUError: Raised when database is not connected
            QueryError: Raised when query execution fails
        """
        
        await asyncio.sleep(0.01)
        
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
            
            try:
                self._rust_instance.delete_data(query)
                print(f"Data deleted successfully with query: {query}")
            except Exception as e:
                raise QueryError(f"Data deletion failed: {str(e)}")
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
        
        await asyncio.sleep(0.01)
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

        await asyncio.sleep(0.01)
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

        await asyncio.sleep(0.01)
        return Path(nodes, edges)
    
    # Enhanced features: Performance configuration and statistics methods
    async def set_cache_size(self, size: int) -> None:
        """
        Set the size of the query result cache asynchronously.
        
        Args:
            size: Cache size in number of entries
            
        Raises:
            MiniGUError: Raised when database is not connected
        """
        
        await asyncio.sleep(0.01)
        
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and hasattr(self._rust_instance, 'set_cache_size'):
            try:
                self._rust_instance.set_cache_size(size)
                self.cache_size = size
                print(f"Cache size set to {size} entries")
            except Exception as e:
                raise DataError(f"Failed to set cache size: {str(e)}")
        else:
            
            raise RuntimeError("Rust bindings required for database operations")
    
    async def set_thread_count(self, count: int) -> None:
        """
        Set the number of threads for parallel query execution asynchronously.
        
        Args:
            count: Number of threads
            
        Raises:
            MiniGUError: Raised when database is not connected
        """
        
        await asyncio.sleep(0.01)
        
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        
        if HAS_RUST_BINDINGS and hasattr(self._rust_instance, 'set_thread_count'):
            try:
                self._rust_instance.set_thread_count(count)
                self.thread_count = count
                print(f"Thread count set to {count}")
            except Exception as e:
                raise DataError(f"Failed to set thread count: {str(e)}")
        else:
            
            raise RuntimeError("Rust bindings required for database operations")
    
    async def enable_query_logging(self, enable: bool = True) -> None:
        """
        Enable or disable query execution logging asynchronously.
        
        Args:
            enable: Whether to enable logging
            
        Raises:
            MiniGUError: Raised when database is not connected
        """

        await asyncio.sleep(0.01)
        
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        
        if HAS_RUST_BINDINGS and hasattr(self._rust_instance, 'enable_query_logging'):
            try:
                self._rust_instance.enable_query_logging(enable)
                self.enable_logging = enable
                status = "enabled" if enable else "disabled"
                print(f"Query logging {status}")
            except Exception as e:
                raise DataError(f"Failed to set query logging: {str(e)}")
        else:
        
            raise RuntimeError("Rust bindings required for database operations")
    
    async def get_performance_stats(self) -> Dict[str, Any]:
        """
        Get database performance statistics asynchronously.
        
        Returns:
            Dictionary containing performance statistics
            
        Raises:
            MiniGUError: Raised when database is not connected
        """

        await asyncio.sleep(0.01)
        
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        
        if HAS_RUST_BINDINGS and hasattr(self._rust_instance, 'get_performance_stats'):
            try:
                stats = self._rust_instance.get_performance_stats()
                return stats
            except Exception as e:
                raise DataError(f"Failed to get performance stats: {str(e)}")
        else:
    
            stats = {
                "cache_hits": 0,
                "cache_misses": 0,
                "query_count": 0,
                "total_query_time_ms": 0.0,
                "average_query_time_ms": 0.0
            }
            return stats
    
    def close(self) -> None:
        """
        Close database connection.
        """
        if self.is_connected:
            if HAS_RUST_BINDINGS and self._rust_instance:
                self._rust_instance.close()
            self.is_connected = False
            print("Database connection closed")
    
    
    async def __aenter__(self):
        return self
    
    async def __aexit__(self, exc_type, exc_val, exc_tb):
        await self.close()
        return False

    async def close(self) -> None:
        """
        Close database connection asynchronously.
        """
        if self.is_connected:
            if HAS_RUST_BINDINGS and self._rust_instance:
                self._rust_instance.close()
            self.is_connected = False
            print("Database connection closed")
        
        await asyncio.sleep(0)  


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
            QueryError: Raised when query execution fails
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
                raise QueryError(f"Query execution failed: {str(e)}")
        else:
            
            print(f"Executing query: {query}")
            

            query_lower = query.lower().strip()
            
            if query_lower.startswith("match") or query_lower.startswith("select"):

                schema = [
                    {"name": "node_id", "type": "Integer"},
                    {"name": "node_label", "type": "String"},
                    {"name": "properties", "type": "Map"}
                ]
                
                if self._stored_data:
                    data = []
                    for i, item in enumerate(self._stored_data):
                        data.append([i+1, item.get("label", "Node"), item])
                else:

                    data = [
                        [1, "Person", {"name": "Alice", "age": 30}],
                        [2, "Person", {"name": "Bob", "age": 25}],
                        [3, "Company", {"name": "TechCorp", "founded": 2010}]
                    ]
                    
                metrics = {
                    "parsing_time_ms": 0.1,
                    "planning_time_ms": 0.3,
                    "execution_time_ms": 1.2
                }
                return QueryResult(schema, data, metrics)
            elif "count" in query_lower:
               
                schema = [
                    {"name": "count", "type": "Integer"}
                ]
                data = [[len(self._stored_data)]] if self._stored_data else [[0]]
                metrics = {
                    "parsing_time_ms": 0.05,
                    "planning_time_ms": 0.1,
                    "execution_time_ms": 0.2
                }
                return QueryResult(schema, data, metrics)
            elif query_lower.startswith("create graph"):
               
                print("Graph created (simulated)")
                return QueryResult()
            elif query_lower.startswith("insert"):
               
                print("Data inserted (simulated)")
                return QueryResult()
            elif query_lower.startswith("delete"):
               
                print("Data deleted (simulated)")
                return QueryResult()
            else:
                
                return QueryResult()
    
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
           
            try:
                
                if self._stored_data:
                    with open(path, 'w', encoding='utf-8') as f:
                        json.dump(self._stored_data, f, ensure_ascii=False, indent=2)
                    print(f"Database saved to {path} as JSON")
                else:
                    
                    with open(path, 'w') as f:
                        f.write("")
                    print(f"Empty database saved to {path}")
            except Exception as e:
                raise DataError(f"Database save failed: {str(e)}")
    
    def create_graph(self, graph_name: str, schema: Optional[Dict] = None) -> None:
        """
        Create a graph database
        
        Args:
            graph_name: Graph name
            schema: Graph schema definition (optional)
            
        Raises:
            MiniGUError: Raised when database is not connected
            GraphError: Raised when graph creation fails
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
          
            try:
                if schema:
                    schema_str = self._format_schema(schema)
                    self._rust_instance.create_graph(graph_name, schema_str)
                else:
                    self._rust_instance.create_graph(graph_name, None)
                print(f"Graph '{graph_name}' created")
            except Exception as e:
                raise GraphError(f"Graph creation failed: {str(e)}")
        else:
          
            if schema:
                query = f"CREATE GRAPH {graph_name} {{ {self._format_schema(schema)} }}"
            else:
                query = f"CREATE GRAPH {graph_name} ANY"
            
           
            self.execute(query)
            print(f"Graph '{graph_name}' created (simulated)")
    
    def _format_schema(self, schema: Dict) -> str:
        """
        Format graph schema definition.
        
        Args:
            schema: Graph schema definition
            
        Returns:
            Formatted schema string
        """
       
        elements = []
        for label, properties in schema.items():
            props = ", ".join([f"{name} {ptype}" for name, ptype in properties.items()])
            elements.append(f"({label} :{label} {{{props}}})")
        return "; ".join(elements)
    
    def set_cache_size(self, size: int) -> None:
        """
        Set the size of the query result cache.
        
        Args:
            size: Cache size in number of entries
            
        Raises:
            MiniGUError: Raised when database is not connected
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
    
        if HAS_RUST_BINDINGS and hasattr(self._rust_instance, 'set_cache_size'):
            try:
                self._rust_instance.set_cache_size(size)
                self.cache_size = size
                print(f"Cache size set to {size} entries")
            except Exception as e:
                raise DataError(f"Failed to set cache size: {str(e)}")
        else:
            
            raise RuntimeError("Rust bindings required for database operations")
    
    def set_thread_count(self, count: int) -> None:
        """
        Set the number of threads for parallel query execution.
        
        Args:
            count: Number of threads
            
        Raises:
            MiniGUError: Raised when database is not connected
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        
        if HAS_RUST_BINDINGS and hasattr(self._rust_instance, 'set_thread_count'):
            try:
                self._rust_instance.set_thread_count(count)
                self.thread_count = count
                print(f"Thread count set to {count}")
            except Exception as e:
                raise DataError(f"Failed to set thread count: {str(e)}")
        else:
        
            self.thread_count = count
            print(f"Thread count set to {count} (simulated)")
    
    def enable_query_logging(self, enable: bool = True) -> None:
        """
        Enable or disable query execution logging.
        
        Args:
            enable: Whether to enable logging
            
        Raises:
            MiniGUError: Raised when database is not connected
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
    
        if HAS_RUST_BINDINGS and hasattr(self._rust_instance, 'enable_query_logging'):
            try:
                self._rust_instance.enable_query_logging(enable)
                self.enable_logging = enable
                status = "enabled" if enable else "disabled"
                print(f"Query logging {status}")
            except Exception as e:
                raise DataError(f"Failed to set query logging: {str(e)}")
        else:
        
            self.enable_logging = enable
            status = "enabled" if enable else "disabled"
            print(f"Query logging {status} (simulated)")
    
    def get_performance_stats(self) -> Dict[str, Any]:
        """
        Get database performance statistics.
        
        Returns:
            Dictionary containing performance statistics
            
        Raises:
            MiniGUError: Raised when database is not connected
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        
        if HAS_RUST_BINDINGS and hasattr(self._rust_instance, 'get_performance_stats'):
            try:
                stats = self._rust_instance.get_performance_stats()
                return stats
            except Exception as e:
                raise DataError(f"Failed to get performance stats: {str(e)}")
        else:
            stats = {
                "cache_hits": 0,
                "cache_misses": 0,
                "query_count": 0,
                "total_query_time_ms": 0.0,
                "average_query_time_ms": 0.0
            }
            return stats
    
    def insert(self, data: Union[List[Dict], str]) -> None:
        """
        Insert data into the current graph
        
        Args:
            data: List of data to insert or GQL INSERT statement
            
        Raises:
            MiniGUError: Raised when database is not connected
            DataError: Raised when data insertion fails
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
            try:
                if isinstance(data, str):
                    self._rust_instance.insert_data(data)
                else:
                    # Format each record and insert as separate statements
                    for item in data:
                        gql_data = self._format_insert_data([item])
                        self._rust_instance.insert_data(gql_data)
                print(f"Data inserted successfully")
            except Exception as e:
                raise DataError(f"Data insertion failed: {str(e)}")
        else:
    
            raise RuntimeError("Rust bindings required for database operations")
    
    def _format_insert_data(self, data: List[Dict]) -> str:
        """
        Format data as GQL INSERT statement
        
        Args:
            data: List of data to insert
            
        Returns:
            GQL INSERT statement fragment
        """
        # Based on the GQL examples, we should use :Label syntax instead of (Label)
        # and generate separate INSERT statements for each record
        statements = []
        for item in data:
            label = item.get("label", "Node")
            # Format properties correctly for GQL
            # Based on examples, we should not put quotes around all values
            props = []
            for k, v in item.items():
                if k != "label":
                    # Handle different data types appropriately
                    if isinstance(v, str):
                        props.append(f"{k}: '{v}'")
                    elif isinstance(v, (int, float)):
                        props.append(f"{k}: {v}")
                    else:
                        # For other types, convert to string and quote
                        props.append(f"{k}: '{str(v)}'")
            
            props_str = ", ".join(props)
            # Based on GQL examples, use :Label syntax and separate INSERT statements
            statement = f"INSERT :{label} {{ {props_str} }}"
            statements.append(statement)
            
        # Join statements with semicolon and space
        return "; ".join(statements)
    
    def create_node(self, label: str, properties: Optional[Dict[str, Any]] = None) -> Node:
        """
        Create a node object
        
        Args:
            label: Node label
            properties: Node properties
            
        Returns:
            Node object
        """
        return Node(label, properties)
    
    def create_edge(self, label: str, src: Union[Node, int], dst: Union[Node, int], 
                    properties: Optional[Dict[str, Any]] = None) -> Edge:
        """
        Create an edge object
        
        Args:
            label: Edge label
            src: Source node or node ID
            dst: Destination node or node ID
            properties: Edge properties
            
        Returns:
            Edge object
        """
        return Edge(label, src, dst, properties)
    
    def create_path(self, nodes: List[Node], edges: List[Edge]) -> Path:
        """
        Create a path object
        
        Args:
            nodes: List of nodes
            edges: List of edges
            
        Returns:
            Path object
        """
        return Path(nodes, edges)
    
    def update(self, query: str) -> None:
        """
        Update data in the current graph using a GQL UPDATE statement.
        
        Args:
            query: GQL UPDATE statement
            
        Raises:
            MiniGUError: Raised when database is not connected
            QueryError: Raised when query execution fails
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
            try:
                self._rust_instance.update_data(query)
                print(f"Data updated successfully with query: {query}")
            except Exception as e:
                raise QueryError(f"Data update failed: {str(e)}")
        else:
            print(f"Executing UPDATE statement: {query}")
            print("Data updated (simulated)")
    
    def delete(self, query: str) -> None:
        """
        Delete data from the current graph using a GQL DELETE statement.
        
        Args:
            query: GQL DELETE statement
            
        Raises:
            MiniGUError: Raised when database is not connected
            QueryError: Raised when query execution fails
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
            try:
                self._rust_instance.delete_data(query)
                print(f"Data deleted successfully with query: {query}")
            except Exception as e:
                raise QueryError(f"Data deletion failed: {str(e)}")
        else:
            print(f"Executing DELETE statement: {query}")
            print("Data deleted (simulated)")
    
    def close(self) -> None:
        """
        Close database connection
        """
        if self.is_connected:
            if HAS_RUST_BINDINGS and self._rust_instance:
                self._rust_instance.close()
            self.is_connected = False
            print("Database connection closed")
    
    def __enter__(self):
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()


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
    connection = AsyncMiniGU(db_path, thread_count, cache_size, enable_logging)
    return connection

if __name__ == "__main__":

    with connect() as db:

        result = db.execute("MATCH (n) RETURN n;")
        print(result)
        print("As dictionary list:", result.to_list())
        
        print("Query metrics:", result.metrics)
        

        sample_data = [
            {"name": "Alice", "age": 30, "label": "Person"},
            {"name": "Bob", "age": 25, "label": "Person"},
            {"name": "TechCorp", "founded": 2010, "label": "Company"}
        ]
        db.load(sample_data)
        
        db.save("example.mgu")
    

    db = connect("example.db")
    try:
        result = db.execute("MATCH (n:Person) RETURN n.name, n.age;")
        print(result.to_list())
        
        db.create_graph("social_network", {
            "Person": {"name": "STRING", "age": "INTEGER"},
            "Company": {"name": "STRING", "founded": "INTEGER"}
        })
        
        
        db.insert([
            {"name": "Charlie", "age": 35, "label": "Person"},
            {"name": "InnovateCo", "founded": 2015, "label": "Company"}
        ])
        
        
        db.insert("VERTEX Person {name: 'David', age: 28}")
        
    
        db.save("social_network.mgu")
    finally:
        db.close()
