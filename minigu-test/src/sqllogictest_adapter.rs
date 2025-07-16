use std::fmt;
use std::sync::Arc;

use async_trait::async_trait;
use minigu::database::{Database, DatabaseConfig};
use minigu::error::Error as MiniGuError;
use sqllogictest::{AsyncDB, DBOutput, DefaultColumnType};

/// 用于 SQLLogicTest 的 MiniGU 数据库适配器
pub struct MiniGuDB {
    database: Arc<Database>,
}

impl MiniGuDB {
    /// 创建新的 MiniGU 数据库适配器
    pub fn new() -> Result<Self, MiniGuError> {
        let config = DatabaseConfig::default();
        let database = Arc::new(Database::open_in_memory(&config)?);
        Ok(Self { database })
    }
}

impl Default for MiniGuDB {
    fn default() -> Self {
        Self::new().expect("Failed to create MiniGU database")
    }
}

/// 错误包装器，用于兼容 SQLLogicTest 的错误处理
#[derive(Debug)]
pub struct SqlLogicTestError(MiniGuError);

impl fmt::Display for SqlLogicTestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MiniGU Error: {}", self.0)
    }
}

impl std::error::Error for SqlLogicTestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl From<MiniGuError> for SqlLogicTestError {
    fn from(error: MiniGuError) -> Self {
        SqlLogicTestError(error)
    }
}

#[async_trait]
impl AsyncDB for MiniGuDB {
    type ColumnType = DefaultColumnType;
    type Error = SqlLogicTestError;

    async fn run(&mut self, sql: &str) -> Result<DBOutput<Self::ColumnType>, Self::Error> {
        // 创建新的会话
        let mut session = self.database.session().map_err(SqlLogicTestError::from)?;

        // 执行查询
        let result = session.query(sql).map_err(SqlLogicTestError::from)?;

        // 检查是否有结果集
        if let Some(schema) = result.schema() {
            let mut records = Vec::new();
            let mut types = Vec::new();

            // 为每个字段创建列类型
            for _field in schema.fields() {
                types.push(DefaultColumnType::Any);
            }

            // 转换数据行
            for chunk in result.iter() {
                for row in chunk.rows() {
                    let mut row_values = Vec::new();
                    for col_index in 0..schema.fields().len() {
                        let value = row
                            .get(col_index)
                            .unwrap_or(minigu::common::value::ScalarValue::Null);
                        let str_value = convert_scalar_value_to_string(&value);
                        row_values.push(str_value);
                    }
                    records.push(row_values);
                }
            }

            Ok(DBOutput::Rows {
                types,
                rows: records,
            })
        } else {
            // 没有结果集，返回语句完成
            Ok(DBOutput::StatementComplete(0))
        }
    }

    async fn shutdown(&mut self) {
        // MiniGU 的内存数据库不需要特殊的关闭操作
    }
}

/// 将 ScalarValue 转换为字符串
fn convert_scalar_value_to_string(value: &minigu::common::value::ScalarValue) -> String {
    use minigu::common::value::ScalarValue;

    match value {
        ScalarValue::Null => "NULL".to_string(),
        ScalarValue::Boolean(Some(b)) => b.to_string(),
        ScalarValue::Boolean(None) => "NULL".to_string(),
        ScalarValue::Int8(Some(i)) => i.to_string(),
        ScalarValue::Int8(None) => "NULL".to_string(),
        ScalarValue::Int16(Some(i)) => i.to_string(),
        ScalarValue::Int16(None) => "NULL".to_string(),
        ScalarValue::Int32(Some(i)) => i.to_string(),
        ScalarValue::Int32(None) => "NULL".to_string(),
        ScalarValue::Int64(Some(i)) => i.to_string(),
        ScalarValue::Int64(None) => "NULL".to_string(),
        ScalarValue::UInt8(Some(i)) => i.to_string(),
        ScalarValue::UInt8(None) => "NULL".to_string(),
        ScalarValue::UInt16(Some(i)) => i.to_string(),
        ScalarValue::UInt16(None) => "NULL".to_string(),
        ScalarValue::UInt32(Some(i)) => i.to_string(),
        ScalarValue::UInt32(None) => "NULL".to_string(),
        ScalarValue::UInt64(Some(i)) => i.to_string(),
        ScalarValue::UInt64(None) => "NULL".to_string(),
        ScalarValue::Float32(Some(f)) => f.to_string(),
        ScalarValue::Float32(None) => "NULL".to_string(),
        ScalarValue::Float64(Some(f)) => f.to_string(),
        ScalarValue::Float64(None) => "NULL".to_string(),
        ScalarValue::String(Some(s)) => s.clone(),
        ScalarValue::String(None) => "NULL".to_string(),
        ScalarValue::Vertex(Some(v)) => format!("{:?}", v),
        ScalarValue::Vertex(None) => "NULL".to_string(),
        ScalarValue::Edge(Some(e)) => format!("{:?}", e),
        ScalarValue::Edge(None) => "NULL".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_functionality() {
        println!("=== 测试基本功能 ===");

        // 创建数据库适配器
        let mut db = MiniGuDB::new().expect("Failed to create database");
        println!("✓ 数据库适配器创建成功");

        // 测试最简单的语句：创建一个测试图
        // 根据 create_test_graph.rs，这个过程没有 schema，应该被认为是目录过程
        println!("测试目录过程: CALL create_test_graph('test')");
        let result = db.run("CALL create_test_graph('test')").await;
        match result {
            Ok(_output) => {
                println!("✓ create_test_graph 成功！");
            }
            Err(e) => {
                println!("✗ create_test_graph 失败: {:?}", e);

                // 如果 create_test_graph 也失败了，那说明问题是根本性的
                // 让我们检查一下是否有任何过程可用
                println!("检查过程列表...");
                let show_result = db.run("CALL show_procedures()").await;
                match show_result {
                    Ok(_) => println!("✓ show_procedures 工作正常"),
                    Err(e2) => println!("✗ show_procedures 也失败: {:?}", e2),
                }

                panic!("测试失败: {:?}", e);
            }
        }
    }
}
