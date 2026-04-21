# Research: OPTIONAL MATCH Implementation

**Date**: 2026-04-17
**Feature**: 001-optional-match

## Code Analysis

### 1. Parser Layer (Complete)

**File**: `minigu/gql/parser/src/ast/query.rs`

```rust
pub enum MatchStatement {
    Simple(Box<Spanned<GraphPatternBindingTable>>),
    Optional(VecSpanned<MatchStatement>),  // ✅ Already defined
}
```

**File**: `minigu/gql/parser/src/parser/impls/query.rs`

```rust
pub fn match_statement(input: &mut TokenStream) -> ModalResult<Spanned<MatchStatement>> {
    dispatch!(peek(any);
        TokenKind::Match => simple_match_statement.map(|t| MatchStatement::Simple(Box::new(t))),
        TokenKind::Optional => optional_match_statement.map(MatchStatement::Optional),
        _ => fail
    ).parse_next(input)
}

pub fn optional_match_statement(
    input: &mut TokenStream,
) -> ModalResult<VecSpanned<MatchStatement>> {
    // Parses: OPTIONAL MATCH ...
}
```

**Status**: ✅ Parser fully supports OPTIONAL MATCH syntax.

### 2. Binder Layer (Partial)

**File**: `minigu/gql/planner/src/bound/query.rs`

```rust
pub enum BoundMatchStatement {
    Simple(Box<BoundGraphPatternBindingTable>),
    Optional,  // ⚠️ Placeholder, no content
}
```

**File**: `minigu/gql/planner/src/binder/query.rs`

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
        MatchStatement::Optional(_) => not_implemented("optional match statement", None),
        // ❌ Returns not_implemented
    }
}
```

**Status**: ⚠️ Need to implement `bind_match_statement` for Optional variant.

**Required Changes**:
1. Change `BoundMatchStatement::Optional` to contain actual bound data
2. Implement binding logic for OPTIONAL MATCH

### 3. Logical Planner Layer (Partial)

**File**: `minigu/gql/planner/src/plan/logical_match.rs`

```rust
pub enum MatchKind {
    Simple,
    Optional,  // ✅ Already defined
}

pub struct LogicalMatch {
    pub base: PlanBase,
    pub kind: MatchKind,
    pub pattern: BoundGraphPattern,
    pub yield_clause: Vec<BoundExpr>,
    pub output_schema: DataSchema,
}
```

**File**: `minigu/gql/planner/src/logical_planner/query.rs`

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
        BoundMatchStatement::Optional => not_implemented("match statement optional", None),
        // ❌ Returns not_implemented
    }
}
```

**Status**: ⚠️ `MatchKind::Optional` exists but planner doesn't use it.

**Required Changes**:
1. Create `LogicalOptionalMatch` plan node (or extend `LogicalMatch`)
2. Implement `plan_match_statement` for Optional variant

### 4. Execution Layer (Not Implemented)

**File**: `minigu/gql/execution/src/builder.rs`

No handling for OPTIONAL MATCH. The builder has cases for:
- `PhysicalFilter`
- `PhysicalNodeScan`
- `PhysicalExpand`
- `PhysicalProject`
- `PhysicalHashJoin`
- etc.

But no `PhysicalOptionalMatch` or equivalent.

**Status**: ❌ Need to create `OptionalMatchExecutor` and corresponding physical plan node.

## Reference Implementations

### HashJoin Executor (for LEFT JOIN reference)

**File**: `minigu/gql/execution/src/executor/join.rs`

```rust
pub struct HashJoinExecutor<L, R, K>
where
    L: Executor,
    R: Executor,
    K: HashJoinKey,
{
    left: L,
    right: R,
    conds: Vec<JoinCond>,
    join_type: JoinType,  // Includes LeftOuter variant
}
```

**Key Insight**: The `JoinType` enum may already include `LeftOuter`. Need to verify.

### NULL Handling

**File**: `minigu/common/src/value.rs`

Need to verify:
1. `Datum::Null` variant exists
2. NULL propagation in binary operations
3. NULL handling in aggregation functions

## Design Decisions

### Decision 1: Separate LogicalOptionalMatch vs Extend LogicalMatch

**Chosen**: Create separate `LogicalOptionalMatch` plan node

**Rationale**:
- Clearer separation of concerns
- Different schema handling (NULL-able columns)
- Easier to implement and test independently

**Alternatives Rejected**:
- Extend `LogicalMatch` with `MatchKind::Optional`: Would complicate schema handling

### Decision 2: Executor Implementation Approach

**Chosen**: Create new `OptionalMatchExecutor` similar to HashJoinExecutor

**Rationale**:
- Reuses proven hash join pattern
- Clear LEFT JOIN semantics
- Can leverage existing join infrastructure

**Alternatives Rejected**:
- Modify existing `HashJoinExecutor`: Would add complexity to existing code

### Decision 3: NULL Value Generation

**Chosen**: Generate NULL arrays when right side doesn't match

**Rationale**:
- Follows Arrow conventions
- Efficient for columnar execution
- Consistent with existing NULL handling

## Open Questions

1. **Chained OPTIONAL MATCH**: How does the current query execution handle multiple statements via NEXT?
   - **Finding**: `BoundLinearQueryStatement::Query { statements, result }` handles multiple statements
   - **Action**: Verify that schema propagates correctly between statements

2. **Aggregation with NULL**: How do aggregate functions handle NULL values?
   - **Finding**: Need to check `AggregateExecutor` implementation
   - **Action**: Ensure COUNT, SUM, MAX, MIN, AVG follow SQL semantics

3. **Variable Scoping**: How are variables scoped across OPTIONAL MATCH boundaries?
   - **Finding**: Variables from left side are preserved; new variables from right side are NULL-able
   - **Action**: Update schema to mark right-side columns as nullable