use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use csv::{ReaderBuilder, WriterBuilder};
use minigu_common::data_type::LogicalType;
use minigu_context::procedure::Procedure;
use serde_json::{json, Value};

/// generate ldbc manifest in procedure.
pub fn build_procedure() -> Procedure {
    let parameters = vec![LogicalType::String, LogicalType::String];
    Procedure::new(parameters, None, move |_context, args| {
        let dataset_dir = args[0]
            .try_as_string()
            .expect("arg must be a string")
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("dataset dir cannot be null"))?
            .to_string();
        let manifest_output = args[1]
            .try_as_string()
            .expect("arg must be a string")
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("manifest output path cannot be null"))?
            .to_string();
        
        let manifest_path = build_manifest_from_ldbc(&dataset_dir, &manifest_output)?;
        
        Ok(vec![])
    })
}

/// 从LDBC数据目录的initial_snapshot构造manifest.json文件
/// 
/// LDBC数据格式：
/// - 目录结构：`{ldbc_dir}/initial_snapshot/dynamic/` 和 `{ldbc_dir}/initial_snapshot/static/`
/// - 每个实体类型是一个子目录，CSV文件在子目录中（如 `Person/part-00000-xxx.csv`）
/// - 顶点目录：`Comment`, `Forum`, `Person`, `Post`, `Organisation`, `Place`, `Tag`, `TagClass`
/// - 边目录：`Comment_hasTag_Tag`, `Forum_hasMember_Person`, `Person_knows_Person` 等
/// 
/// 参数：
/// - `ldbc_dir`: LDBC数据目录路径（应该包含initial_snapshot子目录）
/// - `manifest_output`: 生成的manifest.json文件路径
/// 
/// 返回：
/// - manifest.json文件的路径
fn build_manifest_from_ldbc(
    ldbc_dir: &str,
    manifest_output: &str,
) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    let ldbc_dir = Path::new(ldbc_dir);
    let initial_snapshot_dir = ldbc_dir.join("initial_snapshot");
    
    if !initial_snapshot_dir.is_dir() {
        return Err(anyhow::anyhow!(
            "initial_snapshot directory does not exist: {}", 
            initial_snapshot_dir.display()
        ).into());
    }
    
    // 1. 扫描dynamic和static目录，找到所有CSV文件
    let mut vertex_files: Vec<(String, PathBuf)> = Vec::new();
    let mut edge_files: Vec<(String, String, String, PathBuf)> = Vec::new();
    
    // 处理dynamic目录
    let dynamic_dir = initial_snapshot_dir.join("dynamic");
    if dynamic_dir.is_dir() {
        scan_ldbc_directory(&dynamic_dir, &mut vertex_files, &mut edge_files)?;
    }
    
    // 处理static目录
    let static_dir = initial_snapshot_dir.join("static");
    if static_dir.is_dir() {
        scan_ldbc_directory(&static_dir, &mut vertex_files, &mut edge_files)?;
    }
    
    // 2. 读取CSV文件，推断schema并创建Manifest JSON
    let mut manifest_vertices: Vec<Value> = Vec::new();
    let mut manifest_edges: Vec<Value> = Vec::new();
    
    // 处理顶点文件
    for (label, path) in vertex_files {
        let properties = infer_properties_from_csv(&path)?;
        
        // 使用相对于ldbc_dir的相对路径
        let relative_path = path.strip_prefix(ldbc_dir)
            .unwrap_or(&path)
            .to_str()
            .unwrap()
            .to_string();
        
        manifest_vertices.push(json!({
            "label": label,
            "file": {
                "path": relative_path,
                "format": "csv"
            },
            "properties": properties
        }));
    }

    // 处理边文件
    for (edge_label, src_label, dst_label, path) in edge_files {
        let properties = infer_properties_from_csv_edge(&path)?;
        
        // 使用相对于ldbc_dir的相对路径
        let relative_path = path.strip_prefix(ldbc_dir)
            .unwrap_or(&path)
            .to_str()
            .unwrap()
            .to_string();
        
        manifest_edges.push(json!({
            "label": edge_label,
            "src_label": src_label,
            "dst_label": dst_label,
            "file": {
                "path": relative_path,
                "format": "csv"
            },
            "properties": properties
        }));
    }
    
    let manifest_json = json!({
        "vertices": manifest_vertices,
        "edges": manifest_edges
    });
    
    let manifest_file_path = Path::new(manifest_output);
    
    if let Some(parent) = manifest_file_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    fs::write(
        &manifest_file_path,
        serde_json::to_string_pretty(&manifest_json)?,
    )?;
    
    Ok(manifest_file_path.to_path_buf())
}

fn scan_ldbc_directory(
    dir: &Path,
    vertex_files: &mut Vec<(String, PathBuf)>,
    edge_files: &mut Vec<(String, String, String, PathBuf)>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            let dir_name = path.file_name()
                .and_then(|s| s.to_str())
                .ok_or_else(|| anyhow::anyhow!("Invalid directory name: {}", path.display()))?;

            if let Some(csv_path) = find_csv_in_directory(&path)? {
                // 特殊处理：Organisation 表需要拆分为 Company 和 University
                if dir_name == "Organisation" {
                    split_organisation_table(&csv_path, &path, vertex_files)?;
                } else if dir_name.contains('_') {
                    // 边文件格式：src_edgeType_dst 或 src_edgeType{编号}_dst
                    let parts: Vec<&str> = dir_name.split('_').collect();
                    if parts.len() >= 3 {
                        let src_label = parts[0].to_string();
                        // 使用 join("") 与 convert_ldbc_edges.rs 保持一致
                        // 这样可以正确处理重命名后的边类型（如 hasTag1）
                        let edge_label = parts[1..parts.len()-1].join("");
                        let dst_label = parts[parts.len()-1].to_string();
                        edge_files.push((edge_label, src_label, dst_label, csv_path));
                    } else {
                        // 如果下划线少于3个，可能是顶点文件
                        vertex_files.push((dir_name.to_string(), csv_path));
                    }
                } else {
                    // 没有下划线，是顶点文件
                    vertex_files.push((dir_name.to_string(), csv_path));
                }
            }
        }
    }
    
    Ok(())
}

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

fn infer_properties_from_csv(
    csv_path: &Path,
) -> Result<Vec<Value>, Box<dyn std::error::Error + Send + Sync>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'|')
        .from_path(csv_path)?;
    
    let headers = rdr.headers()?.iter().collect::<Vec<_>>();
    if headers.is_empty() {
        return Ok(Vec::new());
    }
    
    let prop_headers = &headers[1..];
    
    let mut properties: Vec<Value> = Vec::new();
    
    let mut data_rdr = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'|')
        .from_path(csv_path)?;
    
    if let Some(result) = data_rdr.records().next() {
        let record = result?;
        for (header, value) in prop_headers.iter().zip(record.iter().skip(1)) {
            let (logical_type_str, nullable) = infer_type_from_value(value);
            properties.push(json!({
                "name": header,
                "logical_type": logical_type_str,
                "nullable": true
            }));
        }
    } else {
        for header in prop_headers {
            properties.push(json!({
                "name": header,
                "logical_type": "String",
                "nullable": true
            }));
        }
    }
    
    Ok(properties)
}

fn infer_properties_from_csv_edge(
    csv_path: &Path,
) -> Result<Vec<Value>, Box<dyn std::error::Error + Send + Sync>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'|')
        .from_path(csv_path)?;
    
    let headers = rdr.headers()?.iter().collect::<Vec<_>>();
    // 转换后的边文件格式：id|src_id|dst_id|prop1|prop2|...|creationDate
    // 所以前3列是 id, src_id, dst_id，属性从第4列开始，最后一列是 creationDate
    if headers.len() <= 3 {
        return Ok(Vec::new());
    }
    
    // 跳过前3列（id, src_id, dst_id）和最后一列（creationDate），只处理属性列
    let prop_headers = &headers[3..headers.len()-1];

    let mut properties: Vec<Value> = Vec::new();

    let mut data_rdr = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'|')
        .from_path(csv_path)?;
    
    if let Some(result) = data_rdr.records().next() {
        let record = result?;
        // 跳过前3列（id, src_id, dst_id），处理属性列，跳过最后一列（creationDate）
        let prop_values: Vec<&str> = record.iter().skip(3).take(prop_headers.len()).collect();
        for (header, value) in prop_headers.iter().zip(prop_values.iter()) {
            let (logical_type_str, nullable) = infer_type_from_value(value);
            properties.push(json!({
                "name": header,
                "logical_type": logical_type_str,
                "nullable": true
            }));
        }
    } else {
        for header in prop_headers {
            properties.push(json!({
                "name": header,
                "logical_type": "String",
                "nullable": true
            }));
        }
    }
    
    Ok(properties)
}

/// 拆分 Organisation 表为 Company 和 University 两个表
/// Organisation 表的格式：id|type|prop1|prop2|...
/// 根据第二列（type）的值拆分为两个表
fn split_organisation_table(
    csv_path: &Path,
    org_dir: &Path,
    vertex_files: &mut Vec<(String, PathBuf)>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'|')
        .from_path(csv_path)?;
    
    let headers = reader.headers()?.iter().collect::<Vec<_>>();
    if headers.len() < 2 {
        return Err(anyhow::anyhow!(
            "Organisation CSV file must have at least 2 columns (id and type): {}", 
            csv_path.display()
        ).into());
    }
    
    // 创建两个输出文件
    let company_dir = org_dir.parent().unwrap().join("Company");
    let university_dir = org_dir.parent().unwrap().join("University");
    fs::create_dir_all(&company_dir)?;
    fs::create_dir_all(&university_dir)?;
    
    let company_csv = find_csv_in_directory(&company_dir)?
        .unwrap_or_else(|| company_dir.join("part-00000.csv"));
    let university_csv = find_csv_in_directory(&university_dir)?
        .unwrap_or_else(|| university_dir.join("part-00000.csv"));
    
    let mut company_writer = WriterBuilder::new()
        .has_headers(true)
        .delimiter(b'|')
        .from_path(&company_csv)?;
    
    let mut university_writer = WriterBuilder::new()
        .has_headers(true)
        .delimiter(b'|')
        .from_path(&university_csv)?;
    
    // 写入 headers（跳过第二列 type）
    let company_headers: Vec<String> = headers.iter()
        .enumerate()
        .filter_map(|(i, h)| if i == 1 { None } else { Some(h.to_string()) })
        .collect();
    let university_headers = company_headers.clone();
    
    company_writer.write_record(&company_headers)?;
    university_writer.write_record(&university_headers)?;
    
    // 重新创建 reader 来读取数据
    let mut data_reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'|')
        .from_path(csv_path)?;
    
    // 处理每一行
    for result in data_reader.records() {
        let record = result?;
        if record.len() < 2 {
            continue;
        }
        
        let org_type = record.get(1).unwrap();
        
        // 构建新记录（跳过第二列 type）
        let mut new_record = Vec::new();
        for (i, field) in record.iter().enumerate() {
            if i != 1 {  // 跳过 type 列
                new_record.push(field.to_string());
            }
        }
        
        // 根据类型写入对应的文件
        match org_type {
            "Company" => {
                company_writer.write_record(&new_record)?;
            }
            "University" => {
                university_writer.write_record(&new_record)?;
            }
            _ => {
                // 忽略未知类型
                continue;
            }
        }
    }
    
    company_writer.flush()?;
    university_writer.flush()?;
    
    // 添加到顶点文件列表
    vertex_files.push(("Company".to_string(), company_csv));
    vertex_files.push(("University".to_string(), university_csv));
    
    Ok(())
}

fn infer_type_from_value(value: &str) -> (String, bool) {
    if value.is_empty() {
        return ("String".to_string(), true);
    }

    if let Ok(_) = value.parse::<i64>() {
        return ("Int64".to_string(), false);
    }

    if let Ok(_) = value.parse::<f64>() {
        return ("Float64".to_string(), false);
    }

    if value == "true" || value == "false" {
        return ("Boolean".to_string(), false);
    }

    ("String".to_string(), false)
}
