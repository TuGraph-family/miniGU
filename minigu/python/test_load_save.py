#!/usr/bin/env python3

"""
测试miniGU的load和save功能
"""

try:
    # 尝试导入Rust绑定
    from minigu_python import PyMiniGU
    print("Successfully imported PyMiniGU")
    
    # 创建数据库实例
    db = PyMiniGU()
    print("Created PyMiniGU instance")
    
    # 测试load_data功能
    test_data = [
        {"name": "Alice", "age": 30, "label": "Person"},
        {"name": "Bob", "age": 25, "label": "Person"},
        {"name": "TechCorp", "founded": 2010, "label": "Company"}
    ]
    
    print("Testing load_data...")
    db.load_data(test_data)
    print("load_data test completed")
    
    # 测试save_to_file功能
    print("Testing save_to_file...")
    db.save_to_file("test_database.mgu")
    print("save_to_file test completed")
    
    # 测试load_from_file功能
    print("Testing load_from_file...")
    db.load_from_file("test_database.mgu")
    print("load_from_file test completed")
    
    print("All tests passed!")
    
except Exception as e:
    print(f"Error: {e}")
    import traceback
    traceback.print_exc()