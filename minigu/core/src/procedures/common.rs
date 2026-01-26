//! Common types and utilities for import/export functionality.

use std::error::Error;
use std::path::Path;
use std::str::FromStr;

use minigu_catalog::property::Property;
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync + 'static>>;
pub type RecordType = Vec<String>;

#[derive(Deserialize, Serialize, Debug)]
pub struct FileSpec {
    pub path: String,   // relative path
    pub format: String, // currently always "csv"
}

impl FileSpec {
    pub fn new(path: String, format: String) -> Self {
        Self { path, format }
    }
}

/// Common metadata for a vertex or edge collection.
#[derive(Deserialize, Serialize, Debug)]
pub struct VertexSpec {
    pub label: String,
    pub file: FileSpec,
    pub properties: Vec<Property>,
}

impl VertexSpec {
    pub fn label_name(&self) -> &String {
        &self.label
    }

    pub fn properties(&self) -> &Vec<Property> {
        &self.properties
    }

    pub fn new(label: String, file: FileSpec, properties: Vec<Property>) -> Self {
        Self {
            label,
            file,
            properties,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EdgeSpec {
    pub label: String,
    pub src_label: String,
    pub dst_label: String,
    pub file: FileSpec,
    pub properties: Vec<Property>,
}

impl EdgeSpec {
    pub fn new(
        label: String,
        src_label: String,
        dst_label: String,
        file: FileSpec,
        properties: Vec<Property>,
    ) -> Self {
        Self {
            label,
            src_label,
            dst_label,
            file,
            properties,
        }
    }

    pub fn src_label(&self) -> &String {
        &self.src_label
    }

    pub fn dst_label(&self) -> &String {
        &self.dst_label
    }

    pub fn label_name(&self) -> &String {
        &self.label
    }

    pub fn properties(&self) -> &Vec<Property> {
        &self.properties
    }
}

// Top-level manifest written to disk.
#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Manifest {
    pub vertices: Vec<VertexSpec>,
    pub edges: Vec<EdgeSpec>,
}

impl Manifest {
    pub fn vertices_spec(&self) -> &Vec<VertexSpec> {
        &self.vertices
    }

    pub fn edges_spec(&self) -> &Vec<EdgeSpec> {
        &self.edges
    }
}

impl FromStr for Manifest {
    type Err = Box<dyn Error + Send + Sync + 'static>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}
