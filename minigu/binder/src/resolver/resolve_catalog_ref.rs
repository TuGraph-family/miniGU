use std::sync::Arc;
use smol_str::ToSmolStr;
use crate::error::{BindError, BindResult};
use crate::program::Binder;
use gql_parser::ast::{SchemaPath, SchemaPathSegment};
use gql_parser::ast::SchemaRef as AstSchemaRef;
use minigu_catalog::provider::{DirectoryOrSchema, DirectoryProvider, DirectoryRef, SchemaRef};

impl Binder {
    pub(crate) fn resolve_schema_ref(&self, schema_ref: &AstSchemaRef) -> BindResult<SchemaRef> {
        match schema_ref {
            AstSchemaRef::Absolute(path) => {
                self.resolve_schema_path(self.catalog.get_root().map_err(|e| BindError::External(Box::new(e)))?, path)
            }
            AstSchemaRef::Relative(path) => {
                let schema = self.schema.as_ref()
                    .ok_or_else(|| BindError::NotSupported("current schema is not set".to_string()))?;

                let current = DirectoryOrSchema::Schema(Arc::clone(schema));
                self.resolve_schema_path(current, path)
            }

            AstSchemaRef::Parameter(param) => {
                Err(BindError::NotSupported("type of parameters is not supported".to_string()))
            }
            AstSchemaRef::Predefined(predefined) => {
                Err(BindError::NotSupported("predefined type is not supported".to_string()))
            }
        }
    }

    pub(crate) fn resolve_schema_path(
        &self,
        mut current: DirectoryOrSchema,
        path: &SchemaPath,
    ) -> BindResult<SchemaRef>  {
        for (i, segment) in path.iter().enumerate() {
            match segment.value() {
                SchemaPathSegment::Parent => {
                    current = match &current {
                        DirectoryOrSchema::Directory(dir) => {
                            dir.parent().map(DirectoryOrSchema::Directory).ok_or_else(
                                || BindError::NotSupported("schema parent".to_string())
                            )?
                        },
                        DirectoryOrSchema::Schema(schema)=> {
                            schema.parent().map(DirectoryOrSchema::Directory).ok_or_else(
                                || BindError::NotSupported("schema parent".to_string())
                            )?
                        }
                    };
                }

                SchemaPathSegment::Name(name) => {
                    let dir = match current {
                        DirectoryOrSchema::Directory(dir) => dir,
                        DirectoryOrSchema::Schema(_) => {
                            return Err(BindError::NotSupported("schema".to_string()));
                        }
                    };
                    let child = dir.get_child(name.as_str()).map_err(|e| BindError::External(Box::new(e)))?
                        .ok_or_else(||BindError::NotSupported("schema".to_string()))?;
                    match &child {
                        DirectoryOrSchema::Schema(_) => {
                            if i != path.len() - 1 {
                                return Err(BindError::NotSupported("schema".to_string()));
                            }
                        }
                        DirectoryOrSchema::Directory(dir) => {
                            //
                        }
                    }
                    current = child
                }
            }
        }
        match current {
            DirectoryOrSchema::Schema(schema) => Ok(schema),
            DirectoryOrSchema::Directory(current) => Err(BindError::NotSupported("schema".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use smol_str::SmolStr;
    use minigu_catalog::provider::CatalogRef;
    use crate::mock_catalog::{MockCatalog, MockGraph};
    use crate::program::{bind, Binder};
    use gql_parser::ast::{SchemaPathSegment, SchemaRef as AstSchemaRef};
    use gql_parser::span::{Spanned, VecSpanned};

    #[test]
    fn test_resolve_schema_ref() {
        let catalog: CatalogRef = Arc::new(MockCatalog::default());
        let binder = Binder::new(catalog.clone(), None);
        let absolute_path: VecSpanned<SchemaPathSegment> = vec![
            Spanned(SchemaPathSegment::Name(SmolStr::new("root")), 0..4),
            Spanned(SchemaPathSegment::Name(SmolStr::new("default")), 5..12),
            Spanned(SchemaPathSegment::Name(SmolStr::new("a")), 13..14),
        ];
        let schema_ref_absolute = AstSchemaRef::Absolute(absolute_path);
        let schema_ref = binder.resolve_schema_ref(&schema_ref_absolute);
        println!("{:#?}", schema_ref);
    }
}