# Implementation Plan: Python API Bindings

**Branch**: `002-python-api` | **Date**: 2026-04-17 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-python-api/spec.md`

## Summary

为 miniGU 图数据库提供 Python API 绑定，使用 PyO3 框架将 Rust 实现暴露给 Python 开发者。支持同步和异步两种 API 风格，提供完整的图数据库操作能力（创建图、执行 GQL 查询、数据加载与持久化）。pyapi 分支已有完整实现，本计划将其合并到主分支并完善。

## Technical Context

**Language/Version**: Rust 1.94+ (nightly), Python 3.7-3.12
**Primary Dependencies**: PyO3 0.24.2, maturin 1.0+, arrow 55.2.0
**Storage**: N/A (使用 miniGU 现有存储层)
**Testing**: pytest, cargo test
**Target Platform**: Linux, macOS, Windows (Python wheel 分发)
**Project Type**: library (Python extension module)
**Performance Goals**: 无特殊性能要求，保持与 Rust 核心相当的性能
**Constraints**: 支持 Python 3.7+ (使用 abi3-py37)，无 unsafe 代码
**Scale/Scope**: 单用户本地使用场景

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Educational Purpose First | ✅ PASS | Python API 作为教学工具，代码应清晰易懂 |
| II. Rust Best Practices | ✅ PASS | 使用 PyO3 最佳实践，遵循 Rust 惯例 |
| III. Test-Driven Development | ✅ PASS | 需要添加 Python 测试用例 |
| IV. Modular Architecture | ✅ PASS | Python 绑定作为独立 crate，不污染核心代码 |
| V. GQL Standard Compliance | ✅ PASS | API 不影响 GQL 标准 compliance |
| VI. Performance Considerations | ✅ PASS | PyO3 零成本抽象，性能损失最小 |

**Gate Result**: ✅ ALL PASS - 可以继续进行 Phase 0

## Project Structure

### Documentation (this feature)

```text
specs/002-python-api/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output
```

### Source Code (repository root)

```text
minigu/python/                    # Python binding crate (NEW)
├── Cargo.toml                    # Rust crate 配置
├── pyproject.toml                # Python 项目配置
├── build.rs                      # 构建脚本 (macOS 链接配置)
├── src/
│   └── lib.rs                    # PyO3 绑定实现
├── __init__.py                   # Python 包入口
├── minigu.py                     # Python API 封装 (同步/异步)
├── README.md                     # 使用文档
└── test_minigu_api.py            # Python 测试用例

# Workspace 更新
Cargo.toml                        # 添加 minigu/python 到 members
                                  # 添加 pyo3 到 workspace.dependencies
```

**Structure Decision**: 在 minigu 目录下创建独立的 python crate，保持与现有模块结构一致。Python 绑定作为可选组件，不影响核心功能。

## Complexity Tracking

> No violations - no justification needed.

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | N/A | N/A |