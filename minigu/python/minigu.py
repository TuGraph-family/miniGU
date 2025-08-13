"""
miniGU Python API
"""

import sys
from typing import Optional, List, Dict, Any, Union
from pathlib import Path
import json

# 尝试导入Rust绑定
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
        print("Warning: Rust bindings not available. Using simulated implementation.")


class MiniGUError(Exception):
    """miniGU database exception class"""
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
    
    def __repr__(self) -> str:
        return f"QueryResult(rows={self.row_count}, columns={len(self.schema)})"


class MiniGU:
    """
    miniGU database connection class
    """
    
    def __init__(self, db_path: Optional[str] = None):
        """
        Initialize miniGU database connection
        
        Args:
            db_path: Database file path, if None creates an in-memory database
        """
        self.db_path = db_path
        self._rust_instance = None
        self.is_connected = False
        self._stored_data = []  # 用于模拟存储数据
        self._connect()
    
    def _connect(self) -> None:
        """
        Establish database connection
        """
        try:
            if HAS_RUST_BINDINGS:
                self._rust_instance = PyMiniGU()
            self.is_connected = True
            print("Database connected")
        except Exception as e:
            raise MiniGUError(f"Failed to connect to database: {str(e)}")
    
    def execute(self, query: str) -> QueryResult:
        """
        Execute GQL query
        
        Args:
            query: GQL query statement
            
        Returns:
            Query result
            
        Raises:
            MiniGUError: Raised when query execution fails
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
            # 使用真实的Rust绑定执行查询
            try:
                result = self._rust_instance.execute(query)
                # 将Rust返回的结果转换为Python对象
                return QueryResult(
                    schema=result.get("schema", []),
                    data=result.get("data", []),
                    metrics=result.get("metrics", {})
                )
            except Exception as e:
                raise MiniGUError(f"Query execution failed: {str(e)}")
        else:
            # 模拟查询过程
            print(f"Executing query: {query}")
            
            # 解析查询类型并模拟结果
            query_lower = query.lower().strip()
            
            if query_lower.startswith("match") or query_lower.startswith("select"):
                # 模拟图查询结果
                schema = [
                    {"name": "node_id", "type": "Integer"},
                    {"name": "node_label", "type": "String"},
                    {"name": "properties", "type": "Map"}
                ]
                
                # 如果有存储的数据，返回它
                if self._stored_data:
                    data = []
                    for i, item in enumerate(self._stored_data):
                        data.append([i+1, item.get("label", "Node"), item])
                else:
                    # 默认数据
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
                # 模拟计数查询
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
                # 模拟创建图
                print("Graph created (simulated)")
                return QueryResult()
            elif query_lower.startswith("insert"):
                # 模拟插入数据
                print("Data inserted (simulated)")
                return QueryResult()
            elif query_lower.startswith("delete"):
                # 模拟删除数据
                print("Data deleted (simulated)")
                return QueryResult()
            else:
                # 模拟其他操作结果
                return QueryResult()
    
    def load(self, data: Union[List[Dict], str, Path]) -> None:
        """
        Load data into the database
        
        Args:
            data: Data to load, can be a list of dictionaries or file path
            
        Raises:
            MiniGUError: Raised when data loading fails
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
            # 使用真实的Rust绑定加载数据
            try:
                if isinstance(data, (str, Path)):
                    # 如果是文件路径，读取文件
                    self._rust_instance.load_from_file(str(data))
                else:
                    # 如果是数据对象，直接加载
                    self._rust_instance.load_data(data)
                    # 保存数据用于模拟
                    self._stored_data = data
                print(f"Data loaded successfully")
            except Exception as e:
                raise MiniGUError(f"Data loading failed: {str(e)}")
        else:
            # 模拟加载过程
            if isinstance(data, (str, Path)):
                file_path = str(data)
                print(f"Loading data from file: {file_path}")
                # 尝试读取JSON文件
                if file_path.endswith('.json'):
                    try:
                        with open(file_path, 'r', encoding='utf-8') as f:
                            file_data = json.load(f)
                            self._stored_data = file_data
                            print(f"Loaded {len(file_data)} records from JSON file")
                    except Exception as e:
                        print(f"Warning: Could not parse JSON file: {e}")
                else:
                    print("File format not recognized, treating as generic file load")
            else:
                self._stored_data = data
                print(f"Loading {len(data)} records into database")
            print("Data loaded (simulated)")
    
    def save(self, path: str) -> None:
        """
        Save the database to the specified path
        
        Args:
            path: Save path
            
        Raises:
            MiniGUError: Raised when save fails
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
            # 使用真实的Rust绑定保存数据
            try:
                self._rust_instance.save_to_file(path)
                print(f"Database saved to {path}")
            except Exception as e:
                raise MiniGUError(f"Database save failed: {str(e)}")
        else:
            # 模拟保存过程
            try:
                # 如果有数据，保存为JSON格式
                if self._stored_data:
                    with open(path, 'w', encoding='utf-8') as f:
                        json.dump(self._stored_data, f, ensure_ascii=False, indent=2)
                    print(f"Database saved to {path} as JSON")
                else:
                    # 创建空文件
                    with open(path, 'w') as f:
                        f.write("")
                    print(f"Empty database saved to {path}")
            except Exception as e:
                raise MiniGUError(f"Database save failed: {str(e)}")
    
    def create_graph(self, graph_name: str, schema: Optional[Dict] = None) -> None:
        """
        Create a graph database
        
        Args:
            graph_name: Graph name
            schema: Graph schema definition (optional)
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
            # 使用真实的Rust绑定创建图
            try:
                if schema:
                    schema_str = self._format_schema(schema)
                    self._rust_instance.create_graph(graph_name, schema_str)
                else:
                    self._rust_instance.create_graph(graph_name, None)
                print(f"Graph '{graph_name}' created")
            except Exception as e:
                raise MiniGUError(f"Graph creation failed: {str(e)}")
        else:
            # 模拟创建图过程
            if schema:
                query = f"CREATE GRAPH {graph_name} {{ {self._format_schema(schema)} }}"
            else:
                query = f"CREATE GRAPH {graph_name} ANY"
            
            # 执行查询
            self.execute(query)
            print(f"Graph '{graph_name}' created (simulated)")
    
    def _format_schema(self, schema: Dict) -> str:
        """
        Format graph schema definition
        
        Args:
            schema: Graph schema definition
            
        Returns:
            Formatted schema string
        """
        # 简单实现，实际应该更复杂
        elements = []
        for label, properties in schema.items():
            props = ", ".join([f"{name} {ptype}" for name, ptype in properties.items()])
            elements.append(f"({label} :{label} {{{props}}})")
        return "; ".join(elements)
    
    def insert(self, data: Union[List[Dict], str]) -> None:
        """
        Insert data into the current graph
        
        Args:
            data: List of data to insert or GQL INSERT statement
        """
        if not self.is_connected:
            raise MiniGUError("Database not connected")
        
        if HAS_RUST_BINDINGS and self._rust_instance:
            # 使用真实的Rust绑定插入数据
            try:
                if isinstance(data, str):
                    # 如果是字符串，直接作为GQL INSERT语句执行
                    self._rust_instance.insert_data(data)
                else:
                    # 如果是数据对象，转换为GQL INSERT语句
                    gql_data = self._format_insert_data(data)
                    self._rust_instance.insert_data(gql_data)
                print(f"Data inserted successfully")
            except Exception as e:
                raise MiniGUError(f"Data insertion failed: {str(e)}")
        else:
            # 模拟插入过程
            if isinstance(data, str):
                print(f"Executing INSERT statement: {data}")
            else:
                print(f"Inserting {len(data)} records")
                # 复用load方法添加数据
                if isinstance(data, list):
                    self._stored_data.extend(data)
            print("Data inserted (simulated)")
    
    def _format_insert_data(self, data: List[Dict]) -> str:
        """
        Format data as GQL INSERT statement
        
        Args:
            data: List of data to insert
            
        Returns:
            GQL INSERT statement fragment
        """
        # 简单实现，实际应该更复杂
        records = []
        for item in data:
            label = item.get("label", "Node")
            props = ", ".join([f"{k}: '{v}'" for k, v in item.items() if k != "label"])
            records.append(f"({label} {{{props}}})")
        return ", ".join(records)
    
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


def connect(db_path: Optional[str] = None) -> MiniGU:
    """
    Create a connection to the miniGU database
    
    Args:
        db_path: Database file path, if None creates an in-memory database
        
    Returns:
        MiniGU database connection object
    """
    if HAS_RUST_BINDINGS:
        # 使用真实的Rust绑定
        return MiniGU()
    else:
        # 使用模拟实现
        return MiniGU(db_path)


# 使用示例
if __name__ == "__main__":
    # 示例1: 使用上下文管理器连接数据库
    with connect() as db:
        # 执行图查询
        result = db.execute("MATCH (n) RETURN n;")
        print(result)
        print("As dictionary list:", result.to_list())
        
        # 查看查询指标
        print("Query metrics:", result.metrics)
        
        # 加载数据
        sample_data = [
            {"name": "Alice", "age": 30, "label": "Person"},
            {"name": "Bob", "age": 25, "label": "Person"},
            {"name": "TechCorp", "founded": 2010, "label": "Company"}
        ]
        db.load(sample_data)
        
        # 保存数据库
        db.save("example.mgu")
    
    # 示例2: 直接创建连接
    db = connect("example.db")
    try:
        result = db.execute("MATCH (n:Person) RETURN n.name, n.age;")
        print(result.to_list())
        
        # 创建图
        db.create_graph("social_network", {
            "Person": {"name": "STRING", "age": "INTEGER"},
            "Company": {"name": "STRING", "founded": "INTEGER"}
        })
        
        # 插入数据
        db.insert([
            {"name": "Charlie", "age": 35, "label": "Person"},
            {"name": "InnovateCo", "founded": 2015, "label": "Company"}
        ])
        
        # 使用GQL INSERT语句
        db.insert("VERTEX Person {name: 'David', age: 28}")
        
        # 保存数据库
        db.save("social_network.mgu")
    finally:
        db.close()