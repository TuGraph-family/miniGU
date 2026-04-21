# Python API Contract

**Feature**: 002-python-api
**Date**: 2026-04-17
**Type**: Public Python API

## Module: `minigu`

### Classes

#### `class MiniGU`

同步 API 主类。

**Constructor**:
```python
def __init__(self, db_path: Optional[str] = None) -> None:
    """
    初始化 MiniGU 实例。

    Args:
        db_path: 数据库路径，None 表示使用内存数据库

    Example:
        >>> db = MiniGU()
        >>> db = MiniGU("/path/to/db")
    """
```

**Methods**:

```python
def init(self) -> None:
    """
    初始化数据库连接。

    Raises:
        ConnectionError: 数据库初始化失败

    Example:
        >>> db = MiniGU()
        >>> db.init()
    """

def create_graph(self, name: str) -> bool:
    """
    创建图。

    Args:
        name: 图名称，仅允许字母数字和下划线

    Returns:
        True 表示创建成功

    Raises:
        GraphError: 图已存在或名称无效
        ConnectionError: 数据库未初始化

    Example:
        >>> db.create_graph("my_graph")
        True
    """

def execute(self, query: str) -> QueryResult:
    """
    执行 GQL 查询。

    Args:
        query: GQL 查询字符串

    Returns:
        QueryResult 对象，包含 schema 和 data

    Raises:
        QuerySyntaxError: GQL 语法错误
        QueryExecutionError: 查询执行错误
        QueryTimeoutError: 查询超时
        ConnectionError: 数据库未初始化

    Example:
        >>> result = db.execute("MATCH (n:Person) RETURN n.name, n.age")
        >>> result.schema
        [{'name': 'n.name', 'data_type': 'String'}, {'name': 'n.age', 'data_type': 'Int64'}]
        >>> result.data
        [['Alice', 30], ['Bob', 25]]
    """

def load(self, data: Union[List[Dict[str, Any]], str]) -> bool:
    """
    加载数据到图数据库。

    Args:
        data: 字典列表或 CSV 文件路径

    Returns:
        True 表示加载成功

    Raises:
        DataError: 数据格式无效或文件不存在

    Example:
        >>> db.load([{"name": "Alice", "age": 30}])
        True
        >>> db.load("/path/to/data.csv")
        True
    """

def save(self, path: str) -> bool:
    """
    保存数据库到指定路径。

    Args:
        path: 保存路径

    Returns:
        True 表示保存成功

    Raises:
        DataError: 路径无效或无写入权限

    Example:
        >>> db.save("/path/to/backup")
        True
    """

@property
def is_connected(self) -> bool:
    """
    检查数据库连接状态。

    Returns:
        True 表示已连接
    """
```

**Context Manager Protocol**:
```python
def __enter__(self) -> MiniGU:
    """进入上下文，自动初始化连接"""

def __exit__(self, exc_type, exc_val, exc_tb) -> None:
    """退出上下文，自动清理资源"""
```

---

#### `class AsyncMiniGU`

异步 API 主类，方法签名与 `MiniGU` 相同但全部为异步方法。

**Constructor**:
```python
def __init__(self, db_path: Optional[str] = None) -> None
```

**Async Methods**:
```python
async def init(self) -> None
async def create_graph(self, name: str) -> bool
async def execute(self, query: str) -> QueryResult
async def load(self, data: Union[List[Dict[str, Any]], str]) -> bool
async def save(self, path: str) -> bool
```

**Async Context Manager Protocol**:
```python
async def __aenter__(self) -> AsyncMiniGU
async def __aexit__(self, exc_type, exc_val, exc_tb) -> None
```

---

#### `class QueryResult`

查询结果封装类。

```python
class QueryResult:
    schema: List[Dict[str, str]]  # 字段信息
     List[List[Any]]          # 数据行
    metrics: Dict[str, Any]        # 查询指标

    def __iter__(self) -> Iterator[List[Any]]
    def __len__(self) -> int
    def __getitem__(self, index: int) -> List[Any]
```

---

### Exceptions

```python
class MiniGUError(Exception):
    """miniGU 基础异常"""

class ConnectionError(MiniGUError):
    """数据库连接错误"""

class QueryError(MiniGUError):
    """查询错误基类"""

class QuerySyntaxError(QueryError):
    """GQL 语法错误"""

class QueryExecutionError(QueryError):
    """查询执行错误"""

class QueryTimeoutError(QueryError):
    """查询超时错误"""

class GraphError(MiniGUError):
    """图操作错误"""

class DataError(MiniGUError):
    """数据加载/保存错误"""

class TransactionError(MiniGUError):
    """事务错误"""
```

---

## Type Definitions

### Schema Field

```python
@dataclass
class SchemaField:
    name: str
    data_type: str  # "String" | "Int64" | "Float64" | "Boolean" | "Vertex" | "Edge"
```

### Data Types Mapping

| GQL Type | Python Type |
|----------|-------------|
| STRING | str |
| INT8/16/32/64 | int |
| FLOAT32/64 | float |
| BOOLEAN | bool |
| NULL | None |
| VERTEX | dict |
| EDGE | dict |
| VECTOR | list[float] |

---

## Versioning

- **Current Version**: 0.1.0
- **Python Support**: 3.7+
- **ABI Stability**: abi3-py37 (单个 wheel 支持所有 Python 3.7+)

## Breaking Change Policy

以下变更视为 breaking change，需要主版本号升级：
- 移除公开方法或类
- 更改方法签名（参数类型、返回类型）
- 更改异常层次结构

以下变更不视为 breaking change：
- 添加新方法或类
- 添加可选参数
- 内部实现变更