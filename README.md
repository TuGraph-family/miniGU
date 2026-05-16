# miniGU 介绍

[![Star](https://shields.io/github/stars/tugraph-family/miniGU?logo=startrek&label=Star&color=yellow)](https://github.com/TuGraph-family/miniGU/stargazers)
[![UT&&IT](https://github.com/TuGraph-family/miniGU/actions/workflows/ci.yml/badge.svg)](https://github.com/TuGraph-family/miniGU/actions/workflows/ci.yml)

MiniGU 是 [TuGraph](https://tugraph.tech) 团队基联合多所高校共建专为零基础的同学设计的图数据库、图计算技术入门学习项目。 

MiniGU 是一个基于 Rust 语言实现的图数据库，旨在帮助学习者快速掌握图数据库和图计算的基本概念和技术。它提供了一个简单易用的交互式 shell 环境，支持基本的图数据操作和查询。

注意：MiniGU正在快速迭代中

# 文档

- [用户指南](docs/user-guide.md) - 安装、快速开始、GQL 语法参考
- [架构设计](docs/architecture.md) - 系统架构、核心模块、扩展指南
- [解析器开发指南](docs/parser/development.md) - GQL 解析器开发文档

## 快速上手

启动交互式 Shell：
```bash
cargo run -- shell    # 调试模式启动
cargo run -r -- shell # 发布模式启动（推荐）
```

执行脚本文件：
```bash
cargo run -- execute path/to/script.gql
```

### 基本示例

```sql
-- 创建图
CREATE GRAPH my_graph;
SET GRAPH my_graph;

-- 插入数据
INSERT (a:Person {name: 'Alice', age: 30}),
       (b:Person {name: 'Bob', age: 25});

-- 查询数据
MATCH (p:Person)
WHERE p.age > 25
RETURN p.name, p.age
ORDER BY p.age DESC;
```

## 系统架构

miniGU 采用分层架构设计：

```
┌─────────────────────────────────────────────────────────────────┐
│                        CLI Layer (minigu-cli)                    │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Core API Layer (minigu/core)                │
└─────────────────────────────────────────────────────────────────┘
                                │
        ┌───────────────────────┼───────────────────────┐
        │                       │                       │
        ▼                       ▼                       ▼
┌───────────────┐    ┌───────────────────┐    ┌─────────────────┐
│    Catalog    │    │      Context      │    │  Transaction    │
└───────────────┘    └───────────────────┘    └─────────────────┘
        │                       │                       │
        └───────────────────────┼───────────────────────┘
                                │
        ┌───────────────────────┼───────────────────────┐
        │                       │                       │
        ▼                       ▼                       ▼
┌───────────────┐    ┌───────────────────┐    ┌─────────────────┐
│    Common     │    │     Storage       │    │      GQL        │
│   (公共类型)   │    │    (存储引擎)      │    │   (查询引擎)    │
└───────────────┘    └───────────────────┘    └─────────────────┘
```

### 核心特性

- **GQL 查询语言**: 支持图模式匹配、过滤、聚合、排序等操作
- **事务支持**: 基于 MVCC 的事务管理，支持快照隔离和可串行化隔离级别
- **向量搜索**: 内置 DiskANN 向量索引，支持近似最近邻搜索
- **嵌入式设计**: 单文件存储格式，无需独立服务器进程
- **持久化存储**: 支持 WAL 和检查点机制

详细架构说明请参阅 [架构设计文档](docs/architecture.md)

# Contributing

TuGraph 社区热情欢迎每一位对图计算、数据库技术、Rust语言热爱的开发者，无论是doc修改和补充、bug fix还是new feature。

MiniGU 开放了一些[新功能的开发](https://github.com/tugraph-family/miniGU/issues?q=is%3Aopen+is%3Aissue+label%3A%22help+wanted%22)，欢迎有兴趣的同学一起共建。

如果你对MiniGU不熟悉也没关系，可以直接联系我们，将会有社区导师指导你上手！更多详情，请参考 [社区贡献](CONTRIBUTING.md)。

# Contributors

感谢对这个项目做过贡献的个人开发者，名单如下：

<a href="https://github.com/TuGraph-family/miniGU/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=TuGraph-family/miniGU" />
</a>

## 联系我们

官网: [tugraph.tech](https://tugraph.tech)

通过钉钉群、微信群、微信公众号、邮箱和电话联系我们:
![contacts](./docs/images/contact.jpeg)



