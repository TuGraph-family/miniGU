#!/usr/bin/env python3

"""
测试miniGU的真实load和save功能
"""

import tempfile
import os

try:
    # 尝试导入Rust绑定
    from minigu import connect, MiniGU
    print("Successfully imported miniGU")
    
    # 测试保存功能
    print("\n=== 测试保存功能 ===")
    with connect() as db:
        # 保存到临时文件
        with tempfile.NamedTemporaryFile(suffix='.mgu', delete=False) as tmp:
            tmp_path = tmp.name
        
        try:
            print(f"保存数据库到文件: {tmp_path}")
            db.save(tmp_path)
            print("保存成功!")
            
            # 检查文件是否存在且不为空
            if os.path.exists(tmp_path):
                size = os.path.getsize(tmp_path)
                print(f"文件大小: {size} 字节")
                if size > 0:
                    print("文件非空，保存功能正常工作")
                else:
                    print("警告: 文件为空")
            else:
                print("错误: 文件未创建")
        finally:
            # 清理临时文件
            if os.path.exists(tmp_path):
                os.unlink(tmp_path)
    
    # 测试加载功能
    print("\n=== 测试加载功能 ===")
    with connect() as db:
        # 创建一个测试文件
        with tempfile.NamedTemporaryFile(suffix='.mgu', delete=False) as tmp:
            tmp_path = tmp.name
            
        try:
            # 先保存一个文件
            db.save(tmp_path)
            
            # 然后尝试加载它
            print(f"从文件加载数据: {tmp_path}")
            db.load(tmp_path)
            print("加载成功!")
            
        except Exception as e:
            print(f"加载过程中出现错误: {e}")
        finally:
            # 清理临时文件
            if os.path.exists(tmp_path):
                os.unlink(tmp_path)
    
    # 测试从数据加载功能
    print("\n=== 测试从数据加载功能 ===")
    with connect() as db:
        # 准备测试数据
        test_data = [
            {"name": "Alice", "age": 30, "label": "Person"},
            {"name": "Bob", "age": 25, "label": "Person"},
            {"name": "TechCorp", "founded": 2010, "label": "Company"}
        ]
        
        print("从Python对象加载数据:")
        db.load(test_data)
        print("数据加载成功!")
        
        # 保存加载的数据
        with tempfile.NamedTemporaryFile(suffix='.mgu', delete=False) as tmp:
            tmp_path = tmp.name
            
        try:
            print(f"保存加载的数据到文件: {tmp_path}")
            db.save(tmp_path)
            print("数据保存成功!")
            
            # 检查文件大小
            size = os.path.getsize(tmp_path)
            print(f"文件大小: {size} 字节")
            
        finally:
            # 清理临时文件
            if os.path.exists(tmp_path):
                os.unlink(tmp_path)
    
    print("\n=== 所有测试完成 ===")
    
except Exception as e:
    print(f"测试过程中出现错误: {e}")
    import traceback
    traceback.print_exc()