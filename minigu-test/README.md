# MiniGU SQLLogicTest 集成

本项目实现了对 MiniGU 数据库的 SQLLogicTest 集成，提供了一个标准化的测试框架来验证 GQL 查询的正确性。

## 背景

之前 MiniGU 使用 `insta` 框架进行端到端测试，但存在以下问题：
- 快照编写不便，需要手动编写或生成预期结果
- 测试用例维护困难，格式变化时需要重新生成大量快照
- 缺乏标准化，难以与其他数据库系统对比

SQLLogicTest 是一个专为 SQL 数据库设计的测试框架，具有以下优势：
- 统一的测试格式 (`.slt` 文件)
- 丰富的扩展功能（错误测试、重试机制等）
- 良好的生态支持（被 DataFusion、RisingWave 等项目采用）
- GQL 兼容性（GQL 查询同样返回表格形式的结果）

## 项目结构

```
minigu-test/
├── src/
│   ├── lib.rs                    # 库入口
│   └── sqllogictest_adapter.rs   # SQLLogicTest 适配器
├── tests/
│   ├── e2e_test.rs               # 现有 insta 测试
│   └── sqllogictest_runner.rs    # SQLLogicTest 运行器
├── slt/                          # SQLLogicTest 测试文件
│   └── basic/
│       ├── echo.slt              # Echo 功能测试
│       ├── show_procedures.slt   # 过程展示测试
│       ├── create_graph.slt      # 图创建测试
│       └── comprehensive.slt     # 综合测试
└── README.md                     # 本文档
```

## 核心组件

### 1. MiniGuDB 适配器

`MiniGuDB` 实现了 `sqllogictest::DB` trait，作为 MiniGU 数据库和 SQLLogicTest 框架之间的桥梁：

```rust
pub struct MiniGuDB {
    database: Arc<Database>,
}

impl DB for MiniGuDB {
    type Error = SqlLogicTestError;
    type ColumnType = DefaultColumnType;
    
    async fn run(&mut self, sql: &str) -> Result<DBOutput<Self::ColumnType>, Self::Error>;
}
```

### 2. 错误处理

`SqlLogicTestError` 包装了 MiniGU 的错误类型，提供与 SQLLogicTest 兼容的错误处理：

```rust
#[derive(Debug)]
pub struct SqlLogicTestError(MiniGuError);
```

### 3. 结果转换

适配器负责将 MiniGU 的 `QueryResult` 转换为 SQLLogicTest 的 `DBOutput`：
- 支持有结果集的查询（`DBOutput::Rows`）
- 支持无结果集的语句（`DBOutput::StatementComplete`）
- 自动转换各种 MiniGU 数据类型为字符串表示

## 使用方法

### 运行测试

```bash
# 运行所有 SQLLogicTest 测试
cargo test --test sqllogictest_runner

# 运行特定测试
cargo test --test sqllogictest_runner test_basic_echo
```

### 编写测试文件

SQLLogicTest 文件使用 `.slt` 扩展名，支持以下语法：

#### 1. 语句测试 (statement)
```sql
# 成功的语句
statement ok
CALL echo('Hello World')

# 预期失败的语句
statement error
CALL create_test_graph('duplicate_name')
```

#### 2. 查询测试 (query)
```sql
# 单列文本查询
query T
CALL echo('Hello')
----
Hello

# 多列查询
query TT
CALL show_procedures()
----
create_test_graph string
echo string
show_procedures (empty)
```

#### 3. 注释
```sql
# 这是注释
# 可以用来描述测试的目的和预期行为
```

### 测试文件示例

#### 基本 Echo 测试 (`slt/basic/echo.slt`)
```sql
# 基本的 echo 功能测试
statement ok
CALL echo('Hello World')

query T
CALL echo('Test String')
----
Test String
```

#### 图创建测试 (`slt/basic/create_graph.slt`)
```sql
# 测试图创建功能
statement ok
CALL create_test_graph('test_graph')

# 重复创建应该失败
statement error
CALL create_test_graph('test_graph')
```

## 支持的功能

### 当前支持的 MiniGU 功能
- ✅ `echo` 过程调用
- ✅ `show_procedures` 过程调用
- ✅ `create_test_graph` 过程调用
- ✅ 基本错误处理
- ✅ 多种数据类型转换

### 计划支持的功能
- 🔄 完整的 GQL 查询支持
- 🔄 图数据操作（顶点、边的增删改查）
- 🔄 复杂查询模式匹配
- 🔄 事务支持
- 🔄 更多内置过程

## 开发指南

### 添加新的测试用例

1. 在 `slt/` 目录下创建新的 `.slt` 文件
2. 使用标准的 SQLLogicTest 语法编写测试
3. 在 `tests/sqllogictest_runner.rs` 中添加对应的测试函数

### 扩展适配器功能

1. 修改 `src/sqllogictest_adapter.rs` 中的 `run` 方法
2. 添加新的数据类型转换逻辑
3. 更新错误处理机制

### 调试测试

```bash
# 启用详细日志
RUST_LOG=debug cargo test --test sqllogictest_runner

# 运行单个测试并显示输出
cargo test --test sqllogictest_runner test_basic_echo -- --nocapture
```

## 与现有测试的对比

| 特性 | Insta 测试 | SQLLogicTest |
|------|------------|--------------|
| 测试编写 | 需要手动生成快照 | 标准化 .slt 格式 |
| 结果验证 | 快照比较 | 直接结果比较 |
| 错误测试 | 有限支持 | 内置错误测试 |
| 可读性 | 中等 | 高 |
| 维护成本 | 高 | 低 |
| 标准化 | 无 | 行业标准 |

## 贡献

欢迎为 MiniGU SQLLogicTest 集成贡献代码！请遵循以下步骤：

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 许可证

本项目遵循与 MiniGU 主项目相同的许可证。

## 参考资料

- [SQLLogicTest 官方文档](https://www.sqlite.org/sqllogictest/doc/trunk/about.wiki)
- [sqllogictest-rs 项目](https://github.com/risinglightdb/sqllogictest-rs)
- [DataFusion SQLLogicTest 实现](https://github.com/apache/datafusion/tree/main/datafusion/sqllogictest)
- [MiniGU 主项目](https://github.com/minigu/minigu)