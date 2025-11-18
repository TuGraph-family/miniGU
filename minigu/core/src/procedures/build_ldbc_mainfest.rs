use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

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

fn build_manifest_from_ldbc(
    ldbc_dir: &str,
    manifest_output: &str,
) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    let ldbc_dir = Path::new(ldbc_dir);

    if !ldbc_dir.is_dir() {
        return Err(anyhow::anyhow!(
            "initial_snapshot directory does not exist: {}",
            ldbc_dir.display()
        )
        .into());
    }

    let mut vertex_files: Vec<(String, PathBuf)> = Vec::new();
    let mut edge_files: Vec<(String, String, String, PathBuf)> = Vec::new();
    scan_ldbc_directory(&ldbc_dir, &mut vertex_files, &mut edge_files)?;

    let mut manifest_vertices: Vec<Value> = Vec::new();
    let mut manifest_edges: Vec<Value> = Vec::new();

    for (label, path) in vertex_files {
        let properties = infer_properties_from_csv_vertex(&path)?;

        let relative_path = path
            .strip_prefix(ldbc_dir)
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

    for (edge_label, src_label, dst_label, path) in edge_files {
        let properties = infer_properties_from_csv_edge_new(&path)?;

        let relative_path = path
            .strip_prefix(ldbc_dir)
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
    // 先收集所有CSV文件
    let mut all_files: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("csv") {
            all_files.push(path);
        }
    }
    
    // 第一步：识别所有顶点文件（没有下划线的），建立顶点标签集合
    let mut vertex_labels: HashSet<String> = HashSet::new();
    for path in &all_files {
        let filename = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid filename: {}", path.display()))?;
        
        let name_without_ext = filename
            .strip_suffix(".csv")
            .unwrap_or(filename);
        
        // 如果没有下划线，肯定是顶点文件
        if !name_without_ext.contains('_') {
            vertex_labels.insert(name_without_ext.to_string());
            vertex_files.push((name_without_ext.to_string(), path.clone()));
        }
    }
    
    // 第二步：处理边文件（包含下划线的文件）
    for path in all_files {
        let filename = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid filename: {}", path.display()))?;
        
        let name_without_ext = filename
            .strip_suffix(".csv")
            .unwrap_or(filename);
        
        // 如果包含下划线，尝试解析为边文件
        if name_without_ext.contains('_') {
            // 检查是否已经在顶点文件中（避免重复）
            if vertex_labels.contains(name_without_ext) {
                continue;
            }
            
            // 尝试从文件名提取 src_label 和 dst_label
            // 格式：src_label_edgeType_dst_label
            // 策略：找到第一个和最后一个单词，如果它们在顶点标签集合中，就是边文件
            let parts: Vec<&str> = name_without_ext.split('_').collect();
            if parts.len() >= 2 {
                let first_part = parts[0];
                let last_part = parts[parts.len() - 1];
                
                // 如果第一个和最后一个部分都在顶点标签集合中，认为是边文件
                if vertex_labels.contains(first_part) && vertex_labels.contains(last_part) {
                    let src_label = first_part.to_string();
                    let dst_label = last_part.to_string();
                    let edge_label = name_without_ext.to_string(); // 整个文件名就是边类型
                    
                    edge_files.push((edge_label, src_label, dst_label, path));
                } else {
                    // 如果无法识别，可能是顶点文件（包含下划线的顶点）
                    vertex_files.push((name_without_ext.to_string(), path));
                }
            } else {
                // 只有一个下划线，可能是顶点文件
                vertex_files.push((name_without_ext.to_string(), path));
            }
        }
    }
    
    Ok(())
}

fn infer_properties_from_csv_vertex(
    csv_path: &Path,
) -> Result<Vec<Value>, Box<dyn std::error::Error + Send + Sync>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(csv_path)?;

    let headers = rdr.headers()?.iter().collect::<Vec<_>>();
    if headers.len() <= 1 {
        return Ok(Vec::new());
    }

    // 跳过第一列（id），从第二列开始是属性
    let prop_headers = &headers[1..];
    let mut properties: Vec<Value> = Vec::new();

    // 重新创建 reader 来读取数据行
    let mut data_rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(csv_path)?;

    if let Some(result) = data_rdr.records().next() {
        let record = result?;
        // 跳过第一列（id），从第二列开始
        let prop_values: Vec<&str> = record.iter().skip(1).collect();
        for (header, value) in prop_headers.iter().zip(prop_values.iter()) {
            let (logical_type_str, nullable) = infer_type_from_value(value);
            properties.push(json!({
                "name": header,
                "logical_type": logical_type_str,
                "nullable": nullable
            }));
        }
    } else {
        // 如果没有数据行，只根据header推断，默认String类型
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

fn infer_properties_from_csv_edge_new(
    csv_path: &Path,
) -> Result<Vec<Value>, Box<dyn std::error::Error + Send + Sync>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(csv_path)?;

    let headers = rdr.headers()?.iter().collect::<Vec<_>>();
    // 新格式：src|dst|prop1|prop2|...
    if headers.len() <= 2 {
        return Ok(Vec::new());
    }

    // 跳过前两列（src, dst），从第3列开始是属性
    let prop_headers = &headers[2..];
    let mut properties: Vec<Value> = Vec::new();

    // 重新创建 reader 来读取数据行
    let mut data_rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(csv_path)?;

    if let Some(result) = data_rdr.records().next() {
        let record = result?;
        // 跳过前两列（src, dst），从第3列开始
        let prop_values: Vec<&str> = record.iter().skip(2).collect();
        for (header, value) in prop_headers.iter().zip(prop_values.iter()) {
            let (logical_type_str, nullable) = infer_type_from_value(value);
            properties.push(json!({
                "name": header,
                "logical_type": logical_type_str,
                "nullable": nullable
            }));
        }
    } else {
        // 如果没有数据行，只根据header推断，默认String类型
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
