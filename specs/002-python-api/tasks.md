# Tasks: Python API Bindings

**Input**: Design documents from `/specs/002-python-api/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/api.md

**Tests**: 包含测试任务，遵循项目 TDD 原则

**Organization**: 任务按用户故事组织，支持独立实现和测试

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 可并行执行（不同文件，无依赖）
- **[Story]**: 所属用户故事 (US1, US2, US3, US4)
- 描述中包含精确文件路径

## Path Conventions

- **Python crate**: `minigu/python/`
- **Workspace root**: `Cargo.toml`, `minigu/core/`

---

## Phase 1: Setup (项目初始化)

**Purpose**: 创建 Python binding crate 基础结构

- [x] T001 添加 pyo3 依赖到 workspace Cargo.toml
- [x] T002 创建 minigu/python/ 目录结构
- [x] T003 [P] 创建 minigu/python/Cargo.toml 配置 PyO3
- [x] T004 [P] 创建 minigu/python/pyproject.toml 配置 maturin
- [x] T005 [P] 创建 minigu/python/build.rs 构建脚本
- [x] T006 更新根 Cargo.toml 添加 minigu/python 到 workspace members

---

## Phase 2: Foundational (基础绑定层)

**Purpose**: 实现 Rust FFI 层，为所有用户故事提供基础

**⚠️ CRITICAL**: 所有用户故事依赖此阶段完成

- [x] T007 创建 minigu/python/src/lib.rs PyO3 模块入口
- [x] T008 实现 PyMiniGU 类基础结构 (init, database/session 管理)
- [x] T009 实现 Arrow 数据类型到 Python 类型的转换函数
- [x] T010 实现 QueryResult 到 Python dict 的序列化
- [x] T011 创建 minigu/python/__init__.py Python 包入口

**Checkpoint**: Rust FFI 层就绪，可开始用户故事实现

---

## Phase 3: User Story 1 - Basic Graph Operations (Priority: P1) 🎯 MVP

**Goal**: Python 开发者可以创建图、执行 GQL 查询并获取结果

**Independent Test**: 创建图、插入数据、执行 MATCH 查询，验证结果正确返回

### Tests for User Story 1

- [x] T012 [P] [US1] 创建 minigu/python/test_minigu_api.py 测试文件
- [x] T013 [P] [US1] 添加 test_create_graph 测试用例
- [x] T014 [P] [US1] 添加 test_execute_query 测试用例
- [x] T015 [P] [US1] 添加 test_context_manager 测试用例

### Implementation for User Story 1

- [x] T016 [US1] 在 lib.rs 实现 PyMiniGU.create_graph() 方法
- [x] T017 [US1] 在 lib.rs 实现 PyMiniGU.execute() 方法
- [x] T018 [US1] 创建 minigu/python/minigu.py 实现 MiniGU 同步类
- [x] T019 [US1] 实现 MiniGU.__enter__/__exit__ 上下文管理器
- [x] T020 [US1] 实现 QueryResult Python 封装类
- [x] T021 [US1] 运行测试验证 US1 功能

**Checkpoint**: US1 完成，可独立测试和演示

---

## Phase 4: User Story 2 - 数据加载与持久化 (Priority: P2)

**Goal**: Python 开发者可以加载数据到图数据库并保存数据库状态

**Independent Test**: 加载字典列表或 CSV 文件，查询验证数据导入正确

### Tests for User Story 2

- [x] T022 [P] [US2] 添加 test_load_dict_list 测试用例到 test_minigu_api.py
- [x] T023 [P] [US2] 添加 test_load_csv_file 测试用例
- [x] T024 [P] [US2] 添加 test_save_database 测试用例
- [x] T025 [P] [US2] 添加 test_load_invalid_data 测试边界情况

### Implementation for User Story 2

- [x] T026 [US2] 在 lib.rs 实现 PyMiniGU.load() 方法 (字典列表)
- [x] T027 [US2] 在 lib.rs 实现 PyMiniGU.load() 方法 (CSV 文件路径)
- [x] T028 [US2] 在 lib.rs 实现 PyMiniGU.save() 方法
- [x] T029 [US2] 在 minigu.py 实现 MiniGU.load() 封装
- [x] T030 [US2] 在 minigu.py 实现 MiniGU.save() 封装
- [x] T031 [US2] 实现数据验证 (路径安全、格式检查)
- [x] T032 [US2] 运行测试验证 US2 功能

**Checkpoint**: US1 + US2 完成，支持完整的数据操作流程

---

## Phase 5: User Story 4 - 错误处理与异常 (Priority: P2)

**Goal**: 提供清晰的 Python 风格异常层次结构

**Independent Test**: 触发各种错误条件，验证正确异常类型被抛出

### Tests for User Story 4

- [x] T033 [P] [US4] 添加 test_query_syntax_error 测试用例
- [x] T034 [P] [US4] 添加 test_connection_error 测试用例
- [x] T035 [P] [US4] 添加 test_graph_error 测试用例
- [x] T036 [P] [US4] 添加 test_data_error 测试用例

### Implementation for User Story 4

- [x] T037 [US4] 在 minigu.py 定义异常类层次结构 (MiniGUError, QueryError 等)
- [x] T038 [US4] 实现 _handle_exception() 错误分类函数
- [x] T039 [US4] 在 lib.rs 添加 is_syntax_error, is_timeout_error 辅助函数
- [x] T040 [US4] 在 MiniGU 方法中集成错误处理
- [x] T041 [US4] 运行测试验证 US4 功能

**Checkpoint**: 所有用户故事具备完善的错误处理

---

## Phase 6: User Story 3 - 异步 API 支持 (Priority: P3)

**Goal**: 提供异步 API 避免阻塞事件循环

**Independent Test**: 在 asyncio 环境中执行异步查询，验证非阻塞行为

### Tests for User Story 3

- [x] T042 [P] [US3] 添加 test_async_create_graph 测试用例
- [x] T043 [P] [US3] 添加 test_async_execute_query 测试用例
- [x] T044 [P] [US3] 添加 test_async_context_manager 测试用例
- [x] T045 [P] [US3] 添加 test_concurrent_queries 测试并发场景

### Implementation for User Story 3

- [x] T046 [US3] 在 minigu.py 实现 AsyncMiniGU 类
- [x] T047 [US3] 使用 asyncio.run_in_executor 包装同步方法
- [x] T048 [US3] 实现 AsyncMiniGU.__aenter__/__aexit__ 异步上下文管理器
- [x] T049 [US3] 实现所有异步方法 (init, create_graph, execute, load, save)
- [x] T050 [US3] 运行测试验证 US3 功能

**Checkpoint**: 同步和异步 API 功能完全对等

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: 完善文档、构建配置和代码质量

- [x] T051 [P] 创建 minigu/python/README.md 使用文档
- [x] T052 [P] 添加 Python docstring 到所有公开 API
- [x] T053 [P] 配置 .gitignore 忽略 Python 构建产物
- [x] T054 验证 maturin develop 开发流程
- [x] T055 验证 maturin build --release 构建流程
- [x] T056 运行 cargo clippy 检查代码质量
- [x] T057 运行完整测试套件验证所有功能
- [x] T058 运行 quickstart.md 中的示例验证文档准确性

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 无依赖，立即开始
- **Foundational (Phase 2)**: 依赖 Setup 完成 - **阻塞所有用户故事**
- **User Stories (Phase 3-6)**: 依赖 Foundational 完成
  - US1 (P1): 可独立开始
  - US2 (P2): 可独立开始，与 US1 并行
  - US4 (P2): 可独立开始，与 US1/US2 并行
  - US3 (P3): 建议在 US1 完成后开始（复用同步实现）
- **Polish (Phase 7)**: 依赖所有用户故事完成

### User Story Dependencies

- **User Story 1 (P1)**: 无依赖 - MVP 核心
- **User Story 2 (P2)**: 无依赖 - 可与 US1 并行
- **User Story 4 (P2)**: 无依赖 - 可与 US1/US2 并行
- **User Story 3 (P3)**: 建议依赖 US1 - 复用同步实现

### Parallel Opportunities

- Phase 1: T003, T004, T005 可并行
- Phase 2: 无并行机会（顺序依赖）
- Phase 3-6: 每个阶段内的测试任务 [P] 可并行
- Phase 7: T051, T052, T053 可并行

---

## Parallel Example: User Story 1 Tests

```bash
# 并行启动 US1 的所有测试任务:
Task T013: "添加 test_create_graph 测试用例"
Task T014: "添加 test_execute_query 测试用例"
Task T015: "添加 test_context_manager 测试用例"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. 完成 Phase 1: Setup
2. 完成 Phase 2: Foundational (阻塞点)
3. 完成 Phase 3: User Story 1
4. **STOP and VALIDATE**: 独立测试 US1
5. 可发布/演示

### Incremental Delivery

1. Setup + Foundational → 基础就绪
2. Add User Story 1 → 独立测试 → 发布 (MVP!)
3. Add User Story 2 + US4 → 独立测试 → 发布
4. Add User Story 3 → 独立测试 → 发布
5. 每个故事独立增值

### Parallel Team Strategy

多人协作:

1. 团队共同完成 Setup + Foundational
2. Foundational 完成后:
   - 开发者 A: User Story 1 (核心)
   - 开发者 B: User Story 2 (数据操作)
   - 开发者 C: User Story 4 (错误处理)
3. User Story 3 在 US1 完成后开始

---

## Notes

- [P] 任务 = 不同文件，无依赖
- [Story] 标签映射任务到用户故事
- 每个用户故事应独立可测试
- 测试先写，确保失败后再实现
- 每个任务或逻辑组完成后提交
- 任何检查点可停止验证故事独立性
- 避免: 模糊任务、同文件冲突、跨故事依赖