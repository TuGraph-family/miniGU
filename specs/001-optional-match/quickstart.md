# Quickstart: OPTIONAL MATCH

**Feature**: 001-optional-match
**Date**: 2026-04-17

## Basic Usage

### Simple OPTIONAL MATCH

Find all persons and optionally their friends:

```sql
MATCH (p:Person)
OPTIONAL MATCH (p)-[e:KNOWS]->(f:Person)
RETURN p.name, f.name;
```

**Result**:
| p.name | f.name |
|--------|--------|
| Alice  | Bob    |
| Alice  | Carol  |
| Bob    | NULL   |
| Carol  | Dave   |

**Explanation**: Bob has no KNOWS relationships, so `f.name` is NULL for Bob's row.

### OPTIONAL MATCH with WHERE

Filter the optional pattern:

```sql
MATCH (p:Person)
OPTIONAL MATCH (p)-[e:TRANSFER]->(a:Account)
WHERE e.amount > 1000
RETURN p.name, a.id, e.amount;
```

**Result**:
| p.name | a.id  | e.amount |
|--------|-------|----------|
| Alice  | 123   | 1500     |
| Bob    | NULL  | NULL     |
| Carol  | NULL  | NULL     |

**Explanation**: Only Alice has transfers over 1000. Bob and Carol's rows are preserved with NULL values.

### OPTIONAL MATCH with Aggregation

Compute statistics over optional patterns:

```sql
MATCH (n:Account {id: 12})
OPTIONAL MATCH (n)-[e:transfer]->(m:Account)
WHERE e.ts > 45 AND e.ts < 50
RETURN
    n,
    sum(e.amount) as totalAmount,
    count(e) as numTransfers;
```

**Result**:
| n.id | totalAmount | numTransfers |
|------|-------------|--------------|
| 12   | 5000.00     | 3            |

**Note**: If Account 12 had no matching transfers, `totalAmount` would be NULL and `numTransfers` would be 0.

## Chained OPTIONAL MATCH (via NEXT)

Multiple OPTIONAL MATCH clauses in sequence:

```sql
-- First, get the account
MATCH (n:Account {id: 12}) RETURN n
NEXT
-- Optionally match outgoing transfers
OPTIONAL MATCH (n)-[e:transfer]->(m:Account)
WHERE e.ts > 45 AND e.ts < 50
RETURN
    sum(e.amount) as sumOutAmount,
    max(e.amount) as maxOutAmount,
    count(e) as numOut
NEXT
-- Optionally match incoming transfers
OPTIONAL MATCH (n)<-[e:transfer]-(m:Account)
WHERE e.ts > 0 AND e.ts < 100
RETURN
    sumOutAmount,
    maxOutAmount,
    numOut,
    sum(e.amount) as sumInAmount,
    max(e.amount) as maxInAmount,
    count(e) as numIn;
```

**Result**:
| sumOutAmount | maxOutAmount | numOut | sumInAmount | maxInAmount | numIn |
|--------------|--------------|--------|-------------|-------------|-------|
| 5000.00      | 2000.00      | 3      | 8000.00     | 3000.00     | 4     |

## NULL Handling

### NULL in Expressions

```sql
OPTIONAL MATCH (p:Person)-[e:KNOWS]->(f:Person)
RETURN
    p.name,
    CASE WHEN f.name IS NULL THEN 'No friends' ELSE f.name END as friend;
```

**Result**:
| p.name | friend    |
|--------|-----------|
| Alice  | Bob       |
| Bob    | No friends|

### NULL in Aggregations

| Function | NULL Input Result |
|----------|-------------------|
| COUNT(*) | Count of all rows |
| COUNT(x) | Count of non-NULL values |
| SUM(x)   | NULL if all NULL, else sum |
| AVG(x)   | NULL if all NULL, else average |
| MAX(x)   | NULL if all NULL, else maximum |
| MIN(x)   | NULL if all NULL, else minimum |

## Common Patterns

### Pattern 1: Check if relationship exists

```sql
MATCH (p:Person)
OPTIONAL MATCH (p)-[e:IS_ADMIN]->(a:Admin)
RETURN p.name, CASE WHEN a IS NOT NULL THEN true ELSE false END as isAdmin;
```

### Pattern 2: Count optional relationships

```sql
MATCH (p:Person)
OPTIONAL MATCH (p)-[e:KNOWS]->(f:Person)
RETURN p.name, count(f) as friendCount;
```

### Pattern 3: Optional pattern with multiple hops

```sql
MATCH (p:Person)
OPTIONAL MATCH (p)-[e1:KNOWS]->(f:Person)-[e2:WORKS_AT]->(c:Company)
RETURN p.name, f.name as friend, c.name as company;
```

## Error Cases

### Invalid: OPTIONAL without preceding MATCH

```sql
-- This will fail
OPTIONAL MATCH (p:Person)-[e]->(f)
RETURN p, f;
```

**Error**: OPTIONAL MATCH requires a preceding MATCH clause to provide the "left" side of the join.

### Valid: Variables from previous clause

```sql
-- This is valid
MATCH (p:Person)
OPTIONAL MATCH (p)-[e]->(f)  -- Uses p from previous MATCH
RETURN p, f;
```

## Performance Tips

1. **Filter early**: Use WHERE clauses in OPTIONAL MATCH to reduce the right-side data
2. **Index properties**: Create indexes on frequently filtered properties
3. **Avoid deep chaining**: Multiple OPTIONAL MATCH clauses can increase complexity