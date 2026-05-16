# miniGU 用户指南

## 目录

- [简介](#简介)
- [安装与构建](#安装与构建)
- [快速开始](#快速开始)
- [GQL 查询语言](#gql-查询语言)
- [数据类型](#数据类型)
- [内置函数](#内置函数)
- [向量搜索](#向量搜索)
- [存储过程](#存储过程)
- [配置与调优](#配置与调优)

---

## 简介

miniGU 是一个由 TuGraph 团队联合多所高校共建的嵌入式图数据库学习项目。它使用 Rust 语言实现，支持 GQL (Graph Query Language) 查询语言，提供交互式 Shell 环境。

### 主要特性

- **GQL 查询语言**: 支持图模式匹配、过滤、聚合、排序等操作
- **事务支持**: 基于 MVCC 的事务管理，支持快照隔离和可串行化隔离级别
- **向量搜索**: 内置 DiskANN 向量索引，支持近似最近邻搜索
- **嵌入式设计**: 单文件存储格式，无需独立服务器进程
- **持久化存储**: 支持 WAL (Write-Ahead Log) 和检查点机制

---

## 安装与构建

### 系统要求

- Rust 1.75+ (推荐使用最新稳定版)
- Cargo 包管理器

### 构建项目

```bash
# 克隆仓库
git clone https://github.com/TuGraph-family/miniGU.git
cd miniGU

# 调试模式构建
cargo build

# 发布模式构建 (推荐用于生产环境)
cargo build --release
```

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test -p minigu-storage

# 运行 GQL 测试
cargo test -p minigu-test
```

---

## 快速开始

### 启动交互式 Shell

```bash
# 调试模式启动
cargo run -- shell

# 发布模式启动 (性能更好)
cargo run -r -- shell
```

### Shell 命令

miniGU Shell 支持以下内置命令：

| 命令 | 说明 |
|------|------|
| `:help` | 显示帮助信息 |
| `:quit` / `:exit` | 退出 Shell |
| `:cd <path>` | 切换工作目录 |
| `:clear` | 清屏 |
| `:reset` | 重置会话 |
| `:set <key> <value>` | 设置会话参数 |

### 执行脚本文件

```bash
# 执行 GQL 脚本文件
cargo run -- execute path/to/script.gql
```

### 基本操作示例

```sql
-- 创建 Schema
CREATE SCHEMA my_schema;

-- 创建图
CREATE GRAPH my_graph;

-- 设置当前图
SET GRAPH my_graph;

-- 创建顶点标签
CREATE NODE TYPE Person {
  name STRING,
  age INT,
  email STRING
};

-- 创建边标签
CREATE EDGE TYPE KNOWS {
  since INT
};

-- 插入顶点
INSERT (a:Person {name: 'Alice', age: 30}),
       (b:Person {name: 'Bob', age: 25}),
       (c:Person {name: 'Charlie', age: 35});

-- 插入边
INSERT (a:Person {name: 'Alice'})-[e:KNOWS {since: 2020}]->(b:Person {name: 'Bob'});

-- 查询顶点
MATCH (p:Person)
RETURN p.name, p.age
ORDER BY p.age DESC
LIMIT 10;

-- 模式匹配查询
MATCH (a:Person)-[e:KNOWS]->(b:Person)
RETURN a.name, b.name, e.since;

-- 过滤查询
MATCH (p:Person)
WHERE p.age > 25
RETURN p.name, p.age;
```

---

## GQL 查询语言

### 数据定义语言 (DDL)

#### 创建 Schema

```sql
CREATE SCHEMA schema_name;
```

#### 删除 Schema

```sql
DROP SCHEMA schema_name;
```

#### 创建图

```sql
-- 创建空图
CREATE GRAPH graph_name;

-- 创建指定类型的图
CREATE GRAPH graph_name OF TYPE graph_type_name;
```

#### 删除图

```sql
DROP GRAPH graph_name;
```

#### ���建顶点类型

```sql
CREATE NODE TYPE LabelName {
  property1 Type1,
  property2 Type2,
  ...
};
```

#### 创建边类型

```sql
CREATE EDGE TYPE EdgeLabel {
  property1 Type1,
  property2 Type2,
  ...
} FROM SourceLabel TO TargetLabel;
```

#### 创建向量索引

```sql
CREATE VECTOR INDEX index_name
FOR (n:Label)
ON n.property_name
DIMENSION 128
METRIC L2;
```

#### 删除向量索引

```sql
DROP VECTOR INDEX index_name;
```

### 数据操作语言 (DML)

#### 插入顶点

```sql
-- 插入单个顶点
INSERT (a:Person {name: 'Alice', age: 30});

-- 插入多个顶点
INSERT (a:Person {name: 'Alice'}),
       (b:Person {name: 'Bob'}),
       (c:Person {name: 'Charlie'});
```

#### 插入边

```sql
-- 插入有向边
INSERT (a:Person {name: 'Alice'})-[e:KNOWS {since: 2020}]->(b:Person {name: 'Bob'});

-- 插入无向边
INSERT (a:Person)-[e:FRIEND {since: 2021}]-(b:Person);
```

#### 更新属性

```sql
MATCH (p:Person {name: 'Alice'})
SET p.age = 31;
```

#### 删除元素

```sql
-- 删除顶点
MATCH (p:Person {name: 'Alice'})
DELETE p;

-- 删除边
MATCH (a:Person)-[e:KNOWS]->(b:Person)
WHERE a.name = 'Alice' AND b.name = 'Bob'
DELETE e;
```

### 数据查询语言 (DQL)

#### MATCH 语句

MATCH 是图模式匹配的核心语句：

```sql
-- 简单顶点匹配
MATCH (p:Person)
RETURN p;

-- 边模式匹配
MATCH (a:Person)-[e:KNOWS]->(b:Person)
RETURN a, e, b;

-- 多跳路径匹配
MATCH (a:Person)-[e1:KNOWS]->(b:Person)-[e2:KNOWS]->(c:Person)
RETURN a.name, c.name;

-- 可变长度路径
MATCH (a:Person)-[e:KNOWS*1..3]->(b:Person)
RETURN a.name, b.name;
```

#### 边方向

```sql
-- 出边 (指向右侧)
MATCH (a)-[e]->(b)

-- 入边 (指向左侧)
MATCH (a)<-[e]-(b)

-- 无向边 (任意方向)
MATCH (a)-[e]-(b)

-- 双向边
MATCH (a)<-[e]->(b)
```

#### 标签表达式

```sql
-- 单标签
MATCH (p:Person)

-- 多标签 (或)
MATCH (p:Person|Animal)

-- 标签交集 (与)
MATCH (p:Person&Employee)

-- 标签取反
MATCH (p:!Bot)
```

#### WHERE 过滤

```sql
-- 比较过滤
MATCH (p:Person)
WHERE p.age > 25
RETURN p;

-- 逻辑组合
MATCH (p:Person)
WHERE p.age > 25 AND p.name STARTS WITH 'A'
RETURN p;

-- 存在性检查
MATCH (p:Person)
WHERE p.email IS NOT NULL
RETURN p;

-- 标签检查
MATCH (p)
WHERE p IS LABELED Person
RETURN p;
```

#### RETURN 投影

```sql
-- 返回属性
MATCH (p:Person)
RETURN p.name, p.age;

-- 使用别名
MATCH (p:Person)
RETURN p.name AS name, p.age AS age;

-- 表达式计算
MATCH (p:Person)
RETURN p.name, p.age * 2 AS double_age;

-- 聚合函数
MATCH (p:Person)
RETURN COUNT(p) AS person_count;

-- 去重
MATCH (p:Person)
RETURN DISTINCT p.age;
```

#### ORDER BY 排序

```sql
-- 升序排序
MATCH (p:Person)
RETURN p.name, p.age
ORDER BY p.age ASC;

-- 降序排序
MATCH (p:Person)
RETURN p.name, p.age
ORDER BY p.age DESC;

-- 多字段排序
MATCH (p:Person)
RETURN p.name, p.age
ORDER BY p.age DESC, p.name ASC;
```

#### LIMIT 和 OFFSET 分页

```sql
-- 限制结果数量
MATCH (p:Person)
RETURN p
LIMIT 10;

-- 分页查询
MATCH (p:Person)
RETURN p
ORDER BY p.name
LIMIT 10 OFFSET 20;
```

#### GROUP BY 分组

```sql
MATCH (p:Person)
RETURN p.age, COUNT(p) AS count
GROUP BY p.age;
```

---

## 数据类型

### 基本类型

| 类型 | 说明 | 示例 |
|------|------|------|
| `BOOL` | 布尔值 | `true`, `false` |
| `INT` | 64位整数 | `42`, `-100` |
| `FLOAT` | 64位浮点数 | `3.14`, `-0.5` |
| `STRING` | 字符串 | `'hello'`, `"world"` |
| `BYTES` | 字节串 | `x'48656c6c6f'` |

### 时间类型

| 类型 | 说明 | 示例 |
|------|------|------|
| `DATE` | 日期 | `DATE '2024-01-15'` |
| `TIME` | 时间 | `TIME '14:30:00'` |
| `DATETIME` | 日期时间 | `DATETIME '2024-01-15T14:30:00'` |
| `DURATION` | 时间间隔 | `DURATION 'P1Y2M3D'` |

### 复合类型

| 类型 | 说明 | 示例 |
|------|------|------|
| `LIST<T>` | 列表 | `[1, 2, 3]` |
| `RECORD` | 记录 | `{name: 'Alice', age: 30}` |
| `VECTOR` | 向量 | `VECTOR [1.0, 2.0, 3.0]` |

### 图元素类型

| 类型 | 说明 |
|------|------|
| `NODE` | 顶点引用 |
| `EDGE` | 边引用 |
| `PATH` | 路径 |

---

## 内置函数

### 聚合函数

| 函数 | 说明 |
|------|------|
| `COUNT(x)` | 计数 |
| `SUM(x)` | 求和 |
| `AVG(x)` | 平均值 |
| `MIN(x)` | 最小值 |
| `MAX(x)` | 最大值 |
| `COLLECT(x)` | 收集为列表 |

### 字符串函数

| 函数 | 说明 |
|------|------|
| `UPPER(s)` | 转大写 |
| `LOWER(s)` | 转小写 |
| `TRIM(s)` | 去除首尾空白 |
| `SUBSTRING(s, start, len)` | 子字符串 |
| `CONCAT(s1, s2, ...)` | 字符串连接 |
| `LENGTH(s)` | 字符串长度 |

### 数值函数

| 函数 | 说明 |
|------|------|
| `ABS(x)` | 绝对值 |
| `FLOOR(x)` | 向下取整 |
| `CEIL(x)` | 向上取整 |
| `ROUND(x)` | 四舍五入 |
| `SQRT(x)` | 平方根 |
| `POWER(x, y)` | 幂运算 |

### 图函数

| 函数 | 说明 |
|------|------|
| `ELEMENT_ID(e)` | 获取元素 ID |
| `LABELS(n)` | 获取顶点标签列表 |
| `PROPERTIES(e)` | 获取元素属性 |
| `START_NODE(e)` | 获取边的起始顶点 |
| `END_NODE(e)` | 获取边的目标顶点 |

---

## 向量搜索

miniGU 内置向量索引支持，可以进行高效的向量相似性搜索。

### 创建向量属性

```sql
-- 创建带向量属性的顶点类型
CREATE NODE TYPE Article {
  title STRING,
  content STRING,
  embedding VECTOR(128)
};

-- 插入带向量的顶点
INSERT (a:Article {
  title: 'Introduction to Graphs',
  content: '...',
  embedding: VECTOR [0.1, 0.2, 0.3, ...]
});
```

### 创建向量索引

```sql
CREATE VECTOR INDEX article_embedding_idx
FOR (a:Article)
ON a.embedding
DIMENSION 128
METRIC L2;
```

支持的距离度量：
- `L2`: 欧几里得距离
- `COSINE`: 余弦相似度
- `INNER_PRODUCT`: 内积

### 向量相似性搜索

```sql
-- 计算向量距离
MATCH (a:Article)
RETURN a.title, VECTOR_DISTANCE([0.1, 0.2, ...], a.embedding, L2) AS distance
ORDER BY distance
LIMIT 10;

-- 使用索引进行近似搜索
MATCH (a:Article)
RETURN a.title, VECTOR_DISTANCE([0.1, 0.2, ...], a.embedding, L2) AS distance
ORDER BY distance
LIMIT APPROXIMATE 10;
```

`LIMIT APPROXIMATE` 提示查询优化器使用向量索引进行近似最近邻搜索，可以显著提升查询性能。

---

## 存储过程

miniGU 提供内置存储过程用于数据库管理。

### 查看图信息

```sql
CALL show_graph()
YIELD name, vertex_count, edge_count
RETURN name, vertex_count, edge_count;
```

### 导入图数据

```sql
CALL import_graph('/path/to/data.json')
YIELD status
RETURN status;
```

### 导出图数据

```sql
CALL export_graph('/path/to/output.json')
YIELD status
RETURN status;
```

### 创建测试图

```sql
CALL create_test_graph()
YIELD status
RETURN status;
```

---

## 配置与调优

### 数据库文件

miniGU 使用单文件存储格式，默认数据文件为 `.minigu` 扩展名。

文件结构：
```
+----------------+--------------------------+-----------------------+
|  Header (256B) |  Checkpoint Region (Var) |    WAL Region (Var)   |
+----------------+--------------------------+-----------------------+
```

### 事务隔离级别

```sql
-- 设置隔离级别
SET TRANSACTION ISOLATION LEVEL SNAPSHOT;
SET TRANSACTION ISOLATION LEVEL SERIALIZABLE;

-- 开始事务
START TRANSACTION;

-- 提交事务
COMMIT;

-- 回滚事务
ROLLBACK;
```

### 性能建议

1. **批量插入**: 使用批量 INSERT 语句减少事务开销
2. **向量索引**: 对于向量搜索场景，创建适当的向量索引
3. **查询优化**: 使用 EXPLAIN 查看查询计划

```sql
EXPLAIN MATCH (p:Person)-[e:KNOWS]->(f:Person)
RETURN p.name, f.name;
```

---

## 常见问题

### Q: 如何重置数据库？

```sql
-- 删除并重建图
DROP GRAPH my_graph;
CREATE GRAPH my_graph;
```

### Q: 如何查看当前会话状态？

```sql
-- 显示当前图
CALL show_graph() RETURN *;
```

### Q: 支持哪些图算法？

当前版本专注于基础图查询功能，图算法支持正在开发中。

---

## 更多资源

- [架构设计文档](architecture.md) - 了解 miniGU 内部实现
- [贡献指南](../CONTRIBUTING.md) - 参与项目开发
- [问题反馈](https://github.com/TuGraph-family/miniGU/issues) - 报告问题或提出建议