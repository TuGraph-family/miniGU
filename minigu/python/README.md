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
pip install --force-reinstall ../../target/wheels/minigu-0.1.0-cp37-abi3-win_amd64.whl
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
    
    # 创建图
    success = await db.create_graph("my_graph")
    if success:
        print("Graph created successfully")
    
    # 执行查询
    result = await db.execute("MATCH (n) RETURN n LIMIT 10")
    print(result.data)
    
    # 使用异步上下文管理器（推荐）
    async with minigu.AsyncMiniGU() as db:
        await db.create_graph("my_graph")
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
    {"name": "Alice", "age": 30},
    {"name": "Bob", "age": 25}
]
success = db.load(data)

# 从文件加载数据
success = db.load("/path/to/data.csv")
```

#### 保存数据

```python
# 保存到指定路径
success = db.save("/path/to/save/location")
```

## 配置选项

### Cargo.toml配置

在[miniGU项目根目录]/minigu/python/Cargo.toml中配置了Python绑定的Rust设置：

```toml
[package]
edition.workspace = true
license.workspace = true
name = "minigu-python"
repository.workspace = true
version.workspace = true

[lib]
crate-type = ["cdylib"]
name = "minigu_python"

[dependencies]
arrow = { workspace = true }
minigu = { workspace = true }
pyo3 = { workspace = true, features = ["extension-module", "abi3-py37"] }

[build-dependencies]
pyo3-build-config = "0.24.2"

[features]
default = ["pyo3/extension-module"]
extension-module = ["pyo3/extension-module"]
```

关键配置说明：
- `crate-type = ["cdylib"]`：指定构建Python动态库
- `name = "minigu_python"`：编译模块的名称
- `pyo3`依赖项带有"extension-module"和"abi3-py37"特性，确保与Python 3.7及以上版本兼容

### pyproject.toml配置

在[miniGU项目根目录]/minigu/python/pyproject.toml中配置了Python包设置：

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

关键配置说明：
- `build-backend = "maturin"`：使用maturin作为构建后端 (参考官方文档: https://maturin.rs/)
- `requires-python = ">=3.7"`：要求Python 3.7或更高版本
- `features = ["pyo3/extension-module"]`：启用PyO3扩展模块特性
- `module-name = "minigu.minigu_python"`：指定模块名称

### build.rs构建脚本

构建脚本处理平台特定的链接选项，特别是macOS上的配置：

```rust
use std::env;

fn main() {
    // 使用PyO3的辅助函数为扩展模块设置正确的链接参数
    #[cfg(target_os = "macos")]
    pyo3_build_config::add_extension_module_link_args();

    // macOS特殊处理
    if env::var("CARGO_CFG_TARGET_OS").is_ok_and(|os| os == "macos") {
        // 添加macOS特定的链接器参数以避免问题
        println!("cargo:rustc-link-arg=-undefined");
        println!("cargo:rustc-link-arg=dynamic_lookup");
    }

    // 启用PyO3自动初始化功能
    println!("cargo:rustc-cfg=pyo3_auto_initialize");
}
```

关键链接选项：
- `cargo:rustc-link-arg=-undefined` 和 `cargo:rustc-link-arg=dynamic_lookup`：这些是macOS上构建Python扩展模块所必需的链接器参数
- `pyo3_auto_initialize`：启用PyO3自动初始化功能

## 封装的接口

### MiniGU类（同步接口）

`MiniGU`类提供了同步访问miniGU数据库的接口：

#### 构造函数
```python
def __init__(self, db_path: Optional[str] = None, 
             thread_count: int = 1,
             cache_size: int = 1000,
             enable_logging: bool = False)
```

参数：
- `db_path`：数据库文件路径，如果为None则创建内存数据库
- `thread_count`：并行执行的线程数
- `cache_size`：查询结果缓存大小
- `enable_logging`：是否启用查询执行日志

#### 核心方法

1. `execute(query: str) -> QueryResult`
   - 执行GQL查询
   - 返回查询结果对象

2. `create_graph(name: str, schema: Optional[Dict] = None) -> bool`
   - 创建新图
   - 返回布尔值表示操作是否成功

3. `load(data: Union[List[Dict], str, Path]) -> bool`
   - 加载数据到数据库
   - 可以是字典列表或文件路径
   - 返回布尔值表示操作是否成功

4. `save(path: str) -> bool`
   - 将数据库保存到指定路径
   - 返回布尔值表示操作是否成功

5. `close() -> None`
   - 关闭数据库连接

8. `close() -> None`
   - 关闭数据库连接

#### 属性和辅助方法

1. `connection_info`（属性）
   - 获取当前连接信息

2. `get_database_status() -> Dict[str, Any]`
   - 获取数据库状态信息

### AsyncMiniGU类（异步接口）

`AsyncMiniGU`类提供了异步访问miniGU数据库的接口，方法签名与`MiniGU`类相同，但所有方法都是异步的：

#### 核心异步方法

1. `async execute(query: str) -> QueryResult`
2. `async create_graph(name: str, schema: Optional[Dict] = None) -> bool`
3. `async load(data: Union[List[Dict], str, Path]) -> bool`
4. `async save(path: str) -> bool`
5. `async begin_transaction() -> None`
6. `async commit() -> None`
7. `async rollback() -> None`
8. `async close() -> None`

### 便捷函数

1. `connect(...) -> MiniGU`
   - 创建同步数据库连接的便捷函数

2. `async_connect(...) -> AsyncMiniGU`
   - 创建异步数据库连接的便捷函数

### Rust绑定接口

在Rust端，通过PyO3封装了以下核心功能：

1. `PyMiniGU`类：
   - `init()`：初始化数据库连接
   - `execute(query: str)`：执行查询
   - `create_graph(graph_name: str, _schema: Option<&str>)`：创建图
   - `load_data(data: Vec<Bound<PyDict>>)`：加载数据
   - `load_from_file(file_path: &str)`：从文件加载数据
   - `save_to_file(file_path: &str)`：保存数据到文件
   - `close()`：关闭连接
   - `begin_transaction()`：开始事务（未实现）
   - `commit()`：提交事务（未实现）
   - `rollback()`：回滚事务（未实现）

2. 错误处理函数：
   - `is_syntax_error(e: &Bound<PyAny>) -> bool`：检查是否为语法错误
   - `is_timeout_error(e: &Bound<PyAny>) -> bool`：检查是否为超时错误
   - `is_transaction_error(e: &Bound<PyAny>) -> bool`：检查是否为事务错误
   - `is_not_implemented_error(e: &Bound<PyAny>) -> bool`：检查是否为未实现功能错误

## 数据结构

### QueryResult

表示查询结果的对象，包含以下属性：
- `schema`：结果模式信息
- `data`：实际查询数据
- `metrics`：查询执行指标

## 异常处理

Python接口定义了以下异常类型：

1. `MiniGUError`：所有miniGU错误的基类
2. `ConnectionError`：连接失败时抛出
3. `QuerySyntaxError`：查询语法错误时抛出
4. `QueryExecutionError`：查询执行错误时抛出
5. `QueryTimeoutError`：查询超时错误时抛出
6. `GraphError`：图相关错误时抛出
7. `TransactionError`：事务相关错误时抛出
8. `DataError`：数据加载/保存错误时抛出

错误处理示例：

```python
import minigu

try:
    db = minigu.MiniGU()
    result = db.execute("INVALID QUERY")
except minigu.QuerySyntaxError as e:
    print(f"Query syntax error: {e}")
except minigu.MiniGUError as e:
    print(f"General miniGU error: {e}")
```

## 运行测试

运行测试的方法：
```
cd minigu/python
python test_minigu_api.py
```

测试会验证所有API方法。

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

1. 某些高级GQL功能可能未完全支持
2. 性能优化正在进行中

我们暴露来自底层Rust实现的实际错误而不是隐藏它们，这有助于开发者准确了解哪些功能已实现，哪些仍在开发中。