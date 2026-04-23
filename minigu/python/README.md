# miniGU Python接口

该目录包含了miniGU图数据库的Python接口，使用PyO3构建，为Rust实现提供原生Python绑定。

## 目录
1. [如何使用Python接口](#如何使用python接口)
2. [配置选项](#配置选项)
3. [封装的接口](#封装的接口)
4. [数据结构](#数据结构)
5. [异常处理](#异常处理)

## 如何使用Python接口

### 安装

miniGU的Python接口可以通过以下方式安装：

#### 方法一：使用maturin（推荐）

```bash
# 创建并激活虚拟环境
python -m venv minigu-env
source minigu-env/bin/activate  # Windows系统：minigu-env\Scripts\activate

# 安装maturin (参考官方文档: https://maturin.rs/)
pip install maturin

# 构建并安装包
cd minigu/python
maturin build --release
pip install --force-reinstall ../../target/wheels/minigu-0.1.0-cp37-abi3-*.whl
```

#### 方法二：开发模式安装

```bash
# 创建并激活虚拟环境
python -m venv minigu-env
source minigu-env/bin/activate  # Windows系统：minigu-env\Scripts\activate

# 安装maturin (参考官方文档: https://maturin.rs/)
pip install maturin

# 以开发模式安装
cd minigu/python
maturin develop
```

### 基本用法

#### 同步接口

```python
import minigu

# 连接数据库
db = minigu.MiniGU()
db.init()

# 创建图
success = db.create_graph("my_graph")
if success:
    print("Graph created successfully")

# 执行查询
result = db.execute("MATCH (n) RETURN n LIMIT 10")
print(result.data)

# 使用上下文管理器（推荐）
with minigu.MiniGU() as db:
    db.create_graph("my_graph")
    result = db.execute("MATCH (n) RETURN n LIMIT 10")
    print(result.data)
```

#### 异步接口

```python
import asyncio
import minigu

async def main():
    # 连接数据库
    db = minigu.AsyncMiniGU()

    async with db:
        # 创建图
        success = await db.create_graph("my_graph")
        if success:
            print("Graph created successfully")

        # 执行查询
        result = await db.execute("MATCH (n) RETURN n LIMIT 10")
        print(result.data)

# 运行异步函数
asyncio.run(main())
```

### 数据操作

#### 加载数据

```python
# 加载字典列表
data = [
    {"label": "Person", "name": "Alice", "age": 30},
    {"label": "Person", "name": "Bob", "age": 25}
]
db.load(data)

# 从文件加载数据
db.load("/path/to/data.csv")
```

#### 保存数据

```python
# 保存到指定路径
db.save("/path/to/save/location")
```

## 配置选项

### Cargo.toml配置

```toml
[package]
name = "minigu-python"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]
name = "minigu_python"

[dependencies]
arrow = { workspace = true }
minigu = { workspace = true }
pyo3 = { workspace = true, features = ["extension-module", "abi3-py37"] }

[build-dependencies]
pyo3-build-config = "0.24.2"
```

### pyproject.toml配置

```toml
[build-system]
build-backend = "maturin"
requires = ["maturin>=1.0,<2.0"]

[project]
name = "minigu"
version = "0.1.0"
description = "A graph database for learning purposes"
requires-python = ">=3.7"

[tool.maturin]
features = ["pyo3/extension-module"]
module-name = "minigu.minigu_python"
```

## 封装的接口

### 同步接口 (MiniGU)

| 方法 | 参数 | 返回值 | 描述 |
|------|------|--------|------|
| `init()` | 无 | None | 初始化数据库连接 |
| `create_graph(name)` | name: str | bool | 创建图 |
| `execute(query)` | query: str | QueryResult | 执行GQL查询 |
| `load(data)` | data: List[Dict] 或 str | bool | 加载数据 |
| `save(path)` | path: str | bool | 保存数据库 |
| `is_connected` | 无 | bool | 检查连接状态 |
| `close()` | 无 | None | 关闭连接 |

### 异步接口 (AsyncMiniGU)

与同步接口相同，但所有方法都是异步的。

## 数据结构

### QueryResult

查询结果封装类：

```python
class QueryResult:
    schema: List[Dict]  # 字段信息
     List[List]    # 数据行
    metrics: Dict       # 查询指标

    def __iter__(self) -> Iterator[List]
    def __len__(self) -> int
    def __getitem__(index: int) -> List
```

### 类型映射

| GQL类型 | Python类型 |
|---------|-----------|
| STRING | str |
| INT8/16/32/64 | int |
| FLOAT32/64 | float |
| BOOLEAN | bool |
| NULL | None |
| VERTEX | dict |
| EDGE | dict |
| VECTOR | list[float] |

## 异常处理

### 异常层次结构

```
MiniGUError (基类)
├── ConnectionError      # 数据库连接错误
├── QueryError           # 查询错误基类
│   ├── QuerySyntaxError     # GQL语法错误
│   ├── QueryExecutionError  # 查询执行错误
│   └── QueryTimeoutError    # 查询超时
├── GraphError           # 图操作错误
├── DataError            # 数据加载/保存错误
└── TransactionError     # 事务错误
```

### 异常处理示例

```python
from minigu import MiniGU, QuerySyntaxError, GraphError

db = MiniGU()
db.init()

try:
    db.execute("INVALID GQL SYNTAX")
except QuerySyntaxError as e:
    print(f"语法错误: {e}")

try:
    db.create_graph("existing_graph")  # 图已存在
except GraphError as e:
    print(f"图操作错误: {e}")
```

## 开发指南

### 运行测试

```bash
# 安装开发依赖
pip install pytest pytest-asyncio

# 运行测试
cd minigu/python
pytest test_minigu_api.py -v
```

### 代码检查

```bash
# 运行 Rust 代码检查
cargo clippy -p minigu-python

# 运行 Python 代码检查 (需要安装 mypy)
mypy minigu.py
```

## 许可证

Apache-2.0