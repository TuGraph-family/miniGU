use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use csv::{ReaderBuilder, WriterBuilder};
use minigu_common::data_type::LogicalType;
use minigu_context::procedure::Procedure;

/// 转换LDBC dynamic目录下的CSV文件格式的Procedure
/// 
/// 对于dynamic目录下的所有CSV文件：
/// - 顶点文件：将creationDate列移到最后一列（如果存在）
/// - 边文件：添加id列作为第一列，将creationDate列移到最后一列
/// 
/// 同时确保边类型唯一：如果边类型有歧义，会修改目录名使其包含完整的 src_edge_dst 信息
/// 
/// 参数：
/// - dataset_dir: LDBC数据目录路径（包含initial_snapshot子目录）
pub fn build_procedure() -> Procedure {
    let parameters = vec![LogicalType::String];
    Procedure::new(parameters, None, move |_context, args| {
        let dataset_dir = args[0]
            .try_as_string()
            .expect("arg must be a string")
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("dataset dir cannot be null"))?
            .to_string();
        
        convert_ldbc_dynamic_csv(&dataset_dir)?;
        
        Ok(vec![])
    })
}

/// 转换LDBC dynamic目录下的所有CSV文件
fn convert_ldbc_dynamic_csv(
    dataset_dir: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let dataset_path = Path::new(dataset_dir);
    let initial_snapshot_dir = dataset_path.join("initial_snapshot");
    
    if !initial_snapshot_dir.is_dir() {
        return Err(anyhow::anyhow!(
            "initial_snapshot directory does not exist: {}", 
            initial_snapshot_dir.display()
        ).into());
    }
    
    // 只处理dynamic目录（dynamic目录下的文件都有creationDate）
    let dynamic_dir = initial_snapshot_dir.join("dynamic");
    
    if dynamic_dir.is_dir() {
        process_dynamic_directory(&dynamic_dir)?;
    }
    
    Ok(())
}

/// 处理dynamic目录下的所有CSV文件（顶点和边）
fn process_dynamic_directory(
    input_dir: &Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 第一遍：收集所有文件，区分顶点和边
    let mut edge_type_map: HashMap<String, Vec<(String, PathBuf)>> = HashMap::new();
    let mut vertex_dirs: Vec<(String, PathBuf)> = Vec::new();
    
    // 遍历所有子目录，区分顶点和边
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            let dir_name = path.file_name()
                .and_then(|s| s.to_str())
                .ok_or_else(|| anyhow::anyhow!("Invalid directory name: {}", path.display()))?
                .to_string();
            
            if let Some(_csv_path) = find_csv_in_directory(&path)? {
                if dir_name.contains('_') {
                    // 边文件：目录名包含下划线，格式为 src_edgeType_dst
                    let parts: Vec<&str> = dir_name.split('_').collect();
                    if parts.len() >= 3 {
                        let edge_type = parts[1..parts.len()-1].join("");
                        edge_type_map
                            .entry(edge_type)
                            .or_insert_with(Vec::new)
                            .push((dir_name.clone(), path.clone()));
                    } else {
                        // 下划线少于3个，可能是顶点文件
                        vertex_dirs.push((dir_name, path));
                    }
                } else {
                    // 顶点文件：目录名不包含下划线
                    vertex_dirs.push((dir_name, path));
                }
            }
        }
    }
    
    // 处理顶点文件：将creationDate移到最后一列
    for (label, vertex_dir) in vertex_dirs {
        if let Some(csv_path) = find_csv_in_directory(&vertex_dir)? {
            let temp_path = csv_path.with_extension("csv.tmp");
            convert_vertex_csv(&csv_path, &temp_path)?;
            fs::rename(&temp_path, &csv_path)?;
        }
    }
    
    // 处理边文件：添加id列，将creationDate移到最后一列，并处理边类型唯一性
    process_edge_files_with_renaming(edge_type_map)?;
    
    Ok(())
}

/// 处理边文件，包括重命名以确保边类型唯一
fn process_edge_files_with_renaming(
    edge_type_map: HashMap<String, Vec<(String, PathBuf)>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 处理每个边类型组
    for (edge_type, dirs) in edge_type_map.iter() {
        if dirs.len() > 1 {
            // 边类型有歧义，需要为除第一个外的其他目录添加编号
            // 例如：Comment_hasTag_Tag 和 Post_hasTag_Tag 都有 hasTag
            // 结果：Comment_hasTag_Tag (保持原样) 和 Post_hasTag1_Tag (添加编号)
            for (index, (dir_name, edge_dir)) in dirs.iter().enumerate() {
                let parts: Vec<&str> = dir_name.split('_').collect();
                if parts.len() >= 3 {
                    let src_label = parts[0];
                    let dst_label = parts[parts.len() - 1];
                    let edge_type_part = &parts[1..parts.len()-1];
                    
                    // 构建新的目录名：src_edgeType{编号}_dst
                    // 使用 join("") 保持与检测逻辑一致，避免引入额外的下划线
                    let new_dir_name = if index == 0 {
                        // 第一个保持原样，不添加编号
                        dir_name.clone()
                    } else {
                        // 其他的添加编号，确保边类型唯一
                        // 例如：hasTag -> hasTag1, hasTag2, ...
                        let edge_type_with_num = format!("{}{}", edge_type_part.join(""), index);
                        format!("{}_{}_{}", src_label, edge_type_with_num, dst_label)
                    };
                    
                    // 决定目标目录
                    let parent = edge_dir.parent().unwrap();
                    let target_dir = if new_dir_name != *dir_name {
                        // 需要创建新目录
                        let new_dir_path = parent.join(&new_dir_name);
                        fs::create_dir_all(&new_dir_path)?;
                        new_dir_path
                    } else {
                        // 使用原目录
                        edge_dir.clone()
                    };
                    
                    // 查找CSV文件并转换
                    if let Some(csv_path) = find_csv_in_directory(edge_dir)? {
                        let csv_filename = csv_path.file_name().unwrap();
                        let target_csv_path = target_dir.join(csv_filename);
                        
                        // 如果目标目录不同，需要转换并复制到新目录
                        if target_dir != *edge_dir {
                            // 转换文件到新目录
                            convert_edge_csv(&csv_path, &target_csv_path)?;
                            
                            // 删除旧目录及其所有内容
                            fs::remove_dir_all(edge_dir)?;
                        } else {
                            // 原地替换
                            let temp_path = csv_path.with_extension("csv.tmp");
                            convert_edge_csv(&csv_path, &temp_path)?;
                            fs::rename(&temp_path, &csv_path)?;
                        }
                    } else if target_dir != *edge_dir {
                        // 如果没有找到CSV文件，但需要重命名，也删除旧目录
                        fs::remove_dir_all(edge_dir)?;
                    }
                }
            }
        } else {
            // 边类型唯一，直接处理
            let (dir_name, edge_dir) = &dirs[0];
            
            // 查找CSV文件并转换（原地替换）
            if let Some(csv_path) = find_csv_in_directory(edge_dir)? {
                let temp_path = csv_path.with_extension("csv.tmp");
                convert_edge_csv(&csv_path, &temp_path)?;
                fs::rename(&temp_path, &csv_path)?;
            }
        }
    }
    
    Ok(())
}

/// 在目录中查找CSV文件
fn find_csv_in_directory(dir: &Path) -> Result<Option<PathBuf>, Box<dyn std::error::Error + Send + Sync>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            let filename = path.file_name()
                .and_then(|s| s.to_str())
                .ok_or_else(|| anyhow::anyhow!("Invalid filename: {}", path.display()))?;
            
            if filename != "_SUCCESS" && path.extension().and_then(|s| s.to_str()) == Some("csv") {
                return Ok(Some(path));
            }
        }
    }
    
    Ok(None)
}

/// 转换顶点CSV文件：将creationDate移到最后一列
/// 输入格式：id|prop1|prop2|...|creationDate 或 creationDate|id|prop1|prop2|...
/// 输出格式：id|prop1|prop2|...|creationDate
fn convert_vertex_csv(
    input_path: &Path,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut header_reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'|')
        .from_path(input_path)?;
    
    let headers = header_reader.headers()?.iter().collect::<Vec<_>>();
    if headers.is_empty() {
        return Err(anyhow::anyhow!("Empty CSV file: {}", input_path.display()).into());
    }
    
    // 查找creationDate列的位置
    let creation_date_index = headers.iter()
        .position(|h| h.to_lowercase() == "creationdate" || h.to_lowercase() == "creation_date")
        .ok_or_else(|| anyhow::anyhow!(
            "creationDate column not found in vertex file: {}", 
            input_path.display()
        ))?;
    
    // 构建新的headers：除了creationDate之外的所有列，最后加上creationDate
    let mut new_headers = Vec::new();
    for (i, header) in headers.iter().enumerate() {
        if i != creation_date_index {
            new_headers.push(header.to_string());
        }
    }
    new_headers.push(headers[creation_date_index].to_string());
    
    // 创建writer
    let mut writer = WriterBuilder::new()
        .has_headers(true)
        .delimiter(b'|')
        .from_path(output_path)?;
    
    writer.write_record(&new_headers)?;
    
    // 重新创建reader来读取数据行
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'|')
        .from_path(input_path)?;
    
    // 处理每一行数据
    for result in reader.records() {
        let record = result?;
        
        if record.len() != headers.len() {
            return Err(anyhow::anyhow!(
                "Row column count mismatch: expected {}, got {} in file {}",
                headers.len(),
                record.len(),
                input_path.display()
            ).into());
        }
        
        // 构建新行：除了creationDate之外的所有列，最后加上creationDate
        let mut new_record = Vec::new();
        for (i, field) in record.iter().enumerate() {
            if i != creation_date_index {
                new_record.push(field.to_string());
            }
        }
        new_record.push(record.get(creation_date_index).unwrap().to_string());
        
        writer.write_record(&new_record)?;
    }
    
    writer.flush()?;
    Ok(())
}

/// 转换单个边CSV文件
/// 输入格式：creationDate|src_id|dst_id|prop1|prop2|...
/// 输出格式：id|src_id|dst_id|prop1|prop2|...|creationDate
fn convert_edge_csv(
    input_path: &Path,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 先读取headers
    let mut header_reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'|')
        .from_path(input_path)?;
    
    let headers = header_reader.headers()?.iter().collect::<Vec<_>>();
    if headers.is_empty() {
        return Err(anyhow::anyhow!("Empty CSV file: {}", input_path.display()).into());
    }
    
    // 第一列应该是creationDate，后面是src_id, dst_id, 然后是属性
    if headers.len() < 3 {
        return Err(anyhow::anyhow!(
            "Edge CSV file must have at least 3 columns: {}", 
            input_path.display()
        ).into());
    }
    
    let creation_date_header = headers[0];
    let src_id_header = headers[1];
    let dst_id_header = headers[2];
    let prop_headers = &headers[3..];
    let header_count = headers.len();
    
    // 创建writer
    let mut writer = WriterBuilder::new()
        .has_headers(true)
        .delimiter(b'|')
        .from_path(output_path)?;
    
    // 写入新的header：id|src_id|dst_id|prop1|prop2|...|creationDate
    let mut new_headers = vec!["id".to_string()];
    new_headers.push(src_id_header.to_string());
    new_headers.push(dst_id_header.to_string());
    new_headers.extend(prop_headers.iter().map(|s| s.to_string()));
    new_headers.push(creation_date_header.to_string());
    writer.write_record(&new_headers)?;
    
    // 重新创建reader来读取数据行
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'|')
        .from_path(input_path)?;
    
    // 处理每一行数据
    let mut edge_id = 1u64;
    for result in reader.records() {
        let record = result?;
        
        if record.len() != header_count {
            return Err(anyhow::anyhow!(
                "Row column count mismatch: expected {}, got {} in file {}",
                header_count,
                record.len(),
                input_path.display()
            ).into());
        }
        
        let creation_date = record.get(0).unwrap();
        let src_id = record.get(1).unwrap();
        let dst_id = record.get(2).unwrap();
        let props: Vec<&str> = record.iter().skip(3).collect();
        
        // 构建新行：id|src_id|dst_id|prop1|prop2|...|creationDate
        let mut new_record = vec![edge_id.to_string()];
        new_record.push(src_id.to_string());
        new_record.push(dst_id.to_string());
        new_record.extend(props.iter().map(|s| s.to_string()));
        new_record.push(creation_date.to_string());
        
        writer.write_record(&new_record)?;
        edge_id += 1;
    }
    
    writer.flush()?;
    Ok(())
}
