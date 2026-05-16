# miniGU 架构设计文档

## 目录

- [概述](#概述)
- [系统架构](#系统架构)
- [核心模块](#核心模块)
- [存储引擎](#存储引擎)
- [查询引擎](#查询引擎)
- [事务管理](#事务管理)
- [向量索引](#向量索引)
- [数据流与执行流程](#数据流与执行流程)
- [扩展指南](#扩展指南)

---

## 概述

miniGU 是一个嵌入式图数据库，采用 Rust 语言实现，支持 GQL (Graph Query Language) 查询语言。系统设计遵循分层架构原则，各模块职责清晰，便于学习和扩展。

### 设计目标

1. **教育性**: 代码结构清晰，适合学习图数据库核心概念
2. **模块化**: 各组件独立，可单独理解和测试
3. **现代性**: 采用 Rust 2024 Edition，利用现代语言特性保证安全性和性能
4. **标准兼容**: 实现 GQL 标准语法

### 技术栈

| 组件 | 技术选型 |
|------|----------|
| 语言 | Rust 2024 Edition |
| 词法分析 | Logos |
| 语法分析 | Winnow (Parser Combinator) |
| 并发数据结构 | DashMap, SkipSet |
| 列式内存 | Arrow |
| 并行计算 | Rayon |
| 向量索引 | DiskANN |
| 测试框架 | SQLLogicTest, Insta |

---

## 系统架构

### 分层架构

```
┌─────────────────────────────────────────────────────────────────┐
│                        CLI Layer (minigu-cli)                    │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │   Shell     │  │  Executor   │  │   Output Formatter      │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Core API Layer (minigu/core)                │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │  Database   │  │   Session   │  │     Procedures          │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                                │
        ┌───────────────────────┼───────────────────────┐
        │                       │                       │
        ▼                       ▼                       ▼
┌───────────────┐    ┌───────────────────┐    ┌─────────────────┐
│    Catalog    │    │      Context      │    │  Transaction    │
│   (元数据)     │    │     (上下文)       │    │    (事务)       │
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

### 模块依赖关系

```
minigu-cli
    │
    └──► minigu (core)
              │
              ├──► catalog
              ├──► context
              ├──► transaction
              │
              ├──► common
              ├──► storage
              │      │
              │      ├──► tp (OLTP)
              │      ├──► ap (OLAP)
              │      └──► diskann-rs (向量索引)
              │
              └──► gql
                     │
                     ├──► parser
                     ├──► planner
                     └──► execution
```

---

## 核心模块

### 项目结构

```
miniGU/
├── minigu/                    # 核心库
│   ├── core/                  # 核心 API
│   ├── common/                # 公共数据类型
│   ├── catalog/               # 元数据管理
│   ├── context/               # 上下文管理
│   ├── transaction/           # 事务管理
│   ├── storage/               # 存储引擎
│   │   ├── src/
│   │   │   ├── tp/           # OLTP 存储
│   │   │   ├── ap/           # OLAP 存储
│   │   │   ├── common/       # 公共组件
│   │   │   └── db_file/      # 数据库文件
│   │   └── diskann-rs/       # 向量索引
│   └── gql/                   # 查询语言
│       ├── parser/           # 解析器
│       ├── planner/          # 规划器
│       └── execution/        # 执行器
│
├── minigu-cli/               # 命令行工具
├── minigu-test/              # 测试框架
└── docs/                     # 文档
```

### 核心模块职责

| 模块 | 职责 |
|------|------|
| `core` | 数据库和会话管理，内置存储过程 |
| `common` | 值类型、数据块、结果集等基础数据结构 |
| `catalog` | Schema、图、标签等元数据管理 |
| `context` | 会话上下文、数据库上下文、图上下文 |
| `transaction` | 事务定义、时间戳管理、隔离级别 |
| `storage` | 数据持久化、内存图、向量索引 |
| `gql/parser` | GQL 词法和语法分析 |
| `gql/planner` | 查询绑定、逻辑计划、物理计划 |
| `gql/execution` | 执行器构建、表达式求值 |

---

## 存储引擎

存储引擎位于 `minigu/storage/`，采用 TP/AP 分离架构。

### 存储架构

```
┌─────────────────────────────────────────────────────────────────┐
│                        Storage Layer                             │
├─────────────────────────┬───────────────────────────────────────┤
│      TP Storage         │              AP Storage               │
│   (Transaction Proc.)   │          (Analytical Proc.)           │
├─────────────────────────┼───────────────────────────────────────┤
│  ┌───────────────────┐  │  ┌─────────────────────────────────┐  │
│  │   MemoryGraph     │  │  │        OlapStorage              │  │
│  │  ┌─────────────┐  │  │  │  ┌───────────────────────────┐  │  │
│  │  │  Vertices   │  │  │  │  │    Dense Vertex Array     │  │  │
│  │  │  (DashMap)  │  │  │  │  └───────────────────────────┘  │  │
│  │  ├─────────────┤  │  │  │  ┌───────────────────────────┐  │  │
│  │  │   Edges     │  │  │  │  │    Edge Blocks (CSR)      │  │  │
│  │  │  (DashMap)  │  │  │  │  └───────────────────────────┘  │  │
│  │  ├─────────────┤  │  │  │  ┌───────────────────────────┐  │  │
│  │  │ Adjacency   │  │  │  │  │  Property Columns (Arrow) │  │  │
│  │  │  (SkipSet)  │  │  │  │  └───────────────────────────┘  │  │
│  │  └─────────────┘  │  │  └─────────────────────────────────┘  │
│  └───────────────────┘  │                                       │
├─────────────────────────┴───────────────────────────────────────┤
│                        Persistence Layer                         │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                      DbFileManager                           ││
│  │  ┌─────────────┐  ┌─────────────────┐  ┌─────────────────┐  ││
│  │  │   Header    │  │   Checkpoint    │  │       WAL       │  ││
│  │  │   (256B)    │  │    (Region)     │  │    (Region)     │  ││
│  │  └─────────────┘  └─────────────────┘  └─────────────────┘  ││
│  └─────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
```

### TP 存储 (OLTP)

TP 存储面向事务处理，位于 `storage/src/tp/`。

#### 核心数据结构

```rust
// 内存图结构
pub struct MemoryGraph {
    // 顶点存储：ID -> 版本化顶点
    pub(super) vertices: DashMap<VertexId, VersionedVertex>,

    // 边存储：ID -> 版本化边
    pub(super) edges: DashMap<EdgeId, VersionedEdge>,

    // 邻接表：顶点ID -> 邻接容器
    pub(super) adjacency_list: DashMap<VertexId, AdjacencyContainer>,

    // 向量索引
    pub(super) vector_indices: DashMap<VectorIndexKey, Arc<RwLock<Box<dyn VectorIndex>>>>,
}

// 邻接表容器
pub(super) struct AdjacencyContainer {
    pub(super) incoming: Arc<SkipSet<Neighbor>>,  // 入边
    pub(super) outgoing: Arc<SkipSet<Neighbor>>,  // 出边
}

// 版本化数据结构 (MVCC)
pub(super) struct VersionChain<D: Clone> {
    pub(super) current: RwLock<CurrentVersion<D>>,
    pub(super) undo_ptr: RwLock<UndoPtr>,  // 撤销链指针
}
```

#### 关键设计

1. **并发控制**: 使用 `DashMap` 实现高效的并发 HashMap，减少锁竞争
2. **邻接表**: 使用无锁跳表 `SkipSet` 存储邻接关系，支持高效遍历
3. **版本链**: MVCC 版本链支持快照读取

### AP 存储 (OLAP)

AP 存储面向分析查询，位于 `storage/src/ap/`。

#### 核心数据结构

```rust
pub struct OlapStorage {
    // ID 映射
    pub logic_id_counter: AtomicU64,
    pub dense_id_map: DashMap<VertexId, VertexId>,  // 稀疏ID -> 密集ID

    // 列式存储
    pub vertices: RwLock<Vec<OlapVertex>>,
    pub edges: RwLock<Vec<EdgeBlock>>,
    pub property_columns: RwLock<Vec<PropertyColumn>>,

    // 压缩存储
    pub is_edge_compressed: AtomicBool,
    pub compressed_edges: RwLock<Vec<CompressedEdgeBlock>>,
    pub is_property_compressed: AtomicBool,
    pub compressed_properties: RwLock<Vec<CompressedPropertyColumn>>,
}

// 边块 (CSR 格式)
pub const BLOCK_CAPACITY: usize = 256;
pub struct EdgeBlock {
    pub src_id: VertexId,
    pub edges: Vec<Edge>,
}
```

#### 压缩策略

```rust
// Delta 编码压缩
pub struct CompressedEdgeBlock {
    pub delta_bit_width: u8,                          // 增量位宽
    pub first_dst_id: VertexId,                       // 起始目标ID
    pub compressed_dst_ids: BitVec<u64, Lsb0>,       // 压缩的目标ID
    pub label_ids: [Option<LabelId>; BLOCK_CAPACITY],
}
```

### 持久化层

#### 数据库文件格式

```
+------------------+--------------------------+-----------------------+
|  Header (256B)   |  Checkpoint Region (Var) |    WAL Region (Var)   |
+------------------+--------------------------+-----------------------+
0                 256            header.wal_offset                   EOF
```

#### 文件头结构

```rust
pub struct DbFileHeader {
    pub magic: [u8; 8],           // "MINIGU\0\0"
    pub version: u32,             // 文件格式版本
    pub header_size: u32,         // 头大小 (256字节)
    pub flags: DbFileFlags,       // 特性标志位
    pub checkpoint_offset: u64,   // 检查点区域偏移
    pub checkpoint_length: u64,   // 检查点区域长度
    pub wal_offset: u64,          // WAL区域偏移
    pub wal_length: u64,          // WAL区域长度
    pub last_lsn: u64,            // 最后的日志序列号
    pub last_commit_ts: u64,      // 最后提交时间戳
    pub header_crc: u32,          // CRC32校验和
}
```

#### WAL (Write-Ahead Log)

```rust
pub struct RedoEntry {
    pub lsn: u64,                  // 日志序列号
    pub txn_id: Timestamp,         // 事务ID
    pub iso_level: IsolationLevel, // 隔离级别
    pub op: Operation,             // 操作类型
}

pub enum Operation {
    BeginTransaction(Timestamp),
    CommitTransaction(Timestamp),
    AbortTransaction,
    Delta(DeltaOp),  // 数据变更
}

pub enum DeltaOp {
    DelVertex(VertexId),
    DelEdge(EdgeId),
    CreateVertex(Vertex),
    CreateEdge(Edge),
    SetVertexProps(VertexId, SetPropsOp),
    SetEdgeProps(EdgeId, SetPropsOp),
    AddLabel(LabelId),
    RemoveLabel(LabelId),
}
```

#### 检查点机制

```rust
pub struct GraphCheckpoint {
    pub meta: CheckpointMetadata,
    pub vertices: HashMap<VertexId, SerializedVertex>,
    pub edges: HashMap<EdgeId, SerializedEdge>,
    pub adjacency_list: HashMap<VertexId, SerializedAdjacency>,
}

// 自动检查点触发
pub struct CheckpointConfig {
    pub wal_threshold: usize,  // WAL条目阈值，默认1000
}
```

---

## 查询引擎

查询引擎位于 `minigu/gql/`，采用经典的 Parser → Planner → Executor 架构。

### 查询处理流程

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   GQL Text  │───►│   Lexer     │───►│   Parser    │───►│    AST      │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
                                                              │
                                                              ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Result    │◄───│  Executor   │◄───│  Optimizer  │◄───│   Binder    │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
                         │                                     │
                         ▼                                     ▼
                   ┌─────────────┐                      ┌─────────────┐
                   │ Physical    │                      │ Logical     │
                   │ Plan        │                      │ Plan        │
                   └─────────────┘                      └─────────────┘
```

### 解析器 (Parser)

位于 `gql/parser/`，使用 Logos + Winnow 实现。

#### 词法分析 (Lexer)

```rust
// 使用 Logos 定义 Token
#[derive(Logos, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[logos(skip r"[ \t\r\n\f]+")]
#[logos(skip r"--[^\n]*")]
#[logos(skip r"//[^\n]*")]
pub enum TokenKind {
    // 关键字
    #[token("MATCH", ignore_case)]
    Match,
    #[token("RETURN", ignore_case)]
    Return,
    #[token("WHERE", ignore_case)]
    Where,
    // ...

    // 标识符和字面量
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", ignore_case)]
    RegularIdentifier,
    #[regex(r"'[^']*'")]
    CharacterStringLiteral,
    #[regex(r"[0-9]+")]
    UnsignedInteger,
    // ...
}
```

#### 语法分析 (Parser)

```rust
// 使用 Winnow 解析器组合子
pub fn parse_gql(gql: &str) -> Result<Spanned<Program>, Error> {
    let tokens = Lexer::new(gql).collect::<Result<Vec<_>, _>>()?;
    let input = LocatedSlice::new(&tokens);
    program.parse(input).map_err(Into::into)
}

// 解析 MATCH 语句示例
fn match_statement(input: &mut Input) -> PResult<Spanned<MatchStatement>> {
    let start = input.start();
    let _ = TokenKind::Match.parse_next(input)?;
    let pattern = graph_pattern.parse_next(input)?;
    let where_clause = opt(where_clause).parse_next(input)?;
    let end = input.end();
    Ok(Spanned::new(
        MatchStatement { pattern, where_clause },
        start..end,
    ))
}
```

#### AST 结构

```rust
// 程序入口
pub struct Program {
    pub activity: OptSpanned<ProgramActivity>,
    pub session_close: Option<Spanned<SessionCloseCommand>>,
}

// 图模式
pub struct GraphPattern {
    pub match_mode: Option<Spanned<MatchMode>>,
    pub element_bindings: Spanned<ElementBindings>,
    pub where_clause: Option<Spanned<WhereClause>>,
}

// 节点模式
pub struct NodePattern {
    pub variable: Option<Spanned<Ident>>,
    pub label_expression: Option<Spanned<LabelExpression>>,
    pub predicate: Option<Spanned<ElementPatternPredicate>>,
}

// 边模式
pub struct EdgePattern {
    pub direction: EdgeDirection,
    pub variable: Option<Spanned<Ident>>,
    pub label_expression: Option<Spanned<LabelExpression>>,
    pub predicate: Option<Spanned<ElementPatternPredicate>>,
    pub quantifier: Option<Spanned<GraphPatternQuantifier>>,
}
```

### 查询规划器 (Planner)

位于 `gql/planner/`，负责将 AST 转换为执行计划。

#### 绑定器 (Binder)

```rust
pub struct Binder<'a> {
    catalog: &'a dyn CatalogProvider,
    current_schema: Option<SchemaRef>,
    home_schema: Option<SchemaRef>,
    current_graph: Option<NamedGraphRef>,
    home_graph: Option<NamedGraphRef>,
    active_data_schema: Option<DataSchema>,
}

impl Binder<'_> {
    pub fn bind(&self, procedure: &Procedure) -> PlanResult<BoundStatement> {
        // 1. 名称解析
        // 2. 类型检查
        // 3. Schema 推导
        // 4. 标签 ID 解析
    }
}
```

#### 逻辑计划节点

```rust
pub enum PlanNode {
    // 扫描
    LogicalMatch(LogicalMatch),
    LogicalVertexPropertyFetch(LogicalVertexPropertyFetch),
    LogicalEdgePropertyFetch(LogicalEdgePropertyFetch),

    // 变换
    LogicalFilter(LogicalFilter),
    LogicalProject(LogicalProject),
    LogicalSort(LogicalSort),
    LogicalLimit(LogicalLimit),
    LogicalOffset(LogicalOffset),

    // 连接
    LogicalHashJoin(LogicalHashJoin),

    // 向量搜索
    LogicalVectorIndexScan(LogicalVectorIndexScan),

    // DDL
    LogicalCreateVectorIndex(LogicalCreateVectorIndex),
    LogicalDropVectorIndex(LogicalDropVectorIndex),

    // 其他
    LogicalOneRow(LogicalOneRow),
    LogicalExplain(LogicalExplain),
    LogicalCall(LogicalCall),
}
```

#### 物理计划节点

```rust
pub enum PhysicalNode {
    // 扫描
    PhysicalNodeScan(PhysicalNodeScan),
    PhysicalExpand(PhysicalExpand),
    PhysicalVertexPropertyFetch(PhysicalVertexPropertyFetch),

    // 变换
    PhysicalFilter(PhysicalFilter),
    PhysicalProject(PhysicalProject),
    PhysicalSort(PhysicalSort),
    PhysicalLimit(PhysicalLimit),
    PhysicalOffset(PhysicalOffset),

    // 连接
    PhysicalHashJoin(PhysicalHashJoin),

    // 向量搜索
    PhysicalVectorIndexScan(PhysicalVectorIndexScan),

    // 聚合
    PhysicalAggregate(PhysicalAggregate),
}
```

#### 优化规则

```rust
// 向量索引重写规则
pub struct VectorIndexScanRewrite;

impl OptimizerRule for VectorIndexScanRewrite {
    fn apply(&self, plan: &PlanNode) -> PlanResult<Option<PlanNode>> {
        // 检测模式: Sort(VECTOR_DISTANCE) + LIMIT APPROXIMATE
        // 重写为: HashJoin(VectorIndexScan, PropertyFetch)
        if let PlanNode::LogicalSort(sort) = plan {
            if let PlanNode::LogicalLimit(limit) = sort.child.as_ref() {
                if limit.is_approximate {
                    // 执行重写
                    return self.rewrite_to_vector_index_scan(plan);
                }
            }
        }
        Ok(None)
    }
}
```

### 查询执行器 (Executor)

位于 `gql/execution/`，采用 Volcano 模型实现。

#### 执行器接口

```rust
pub trait Executor: Debug + Send {
    fn next_chunk(&mut self) -> Option<ExecutionResult<DataChunk>>;
}

// 类型别名
pub type BoxedExecutor = Box<dyn Executor>;
```

#### 执行器构建

```rust
pub struct ExecutorBuilder {
    session: SessionContext,
}

impl ExecutorBuilder {
    pub fn build(self, plan: &PlanNode) -> BoxedExecutor {
        match plan {
            PlanNode::PhysicalNodeScan(scan) => self.build_node_scan(scan),
            PlanNode::PhysicalExpand(expand) => self.build_expand(expand),
            PlanNode::PhysicalFilter(filter) => self.build_filter(filter),
            PlanNode::PhysicalProject(project) => self.build_project(project),
            PlanNode::PhysicalSort(sort) => self.build_sort(sort),
            PlanNode::PhysicalLimit(limit) => self.build_limit(limit),
            PlanNode::PhysicalHashJoin(join) => self.build_hash_join(join),
            PlanNode::PhysicalVectorIndexScan(scan) => self.build_vector_scan(scan),
            PlanNode::PhysicalAggregate(agg) => self.build_aggregate(agg),
            // ...
        }
    }
}
```

#### 核心执行器

```rust
// 过滤执行器
pub struct FilterExecutor {
    child: BoxedExecutor,
    predicate: Box<dyn Evaluator>,
}

impl Executor for FilterExecutor {
    fn next_chunk(&mut self) -> Option<ExecutionResult<DataChunk>> {
        while let Some(result) = self.child.next_chunk() {
            let chunk = result?;
            let mask = self.predicate.evaluate(&chunk)?.as_bool_mask();
            if mask.any() {
                return Some(Ok(chunk.filter(&mask)));
            }
        }
        None
    }
}

// 投影执行器
pub struct ProjectExecutor {
    child: BoxedExecutor,
    expressions: Vec<Box<dyn Evaluator>>,
}

impl Executor for ProjectExecutor {
    fn next_chunk(&mut self) -> Option<ExecutionResult<DataChunk>> {
        self.child.next_chunk().map(|result| {
            let chunk = result?;
            let columns: Vec<ArrayRef> = self.expressions
                .iter()
                .map(|expr| expr.evaluate(&chunk)?.into_array())
                .collect();
            Ok(DataChunk::new(columns))
        })
    }
}

// 扩展执行器 (图遍历)
pub struct ExpandExecutor<S: ExpandSource> {
    child: BoxedExecutor,
    input_column_index: usize,
    edge_labels: Option<Vec<Vec<LabelId>>>,
    target_vertex_labels: Option<Vec<Vec<LabelId>>>,
    source: Arc<S>,
}

impl<S: ExpandSource> Executor for ExpandExecutor<S> {
    fn next_chunk(&mut self) -> Option<ExecutionResult<DataChunk>> {
        // 从子执行器获取顶点ID
        // 通过邻接表扩展边
        // 返回扩展结果
    }
}
```

#### 表达式求值

```rust
pub trait Evaluator: Debug + Send + Sync {
    fn evaluate(&self, chunk: &DataChunk) -> ExecutionResult<DatumRef>;
}

// 常量求值器
pub struct Constant {
    value: Datum,
}

// 列引用求值器
pub struct ColumnRef {
    index: usize,
}

// 二元运算求值器
pub struct Binary {
    left: Box<dyn Evaluator>,
    op: BinaryOp,
    right: Box<dyn Evaluator>,
}

// 向量距离求值器
pub struct VectorDistanceEvaluator {
    left: Box<dyn Evaluator>,
    right: Box<dyn Evaluator>,
    metric: VectorMetric,
}
```

---

## 事务管理

事务管理位于 `minigu/transaction/` 和 `storage/src/tp/`。

### MVCC 架构

```
┌─────────────────────────────────────────────────────────────────┐
│                      Transaction Manager                         │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  active_txns: SkipMap<Timestamp, Arc<Transaction>>        │  │
│  │  committed_txns: SkipMap<Timestamp, Arc<Transaction>>     │  │
│  │  commit_lock: Mutex<()>                                    │  │
│  │  latest_commit_ts: AtomicU64                               │  │
│  │  watermark: AtomicU64                                      │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                        Transaction                               │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  start_ts: Timestamp          // 开始时间戳                │  │
│  │  commit_ts: OnceLock<Timestamp> // 提交时间戳              │  │
│  │  isolation_level: IsolationLevel                          │  │
│  │  vertex_reads: DashSet<VertexId>  // 读集合                │  │
│  │  edge_reads: DashSet<EdgeId>      // 边读集合              │  │
│  │  undo_buffer: Vec<UndoEntry>      // 撤销日志              │  │
│  │  redo_buffer: Vec<RedoEntry>      // 重做日志              │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### 事务结构

```rust
pub struct MemTransaction {
    graph: Arc<MemoryGraph>,
    isolation_level: IsolationLevel,
    start_ts: Timestamp,
    commit_ts: OnceLock<Timestamp>,
    txn_id: Timestamp,

    // 读集合 (用于可串行化验证)
    vertex_reads: DashSet<VertexId>,
    edge_reads: DashSet<EdgeId>,

    // 日志缓冲
    undo_buffer: RwLock<Vec<Arc<UndoEntry>>>,
    redo_buffer: RwLock<Vec<RedoEntry>>,

    is_handled: Arc<AtomicBool>,
}
```

### 隔离级别

```rust
pub enum IsolationLevel {
    Snapshot,       // 快照隔离
    Serializable,   // 可串行化
}
```

### 提交协议

```rust
pub fn commit_at(&self, commit_ts: Option<Timestamp>, skip_wal: bool) -> StorageResult<Timestamp> {
    // 1. 获取提交时间戳
    let commit_ts = global_timestamp_generator().next()?;

    // 2. 获取全局提交锁
    let _guard = self.graph.txn_manager.commit_lock.lock().unwrap();

    // 3. 可串行化验证
    if let IsolationLevel::Serializable = self.isolation_level {
        self.validate_read_sets()?;
    }

    // 4. 设置提交时间戳
    self.commit_ts.set(commit_ts)?;

    // 5. 处理撤销缓冲区
    for undo_entry in undo_entries.iter() {
        // 更新版本链
    }

    // 6. 写入 WAL 并刷盘
    for entry in redo_entries {
        self.graph.persistence.append_wal(&entry)?;
    }
    self.graph.persistence.flush_wal()?;

    // 7. 更新最新提交时间戳
    self.graph.txn_manager.finish_transaction(self)?;

    // 8. 检查自动检查点
    self.graph.check_auto_checkpoint()?;

    Ok(commit_ts)
}
```

### 垃圾回收

```rust
fn garbage_collect(&self, graph: &MemoryGraph) -> Result<(), StorageError> {
    let min_read_ts = self.low_watermark().raw();

    // 1. 收集过期事务
    for entry in self.committed_txns.iter() {
        if entry.key().raw() > min_read_ts { break; }
        expired_txns.push(entry.value().clone());
    }

    // 2. 清理版本链中的过期版本
    self.cleanup_version_chains(graph, &expired_undo_entries)?;

    // 3. 移除过期事务记录
    for txn in expired_txns {
        self.committed_txns.remove(&txn.commit_ts()?);
    }
}
```

---

## 向量索引

向量索引基于 DiskANN 算法实现，位于 `storage/diskann-rs/`。

### 架构设计

```
┌─────────────────────────────────────────────────────────────────┐
│                     Vector Index Interface                       │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  build(vectors) -> ()                                      │  │
│  │  ann_search(query, k, l, filter) -> Vec<(id, distance)>   │  │
│  │  insert(vectors) -> ()                                     │  │
│  │  soft_delete(ids) -> ()                                    │  │
│  │  save(path) / load(path)                                   │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                      InMemANNAdapter                             │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  inner: Box<dyn ANNInmemIndex<f32>>  // DiskANN 核心       │  │
│  │  dimension: usize                                          │  │
│  │  node_to_vector: DashMap<u64, u32>   // 节点->向量映射     │  │
│  │  vector_to_node: ShardedVectorMap    // 向量->节点映射     │  │
│  │  next_vector_id: AtomicU32                                  │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### 核心接口

```rust
pub trait VectorIndex: Send + Sync {
    /// 构建索引
    fn build(&mut self, vectors: &[(u64, &[f32])]) -> StorageResult<()>;

    /// 近似最近邻搜索
    fn ann_search(
        &self,
        query: &[f32],
        k: usize,
        l_value: u32,
        filter_mask: Option<&dyn DiskANNFilterMask>,
        should_pre: bool,
    ) -> StorageResult<Vec<(u64, f32)>>;

    /// 带过滤的搜索
    fn search(
        &self,
        query: &[f32],
        k: usize,
        l_value: u32,
        filter_mask: Option<&FilterMask>,
        should_pre: bool,
    ) -> StorageResult<Vec<(u64, f32)>>;

    /// 插入向量
    fn insert(&mut self, vectors: &[(u64, &[f32])]) -> StorageResult<()>;

    /// 软删除
    fn soft_delete(&mut self, node_ids: &[u64]) -> StorageResult<()>;

    /// 持久化
    fn save(&mut self, path: &str) -> StorageResult<()>;
    fn load(&mut self, path: &str) -> StorageResult<()>;
}
```

### 分片映射优化

```rust
// 分片向量映射，减少锁竞争
pub struct ShardedVectorMap {
    shards: Vec<RwLock<Vec<Option<u64>>>>,  // 16个分片
    shard_bits: u32,
}

impl ShardedVectorMap {
    fn get_shard_and_index(&self, vector_id: u32) -> (usize, usize) {
        let shard_mask = (1u32 << self.shard_bits) - 1;
        let shard_idx = (vector_id & shard_mask) as usize;
        let local_idx = (vector_id >> self.shard_bits) as usize;
        (shard_idx, local_idx)
    }
}
```

### 智能搜索策略

```rust
pub const SELECTIVITY_THRESHOLD: f32 = 0.1;  // 10% 选择率阈值

fn search(&self, query: &[f32], k: usize, l_value: u32,
          filter_mask: Option<&FilterMask>, should_pre: bool) -> StorageResult<Vec<(u64, f32)>> {
    let selectivity = mask.selectivity();

    if selectivity < SELECTIVITY_THRESHOLD {
        // 低选择率：暴力搜索更高效
        self.brute_force_search(query, k, mask)
    } else {
        // 高选择率：使用索引搜索
        self.filter_search(query, k, l_value, mask, should_pre)
    }
}
```

### SIMD 优化

```rust
// 64字节对齐的向量数据访问
fn ensure_query_aligned(query: &[f32]) -> StorageResult<AlignedQueryBuffer<'_>> {
    if query.as_ptr().align_offset(64) == 0 {
        Ok(AlignedQueryBuffer::Borrowed(query))
    } else {
        let mut aligned = AlignedBoxWithSlice::<f32>::new(query.len(), 64)?;
        aligned.as_mut_slice().copy_from_slice(query);
        Ok(AlignedQueryBuffer::Owned(aligned))
    }
}
```

---

## 数据流与执行流程

### 查询执行流程

以一个典型的图查询为例：

```sql
MATCH (a:Person)-[e:KNOWS]->(b:Person)
WHERE a.age > 25
RETURN a.name, b.name
ORDER BY a.name
LIMIT 10
```

#### 1. 解析阶段

```
GQL Text
    │
    ▼
┌─────────────────────────────────────────────────────────────────┐
│ Lexer (Logos)                                                   │
│  "MATCH" → Token::Match                                         │
│  "(a:Person)" → Token::LParen, Token::Ident, Token::Colon, ...  │
└─────────────────────────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────────────────────────┐
│ Parser (Winnow)                                                 │
│  Tokens → AST                                                   │
│  MatchStatement {                                               │
│    pattern: GraphPattern {                                      │
│      element_bindings: [(a:Person)-[e:KNOWS]->(b:Person)],      │
│    },                                                           │
│    where_clause: Some(WhereClause { predicate: a.age > 25 }),   │
│  }                                                              │
└─────────────────────────────────────────────────────────────────┘
```

#### 2. 规划阶段

```
AST
    │
    ▼
┌─────────────────────────────────────────────────────────────────┐
│ Binder                                                          │
│  - 解析标签 Person → LabelId(1)                                 │
│  - 解析标签 KNOWS → LabelId(2)                                  │
│  - 类型检查 a.age > 25 (INT > INT → BOOL)                       │
│  - 生成 BoundStatement                                          │
└─────────────────────────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────────────────────────┐
│ LogicalPlanner                                                  │
│  LogicalMatch                                                   │
│  └── LogicalFilter (a.age > 25)                                 │
│      └── LogicalProject (a.name, b.name)                        │
│          └── LogicalSort (a.name ASC)                           │
│              └── LogicalLimit (10)                              │
└─────────────────────────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────────────────────────┐
│ Optimizer                                                        │
│  PhysicalNodeScan (Person)                                      │
│  └── PhysicalExpand (KNOWS, outgoing)                           │
│      └── PhysicalFilter (a.age > 25)                            │
│          └── PhysicalVertexPropertyFetch (a.name, b.name)       │
│              └── PhysicalProject                                │
│                  └── PhysicalSort                               │
│                      └── PhysicalLimit                          │
└─────────────────────────────────────────────────────────────────┘
```

#### 3. 执行阶段

```
PhysicalPlan
    │
    ▼
┌─────────────────────────────────────────────────────────────────┐
│ ExecutorBuilder                                                  │
│  构建 Executor 树                                               │
└─────────────────────────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────────────────────────┐
│ Execution (Volcano Model)                                       │
│                                                                 │
│  LimitExecutor.next_chunk()                                     │
│    └── SortExecutor.next_chunk()                                │
│          └── ProjectExecutor.next_chunk()                       │
│                └── PropertyFetchExecutor.next_chunk()           │
│                      └── FilterExecutor.next_chunk()            │
│                            └── ExpandExecutor.next_chunk()      │
│                                  └── NodeScanExecutor.next_chunk()│
│                                        │                        │
│                                        ▼                        │
│                                  MemoryGraph                    │
│                                  (读取顶点和边)                  │
└─────────────────────────────────────────────────────────────────┘
    │
    ▼
DataChunk (结果)
```

### 事务执行流程

```
START TRANSACTION
    │
    ▼
┌─────────────────────────────────────────────────────────────────┐
│ 生成开始时间戳 start_ts                                          │
│ 创建 Transaction 对象                                            │
│ 注册到 active_txns                                               │
└─────────────────────────────────────────────────────────────────┘
    │
    ▼
执行 SQL 语句
    │
    ├── 读操作: 记录到 vertex_reads/edge_reads
    │           读取 start_ts 时可见的版本
    │
    └── 写操作: 创建新版本
                记录到 undo_buffer
                记录到 redo_buffer
    │
    ▼
COMMIT / ROLLBACK
    │
    ├── COMMIT:
    │     ├── 获取 commit_ts
    │     ├── 获取 commit_lock
    │     ├── 可串行化验证 (如需要)
    │     ├── 设置版本链 commit_ts
    │     ├── 写入 WAL
    │     ├── 刷盘 WAL
    │     ├── 更新 latest_commit_ts
    │     ├── 移动到 committed_txns
    │     └── 检查自动检查点
    │
    └── ROLLBACK:
          ├── 遍历 undo_buffer
          ├── 撤销所有修改
          └── 从 active_txns 移除
```

---

## 扩展指南

### 添加新的查询语法

1. **定义 AST 节点** (`gql/parser/src/ast/`)
   ```rust
   // 在适当的文件中添加新的 AST 结构
   pub struct NewStatement {
       pub fields: Vec<Spanned<Field>>,
   }
   ```

2. **扩展 Lexer** (`gql/parser/src/lexer.rs`)
   ```rust
   #[derive(Logos)]
   pub enum TokenKind {
       // 添加新的关键字
       #[token("NEW_KEYWORD", ignore_case)]
       NewKeyword,
   }
   ```

3. **实现 Parser** (`gql/parser/src/parser/impls/`)
   ```rust
   fn new_statement(input: &mut Input) -> PResult<Spanned<NewStatement>> {
       // 解析逻辑
   }
   ```

4. **添加 Binder** (`gql/planner/src/binder/`)
   ```rust
   impl Binder<'_> {
       pub fn bind_new_statement(&self, stmt: &NewStatement) -> PlanResult<BoundNewStatement> {
           // 绑定逻辑
       }
   }
   ```

5. **创建计划节点** (`gql/planner/src/plan/`)
   ```rust
   pub struct LogicalNewStatement {
       pub bound: BoundNewStatement,
   }
   ```

6. **实现执行器** (`gql/execution/src/executor/`)
   ```rust
   pub struct NewStatementExecutor {
       // 执行器字段
   }

   impl Executor for NewStatementExecutor {
       fn next_chunk(&mut self) -> Option<ExecutionResult<DataChunk>> {
           // 执行逻辑
       }
   }
   ```

### 添加新的存储后端

1. **实现 Trait** (`storage/src/`)
   ```rust
   pub trait GraphStorage: Send + Sync {
       fn create_vertex(&self, txn: &Transaction, vertex: &Vertex) -> StorageResult<VertexId>;
       fn get_vertex(&self, txn: &Transaction, id: VertexId) -> StorageResult<Option<Vertex>>;
       // ... 其他方法
   }
   ```

2. **实现事务支持**
   ```rust
   pub trait TransactionManager: Send + Sync {
       fn begin_transaction(&self, isolation_level: IsolationLevel) -> StorageResult<Transaction>;
       fn commit(&self, txn: Transaction) -> StorageResult<Timestamp>;
       fn rollback(&self, txn: Transaction) -> StorageResult<()>;
   }
   ```

### 添加新的内置函数

1. **定义函数** (`gql/execution/src/evaluator/`)
   ```rust
   pub struct NewFunctionEvaluator {
       args: Vec<Box<dyn Evaluator>>,
   }

   impl Evaluator for NewFunctionEvaluator {
       fn evaluate(&self, chunk: &DataChunk) -> ExecutionResult<DatumRef> {
           // 计算逻辑
       }
   }
   ```

2. **注册函数** (`gql/planner/src/binder/`)
   ```rust
   impl Binder<'_> {
       pub fn bind_function_call(&self, name: &str, args: &[Expr]) -> PlanResult<Box<dyn Evaluator>> {
           match name.to_lowercase().as_str() {
               "new_function" => Ok(Box::new(NewFunctionEvaluator::new(bound_args))),
               // ... 其他函数
           }
       }
   }
   ```

### 添加新的优化规则

1. **定义规则** (`gql/planner/src/optimizer/`)
   ```rust
   pub struct NewOptimizationRule;

   impl OptimizerRule for NewOptimizationRule {
       fn apply(&self, plan: &PlanNode) -> PlanResult<Option<PlanNode>> {
           // 检测可优化的模式
           // 返回优化后的计划
       }
   }
   ```

2. **注册规则**
   ```rust
   impl Optimizer {
       pub fn new() -> Self {
           Self {
               rules: vec![
                   Box::new(VectorIndexScanRewrite),
                   Box::new(NewOptimizationRule),  // 添加新规则
               ],
           }
       }
   }
   ```

---

## 参考资料

- [GQL 标准](https://www.iso.org/standard/76120.html) - ISO/IEC 39075
- [Logos](https://github.com/maciejhirsz/logos) - 词法分析器生成器
- [Winnow](https://github.com/winnow-rs/winnow) - 解析器组合子库
- [Arrow](https://arrow.apache.org/) - 列式内存格式
- [DiskANN](https://github.com/microsoft/DiskANN) - 向量索引算法