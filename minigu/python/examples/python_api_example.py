#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
miniGU Python API 使用示例
展示如何使用miniGU Python API进行基本的数据库操作
"""

import sys
import os
import tempfile

# 添加项目路径以便导入
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

import minigu.python.minigu as minigu


def basic_workflow():
    """演示基本工作流程"""
    print("=== 基本工作流程 ===")
    
    # 1. 连接到数据库
    print("1. 连接到数据库...")
    with minigu.connect() as db:
        # 2. 加载数据
        print("2. 加载数据...")
        sample_data = [
            {"name": "Alice", "age": 30, "label": "Person"},
            {"name": "Bob", "age": 25, "label": "Person"},
            {"name": "Charlie", "age": 35, "label": "Person"}
        ]
        db.load(sample_data)
        
        # 3. 执行查询
        print("3. 执行查询...")
        result = db.execute("MATCH (n:Person) RETURN n.name, n.age")
        print(f"查询结果: {result}")
        
        # 4. 保存数据库
        print("4. 保存数据库...")
        db.save("example_database.mgu")
        print("数据库已保存到 example_database.mgu")


def data_loading_examples():
    """数据加载示例"""
    print("\n=== 数据加载示例 ===")
    
    with minigu.connect() as db:
        # 从字典列表加载数据
        people_data = [
            {"name": "David", "age": 28, "city": "Beijing"},
            {"name": "Eva", "age": 32, "city": "Shanghai"},
            {"name": "Frank", "age": 29, "city": "Guangzhou"}
        ]
        
        print("从字典列表加载数据...")
        db.load(people_data)
        
        # 从文件加载数据 (模拟)
        print("从文件加载数据...")
        # db.load("data.json")  # 如果有实际文件的话






def advanced_queries():
    """演示高级查询功能"""
    print("\n=== 高级查询功能 ===")
    
    # 连接到数据库
    with minigu.connect() as db:
        # 执行简单的查询
        print("1. 执行简单查询...")
        result = db.execute("MATCH (n) RETURN n LIMIT 5;")
        print(f"   查询结果: {result}")
        print(f"   行数: {result.row_count}")
        
        # 查看查询指标
        print("2. 查询指标:")
        for key, value in result.metrics.items():
            print(f"   {key}: {value}")
        
        # 将结果转换为字典列表
        print("3. 结果转换为字典列表:")
        dict_list = result.to_list()
        for i, row in enumerate(dict_list):
            print(f"   行 {i}: {row}")


def file_operations():
    """演示文件操作功能"""
    print("\n=== 文件操作功能 ===")
    
    # 从文件加载数据
    print("1. 从文件加载数据...")
    try:
        # 这里只是示例，实际使用时需要有对应的文件
        # db.load_from_file("sample_data.json")
        print("   从文件加载数据功能演示 (占位实现)")
    except Exception as e:
        print(f"   加载文件时出错: {e}")
    
    with minigu.connect() as db:
        # 加载一些示例数据
        sample_data = [
            {"name": "Alice", "age": 30, "label": "Person"},
            {"name": "Bob", "age": 25, "label": "Person"},
            {"name": "TechCorp", "founded": 2010, "label": "Company"}
        ]
        db.load(sample_data)
        
        # 保存到文件
        with tempfile.NamedTemporaryFile(suffix='.mgu', delete=False) as tmp:
            save_path = tmp.name
            
        try:
            print(f"1. 保存数据库到文件: {save_path}")
            db.save(save_path)
            
            if os.path.exists(save_path):
                size = os.path.getsize(save_path)
                print(f"   文件保存成功，大小: {size} 字节")
            
            # 从文件加载
            print("2. 从文件加载数据...")
            db.load(save_path)
            print("   数据加载成功")
            
        finally:
            # 清理临时文件
            if os.path.exists(save_path):
                os.unlink(save_path)


def main():
    """主函数"""
    print("miniGU Python API 示例")
    print("=" * 50)
    
    try:
        basic_workflow()
        advanced_queries()
        file_operations()
        
        print("\n所有示例执行完成!")
        
    except Exception as e:
        print(f"执行示例时出错: {e}")
        raise


if __name__ == "__main__":
    main()