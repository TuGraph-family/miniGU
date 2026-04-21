# miniGU User Guide

miniGU is a graph database designed for learning purposes, implemented in Rust. This guide covers the basic usage and GQL query syntax.

## Table of Contents

- [Getting Started](#getting-started)
- [GQL Query Syntax](#gql-query-syntax)
  - [MATCH Clause](#match-clause)
  - [OPTIONAL MATCH Clause](#optional-match-clause)
  - [INSERT Clause](#insert-clause)
  - [WHERE Clause](#where-clause)
  - [RETURN Clause](#return-clause)
- [Graph Operations](#graph-operations)
- [Examples](#examples)

## Getting Started

Start the interactive shell:

```bash
cargo run -- shell    # debug mode
cargo run -r -- shell # release mode
```

## GQL Query Syntax

miniGU supports a subset of the ISO GQL standard.

### MATCH Clause

The `MATCH` clause is used to query graph patterns.

```sql
-- Simple vertex match
MATCH (n:Person) RETURN n;

-- Edge pattern
MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a, b, r;

-- Path pattern with variable length
MATCH (n:Person)-[r:KNOWS]->{1,3}(m:Person) RETURN n, m;
```

### OPTIONAL MATCH Clause

`OPTIONAL MATCH` implements LEFT JOIN semantics - all rows from the preceding query are preserved, and NULL values are generated for the optional pattern when no match is found.

#### Syntax

```sql
OPTIONAL MATCH <pattern> [WHERE <condition>] RETURN <variables>
```

#### Semantics

- **LEFT JOIN behavior**: All rows from the previous result are preserved
- **NULL generation**: When the optional pattern doesn't match, NULL values are generated for the pattern's variables
- **Multiple matches**: When multiple matches exist, each match produces a separate result row (cross product)

#### Examples

```sql
-- Basic OPTIONAL MATCH - returns NULL if no match
OPTIONAL MATCH (n:Person) WHERE n.id = 999 RETURN n.name;

-- OPTIONAL MATCH with edge pattern
OPTIONAL MATCH (a:Account)-[e:transfer]->(b:Account)
WHERE e.amount > 100
RETURN a, b, e.amount;

-- Combining MATCH and OPTIONAL MATCH
MATCH (m:Person) WHERE m.id = 274877907096
OPTIONAL MATCH (m)-[e:replyOf]->(c:Comment)
RETURN m, c;
```

#### NULL Handling

When an `OPTIONAL MATCH` doesn't find a match:

- Vertex variables return NULL
- Edge variables return NULL
- Property access on NULL returns NULL
- Use `IS NULL` or `IS NOT NULL` to check for NULL values

```sql
MATCH (n:Person)
OPTIONAL MATCH (n)-[r:KNOWS]->(friend:Person)
RETURN n.name, friend.name IS NULL AS has_no_friends;
```

### INSERT Clause

Insert vertices and edges into the graph.

```sql
-- Insert a vertex
INSERT (n:Person {name: 'Alice', age: 30});

-- Insert an edge
MATCH (a:Person {name: 'Alice'}), (b:Person {name: 'Bob'})
INSERT (a)-[r:KNOWS {since: 2020}]->(b);
```

### WHERE Clause

Filter query results with conditions.

```sql
-- Simple condition
MATCH (n:Person) WHERE n.age > 25 RETURN n;

-- Pattern existence
MATCH (p:Person)-[r:IS_FRIENDS_WITH]->(friend:Person)
WHERE EXISTS (MATCH (p)-[:WORKS_FOR]->(:Company {name: "GQL, Inc."}))
RETURN p, r, friend;

-- LIKE pattern matching
MATCH (n:Person) WHERE n.name LIKE '%ob%' RETURN n;
```

### RETURN Clause

Specify what to return from the query.

```sql
-- Return variables
MATCH (n:Person) RETURN n.name, n.age;

-- With aliases
MATCH (n:Person) RETURN n.name AS name, n.age AS age;

-- With aggregation
MATCH (n:Person) RETURN COUNT(n) AS total;
MATCH (n:Person)-[r:KNOWS]->(m:Person) RETURN SUM(m.age), MAX(m.age), MIN(m.age);

-- With ORDER BY and LIMIT
MATCH (n:Person) RETURN n.name, n.age ORDER BY n.age DESC LIMIT 10;
```

## Graph Operations

### Creating a Graph

```sql
-- Create a graph with schema
CREATE GRAPH my_graph {
  (Person: PersonLabel {
    id STRING,
    name STRING,
    age INT64,
    PRIMARY KEY (id)
  }),
  (Person)-[KNOWS: KnowsLabel {
    since INT64,
    PRIMARY KEY (SOURCE_PRIMARY_KEY, since, DESTINATION_PRIMARY_KEY)
  }]->(Person)
};

-- Use the graph
USE GRAPH my_graph;
```

### Dropping a Graph

```sql
DROP GRAPH my_graph;
```

### Built-in Procedures

```sql
-- Create a test graph with sample data
CALL create_test_graph('test_graph');

-- Create a test graph with specific number of vertices
CALL create_test_graph_data('my_graph', 10);

-- Show available procedures
CALL show_procedures();

-- Show current graph
CALL show_graph();
```

## Examples

### Social Network Query

```sql
-- Create test data
CALL create_test_graph_data('social', 20);
USE GRAPH social;

-- Find all persons and their friends (including those without friends)
MATCH (p:Person)
OPTIONAL MATCH (p)-[:FRIEND]->(friend:Person)
RETURN p.name, friend.name;

-- Find persons who work at a company, optionally with their friends
MATCH (p:Person)-[:WORKS_AT]->(c:Company)
OPTIONAL MATCH (p)-[f:FRIEND]->(friend:Person)
RETURN p.name, c.name, friend.name;
```

### Financial Transaction Analysis (FinBench)

```sql
-- TSR2: Complex read with multiple OPTIONAL MATCH
MATCH (n:Account{id:12}) RETURN n
NEXT
OPTIONAL MATCH (n)-[e:transfer]->(m:Account)
WHERE e.ts > 45 AND e.ts < 50
RETURN n, SUM(e.amount) AS sumEdge1Amount, MAX(e.amount) AS maxEdge1Amount, COUNT(e) AS numEdge1
NEXT
OPTIONAL MATCH (n)<-[e:transfer]-(m:Account)
WHERE e.ts > 0 AND e.ts < 100
RETURN sumEdge1Amount, maxEdge1Amount, numEdge1, SUM(e.amount) AS sumEdge2Amount;
```

### Path Queries

```sql
-- Find paths of length 1-3
MATCH p=(n:Person)-[r:KNOWS]->{1,3}(m:Person)
RETURN p;

-- With specific length
MATCH (n:Person)-[r:KNOWS]->{2}(m:Person)
RETURN n.name, m.name;

-- Variable length range
MATCH (n:Person)-[r:KNOWS]->{1,5}(m:Person)
RETURN n.name, m.name LIMIT 10;
```

## Notes

- miniGU uses in-memory storage by default
- All queries are case-sensitive for labels and property names
- The `NEXT` keyword separates multiple query statements in a batch