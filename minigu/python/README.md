# miniGU Python接口

该目录包含了miniGU图数据库的Python接口，使用PyO3构建，为Rust实现提供原生Python绑定。

## 功能特性

- 原生Python接口访问miniGU图数据库
- 支持异步API
- 支持上下文管理器自动管理资源
- 与Python标准异常处理机制集成
- 完全兼容Python 3.7+

## 环境要求

- Python 3.7或更高版本
- Rust工具链（用于从源码构建）
- [maturin](https://github.com/PyO3/maturin)用于构建Python包

## 安装方法

### 方法一：使用maturin（推荐）

1. 创建并激活虚拟环境：
   ```bash
   python -m venv minigu-env
   source minigu-env/bin/activate  # Windows系统：minigu-env\Scripts\activate
   ```

2. 安装maturin：
   ```bash
   pip install maturin
   ```

3. 构建并安装包：
   ```bash
   cd minigu/python
   maturin build --release
   pip install --force-reinstall ../../target/wheels/minigu-0.1.0-cp37-abi3-win_amd64.whl
   ```

### 方法二：开发模式安装

1. 创建并激活虚拟环境：
   ```bash
   python -m venv minigu-env
   source minigu-env/bin/activate  # Windows系统：minigu-env\Scripts\activate
   ```

2. 安装maturin：
   ```bash
   pip install maturin
   ```

3. 以开发模式安装：
   ```bash
   cd minigu/python
   maturin develop
   ```

## 配置说明

### Cargo.toml

[Cargo.toml](file:///d:/oo/dad/miniGU/minigu/Cargo.toml)文件包含Python绑定的Rust配置：

- `crate-type = ["cdylib"]`：指定构建Python动态库
- `name = "minigu_python"`：编译模块的名称
- 依赖项：
  - `pyo3`：Python绑定库，带有"extension-module"和"abi3-py37"特性
  - `arrow`：用于数据处理
  - `minigu`：主miniGU库

### pyproject.toml

[pyproject.toml](file:///d:/oo/dad/miniGU/minigu/python/pyproject.toml)文件包含Python包配置：

- `bindings = "pyo3"`：指定使用PyO3绑定
- `compatibility = "cp37"`：目标Python 3.7兼容性
- `features = ["extension-module"]`：启用扩展模块特性

## 可用接口

Python接口提供两个主要类：

### MiniGU（同步）

miniGU数据库的同步接口：

- `connect()`：创建数据库连接
- `execute(query)`：执行GQL查询
- `create_graph(name, schema)`：创建新图
- `begin_transaction()`：开始事务（当前未实现，调用会抛出[TransactionError](file://d:\oo\dad\miniGU\minigu\python\minigu.py#L96-L98)）
- `commit()`：提交当前事务（当前未实现，调用会抛出[TransactionError](file://d:\oo\dad\miniGU\minigu\python\minigu.py#L96-L98)）
- `rollback()`：回滚当前事务（当前未实现，调用会抛出[TransactionError](file://d:\oo\dad\miniGU\minigu\python\minigu.py#L96-L98)）
- 支持`with`语句的上下文管理器

### AsyncMiniGU（异步）

miniGU数据库的异步接口：

- `async_connect()`：创建异步数据库连接
- `execute(query)`：异步执行GQL查询
- `create_graph(name, schema)`：异步创建新图
- `begin_transaction()`：异步开始事务（当前未实现，调用会抛出[TransactionError](file://d:\oo\dad\miniGU\minigu\python\minigu.py#L96-L98)）
- `commit()`：异步提交当前事务（当前未实现，调用会抛出[TransactionError](file://d:\oo\dad\miniGU\minigu\python\minigu.py#L96-L98)）
- `rollback()`：异步回滚当前事务（当前未实现，调用会抛出[TransactionError](file://d:\oo\dad\miniGU\minigu\python\minigu.py#L96-L98)）
- 支持`async with`语句的上下文管理器

### 数据结构

- `QueryResult`：表示查询结果，包含模式、数据和指标
- `Vertex`：表示图节点
- `Edge`：表示图边
- `Path`：表示节点间路径

### 异常类型

- `MiniGUError`：所有miniGU错误的基类
- `ConnectionError`：连接失败时抛出
- `QuerySyntaxError`：查询语法错误时抛出
- `QueryExecutionError`：查询执行错误时抛出
- `QueryTimeoutError`：查询超时时抛出
- `GraphError`：图相关错误时抛出
- `TransactionError`：事务相关错误时抛出
- `DataError`：数据加载/保存错误时抛出

## 使用示例

### 基本用法

```
import minigu

# 连接数据库
db = minigu.connect()

# 创建图
db.create_graph("my_graph")

# 执行查询
result = db.execute("MATCH (n) RETURN n LIMIT 10")

# 打印结果
print(result.data)
```

### 使用上下文管理器

```
import minigu

# 完成后自动关闭连接
with minigu.connect() as db:
    db.create_graph("my_graph")
    result = db.execute("MATCH (n) RETURN n LIMIT 10")
    print(result.data)
```

### 异步用法

```
import asyncio
import minigu

async def main():
    # 异步连接数据库
    db = await minigu.async_connect()
    
    # 创建图
    await db.create_graph("my_graph")
    
    # 执行查询
    result = await db.execute("MATCH (n) RETURN n LIMIT 10")
    
    # 打印结果
    print(result.data)

# 运行异步函数
asyncio.run(main())
```

### 异步上下文管理器

```
import asyncio
import minigu

async def main():
    # 完成后自动关闭连接
    async with await minigu.async_connect() as db:
        await db.create_graph("my_graph")
        result = await db.execute("MATCH (n) RETURN n LIMIT 10")
        print(result.data)

# 运行异步函数
asyncio.run(main())
```

## 运行测试

运行测试的方法：
```
cd minigu/python
python test_minigu_api.py
```

测试会验证所有API方法，包括事务方法会正确抛出[TransactionError](file://d:\oo\dad\miniGU\minigu\python\minigu.py#L96-L98)异常。

## 故障排除

### "Rust bindings not available"错误

当Python接口无法加载Rust扩展模块时会出现此错误。解决方法：

1. 确保已构建整个miniGU项目：
   ```bash
   cargo build
   ```

2. 确保已构建Python包：
   ```bash
   cd minigu/python
   maturin build
   ```

3. 验证wheel包已安装：
   ```bash
   pip install --force-reinstall ../../target/wheels/minigu-0.1.0-cp37-abi3-win_amd64.whl
   ```

### ImportError问题

如果遇到导入错误，请确保：

1. 使用了正确的虚拟环境
2. 包已安装在虚拟环境中
3. 没有尝试从源代码目录导入

## 跨平台构建

Python接口可以为不同平台构建。maturin工具会自动检测您的平台并构建相应的包。如需交叉编译，请参考maturin文档。

## 当前状态

Python接口功能完整，所有核心功能均已实现：

1. 事务支持部分完成 - `begin_transaction()`尚未实现，但`commit()`和`rollback()`功能正常
2. 某些高级GQL功能可能未完全支持
3. 性能优化正在进行中

我们暴露来自底层Rust实现的实际错误而不是隐藏它们，这有助于开发者准确了解哪些功能已实现，哪些仍在开发中。

# MiniGU Python API

该软件包为 MiniGU 图数据库提供 Python 绑定，允许您使用 Python 与图数据进行交互。

## 功能特性

- 连接到 MiniGU 数据库
- 创建和管理图
- 执行 GQL 查询
- 事务支持（开始、提交、回滚）
- 同步和异步 API

## 安装

要安装该软件包，请确保您已构建 Rust 库，然后使用：

```bash
pip install .
```

或者用于开发：

```bash
maturin develop
```

## 使用方法

### 同步 API

```
import minigu

# 连接到数据库
db = minigu.MiniGU()

# 创建图
db.create_graph("my_graph")

# 开始事务
db.begin_transaction()

# 执行查询
db.execute("CREATE (:Person {name: 'Alice', age: 30})")
db.execute("CREATE (:Person {name: 'Bob', age: 25})")

# 提交事务
db.commit()

# 查询数据
result = db.execute("MATCH (p:Person) RETURN p.name, p.age")
print(result.data)

# 回滚事务
db.begin_transaction()
# ... 执行操作 ...
db.rollback()
```

### 异步 API

```
import asyncio
import minigu

async def main():
    # 连接到数据库
    db = minigu.AsyncMiniGU()
    
    # 创建图
    await db.create_graph("my_graph")
    
    # 开始事务
    await db.begin_transaction()
    
    # 执行查询
    await db.execute("CREATE (:Person {name: 'Alice', age: 30})")
    await db.execute("CREATE (:Person {name: 'Bob', age: 25})")
    
    # 提交事务
    await db.commit()
    
    # 查询数据
    result = await db.execute("MATCH (p:Person) RETURN p.name, p.age")
    print(result.data)
    
    # 回滚事务
    await db.begin_transaction()
    # ... 执行操作 ...
    await db.rollback()

asyncio.run(main())
```

## API 参考

### MiniGU 类 (同步)

#### `__init__(self)`
初始化 MiniGU 客户端。

#### `connect(self) -> None`
连接到数据库。

#### `create_graph(self, name: str) -> None`
创建具有给定名称的新图。

#### `begin_transaction(self) -> None`
开始新事务。

#### `commit(self) -> None`
提交当前事务。

#### `rollback(self) -> None`
回滚当前事务。

#### `execute(self, query: str) -> QueryResult`
执行 GQL 查询并返回结果。

### AsyncMiniGU 类 (异步)

所有方法都是同步 API 的异步对应方法。

## 错误处理

API 为不同的错误情况引发特定的异常：

- `MiniGUError`：MiniGU 相关错误的基异常
- `ConnectionError`：数据库连接失败时引发
- `TransactionError`：事务操作失败时引发
- `QueryExecutionError`：查询执行失败时引发

## 开发

要开发和测试 Python 绑定：

1. 确保您已构建 Rust 库：
   ```bash
   cargo build
   ```

2. 以开发模式安装 Python 包：
   ```bash
   maturin develop
   ```

3. 运行测试：
   ```bash
   python -m pytest
   ```

## 许可证

MIT
