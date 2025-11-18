#!/usr/bin/env python3

import os
import re
from pathlib import Path

BENCHMARK_DIR = "/Users/colin/dev/Safebound_zxy/SafeBound/benchmark"
OUTPUT_DIR = "/Users/colin/dev/miniGU/experiment"
BINCODE_PATH = "/Users/colin/dev/pathce/gcard_es/statistic_graph_sf0.1_safebound.bincode"

HEADER_LINES = [
    'call import("ldbc", "/Users/colin/dev/pathce/datasets/ldbc/sf0.1", "/Users/colin/dev/pathce/datasets/ldbc/sf0.1/schema.json")',
    'session set graph ldbc'
]


def get_json_files(folder_path, folder_name):
    json_files = []
    pattern = re.compile(rf"^{re.escape(folder_name)}_p(\d+)\.json$")
    
    for file in os.listdir(folder_path):
        match = pattern.match(file)
        if match:
            number = int(match.group(1))
            json_files.append((number, file))
    
    json_files.sort(key=lambda x: x[0])
    return [f[1] for f in json_files]


def generate_test_file(folder_name, json_files):
    if not json_files:
        print(f"警告: {folder_name} 文件夹中没有找到 JSON 文件，跳过")
        return
    
    output_file = os.path.join(OUTPUT_DIR, f"{folder_name}.txt")
    folder_path = os.path.join(BENCHMARK_DIR, folder_name)
    
    with open(output_file, 'w') as f:
        for line in HEADER_LINES:
            f.write(line + '\n')
        
        for json_file in json_files:
            json_path = os.path.join(folder_path, json_file)
            query_line = f'call gcard_query("{BINCODE_PATH}", "{json_path}", 2, 2000)'
            f.write(query_line + '\n')
    
    print(f"已生成: {output_file} (包含 {len(json_files)} 个查询)")


def main():
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    
    if not os.path.exists(BENCHMARK_DIR):
        print(f"错误: benchmark 目录不存在: {BENCHMARK_DIR}")
        return

    folders = [f for f in os.listdir(BENCHMARK_DIR) 
               if os.path.isdir(os.path.join(BENCHMARK_DIR, f))]
    
    folders.sort()
    
    print(f"找到 {len(folders)} 个文件夹")
    
    for folder_name in folders:
        folder_path = os.path.join(BENCHMARK_DIR, folder_name)
        json_files = get_json_files(folder_path, folder_name)
        generate_test_file(folder_name, json_files)
    
    print("\n完成！")


if __name__ == "__main__":
    main()
