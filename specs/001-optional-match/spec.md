# Feature Specification: OPTIONAL MATCH

**Feature Branch**: `001-optional-match`
**Created**: 2026-04-17
**Status**: Draft
**Input**: 实现 OPTIONAL MATCH 功能，支持图查询中的左连接语义。当右边的模式不匹配时，返回 NULL 值而不是过滤掉整行结果。参考测试用例 finbench/tsr2.gql 和 snb/is7.gql。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Basic Optional Pattern Matching (Priority: P1)

As a graph database user, I want to query for nodes and optionally match related patterns, so that I can retrieve all nodes of interest even when some related patterns don't exist.

**Why this priority**: This is the core functionality of OPTIONAL MATCH - the LEFT JOIN semantic for graph queries. Without this, users cannot perform queries that preserve unmatched patterns.

**Independent Test**: Can be fully tested by executing a simple OPTIONAL MATCH query where some nodes don't have matching patterns, and verifying that NULL values are returned for unmatched columns.

**Acceptance Scenarios**:

1. **Given** a graph with Person nodes where some have KNOWS edges and some don't, **When** I execute `MATCH (p:Person) OPTIONAL MATCH (p)-[e:KNOWS]->(f:Person) RETURN p.name, f.name`, **Then** all Person nodes are returned, with NULL for `f.name` when no KNOWS edge exists.

2. **Given** a graph with Account nodes and transfer edges, **When** I execute a query with OPTIONAL MATCH that filters on edge properties, **Then** accounts without matching transfers still appear in results with NULL values for edge-related columns.

3. **Given** a query with multiple OPTIONAL MATCH clauses, **When** some patterns match and others don't, **Then** results correctly combine matched and NULL values across all optional patterns.

---

### User Story 2 - Optional Match with Aggregation (Priority: P1)

As a graph database user, I want to use aggregation functions (COUNT, SUM, MAX, etc.) with OPTIONAL MATCH, so that I can compute statistics over optionally matched patterns without losing rows.

**Why this priority**: Aggregation with OPTIONAL MATCH is critical for analytical queries like those in FinBench. The test case `finbench/tsr2.gql` specifically requires this functionality.

**Independent Test**: Can be tested by executing queries with `sum()`, `count()`, `max()` on optionally matched edges and verifying correct NULL handling in aggregations.

**Acceptance Scenarios**:

1. **Given** an Account with no outgoing transfer edges, **When** I execute `OPTIONAL MATCH (n)-[e:transfer]->(m) RETURN count(e) as numEdges`, **Then** the result is `0` (not NULL) for that account.

2. **Given** an Account with transfer edges matching the filter, **When** I execute `OPTIONAL MATCH (n)-[e:transfer]->(m) WHERE e.amount > 100 RETURN sum(e.amount)`, **Then** the sum is correctly computed.

3. **Given** an OPTIONAL MATCH with no matches, **When** I use aggregation functions like `sum()` and `max()`, **Then** `sum()` returns NULL or 0 (depending on SQL semantics), `count()` returns 0, and `max()` returns NULL.

---

### User Story 3 - Chained Optional Matches (Priority: P2)

As a graph database user, I want to chain multiple OPTIONAL MATCH clauses with NEXT statements, so that I can build complex multi-hop queries where each step preserves previous results.

**Why this priority**: Complex analytical queries like FinBench TSR2 require chaining multiple OPTIONAL MATCH statements. This is essential for real-world graph analytics.

**Independent Test**: Can be tested by executing a query with two or more OPTIONAL MATCH clauses chained with NEXT, and verifying that results from earlier clauses are preserved.

**Acceptance Scenarios**:

1. **Given** a query starting with MATCH followed by two OPTIONAL MATCH clauses, **When** the first OPTIONAL MATCH has matches but the second doesn't, **Then** results from the first OPTIONAL MATCH are preserved with NULLs for the second.

2. **Given** the FinBench TSR2 query pattern, **When** executed against test data, **Then** results match expected output with correct handling of NULL propagations across NEXT boundaries.

---

### User Story 4 - Optional Match with WHERE Filtering (Priority: P2)

As a graph database user, I want to filter optionally matched patterns with WHERE clauses, so that I can selectively match patterns based on property conditions.

**Why this priority**: Most real-world OPTIONAL MATCH queries include filtering conditions. This is demonstrated in both test cases.

**Independent Test**: Can be tested by executing OPTIONAL MATCH with WHERE clause filtering on edge or node properties.

**Acceptance Scenarios**:

1. **Given** OPTIONAL MATCH with WHERE clause on edge properties, **When** no edges satisfy the WHERE condition, **Then** the row is preserved with NULL values for the optional pattern.

2. **Given** OPTIONAL MATCH with WHERE clause combining multiple conditions, **When** some conditions are satisfied, **Then** only matching patterns contribute non-NULL values.

---

### Edge Cases

- What happens when OPTIONAL MATCH follows another OPTIONAL MATCH with no intervening MATCH? The second OPTIONAL should operate on the results of the first, including NULL values.
- How does the system handle OPTIONAL MATCH with NULL values in the input binding? Variables from previous clauses that are NULL should still allow the OPTIONAL to execute.
- What happens with OPTIONAL MATCH that has no variables from previous clauses? This should behave like a regular MATCH (all rows cross-joined with match results).
- How are CASE WHEN expressions with NULL from OPTIONAL MATCH evaluated? NULL comparisons should follow SQL three-valued logic.
- What happens with ORDER BY on columns that may be NULL from OPTIONAL MATCH? NULL values should sort according to SQL semantics (typically first or last depending on NULLS FIRST/LAST).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST parse OPTIONAL MATCH statements as defined in the GQL standard, including the OPTIONAL keyword followed by MATCH and a graph pattern.

- **FR-002**: The system MUST implement LEFT JOIN semantics for OPTIONAL MATCH, where rows from the left (previous results) are preserved even when the optional pattern doesn't match.

- **FR-003**: When the optional pattern doesn't match, the system MUST return NULL values for all variables introduced in the OPTIONAL MATCH clause.

- **FR-004**: The system MUST correctly handle aggregation functions over optionally matched patterns, following SQL semantics for NULL handling in aggregations.

- **FR-005**: The system MUST support WHERE clauses in OPTIONAL MATCH for filtering the optional pattern.

- **FR-006**: The system MUST support chaining multiple OPTIONAL MATCH clauses via NEXT statements, preserving results from earlier clauses.

- **FR-007**: The system MUST correctly propagate NULL values through expressions (e.g., `CASE WHEN x IS NULL THEN ...`).

- **FR-008**: The system MUST support optional patterns on both incoming and outgoing edges (directional OPTIONAL MATCH).

- **FR-009**: The system MUST handle OPTIONAL MATCH with complex graph patterns including multiple hops and label expressions.

### Key Entities

- **OptionalMatchPlanNode**: A logical plan node representing the OPTIONAL MATCH operation, similar to a LEFT OUTER JOIN in relational databases.

- **OptionalMatchExecutor**: An executor that implements the LEFT JOIN semantics, producing rows with NULL values when the right side doesn't match.

- **NullDatum**: Representation of NULL values in the execution engine, which must propagate correctly through expressions and aggregations.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All existing test cases pass after OPTIONAL MATCH implementation (no regressions).

- **SC-002**: The FinBench TSR2 test case (`finbench/tsr2.gql`) executes successfully and produces correct results.

- **SC-003**: The SNB IS7 test case (`snb/is7.gql`) executes successfully and produces correct results.

- **SC-004**: OPTIONAL MATCH queries return results within acceptable performance bounds (no more than 2x the time of equivalent regular MATCH queries when patterns match).

- **SC-005**: Queries with OPTIONAL MATCH that have no matches complete successfully without errors, returning NULL values appropriately.

- **SC-006**: Aggregation functions (COUNT, SUM, MAX, MIN, AVG) correctly handle NULL values from OPTIONAL MATCH, following SQL semantics.

## Assumptions

- The GQL parser already supports parsing OPTIONAL MATCH syntax (verified: `MatchStatement::Optional` exists in AST).

- The existing execution engine can be extended with a new executor type for OPTIONAL MATCH.

- NULL value representation exists in the data type system (`Datum::Null` or equivalent).

- The existing LEFT JOIN or hash join infrastructure can be adapted or extended for OPTIONAL MATCH semantics.

- Test data for FinBench and SNB test cases is available or will be created as part of implementation.

- The query planner can be extended to generate OptionalMatch plan nodes from the parsed AST.

- Variable scoping across NEXT statements is already handled by the existing query execution framework.

## Out of Scope

- OPTIONAL MATCH with path patterns (variable-length paths) - to be addressed in a future iteration if needed.

- OPTIONAL MATCH with quantifiers (*, +, {m,n}) - depends on general path quantifier support.

- Performance optimizations specific to OPTIONAL MATCH (e.g., predicate pushdown) - basic implementation first.

## Dependencies

- Existing MATCH implementation for reference on graph pattern matching.

- Binder infrastructure for variable resolution and type checking.

- Executor framework for implementing the new OPTIONAL MATCH executor.

- Aggregation executor for handling aggregations over optional results.

- Test framework (SQLLogicTest) for validating implementation.