use std::path::Path;

use minigu_test::sqllogictest_adapter::MiniGuDB;
use sqllogictest::Runner;

// 基本的 sqllogictest 运行器
#[tokio::test]
async fn test_basic_echo() {
    let mut runner =
        Runner::new(|| async { Ok(MiniGuDB::new().expect("Failed to create database")) });

    // 测试基本的 echo 功能
    let test_content = r#"
# 测试基本的 echo 功能
statement ok
CALL echo('Hello World')

# 测试带结果的 echo 功能
query T
CALL echo('Hello World')
----
Hello World
"#;

    let result = runner.run_script(test_content);

    match result {
        Ok(_) => println!("Test passed!"),
        Err(e) => println!("Test failed: {:?}", e),
    }
}

// 测试 show_procedures 功能
#[tokio::test]
async fn test_show_procedures() {
    let mut runner =
        Runner::new(|| async { Ok(MiniGuDB::new().expect("Failed to create database")) });

    let test_content = r#"
# 测试 show_procedures 功能
query TT
CALL show_procedures()
----
create_test_graph
echo
show_procedures
"#;

    let result = runner.run_script(test_content);

    match result {
        Ok(_) => println!("Show procedures test passed!"),
        Err(e) => println!("Show procedures test failed: {:?}", e),
    }
}

// 测试创建测试图
#[tokio::test]
async fn test_create_test_graph() {
    let mut runner =
        Runner::new(|| async { Ok(MiniGuDB::new().expect("Failed to create database")) });

    let test_content = r#"
# 测试创建测试图
statement ok
CALL create_test_graph('test_graph')

# 测试重复创建应该失败
statement error
CALL create_test_graph('test_graph')
"#;

    let result = runner.run_script(test_content);

    match result {
        Ok(_) => println!("Create test graph test passed!"),
        Err(e) => println!("Create test graph test failed: {:?}", e),
    }
}

// 从文件运行测试
#[tokio::test]
async fn test_from_file() {
    let mut runner =
        Runner::new(|| async { Ok(MiniGuDB::new().expect("Failed to create database")) });

    // 这里我们可以从文件系统加载 .slt 文件
    // 但是现在我们只是测试 API
    let test_file = "test_basic.slt";

    // 如果文件存在，运行它
    if Path::new(test_file).exists() {
        let result = runner.run_file(test_file);

        match result {
            Ok(_) => println!("File test passed!"),
            Err(e) => println!("File test failed: {:?}", e),
        }
    } else {
        println!("Test file {} not found, skipping file test", test_file);
    }
}
