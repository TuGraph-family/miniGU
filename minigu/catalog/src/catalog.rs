use std::collections::HashMap;
use std::sync::{OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard};

use smol_str::ToSmolStr;

use crate::error::Error;
pub(crate) use crate::schema::Schema;
use crate::types::*;

// Schema Node
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaNode {
    pub name: Ident,
    pub children: HashMap<Ident, SchemaNode>,
    pub schema: Option<Schema>,
}

#[derive(Debug)]
pub struct SchemaTire {
    pub root: SchemaNode,
}

impl SchemaNode {
    pub fn print(&self) {
        let indent_str = " ".repeat(4);
        println!("{}Node: {}", indent_str, self.name);

        if let Some(schema) = &self.schema {
            println!("{}  Schema: {:?}", indent_str, schema);
        }

        for (child_name, child_node) in &self.children {
            println!("{}  Child: {}", indent_str, child_name);
            child_node.print();
        }
    }
}
impl SchemaTire {
    pub fn new() -> Self {
        let mut root = SchemaNode {
            name: "".to_smolstr(),
            children: HashMap::new(),
            schema: None,
        };

        // Will automatically create a default node when init.
        let mut default_node = SchemaNode {
            name: "default".to_smolstr(),
            children: HashMap::new(),
            schema: Some(Schema::new("default".to_smolstr())),
        };

        if let Some(schema) = default_node.schema.as_mut() {
            schema.id = SchemaId::new(1);
        }

        root.children
            .insert("default".to_smolstr(), default_node.clone());

        SchemaTire { root }
    }

    pub fn create_schema(&mut self, path: &[Ident], schema: Schema) -> Result<(), Error> {
        let mut current_node = &mut self.root;
        for segment in path {
            current_node = current_node
                .children
                .entry(segment.clone())
                .or_insert_with(|| SchemaNode {
                    name: segment.clone(),
                    children: HashMap::new(),
                    schema: None,
                })
        }
        if current_node.schema.is_some() {
            return Err(Error::SchemaAlreadyExists(path.join("/")));
        }
        current_node.schema = Some(schema);
        Ok(())
    }

    pub fn get_schema(&mut self, path: &[Ident]) -> Result<&mut Schema, Error> {
        let mut current_node = &mut self.root;
        for segment in path {
            current_node = current_node
                .children
                .get_mut(segment)
                .ok_or_else(|| Error::SchemaNotExists(path.to_vec().join("/")))?;
        }
        if current_node.schema.is_some() {
            Ok(current_node.schema.as_mut().unwrap())
        } else {
            Err(Error::SchemaNotExists(path.join("/")))
        }
    }

    fn remove_recursive(node: &mut SchemaNode, path: &[Ident]) -> Result<(), Error> {
        // At the parent node of the schema node.
        if path.len() == 1 {
            if let Some(child) = node.children.get_mut(&path[0]) {
                if child.schema.is_none() {
                    Err(Error::SchemaNotExists(path[0].to_string()))
                } else {
                    if child.children.is_empty() {
                        node.children.remove(&path[0]);
                    } else {
                        node.schema = None;
                    }
                    Ok(())
                }
            } else {
                Err(Error::SchemaNotExists(path[0].to_string()))
            }
        } else {
            let next = &path[0];
            let rest = &path[1..];
            if let Some(child) = node.children.get_mut(next) {
                match Self::remove_recursive(child, rest) {
                    Ok(()) => {
                        if child.children.is_empty() {
                            node.children.remove(next);
                        }
                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            } else {
                Err(Error::SchemaNotExists(next.to_string()))
            }
        }
    }

    pub fn delete_schema(&mut self, path: &[Ident]) -> Result<(), Error> {
        Self::remove_recursive(&mut self.root, path)
            .map_err(|_| Error::SchemaNotExists(path.join("/")))
    }

    pub fn print(&self) {
        println!("SchemaTire:");
        self.root.print();
    }
}

pub struct CatalogInstance {
    pub schema: SchemaTire,
}

static INSTANCE: OnceLock<RwLock<CatalogInstance>> = OnceLock::new();

impl CatalogInstance {
    fn init() -> RwLock<CatalogInstance> {
        RwLock::new(CatalogInstance {
            schema: SchemaTire::new(),
        })
    }

    pub fn global() -> &'static RwLock<CatalogInstance> {
        INSTANCE.get_or_init(Self::init)
    }

    pub fn read() -> RwLockReadGuard<'static, CatalogInstance> {
        Self::global()
            .read()
            .expect("Failed to acquire read lock on CatalogInstance")
    }

    pub fn write() -> RwLockWriteGuard<'static, CatalogInstance> {
        Self::global()
            .write()
            .expect("Failed to acquire write lock on CatalogInstance")
    }

    pub fn create_schema(&mut self, path: &[Ident], schema: Schema) -> Result<(), Error> {
        self.schema.create_schema(path, schema)
    }

    pub fn get_schema(&mut self, path: &[Ident]) -> Result<&mut Schema, Error> {
        self.schema.get_schema(path)
    }

    pub fn delete_schema(&mut self, path: &[Ident]) -> Result<(), Error> {
        self.schema.delete_schema(path)
    }
}

#[cfg(test)]
mod tests {
    use smol_str::ToSmolStr;

    use super::*;
    use crate::catalog::{Ident, Schema, SchemaTire};
    use crate::error::Error;

    fn make_path(parts: &[&str]) -> Vec<Ident> {
        parts.iter().map(|p| p.to_smolstr()).collect()
    }
    #[test]
    fn test_create_and_get_schema() {
        let mut tire = SchemaTire::new();
        assert!(tire.get_schema(&make_path(&["default"])).is_ok());
        let path = make_path(&["foo", "bar"]);
        let schema = Schema::new("bar".to_smolstr());
        assert!(tire.create_schema(&path, schema).is_ok());
        let schema_opt = tire.get_schema(&path);
        assert!(schema_opt.is_ok());
    }

    #[test]
    fn test_create_duplicate_schema() {
        let mut tire = SchemaTire::new();
        let path = make_path(&["default"]);
        assert!(matches!(
            tire.create_schema(&path, Schema::new("default".to_smolstr())),
            Err(Error::SchemaAlreadyExists(_))
        ));
        let path2 = make_path(&["foo", "bar"]);
        assert!(
            tire.create_schema(&path2, Schema::new("bar".to_smolstr()))
                .is_ok()
        );
        assert!(
            tire.create_schema(&path2, Schema::new("bar".to_smolstr()))
                .is_err()
        );
    }

    #[test]
    fn test_delete_schema() {
        /// schema path is :
        ///  /default
        ///  /default/foo
        ///  /default/foo/bar
        let mut tire = SchemaTire::new();
        let path = make_path(&["default", "foo"]);
        assert!(
            tire.create_schema(&path, Schema::new("foo".to_smolstr()))
                .is_ok()
        );
        assert!(tire.get_schema(&path).is_ok());
        assert!(
            tire.create_schema(
                &make_path(&["default", "foo", "bar"]),
                Schema::new("bar".to_smolstr())
            )
            .is_ok()
        );
        assert!(tire.get_schema(&path).is_ok());
        assert!(tire.delete_schema(&make_path(&["default", "foo"])).is_ok());
        assert!(
            tire.get_schema(&make_path(&["default", "foo", "bar"]))
                .is_ok()
        );
    }

    #[test]
    fn test_delete_nonexistent_schema() {
        let mut tire = SchemaTire::new();
        let path = make_path(&["default", "nonexistent"]);
        let result = tire.delete_schema(&path);
        assert!(matches!(result, Err(Error::SchemaNotExists(_))));
    }

    #[test]
    fn test_catalog_instance_singleton() {
        {
            let mut instance = CatalogInstance::write();
            let path = make_path(&["default", "test"]);
            let schema = Schema::new("test".to_smolstr());
            assert!(instance.create_schema(&path, schema).is_ok());
        }

        {
            let mut instance = CatalogInstance::write();
            let path = make_path(&["default", "test"]);
            let schema = instance.get_schema(&path);
            assert!(schema.is_ok());
        }
    }

    #[test]
    fn test_catalog_remove_schema() {
        {
            let mut instance = CatalogInstance::write();
            let path = make_path(&["default", "removable"]);
            let schema = Schema::new("removable".to_smolstr());
            assert!(instance.create_schema(&path, schema).is_ok());
            assert!(instance.delete_schema(&path).is_ok());
            assert!(instance.get_schema(&path).is_err());
        }
    }
}
