use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use csv::ReaderBuilder;
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
        
        let path = path
            .to_str()
            .unwrap()
            .to_string();
        
        manifest_vertices.push(json!({
            "label": label,
            "file": {
                "path": path,
                "format": "csv"
            },
            "properties": properties
        }));
    }
    
    // 处理边文件
    for (edge_label, src_label, dst_label, path) in edge_files {
        let properties = infer_properties_from_csv_edge(&path)?;
        
        let path = path
            .to_str()
            .unwrap()
            .to_string();
        
        manifest_edges.push(json!({
            "label": edge_label,
            "src_label": src_label,
            "dst_label": dst_label,
            "file": {
                "path": path,
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
                if dir_name.contains('_') {
                    let parts: Vec<&str> = dir_name.split('_').collect();
                    if parts.len() >= 3 {
                        let src_label = parts[0].to_string();
                        let edge_label = parts[1..parts.len()-1].join("_");
                        let dst_label = parts[parts.len()-1].to_string();
                        edge_files.push((edge_label, src_label, dst_label, csv_path));
                    } else {
                        vertex_files.push((dir_name.to_string(), csv_path));
                    }
                } else {
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
                "nullable": nullable
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
    if headers.len() < 3 {
        return Ok(Vec::new());
    }
    
    let prop_headers = &headers[3..];

    let mut properties: Vec<Value> = Vec::new();

    let mut data_rdr = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'|')
        .from_path(csv_path)?;
    
    if let Some(result) = data_rdr.records().next() {
        let record = result?;
        for (header, value) in prop_headers.iter().zip(record.iter().skip(3)) {
            let (logical_type_str, nullable) = infer_type_from_value(value);
            properties.push(json!({
                "name": header,
                "logical_type": logical_type_str,
                "nullable": nullable
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
