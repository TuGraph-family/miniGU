# Research: Python API Bindings

**Feature**: 002-python-api
**Date**: 2026-04-17

## Research Tasks

### 1. PyO3 绑定最佳实践

**Decision**: 使用 PyO3 0.24.2 版本，配合 abi3-py37 特性支持 Python 3.7+

**Rationale**:
- PyO3 是 Rust 生态中最成熟的 Python 绑定框架
- abi3-py37 特性允许构建一个 wheel 支持所有 Python 3.7+ 版本
- pyapi 分支已验证该技术栈的可行性

**Alternatives Considered**:
- **pyo3-asyncio**: 考虑用于异步支持，但 pyapi 分支使用纯 Python async wrapper 更简单
- **uniffi**: Mozilla 的多语言绑定工具，但对 Python 支持不如 PyO3 成熟

### 2. 构建与分发策略

**Decision**: 使用 maturin 作为构建工具，发布到 PyPI

**Rationale**:
- maturin 是 PyO3 官方推荐的构建工具
- 支持自动生成 manylinux、macOS、Windows wheel
- 开发时可使用 `maturin develop` 快速迭代

**Build Commands**:
```bash
# 开发模式
maturin develop

# 构建 wheel
maturin build --release

# 发布到 PyPI
maturin publish
```

**Alternatives Considered**:
- **setuptools-rust**: 功能较老，maturin 更现代化
- **手动构建**: 复杂度高，不推荐

### 3. 异步 API 实现方案

**Decision**: 在 Python 层使用 asyncio 包装同步 Rust 调用

**Rationale**:
- PyO3 的异步支持仍在发展中，纯 Python wrapper 更稳定
- 使用 `asyncio.to_thread()` 或 `loop.run_in_executor()` 将同步调用转为异步
- 避免在 Rust 层处理 Python GIL 和异步运行时的复杂交互

**Implementation Pattern**:
```python
class AsyncMiniGU:
    def __init__(self):
        self._sync_db = MiniGU()  # 同步实例

    async def execute(self, query: str):
        loop = asyncio.get_event_loop()
        return await loop.run_in_executor(None, self._sync_db.execute, query)
```

**Alternatives Considered**:
- **pyo3-asyncio**: 需要处理更多边界情况，复杂度高
- **Tokio + PyO3**: 需要在 Rust 层管理异步运行时，增加依赖

### 4. 数据类型映射

**Decision**: 使用 Arrow 作为中间表示，在 Python 层转换为原生类型

**Rationale**:
- miniGU 内部使用 Arrow 列式存储
- Arrow 有成熟的 Python 绑定 (pyarrow)
- 对于简单场景，直接转换为 Python 原生类型更友好

**Type Mapping**:
| Rust/Arrow Type | Python Type |
|-----------------|-------------|
| Int8/16/32/64 | int |
| Float32/64 | float |
| String | str |
| Boolean | bool |
| Null | None |
| Vertex | dict (id, label, properties) |
| Edge | dict (id, src, dst, label, properties) |
| Vector | list[float] |

### 5. 错误处理策略

**Decision**: 在 Python 层进行错误分类和异常转换

**Rationale**:
- PyO3 自动将 Rust `Err` 转换为 Python `Exception`
- 在 Python wrapper 中根据错误消息进行分类
- 提供清晰的异常层次结构

**Exception Hierarchy**:
```
MiniGUError (base)
├── ConnectionError
├── QueryError
│   ├── QuerySyntaxError
│   ├── QueryExecutionError
│   └── QueryTimeoutError
├── GraphError
├── DataError
└── TransactionError
```

### 6. 测试策略

**Decision**: 使用 pytest 进行 Python API 测试

**Rationale**:
- pytest 是 Python 生态的标准测试框架
- 支持参数化测试、fixtures、异步测试
- 可与 CI/CD 集成

**Test Categories**:
1. **单元测试**: 测试单个 API 方法
2. **集成测试**: 测试完整的查询流程
3. **错误处理测试**: 验证异常类型和消息

## Open Questions

### Q1: 是否需要支持连接远程服务器？

**Answer**: 暂不支持。当前版本仅支持本地嵌入式模式，保持简单。远程连接可作为后续功能。

### Q2: 是否需要支持事务 API？

**Answer**: 暂不支持。miniGU 当前使用自动提交模式，事务 API 需要核心层支持后再添加。

### Q3: wheel 分发策略？

**Answer**: 优先支持主流平台 (manylinux2014, macOS x86_64/arm64, Windows)。用户也可从源码构建。

## Dependencies

### Rust Dependencies (minigu/python/Cargo.toml)

```toml
[dependencies]
arrow = { workspace = true }
minigu = { workspace = true }
pyo3 = { workspace = true, features = ["extension-module", "abi3-py37"] }

[build-dependencies]
pyo3-build-config = "0.24.2"
```

### Python Dependencies (pyproject.toml)

```toml
[project]
requires-python = ">=3.7"
# 无运行时依赖，所有功能由 Rust 扩展提供
```

### Dev Dependencies

```toml
[project.optional-dependencies]
dev = ["pytest>=7.0", "pytest-asyncio>=0.21"]
```

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| PyO3 API 变更 | 中 | 使用稳定版本，锁定版本号 |
| Python 版本兼容性 | 低 | 使用 abi3-py37，测试多版本 |
| 构建环境复杂 | 中 | 提供 Docker 构建镜像，详细文档 |
| GIL 性能影响 | 低 | 当前为单用户场景，影响有限 |