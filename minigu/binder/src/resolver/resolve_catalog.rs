use minigu_catalog::provider::{DirectoryOrSchema, SchemaRef};
use crate::binder::binder::Binder;
use crate::bound_statement::object_ref::CanonicalSchemaPath;
use crate::catalog_ref::SchemaCatalogRef;
use crate::error::{BindError, BindResult};

impl Binder {
    pub(crate) fn resolve_canonical_schema_path(
        &self,
        path: &CanonicalSchemaPath,
    ) -> BindResult<SchemaCatalogRef> {
        let err = || BindError::SchemaNotExists(path.to_string());
        let mut current = match self.catalog.get_root().map_err(|_| err())? {
            DirectoryOrSchema::Directory(dir) => dir,
            _ => return Err(err())
        };
        for segment in &path.segments[..path.segments.len().saturating_sub(1)] {
            match current.get_directory_or_schema(segment).map_err(|_| err())? {
                Some(DirectoryOrSchema::Directory(dir)) => current = dir,
                _ => return Err(err())
            }
        }

        let final_name = path.segments.last().ok_or_else(err)?;
        match current.get_directory_or_schema(final_name).map_err(|_| err())? {
            Some(DirectoryOrSchema::Schema(schema)) => Ok(SchemaCatalogRef {
                name: final_name.clone(),
                schema_ref: schema,
            }),
            _ => Err(err())
        }
    }

    pub(crate) fn resolve_schema_ref(&self, schema_ref: &gql_parser::ast::SchemaRef) -> BindResult<SchemaRef> {
        Err(BindError::NotSupported("get_schema_ref".to_string()))
    }
}
