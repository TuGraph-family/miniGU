# MiniGU Lab Testing Guide

## Quick Commands

```powershell
# Lab1 - Filter Tests
cargo test --package minigu-test --test lab1_filter_test -- --nocapture

# Lab2 - Executor Tests
cargo test --package minigu-test --test lab2_executor_test -- --nocapture

# Lab3 - Optimizer Tests
cargo test --package minigu-test --test lab3_optimizer_test -- --nocapture

# Run specific test
cargo test --package minigu-test --test lab1_filter_test test_basic_where_clause -- --nocapture
```

## Test Overview

| Lab | File | Tests | Focus |
|-----|------|-------|-------|
| Lab1 | `lab1_filter_test.rs` | 7 | WHERE clause Filter node |
| Lab2 | `lab2_executor_test.rs` | 10 | Project & Expand operators |
| Lab3 | `lab3_optimizer_test.rs` | 7 | Predicate pushdown |

## Test Data

```sql
CALL create_test_graph_data('test_graph', 10)
```

- **Nodes:** `PERSON`(name, age), `COMPANY`(name, revenue), `CITY`(name, population)
- **Edges:** `FRIEND`, `WORKS_AT`, `LOCATED_IN`

## Source Files

- Tests: `minigu-test/src/lab*_test.rs`
- Planner: `minigu/gql/planner/src/`
- Executor: `minigu/gql/execution/src/`
