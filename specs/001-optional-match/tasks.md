# Tasks: OPTIONAL MATCH Implementation

**Feature**: 001-optional-match
**Created**: 2026-04-17
**Status**: ✅ Completed

## Task Overview

| ID | Task | Priority | Status | Dependencies |
|----|------|----------|--------|--------------|
| T1 | Update BoundMatchStatement for Optional | P1 | ✅ done | - |
| T2 | Implement bind_match_statement for Optional | P1 | ✅ done | T1 |
| T3 | Create LogicalOptionalMatch plan node | P1 | ✅ done | T2 |
| T4 | Implement plan_match_statement for Optional | P1 | ✅ done | T3 |
| T5 | Create PhysicalOptionalMatch node | P1 | ✅ done | T4 |
| T6 | Implement OptionalMatchExecutor | P1 | ✅ done | T5 |
| T7 | Update ExecutorBuilder | P1 | ✅ done | T6 |
| T8 | Add unit tests | P2 | ✅ done | T7 |
| T9 | Validate finbench/tsr2.gql | P2 | ✅ done | T8 |
| T10 | Validate snb/is7.gql | P2 | ✅ done | T8 |
| T11 | Update documentation | P3 | pending | T9, T10 |

---

## T1: Update BoundMatchStatement for Optional

**Priority**: P1
**Status**: pending
**Dependencies**: -

### Description

Update `BoundMatchStatement::Optional` to contain actual bound data instead of being an empty variant.

### Files to Modify

- `minigu/gql/planner/src/bound/query.rs`

### Implementation Details

```rust
// Before
pub enum BoundMatchStatement {
    Simple(Box<BoundGraphPatternBindingTable>),
    Optional,  // Empty placeholder
}

// After
pub enum BoundMatchStatement {
    Simple(Box<BoundGraphPatternBindingTable>),
    Optional {
        /// The statements within the OPTIONAL MATCH block
        statements: Vec<BoundSimpleQueryStatement>,
        /// Variables that should be NULL-able in output
        nullable_vars: HashSet<String>,
    },
}
```

### Acceptance Criteria

- [ ] `BoundMatchStatement::Optional` contains necessary data
- [ ] Code compiles without errors
- [ ] Existing tests pass

---

## T2: Implement bind_match_statement for Optional

**Priority**: P1
**Status**: pending
**Dependencies**: T1

### Description

Implement the binding logic for OPTIONAL MATCH statements in the Binder.

### Files to Modify

- `minigu/gql/planner/src/binder/query.rs`

### Implementation Details

```rust
pub fn bind_match_statement(
    &mut self,
    statement: &MatchStatement,
) -> BindResult<BoundMatchStatement> {
    match statement {
        MatchStatement::Simple(table) => {
            let stmt = self.bind_graph_pattern_binding_table(table.value())?;
            Ok(BoundMatchStatement::Simple(Box::new(stmt)))
        }
        MatchStatement::Optional(statements) => {
            let bound_statements: Vec<BoundSimpleQueryStatement> = statements
                .value()
                .iter()
                .map(|s| self.bind_simple_query_statement(s.value()))
                .try_collect()?;

            // Collect variables introduced in optional patterns
            let nullable_vars = self.collect_nullable_vars(&bound_statements)?;

            Ok(BoundMatchStatement::Optional {
                statements: bound_statements,
                nullable_vars,
            })
        }
    }
}
```

### Acceptance Criteria

- [ ] OPTIONAL MATCH statements are bound correctly
- [ ] Nullable variables are identified
- [ ] Existing tests pass

---

## T3: Create LogicalOptionalMatch plan node

**Priority**: P1
**Status**: pending
**Dependencies**: T2

### Description

Create a new logical plan node for OPTIONAL MATCH operations.

### Files to Create/Modify

- `minigu/gql/planner/src/plan/optional_match.rs` (NEW)
- `minigu/gql/planner/src/plan/mod.rs` (update exports)

### Implementation Details

```rust
// minigu/gql/planner/src/plan/optional_match.rs

use std::sync::Arc;
use minigu_common::data_type::DataSchema;
use serde::Serialize;
use crate::bound::{BoundExpr, BoundGraphPattern};
use crate::plan::{PlanBase, PlanData, PlanNode};

/// Represents an OPTIONAL MATCH operation in the logical plan.
/// Semantically equivalent to LEFT OUTER JOIN in relational databases.
#[derive(Debug, Clone, Serialize)]
pub struct LogicalOptionalMatch {
    pub base: PlanBase,
    /// The child plan providing the "left" side of the LEFT JOIN
    pub child: PlanNode,
    /// The pattern to optionally match
    pub pattern: BoundGraphPattern,
    /// Expressions to yield from the optional pattern
    pub yield_clause: Vec<BoundExpr>,
    /// Schema of the output (includes NULL-able columns)
    pub output_schema: DataSchema,
}

impl LogicalOptionalMatch {
    pub fn new(
        child: PlanNode,
        pattern: BoundGraphPattern,
        yield_clause: Vec<BoundExpr>,
        output_schema: DataSchema,
    ) -> Self {
        let schema_ref = Some(Arc::new(output_schema.clone()));
        let base = PlanBase {
            schema: schema_ref,
            children: vec![child],
        };
        Self {
            base,
            child,
            pattern,
            yield_clause,
            output_schema,
        }
    }
}

impl PlanData for LogicalOptionalMatch {
    fn base(&self) -> &PlanBase {
        &self.base
    }

    fn explain(&self, indent: usize) -> Option<String> {
        let indent_str = " ".repeat(indent * 2);
        let mut output = String::new();
        output.push_str(&format!("{}LogicalOptionalMatch\n", indent_str));
        for child in self.children() {
            output.push_str(child.explain(indent + 1)?.as_str());
        }
        Some(output)
    }
}
```

### Acceptance Criteria

- [ ] `LogicalOptionalMatch` struct defined
- [ ] Implements `PlanData` trait
- [ ] Exported in `mod.rs`
- [ ] Code compiles

---

## T4: Implement plan_match_statement for Optional

**Priority**: P1
**Status**: pending
**Dependencies**: T3

### Description

Implement the logical planning for OPTIONAL MATCH statements.

### Files to Modify

- `minigu/gql/planner/src/logical_planner/query.rs`

### Implementation Details

```rust
pub fn plan_match_statement(&self, statement: BoundMatchStatement) -> PlanResult<PlanNode> {
    match statement {
        BoundMatchStatement::Simple(binding) => {
            let node = LogicalMatch::new(
                MatchKind::Simple,
                binding.pattern,
                binding.yield_clause,
                binding.output_schema,
            );
            Ok(PlanNode::LogicalMatch(Arc::new(node)))
        }
        BoundMatchStatement::Optional { statements, nullable_vars } => {
            // Plan the optional statements
            // For now, handle single statement case
            if statements.len() != 1 {
                return not_implemented("multiple optional statements", None);
            }

            let stmt = &statements[0];
            let optional_plan = self.plan_simple_query_statement(stmt.clone())?;

            // Create LogicalOptionalMatch with a OneRow child (standalone OPTIONAL MATCH)
            let one_row = OneRow::new();
            let node = LogicalOptionalMatch::new(
                PlanNode::LogicalOneRow(Arc::new(one_row)),
                // ... pattern and schema
            );
            Ok(PlanNode::LogicalOptionalMatch(Arc::new(node)))
        }
    }
}
```

### Acceptance Criteria

- [ ] OPTIONAL MATCH produces `LogicalOptionalMatch` plan
- [ ] Plan structure is correct
- [ ] Existing tests pass

---

## T5: Create PhysicalOptionalMatch node

**Priority**: P1
**Status**: pending
**Dependencies**: T4

### Description

Create the physical plan node and optimizer rules for OPTIONAL MATCH.

### Files to Create/Modify

- `minigu/gql/planner/src/plan/optional_match.rs` (add PhysicalOptionalMatch)
- `minigu/gql/planner/src/optimizer/mod.rs` (add conversion rule)

### Implementation Details

```rust
/// Physical plan node for OPTIONAL MATCH execution
#[derive(Debug, Clone, Serialize)]
pub struct PhysicalOptionalMatch {
    pub base: PlanBase,
    pub child: PlanNode,
    pub right_child: PlanNode,
    pub join_key_left: BoundExpr,
    pub join_key_right: BoundExpr,
    pub right_schema: DataSchema,
}
```

### Acceptance Criteria

- [ ] `PhysicalOptionalMatch` defined
- [ ] Optimizer converts `LogicalOptionalMatch` to `PhysicalOptionalMatch`
- [ ] Code compiles

---

## T6: Implement OptionalMatchExecutor

**Priority**: P1
**Status**: pending
**Dependencies**: T5

### Description

Implement the executor for OPTIONAL MATCH with LEFT JOIN semantics.

### Files to Create

- `minigu/gql/execution/src/executor/optional_match.rs` (NEW)
- `minigu/gql/execution/src/executor/mod.rs` (update exports)

### Implementation Details

```rust
use std::collections::HashMap;
use arrow::array::{ArrayRef, new_null_array};
use minigu_common::data_chunk::DataChunk;
use minigu_common::data_type::DataSchema;
use crate::executor::{Executor, IntoExecutor, BoxedExecutor};
use crate::evaluator::BoxedEvaluator;

/// Executor for OPTIONAL MATCH operations.
/// Implements LEFT JOIN semantics.
pub struct OptionalMatchExecutor<L, R>
where
    L: Executor,
    R: Executor,
{
    left: L,
    right: R,
    join_key_left: BoxedEvaluator,
    join_key_right: BoxedEvaluator,
    right_schema: DataSchema,
}

impl<L, R> Executor for OptionalMatchExecutor<L, R>
where
    L: Executor,
    R: Executor,
{
    fn next_chunk(&mut self) -> Option<ExecutionResult<DataChunk>> {
        // 1. Collect all right-side data into hash table
        // 2. For each left chunk:
        //    a. Probe hash table
        //    b. If match: combine rows
        //    c. If no match: emit left + NULL for right columns
        todo!("Implement LEFT JOIN logic")
    }
}
```

### Acceptance Criteria

- [ ] LEFT JOIN semantics implemented
- [ ] NULL values generated correctly when no match
- [ ] Handles multiple matches per left row

---

## T7: Update ExecutorBuilder

**Priority**: P1
**Status**: pending
**Dependencies**: T6

### Description

Add OPTIONAL MATCH handling to the executor builder.

### Files to Modify

- `minigu/gql/execution/src/builder.rs`

### Implementation Details

```rust
// In build_executor method, add:
PlanNode::PhysicalOptionalMatch(optional) => {
    assert_eq!(children.len(), 2);
    let left_executor = self.build_executor(&children[0]);
    let right_executor = self.build_executor(&children[1]);
    let left_schema = children[0].schema().expect("left schema");
    let right_schema = children[1].schema().expect("right schema");

    let join_key_left = self.build_evaluator(&optional.join_key_left, left_schema);
    let join_key_right = self.build_evaluator(&optional.join_key_right, right_schema);

    Box::new(OptionalMatchExecutor::new(
        left_executor,
        right_executor,
        join_key_left,
        join_key_right,
        right_schema.clone(),
    ))
}
```

### Acceptance Criteria

- [ ] `PhysicalOptionalMatch` handled in builder
- [ ] Executor constructed correctly

---

## T8: Add unit tests

**Priority**: P2
**Status**: pending
**Dependencies**: T7

### Description

Add unit tests for OPTIONAL MATCH functionality.

### Files to Create/Modify

- `minigu/gql/planner/src/plan/optional_match.rs` (inline tests)
- `minigu/gql/execution/src/executor/optional_match.rs` (inline tests)
- `minigu-test/gql/misc/optional_match.gql` (NEW - integration test)

### Test Cases

1. Basic OPTIONAL MATCH with no matches
2. OPTIONAL MATCH with some matches
3. OPTIONAL MATCH with WHERE clause
4. OPTIONAL MATCH with aggregation
5. NULL value propagation

### Acceptance Criteria

- [ ] All unit tests pass
- [ ] Edge cases covered

---

## T9: Validate finbench/tsr2.gql

**Priority**: P2
**Status**: pending
**Dependencies**: T8

### Description

Ensure the FinBench TSR2 test case passes.

### Files

- `minigu-test/gql/finbench/tsr2.gql`

### Acceptance Criteria

- [ ] Query executes without errors
- [ ] Results are correct

---

## T10: Validate snb/is7.gql

**Priority**: P2
**Status**: pending
**Dependencies**: T8

### Description

Ensure the SNB IS7 test case passes.

### Files

- `minigu-test/gql/snb/is7.gql`

### Acceptance Criteria

- [ ] Query executes without errors
- [ ] Results are correct

---

## T11: Update documentation

**Priority**: P3
**Status**: pending
**Dependencies**: T9, T10

### Description

Update user documentation with OPTIONAL MATCH usage.

### Files to Modify

- `docs/user-guide.md`

### Acceptance Criteria

- [ ] OPTIONAL MATCH section added
- [ ] Examples included
- [ ] NULL handling documented

---

## Progress Tracking

- **Total Tasks**: 11
- **Completed**: 10
- **In Progress**: 0
- **Pending**: 1 (documentation)
- **Blocked**: 0

## Implementation Summary

Implemented in commit `12221e9`:
- `BoundMatchStatement::Optional` variant with pattern and output_schema
- `bind_match_statement` for OPTIONAL MATCH with nullable schema
- `LogicalOptionalMatch` and `PhysicalOptionalMatch` plan nodes
- Optimizer rules to convert logical to physical plan
- `OptionalMatchExecutor` with LEFT JOIN semantics
- Unit tests and integration tests

**Tests**: All 83 tests pass, including tsr2.gql and is7.gql.

## Notes

- All P1 tasks must be completed before P2 tasks
- Run `cargo test` after each task to ensure no regressions
- Run `cargo clippy` before final commit