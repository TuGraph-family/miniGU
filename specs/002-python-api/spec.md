# Feature Specification: Python API Bindings

**Feature Branch**: `002-python-api`
**Created**: 2026-04-17
**Status**: Draft
**Input**: User description: "Python API for miniGU graph database - provide Python bindings using PyO3 to allow Python developers to use miniGU for graph database operations including creating graphs, executing GQL queries, loading data, and managing database connections. The implementation should support both synchronous and asynchronous APIs."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Basic Graph Operations (Priority: P1)

Python 开发者需要能够使用 Python 代码创建图数据库、执行 GQL 查询并获取结果。这是最核心的功能，所有其他功能都建立在此基础之上。

**Why this priority**: 这是 Python API 的核心价值，没有这个功能，整个 API 就没有意义。用户使用图数据库的首要目的就是执行查询。

**Independent Test**: 可以通过创建一个简单的图、插入数据并执行 MATCH 查询来独立测试，验证查询结果是否正确返回。

**Acceptance Scenarios**:

1. **Given** 用户已安装 minigu Python 包, **When** 用户创建 MiniGU 实例并调用 `create_graph("test_graph")`, **Then** 图创建成功并返回 True
2. **Given** 用户已创建图, **When** 用户执行 `execute("MATCH (n) RETURN n LIMIT 10")`, **Then** 返回包含 schema 和 data 的结果对象
3. **Given** 用户执行查询, **When** 查询语法错误, **Then** 抛出 QuerySyntaxError 异常

---

### User Story 2 - 数据加载与持久化 (Priority: P2)

Python 开发者需要能够将数据加载到图数据库中，并将数据库状态保存到文件系统。

**Why this priority**: 数据操作是图数据库的核心功能之一，但可以在 P1 完成后独立开发和测试。

**Independent Test**: 可以通过加载字典列表数据或 CSV 文件，然后查询验证数据是否正确导入来独立测试。

**Acceptance Scenarios**:

1. **Given** 用户已创建图, **When** 用户调用 `load([{"name": "Alice", "age": 30}])`, **Then** 数据成功加载并返回 True
2. **Given** 用户已加载数据, **When** 用户调用 `save("/path/to/save")`, **Then** 数据库状态成功保存到指定路径
3. **Given** 用户调用 load 方法, **When** 传入无效数据格式, **Then** 抛出 DataError 异常

---

### User Story 3 - 异步 API 支持 (Priority: P3)

Python 开发者需要异步 API 以便在异步应用中高效使用 miniGU，避免阻塞事件循环。

**Why this priority**: 异步支持对于现代 Python 应用很重要，但同步 API 已经能满足基本需求，异步可以作为增强功能。

**Independent Test**: 可以通过在 asyncio 环境中创建 AsyncMiniGU 实例并执行异步查询来独立测试。

**Acceptance Scenarios**:

1. **Given** 用户在异步环境中, **When** 用户创建 AsyncMiniGU 实例并调用 `await create_graph("test")`, **Then** 图创建成功
2. **Given** 用户使用异步 API, **When** 用户使用 `async with` 上下文管理器, **Then** 资源正确管理
3. **Given** 用户并发执行多个异步查询, **When** 查询执行中, **Then** 不阻塞事件循环

---

### User Story 4 - 错误处理与异常 (Priority: P2)

Python 开发者需要清晰的 Python 风格异常来处理各种错误情况。

**Why this priority**: 良好的错误处理对于开发者体验至关重要，应该与核心功能一起提供。

**Independent Test**: 可以通过触发各种错误条件（语法错误、连接错误等）来验证正确的异常类型被抛出。

**Acceptance Scenarios**:

1. **Given** 用户执行查询, **When** GQL 语法错误, **Then** 抛出 QuerySyntaxError 异常
2. **Given** 用户连接数据库, **When** 数据库不可用, **Then** 抛出 ConnectionError 异常
3. **Given** 用户执行查询超时, **When** 超过设定时间, **Then** 抛出 QueryTimeoutError 异常

---

### Edge Cases

- 当用户尝试创建已存在的图时会发生什么？（应抛出 GraphError）
- 当用户在未初始化的情况下调用 execute 方法时会发生什么？（应抛出 ConnectionError）
- 当用户加载空数据列表时会发生什么？（应正常处理，不抛出异常）
- 当用户保存到没有写入权限的路径时会发生什么？（应抛出 DataError）
- 当用户传入 None 作为查询字符串时会发生什么？（应抛出 TypeError）

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Python API 必须提供 `MiniGU` 类作为同步 API 的入口点
- **FR-002**: Python API 必须提供 `AsyncMiniGU` 类作为异步 API 的入口点
- **FR-003**: 用户必须能够通过 `create_graph(name)` 方法创建图
- **FR-004**: 用户必须能够通过 `execute(query)` 方法执行 GQL 查询
- **FR-005**: 用户必须能够通过 `load(data)` 方法加载数据（支持字典列表和 CSV 文件路径）
- **FR-006**: 用户必须能够通过 `save(path)` 方法保存数据库状态
- **FR-007**: 查询结果必须包含 schema（字段名称和类型）和 data（实际数据行）
- **FR-008**: API 必须支持 Python 上下文管理器协议（`with` 语句）
- **FR-009**: 异步 API 必须支持异步上下文管理器协议（`async with` 语句）
- **FR-010**: API 必须提供以下异常类型：MiniGUError, ConnectionError, QuerySyntaxError, QueryExecutionError, QueryTimeoutError, GraphError, DataError, TransactionError

### Key Entities

- **MiniGU**: 同步 API 的主类，封装数据库连接和会话管理
- **AsyncMiniGU**: 异步 API 的主类，提供与 MiniGU 相同功能的异步版本
- **QueryResult**: 查询结果对象，包含 schema 和 data 属性
- **Schema**: 结果集结构信息，包含字段名称和类型列表
- **MiniGUError**: 基础异常类，所有 miniGU 特定异常的父类

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Python 开发者可以在 5 分钟内完成安装并执行第一条 GQL 查询
- **SC-002**: API 支持所有 miniGU 核心 GQL 功能（CREATE, MATCH, INSERT, DELETE 等）
- **SC-003**: 同步 API 和异步 API 的功能完全对等
- **SC-004**: 所有公开 API 都有 Python docstring 文档
- **SC-005**: 异常消息清晰易懂，帮助开发者快速定位问题
- **SC-006**: 支持 Python 3.7 到 3.12 版本

## Assumptions

- 用户已安装 Python 3.7 或更高版本
- 用户有基本的 GQL 查询语言知识
- 用户在本地运行 miniGU（暂不考虑远程连接场景）
- 使用 PyO3 作为 Rust 到 Python 的绑定框架
- 使用 maturin 作为 Python 包构建工具
- 暂不支持连接池和连接参数配置（使用默认配置）
- 暂不支持事务 API（使用自动提交模式）