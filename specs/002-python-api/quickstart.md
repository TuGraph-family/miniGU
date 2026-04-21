# Quickstart: Python API Bindings

**Feature**: 002-python-api
**Date**: 2026-04-17

## Installation

### From PyPI (推荐)

```bash
pip install minigu
```

### From Source

```bash
# 克隆仓库
git clone https://github.com/TuGraph-family/miniGU.git
cd miniGU

# 安装 maturin
pip install maturin

# 开发模式安装
cd minigu/python
maturin develop
```

## Quick Examples

### 1. 基本使用 (同步 API)

```python
import minigu

# 创建数据库实例
db = minigu.MiniGU()
db.init()

# 创建图
db.create_graph("social_network")

# 插入数据
db.execute("""
    INSERT VERTEX Person (name, age)
    VALUES 1:("Alice", 30), 2:("Bob", 25)
""")

db.execute("""
    INSERT EDGE knows (since)
    VALUES 1->2:(2020)
""")

# 查询数据
result = db.execute("""
    MATCH (a:Person)-[e:knows]->(b:Person)
    RETURN a.name, b.name, e.since
""")

# 遍历结果
for row in result:
    print(f"{row[0]} knows {row[1]} since {row[2]}")
```

### 2. 使用上下文管理器

```python
import minigu

with minigu.MiniGU() as db:
    db.create_graph("test_graph")
    result = db.execute("MATCH (n) RETURN n LIMIT 10")
    print(f"Found {len(result)} nodes")
```

### 3. 异步 API

```python
import asyncio
import minigu

async def main():
    async with minigu.AsyncMiniGU() as db:
        await db.create_graph("async_graph")

        # 并发执行多个查询
        results = await asyncio.gather(
            db.execute("MATCH (n) RETURN n.name"),
            db.execute("MATCH (n) RETURN n.age")
        )

        print(f"Names: {results[0].data}")
        print(f"Ages: {results[1].data}")

asyncio.run(main())
```

### 4. 错误处理

```python
import minigu
from minigu import QuerySyntaxError, GraphError

db = minigu.MiniGU()
db.init()

try:
    db.execute("INVALID GQL SYNTAX")
except QuerySyntaxError as e:
    print(f"语法错误: {e}")

try:
    db.create_graph("existing_graph")  # 图已存在
except GraphError as e:
    print(f"图操作错误: {e}")
```

### 5. 数据加载

```python
import minigu

db = minigu.MiniGU()
db.init()
db.create_graph("users")

# 从字典列表加载
data = [
    {"id": 1, "name": "Alice", "age": 30},
    {"id": 2, "name": "Bob", "age": 25}
]
db.load(data)

# 从 CSV 文件加载
db.load("/path/to/users.csv")

# 查询验证
result = db.execute("MATCH (n) RETURN n.name, n.age")
for row in result:
    print(row)
```

### 6. 数据持久化

```python
import minigu

# 创建并保存数据库
db = minigu.MiniGU()
db.init()
db.create_graph("persistent_graph")
db.execute('INSERT VERTEX Person (name) VALUES 1:("Charlie")')
db.save("/path/to/backup")

# 从备份恢复
db2 = minigu.MiniGU(db_path="/path/to/backup")
db2.init()
result = db2.execute("MATCH (n) RETURN n.name")
print(result.data)  # [['Charlie']]
```

## Common Patterns

### 批量插入

```python
import minigu

db = minigu.MiniGU()
db.init()
db.create_graph("batch_test")

# 批量插入顶点
vertices = [
    {"id": i, "name": f"Person_{i}", "age": 20 + i}
    for i in range(1000)
]
db.load(vertices)
```

### 结果处理

```python
import minigu

db = minigu.MiniGU()
db.init()
db.create_graph("result_test")

result = db.execute("MATCH (n:Person) RETURN n.name, n.age")

# 检查 schema
print("Schema:", result.schema)
# [{'name': 'n.name', 'data_type': 'String'}, {'name': 'n.age', 'data_type': 'Int64'}]

# 按索引访问
print("First row:", result[0])

# 遍历
for name, age in result:
    print(f"{name} is {age} years old")

# 转换为 Pandas DataFrame (如果安装了 pandas)
import pandas as pd
df = pd.DataFrame(result.data, columns=[f['name'] for f in result.schema])
```

## Next Steps

1. 阅读 [API 文档](./contracts/api.md) 了解完整 API
2. 查看 [数据模型](./data-model.md) 了解内部结构
3. 运行测试验证安装: `pytest minigu/python/test_minigu_api.py`