# Implementation Plan: OPTIONAL MATCH

**Branch**: `001-optional-match` | **Date**: 2026-04-17 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-optional-match/spec.md`

## Summary

实现 OPTIONAL MATCH 功能，为 miniGU 图数据库添加 LEFT JOIN 语义支持。当图模式不匹配时，保留左侧行并返回 NULL 值。实现涉及 Planner 层（Binder + LogicalPlanner + Optimizer）和 Execution 层（Physical Plan + Executor）。

## Technical Context

**Language/Version**: Rust 2024 Edition
**Primary Dependencies**: Logos (lexer), Winnow (parser), Arrow (columnar data), DashMap (concurrent)
**Storage**: MemoryGraph (TP), OlapStorage (AP), WAL + Checkpoint (persistence)
**Testing**: cargo test, SQLLogicTest, Insta (snapshot testing)
**Target Platform**: Cross-platform (Linux, macOS, Windows)
**Project Type**: Embedded graph database library with CLI
**Performance Goals**: OPTIONAL MATCH queries should complete within 2x time of equivalent regular MATCH
**Constraints**: Must pass all existing tests (no regressions), must handle NULL correctly in aggregations
**Scale/Scope**: Single feature, affects ~10 files across planner and execution modules

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Educational Purpose First | ✅ PASS | Implementation will be well-documented with clear comments explaining LEFT JOIN semantics |
| II. Rust Best Practices | ✅ PASS | Will use Result/Option properly, no unsafe code needed |
| III. Test-Driven Development | ✅ PASS | Will add unit tests and use existing finbench/snb test cases |
| IV. Modular Architecture | ✅ PASS | Changes follow existing layered architecture |
| V. GQL Standard Compliance | ✅ PASS | Following ISO GQL OPTIONAL MATCH semantics |
| VI. Performance Considerations | ✅ PASS | Will use efficient hash join approach, no premature optimization |

## Project Structure

### Documentation (this feature)

```text
specs/001-optional-match/
├── spec.md              # Feature specification (complete)
├── plan.md              # This file
├── research.md          # Phase 0 output - code analysis
├── data-model.md        # Phase 1 output - data structures
├── quickstart.md        # Phase 1 output - usage examples
└── tasks.md             # Phase 2 output - task breakdown
```

### Source Code (repository root)

```text
minigu/gql/
├── parser/src/
│   └── ast/query.rs           # MatchStatement::Optional (✅ already exists)
├── planner/src/
│   ├── bound/
│   │   ├── mod.rs             # Export BoundOptionalMatch
│   │   └── query.rs           # BoundMatchStatement::Optional (✅ exists, needs content)
│   ├── binder/
│   │   ├── mod.rs             # Binder exports
│   │   └── query.rs           # bind_match_statement (needs Optional branch)
│   ├── plan/
│   │   ├── mod.rs             # Export LogicalOptionalMatch
│   │   ├── logical_match.rs   # MatchKind::Optional (✅ exists)
│   │   └── optional_match.rs  # NEW: LogicalOptionalMatch plan node
│   ├── logical_planner/
│   │   └── query.rs           # plan_match_statement (needs Optional branch)
│   └── optimizer/
│       └── mod.rs             # Physical plan generation for Optional
├── execution/src/
│   ├── executor/
│   │   ├── mod.rs             # Export OptionalMatchExecutor
│   │   └── optional_match.rs  # NEW: OptionalMatchExecutor implementation
│   └── builder.rs             # Build OptionalMatchExecutor

minigu-test/gql/
├── finbench/tsr2.gql          # Test case (✅ exists)
└── snb/is7.gql                # Test case (✅ exists)
```

**Structure Decision**: Following existing modular architecture. New files added only where necessary (optional_match.rs in planner/plan and execution/executor).

## Phase 0: Research & Analysis

### Current Implementation Status

| Layer | Component | Status | Action Needed |
|-------|-----------|--------|---------------|
| Parser | `MatchStatement::Optional` | ✅ Complete | None |
| Binder | `BoundMatchStatement::Optional` | ⚠️ Placeholder | Implement binding logic |
| Planner | `MatchKind::Optional` | ✅ Defined | Use in LogicalOptionalMatch |
| Planner | `plan_match_statement` | ❌ Not implemented | Add Optional branch |
| Execution | `OptionalMatchExecutor` | ❌ Not exists | Create new |

### Key Findings

1. **Parser is Ready**: `MatchStatement::Optional(VecSpanned<MatchStatement>)` parses `OPTIONAL MATCH ...` correctly

2. **Binder Placeholder**: `bind_match_statement` returns `not_implemented` for Optional variant

3. **LogicalMatch has MatchKind**: `LogicalMatch` already includes `MatchKind::Optional` enum variant

4. **Executor Pattern**: Existing executors use Volcano model with `next_chunk()` method

5. **NULL Handling**: Need to verify `Datum::Null` exists and propagates correctly

### Reference: Existing HashJoin Implementation

The existing `PhysicalHashJoin` executor can serve as a reference for implementing the LEFT JOIN semantics. Key difference: LEFT JOIN must emit rows with NULL values when right side doesn't match.

## Phase 1: Design & Contracts

### Data Model

```rust
// In minigu/gql/planner/src/plan/optional_match.rs (NEW FILE)

/// Represents an OPTIONAL MATCH operation in the logical plan.
/// Semantically equivalent to LEFT OUTER JOIN in relational databases.
#[derive(Debug, Clone, Serialize)]
pub struct LogicalOptionalMatch {
    pub base: PlanBase,
    /// The pattern to optionally match
    pub pattern: BoundGraphPattern,
    /// Expressions to yield from the optional pattern
    pub yield_clause: Vec<BoundExpr>,
    /// Schema of the output (includes NULL-able columns from optional pattern)
    pub output_schema: DataSchema,
    /// Child plan (provides the "left" side of the LEFT JOIN)
    pub child: PlanNode,
}
```

```rust
// In minigu/gql/execution/src/executor/optional_match.rs (NEW FILE)

/// Executor for OPTIONAL MATCH operations.
/// Implements LEFT JOIN semantics: preserves left rows even when right side doesn't match.
pub struct OptionalMatchExecutor<L, R>
where
    L: Executor,
    R: Executor,
{
    /// Left child executor (always produces rows)
    left: L,
    /// Right child executor (optional pattern, may produce no rows for some left rows)
    right_builder: R,
    /// Join condition (typically on vertex ID)
    join_key_left: BoxedEvaluator,
    join_key_right: BoxedEvaluator,
    /// Schema for left side (to generate NULL values)
    left_schema: DataSchemaRef,
    /// Schema for right side (columns that become NULL on no match)
    right_schema: DataSchemaRef,
}
```

### Execution Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                    OptionalMatchExecutor                         │
├─────────────────────────────────────────────────────────────────┤
│  1. Build hash table from right side (optional pattern)          │
│  2. For each row from left side:                                 │
│     a. Probe hash table with join key                            │
│     b. If match found: emit combined row                         │
│     c. If no match: emit left row + NULL values for right cols   │
└─────────────────────────────────────────────────────────────────┘
```

### NULL Value Handling

```rust
// When right side doesn't match, generate NULL values
fn generate_null_row(left_chunk: DataChunk, right_schema: &DataSchema) -> DataChunk {
    let mut columns = left_chunk.columns().to_vec();
    for field in right_schema.fields() {
        let null_array = new_null_array(field.ty(), left_chunk.num_rows());
        columns.push(null_array);
    }
    DataChunk::new(columns)
}
```

### Quickstart

```sql
-- Basic OPTIONAL MATCH
MATCH (p:Person)
OPTIONAL MATCH (p)-[e:KNOWS]->(f:Person)
RETURN p.name, f.name;

-- With aggregation
MATCH (n:Account{id:12})
OPTIONAL MATCH (n)-[e:transfer]->(m:Account)
WHERE e.ts > 45 AND e.ts < 50
RETURN n, sum(e.amount) as totalAmount, count(e) as numTransfers;

-- Chained OPTIONAL MATCH (via NEXT)
MATCH (n:Account{id:12}) RETURN n
NEXT
OPTIONAL MATCH (n)-[e:transfer]->(m:Account)
RETURN sum(e.amount) as sumOut
NEXT
OPTIONAL MATCH (n)<-[e:transfer]-(m:Account)
RETURN sumOut, sum(e.amount) as sumIn;
```

## Phase 2: Task Breakdown

Tasks will be generated by `/speckit.tasks` command. Expected task groups:

1. **Binder Implementation** - Implement `bind_match_statement` Optional branch
2. **Logical Plan** - Create `LogicalOptionalMatch` plan node
3. **Physical Plan** - Add `PhysicalOptionalMatch` and optimizer rules
4. **Executor** - Implement `OptionalMatchExecutor` with LEFT JOIN semantics
5. **NULL Handling** - Ensure NULL propagation in expressions and aggregations
6. **Testing** - Unit tests, integration tests, validate finbench/snb cases
7. **Documentation** - Update user-guide.md with OPTIONAL MATCH usage

## Complexity Tracking

> No constitution violations. Implementation follows existing patterns.

| Aspect | Complexity | Justification |
|--------|------------|---------------|
| Binder changes | Low | Follow existing `bind_graph_pattern_binding_table` pattern |
| Logical plan | Low | Similar to existing `LogicalMatch` |
| Executor | Medium | Need to implement LEFT JOIN semantics correctly |
| NULL handling | Medium | Must ensure correct propagation through expressions |
| Testing | Medium | Multiple edge cases to cover |

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| NULL handling bugs | Medium | High | Comprehensive unit tests for NULL propagation |
| Performance regression | Low | Medium | Benchmark against regular MATCH queries |
| Edge cases in chaining | Medium | Medium | Test with finbench/tsr2.gql specifically |
| Aggregation with NULL | Medium | High | Verify SQL semantics for each aggregate function |

## Dependencies

- `minigu/gql/parser` - AST definitions (✅ ready)
- `minigu/gql/planner` - Binder and plan infrastructure
- `minigu/gql/execution` - Executor framework
- `minigu/common` - DataChunk, Datum, NULL representation
- `minigu-test` - Test framework and test cases

## Next Steps

Run `/speckit.tasks` to generate detailed task breakdown with dependencies.