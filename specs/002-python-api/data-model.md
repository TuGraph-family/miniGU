# Data Model: Python API Bindings

**Feature**: 002-python-api
**Date**: 2026-04-17

## Entity Overview

Python API 的数据模型主要涉及用户交互的 Python 类，这些类封装了底层的 Rust 实现。

```
┌─────────────────────────────────────────────────────────────┐
│                     Python API Layer                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐     ┌──────────────────┐                  │
│  │   MiniGU    │────▶│   QueryResult    │                  │
│  │  (sync)     │     │  - schema        │                  │
│  └─────────────┘     │  - data          │                  │
│                      │  - metrics       │                  │
│  ┌─────────────┐     └──────────────────┘                  │
│  │ AsyncMiniGU │                                            │
│  │  (async)    │     ┌──────────────────┐                  │
│  └─────────────┘────▶│  Exception Types │                  │
│                      │  - MiniGUError   │                  │
│                      │  - QueryError    │                  │
│                      │  - ...           │                  │
│                      └──────────────────┘                  │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                     Rust FFI Layer                          │
├─────────────────────────────────────────────────────────────┤
│  PyMiniGU (PyO3 class)                                      │
│  - init()                                                   │
│  - execute(query) -> dict                                   │
│  - create_graph(name) -> bool                               │
│  - load(data) -> bool                                       │
│  - save(path) -> bool                                       │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                     miniGU Core                             │
├─────────────────────────────────────────────────────────────┤
│  Database, Session, QueryResult (Rust types)                │
└─────────────────────────────────────────────────────────────┘
```

## Entity Definitions

### 1. MiniGU (同步 API 主类)

**Purpose**: 提供同步方式的图数据库操作接口

**Fields**:
| Field | Type | Description |
|-------|------|-------------|
| _db | PyMiniGU | Rust 绑定实例 (内部) |
| _connected | bool | 连接状态 |

**Methods**:
| Method | Parameters | Return | Description |
|--------|------------|--------|-------------|
| `__init__` | db_path: Optional[str] = None | None | 初始化实例 |
| `init` | - | None | 初始化数据库连接 |
| `create_graph` | name: str | bool | 创建图 |
| `execute` | query: str | QueryResult | 执行 GQL 查询 |
| `load` |  Union[List[Dict], str] | bool | 加载数据 |
| `save` | path: str | bool | 保存数据库 |
| `is_connected` | - | bool | 检查连接状态 |
| `__enter__` | - | Self | 上下文管理器入口 |
| `__exit__` | *args | None | 上下文管理器出口 |

**State Transitions**:
```
[未初始化] --init()--> [已连接] --close()--> [已断开]
     │                                       │
     └─────────── execute() raises ──────────┘
                  ConnectionError
```

### 2. AsyncMiniGU (异步 API 主类)

**Purpose**: 提供异步方式的图数据库操作接口

**Fields**:
| Field | Type | Description |
|-------|------|-------------|
| _sync_db | MiniGU | 同步实例 (内部) |
| _connected | bool | 连接状态 |

**Methods**:
| Method | Parameters | Return | Description |
|--------|------------|--------|-------------|
| `__init__` | db_path: Optional[str] = None | None | 初始化实例 |
| `init` | - | None | 异步初始化数据库连接 |
| `create_graph` | name: str | bool | 异步创建图 |
| `execute` | query: str | QueryResult | 异步执行 GQL 查询 |
| `load` | data: Union[List[Dict], str] | bool | 异步加载数据 |
| `save` | path: str | bool | 异步保存数据库 |
| `is_connected` | - | bool | 检查连接状态 |
| `__aenter__` | - | Self | 异步上下文管理器入口 |
| `__aexit__` | *args | None | 异步上下文管理器出口 |

### 3. QueryResult (查询结果)

**Purpose**: 封装查询结果，提供友好的数据访问接口

**Fields**:
| Field | Type | Description |
|-------|------|-------------|
| schema | List[Dict] | 字段信息列表 |
| data | List[List] | 数据行列表 |
| metrics | Dict[str, Any] | 查询指标 (可选) |

**Schema Structure**:
```python
schema = [
    {"name": "n", "data_type": "Vertex"},
    {"name": "age", "data_type": "Int64"}
]
```

**Methods**:
| Method | Return | Description |
|--------|--------|-------------|
| `__iter__` | Iterator | 迭代数据行 |
| `__len__` | int | 返回行数 |
| `__getitem__` | List | 按索引访问行 |

### 4. Exception Types (异常类型)

**Hierarchy**:
```
Exception (Python built-in)
└── MiniGUError
    ├── ConnectionError      # 数据库连接失败
    ├── QueryError           # 查询相关错误 (基类)
    │   ├── QuerySyntaxError     # GQL 语法错误
    │   ├── QueryExecutionError  # 查询执行错误
    │   └── QueryTimeoutError    # 查询超时
    ├── GraphError           # 图操作错误
    ├── DataError            # 数据加载/保存错误
    └── TransactionError     # 事务错误
```

**Exception Fields**:
| Exception | Additional Fields | Common Use |
|-----------|-------------------|------------|
| QuerySyntaxError | query: str (可选) | 无效 GQL 语法 |
| QueryTimeoutError | timeout: int (可选) | 查询超时 |
| GraphError | graph_name: str (可选) | 图已存在/不存在 |

## Validation Rules

### Graph Name Validation
```python
# 规则：仅允许字母数字和下划线
def validate_graph_name(name: str) -> str:
    sanitized = ''.join(c for c in name if c.isalnum() or c == '_')
    if not sanitized:
        raise GraphError("Invalid graph name")
    return sanitized
```

### File Path Validation
```python
# 规则：防止目录遍历攻击
def validate_path(path: str) -> str:
    if '..' in path:
        raise DataError("Directory traversal not allowed")
    return path.replace('\'', '').replace('"', '')
```

### Query Validation
```python
# 规则：查询字符串不能为空
def validate_query(query: Optional[str]) -> str:
    if query is None or not query.strip():
        raise TypeError("Query cannot be None or empty")
    return query
```

## Data Flow

### Query Execution Flow

```
User Code                Python API              Rust FFI              miniGU Core
    │                        │                      │                      │
    │ execute("MATCH...")    │                      │                      │
    ├───────────────────────▶│                      │                      │
    │                        │ validate_query()     │                      │
    │                        ├──────────────────────▶                      │
    │                        │                      │ query()               │
    │                        │                      ├─────────────────────▶│
    │                        │                      │                      │ execute
    │                        │                      │                      │───┐
    │                        │                      │                      │   │
    │                        │                      │                      │◀──┘
    │                        │                      │ QueryResult          │
    │                        │                      │◀─────────────────────│
    │                        │ dict (schema+data)   │                      │
    │                        │◀─────────────────────│                      │
    │ QueryResult            │                      │                      │
    │◀───────────────────────│                      │                      │
```

### Error Handling Flow

```
Rust Error              Python API
    │                       │
    │ PyErr (from Err)      │
    ├──────────────────────▶│
    │                       │ _handle_exception()
    │                       ├─────────────────▶
    │                       │                  │
    │                       │ check error msg  │
    │                       │◀─────────────────┘
    │                       │
    │                       │ raise QuerySyntaxError
    │                       │──────────────────▶ User
```