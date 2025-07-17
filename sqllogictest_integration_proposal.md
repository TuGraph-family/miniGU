# MiniGU SQLLogicTest 集成方案

## 1. 项目背景

### 1.1 当前测试框架的局限性

目前 MiniGU 项目使用 `insta` 框架进行端到端测试，但存在以下问题：

1. **快照编写不便**：对于 e2e 测试，需要手动编写或生成预期结果（快照），这在 `insta` 中较为不便
2. **测试用例维护困难**：当查询结果格式发生变化时，需要重新生成大量快照文件
3. **缺乏标准化**：没有统一的测试用例格式，难以与其他图数据库系统进行对比测试

### 1.2 SQLLogicTest 的优势

SQLLogicTest 是一个专门为 SQL 数据库设计的测试框架，具有以下优势：

1. **统一的测试格式**：提供标准化的 `.slt` 文件格式，便于编写和维护测试用例
2. **丰富的扩展功能**：支持错误测试、重试机制、环境变量替换等高级功能
3. **良好的生态支持**：已被 RisingWave、DataFusion、Databend 等多个知名数据库项目采用
4. **GQL 兼容性**：虽然原为 SQL 设计，但 GQL 查询同样返回表格形式的结果，完全适用

## 2. 技术方案设计

### 2.1 整体架构

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   .slt 测试文件  │───▶│  sqllogictest-rs │───▶│   miniGU 适配器  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                                       │
                                                       ▼
                                          ┌─────────────────┐
                                          │  miniGU Session │
                                          └─────────────────┘
                                                       │
                                                       ▼
                                          ┌─────────────────┐
                                          │    查询结果      │
                                          └─────────────────┘
```

### 2.2 核心组件设计

#### 2.2.1 MiniGU 数据库适配器

需要实现 `sqllogictest::DB` trait：

```rust
use sqllogictest::{DB, DBOutput, DefaultColumnType};
use minigu::Database;

pub struct MiniGuDB {
    database: Database,
}

impl DB for MiniGuDB {
    type Error = MiniGuError;
    type ColumnType = DefaultColumnType;
    
    fn run(&mut self, sql: &str) -> Result<DBOutput<Self::ColumnType>, Self::Error> {
        // 实现查询执行逻辑
    }
}
```

#### 2.2.2 错误类型定义

```rust
#[derive(Debug, thiserror::Error)]
pub enum MiniGuError {
    #[error("Database error: {0}")]
    Database(#[from] minigu::error::Error),
    
    #[error("Query execution error: {0}")]
    QueryExecution(String),
    
    #[error("Type conversion error: {0}")]
    TypeConversion(String),
}
```

#### 2.2.3 列类型映射

使用 `DefaultColumnType` 或自定义列类型：

```rust
pub enum MiniGuColumnType {
    Boolean,
    Integer,
    BigInt,
    Float,
    String,
    Null,
}

impl std::fmt::Display for MiniGuColumnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiniGuColumnType::Boolean => write!(f, "T"),
            MiniGuColumnType::Integer => write!(f, "I"),
            MiniGuColumnType::BigInt => write!(f, "I"),
            MiniGuColumnType::Float => write!(f, "R"),
            MiniGuColumnType::String => write!(f, "T"),
            MiniGuColumnType::Null => write!(f, "NULL"),
        }
    }
}
```

## 3. 详细实现方案

### 3.1 项目结构调整

```
minigu-test/
├── Cargo.toml                    # 添加 sqllogictest 依赖
├── src/
│   ├── lib.rs                   # 现有代码
│   ├── sqllogictest_adapter.rs  # 新增：sqllogictest 适配器
│   └── minigu_db.rs            # 新增：MiniGU 数据库封装
├── tests/
│   ├── e2e_test.rs             # 保留现有 insta 测试
│   └── sqllogictest_runner.rs  # 新增：sqllogictest 运行器
└── slt/                        # 新增：sqllogictest 测试文件目录
    ├── basic/
    │   ├── create_graph.slt
    │   ├── vertex_operations.slt
    │   └── edge_operations.slt
    ├── finbench/
    │   ├── tsr1.slt
    │   ├── tsr2.slt
    │   └── ...
    └── opengql/
        ├── match.slt
        ├── insert.slt
        └── ...
```

### 3.2 依赖配置

在 `minigu-test/Cargo.toml` 中添加：

```toml
[dependencies]
sqllogictest = "0.28"
async-trait = "0.1"
tokio = { version = "1.0", features = ["full"] }

[dev-dependencies]
tempfile = "3.0"
```

### 3.3 核心适配器实现

#### 3.3.1 数据库适配器 (`src/sqllogictest_adapter.rs`)

```rust
use std::sync::Arc;
use sqllogictest::{DB, DBOutput, DefaultColumnType};
use minigu::{Database, Session};
use minigu::error::Error as MiniGuError;
use minigu::result::QueryResult;

pub struct MiniGuDB {
    database: Arc<Database>,
}

impl MiniGuDB {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
    
    fn convert_query_result(&self, result: QueryResult) -> Result<DBOutput<DefaultColumnType>, MiniGuError> {
        // 检查是否有结果集
        if let Some(schema) = result.schema() {
            // 转换为 Records 格式
            let mut records = Vec::new();
            let mut types = Vec::new();
            
            // 提取列类型
            for field in schema.fields() {
                let col_type = match field.data_type() {
                    minigu_common::data_type::LogicalType::Boolean => DefaultColumnType::Boolean,
                    minigu_common::data_type::LogicalType::Int32 => DefaultColumnType::Integer,
                    minigu_common::data_type::LogicalType::Int64 => DefaultColumnType::BigInt,
                    minigu_common::data_type::LogicalType::Float32 => DefaultColumnType::Float,
                    minigu_common::data_type::LogicalType::Float64 => DefaultColumnType::Float,
                    minigu_common::data_type::LogicalType::String => DefaultColumnType::Text,
                    _ => DefaultColumnType::Text, // 默认为文本类型
                };
                types.push(col_type);
            }
            
            // 转换数据行
            for chunk in result.iter() {
                for row_index in 0..chunk.len() {
                    let mut row_values = Vec::new();
                    for (col_index, _) in schema.fields().iter().enumerate() {
                        let value = chunk.column(col_index).get_value(row_index);
                        let str_value = self.convert_value_to_string(value)?;
                        row_values.push(str_value);
                    }
                    records.push(row_values);
                }
            }
            
            Ok(DBOutput::Records {
                types,
                records,
            })
        } else {
            // 没有结果集，返回语句完成
            Ok(DBOutput::StatementComplete(0))
        }
    }
    
    fn convert_value_to_string(&self, value: &minigu_common::value::ScalarValue) -> Result<String, MiniGuError> {
        match value {
            minigu_common::value::ScalarValue::Boolean(Some(b)) => Ok(b.to_string()),
            minigu_common::value::ScalarValue::Int32(Some(i)) => Ok(i.to_string()),
            minigu_common::value::ScalarValue::Int64(Some(i)) => Ok(i.to_string()),
            minigu_common::value::ScalarValue::Float32(Some(f)) => Ok(f.to_string()),
            minigu_common::value::ScalarValue::Float64(Some(f)) => Ok(f.to_string()),
            minigu_common::value::ScalarValue::String(Some(s)) => Ok(s.clone()),
            _ => Ok("NULL".to_string()),
        }
    }
}

#[async_trait::async_trait]
impl DB for MiniGuDB {
    type Error = MiniGuError;
    type ColumnType = DefaultColumnType;
    
    async fn run(&mut self, sql: &str) -> Result<DBOutput<Self::ColumnType>, Self::Error> {
        // 创建新的会话
        let mut session = self.database.session()?;
        
        // 执行查询
        let result = session.query(sql)?;
        
        // 转换结果
        self.convert_query_result(result)
    }
}
```

#### 3.3.2 测试运行器 (`tests/sqllogictest_runner.rs`)

```rust
use std::path::Path;
use minigu::Database;
use minigu_test::sqllogictest_adapter::MiniGuDB;
use sqllogictest::Runner;

#[tokio::test]
async fn run_sqllogictest() {
    let config = minigu::DatabaseConfig::default();
    let database = Database::open_in_memory(&config).unwrap();
    
    let mut runner = Runner::new(|| async {
        Ok(MiniGuDB::new(database.clone()))
    });
    
    // 运行所有 .slt 文件
    runner.run_file_async("slt/basic/create_graph.slt").await.unwrap();
    runner.run_file_async("slt/basic/vertex_operations.slt").await.unwrap();
    runner.run_file_async("slt/basic/edge_operations.slt").await.unwrap();
}

#[tokio::test]
async fn run_finbench_tests() {
    let config = minigu::DatabaseConfig::default();
    let database = Database::open_in_memory(&config).unwrap();
    
    let mut runner = Runner::new(|| async {
        Ok(MiniGuDB::new(database.clone()))
    });
    
    // 运行 finbench 测试用例
    let test_files = [
        "slt/finbench/tsr1.slt",
        "slt/finbench/tsr2.slt",
        "slt/finbench/tsr3.slt",
        "slt/finbench/tsr4.slt",
        "slt/finbench/tsr5.slt",
        "slt/finbench/tsr6.slt",
    ];
    
    for test_file in &test_files {
        runner.run_file_async(test_file).await.unwrap();
    }
}
```

### 3.4 测试用例迁移

#### 3.4.1 基本操作测试 (`slt/basic/create_graph.slt`)

```sql
# 创建图类型和图的测试
statement ok
CREATE GRAPH TYPE test_graph_type (
    Person VERTEX (name VARCHAR, age INTEGER),
    KNOWS EDGE (Person -> Person) (since DATE)
);

statement ok
CREATE GRAPH test_graph OF TYPE test_graph_type;

# 验证图是否创建成功
query T
SHOW GRAPHS;
----
test_graph
```

#### 3.4.2 顶点操作测试 (`slt/basic/vertex_operations.slt`)

```sql
# 创建测试环境
statement ok
CREATE GRAPH TYPE person_graph (
    Person VERTEX (name VARCHAR, age INTEGER)
);

statement ok
CREATE GRAPH pg OF TYPE person_graph;

# 插入顶点数据
statement ok
INSERT INTO pg 
VERTEX (:Person {name: 'Alice', age: 30}),
VERTEX (:Person {name: 'Bob', age: 25});

# 查询顶点
query TI rowsort
MATCH (p:Person) RETURN p.name, p.age;
----
Alice 30
Bob 25
```

#### 3.4.3 边操作测试 (`slt/basic/edge_operations.slt`)

```sql
# 创建包含边的图类型
statement ok
CREATE GRAPH TYPE social_graph (
    Person VERTEX (name VARCHAR, age INTEGER),
    KNOWS EDGE (Person -> Person) (since INTEGER)
);

statement ok
CREATE GRAPH sg OF TYPE social_graph;

# 插入顶点和边
statement ok
INSERT INTO sg 
VERTEX (:Person {name: 'Alice', age: 30}),
VERTEX (:Person {name: 'Bob', age: 25}),
EDGE (:Person {name: 'Alice'}) -> (:Person {name: 'Bob'}) [:KNOWS {since: 2020}];

# 查询路径
query TTI rowsort
MATCH (a:Person)-[k:KNOWS]->(b:Person) 
RETURN a.name, b.name, k.since;
----
Alice Bob 2020
```

### 3.5 现有测试用例转换

#### 3.5.1 自动转换工具

可以创建一个工具来自动转换现有的 GQL 文件为 .slt 格式：

```rust
// tools/convert_to_slt.rs
use std::fs;
use std::path::Path;

pub fn convert_gql_to_slt(gql_file: &Path, output_file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(gql_file)?;
    let mut slt_content = String::new();
    
    // 简单的转换逻辑
    slt_content.push_str("# Converted from ");
    slt_content.push_str(gql_file.to_str().unwrap());
    slt_content.push_str("\n\n");
    
    // 假设 GQL 文件包含多个查询，每个查询一行
    for line in content.lines() {
        if line.trim().is_empty() || line.trim().starts_with('#') {
            continue;
        }
        
        // 判断是否是查询或语句
        if line.trim().to_uppercase().starts_with("SELECT") 
            || line.trim().to_uppercase().starts_with("MATCH") {
            slt_content.push_str("query T\n");
        } else {
            slt_content.push_str("statement ok\n");
        }
        
        slt_content.push_str(line);
        slt_content.push_str("\n");
        
        // 对于查询，需要添加预期结果
        if line.trim().to_uppercase().starts_with("SELECT") 
            || line.trim().to_uppercase().starts_with("MATCH") {
            slt_content.push_str("----\n");
            slt_content.push_str("# TODO: Add expected results\n\n");
        } else {
            slt_content.push_str("\n");
        }
    }
    
    fs::write(output_file, slt_content)?;
    Ok(())
}
```

### 3.6 CI/CD 集成

#### 3.6.1 GitHub Actions 配置

在 `.github/workflows/ci.yml` 中添加：

```yaml
  sqllogictest:
    name: SQLLogicTest
    runs-on: ubuntu-latest
    timeout-minutes: 30
    steps:
    - uses: actions/checkout@v4
    - uses: actions-rust-lang/setup-rust-toolchain@v1
    - uses: Swatinem/rust-cache@v2
    - name: Run SQLLogicTest
      run: |
        cd minigu-test
        cargo test --test sqllogictest_runner -- --nocapture
```

#### 3.6.2 测试脚本

```bash
#!/usr/bin/env bash
# scripts/run_sqllogictest.sh

set -euo pipefail

# 构建项目
cargo build --release

# 运行 sqllogictest
cd minigu-test
cargo test --test sqllogictest_runner

# 生成测试报告
cargo test --test sqllogictest_runner -- --format=json > test_results.json

echo "SQLLogicTest completed successfully"
```

## 4. 实施计划

### 4.1 第一阶段（1-2周）

1. **环境准备**
   - 添加 `sqllogictest` 依赖
   - 创建基础项目结构

2. **核心适配器实现**
   - 实现 `MiniGuDB` 适配器
   - 实现 `DB` trait
   - 基本的类型转换逻辑

3. **基础测试用例**
   - 创建几个简单的 `.slt` 测试文件
   - 验证基本的查询和语句执行

### 4.2 第二阶段（2-3周）

1. **现有测试用例迁移**
   - 将现有的 `resources/gql` 下的测试用例转换为 `.slt` 格式
   - 添加预期结果和错误测试

2. **高级功能实现**
   - 错误处理和测试
   - 环境变量支持
   - 重试机制

3. **性能优化**
   - 批量测试执行
   - 并行测试支持

### 4.3 第三阶段（1-2周）

1. **CI/CD 集成**
   - 更新 GitHub Actions 配置
   - 添加测试报告生成

2. **文档和工具**
   - 编写使用文档
   - 创建测试用例编写指南
   - 开发转换工具

3. **测试和优化**
   - 全面测试新的测试框架
   - 性能调优
   - 错误处理完善

## 5. 技术风险和解决方案

### 5.1 主要风险

1. **类型转换复杂性**
   - 风险：GQL 的复杂数据类型（如图结构）转换为表格形式可能存在困难
   - 解决方案：设计灵活的类型转换系统，支持自定义序列化格式

2. **异步执行兼容性**
   - 风险：sqllogictest 的异步支持可能与 MiniGU 的执行模型不匹配
   - 解决方案：使用 `async-trait` 和 `tokio` 进行异步适配

3. **测试用例维护**
   - 风险：大量测试用例的维护和更新工作量大
   - 解决方案：开发自动化工具，支持批量更新和验证

### 5.2 性能考虑

1. **测试执行速度**
   - 每个测试都需要创建新的数据库实例
   - 解决方案：使用内存数据库和连接池优化

2. **并发测试**
   - 大量测试用例的并发执行
   - 解决方案：实现智能的测试分片和并行执行

## 6. 预期收益

### 6.1 开发效率提升

1. **标准化测试格式**：统一的 `.slt` 格式便于编写和维护
2. **自动化测试生成**：减少手工编写测试用例的工作量
3. **更好的错误检测**：丰富的错误测试能力

### 6.2 测试质量提升

1. **更全面的测试覆盖**：支持更多测试场景和边界情况
2. **更可靠的回归测试**：标准化的测试格式和执行方式
3. **更好的可维护性**：清晰的测试结构和文档

### 6.3 项目生态改善

1. **与社区标准对齐**：采用广泛使用的测试框架
2. **更好的对比测试**：便于与其他数据库系统进行对比
3. **更容易的贡献**：降低外部贡献者的参与门槛

## 7. 总结

本方案详细描述了如何为 MiniGU 项目集成 SQLLogicTest 测试框架，通过实现 `DB` trait 和相关适配器，我们可以：

1. **无缝集成**：保持现有测试的同时，逐步迁移到新的测试框架
2. **提升效率**：标准化的测试格式和自动化工具大大提高开发效率
3. **增强可靠性**：更全面的测试覆盖和错误检测能力
4. **改善维护性**：清晰的项目结构和文档

该方案设计充分考虑了项目的实际需求，提供了详细的实施计划和风险控制措施，确保能够高效地解决 issue 中提出的需求，并为项目的长期发展奠定良好基础。 