use std::sync::Arc;

use gql_parser::ast::{SchemaPath, SchemaPathSegment, SchemaRef as AstSchemaRef};
use minigu_catalog::provider::{DirectoryOrSchema, DirectoryProvider, SchemaRef};
use smol_str::ToSmolStr;

use crate::error::{BindError, BindResult};
use crate::program::Binder;

pub fn schema_path_to_string(path: &SchemaPath) -> String {
    path.iter()
        .map(|seg| match seg.value() {
            SchemaPathSegment::Name(ident) => ident.as_str(), // or `ident` if it's already `&str`
            SchemaPathSegment::Parent => "..",
        })
        .collect::<Vec<_>>()
        .join("/")
}

impl Binder {
    pub(crate) fn resolve_schema_ref(&self, schema_ref: &AstSchemaRef) -> BindResult<SchemaRef> {
        match schema_ref {
            AstSchemaRef::Absolute(path) => self.resolve_schema_path(
                self.catalog
                    .get_root()
                    .map_err(|e| BindError::External(Box::new(e)))?,
                path,
            ),
            AstSchemaRef::Relative(path) => {
                let schema = self
                    .schema
                    .as_ref()
                    .ok_or_else(|| BindError::SchemaNotSpecified)?;

                let current = DirectoryOrSchema::Schema(Arc::clone(schema));
                self.resolve_schema_path(current, path)
            }

            AstSchemaRef::Parameter(param) => Err(BindError::NotSupported(
                "type of parameters is not supported".to_string(),
            )),
            AstSchemaRef::Predefined(predefined) => Err(BindError::NotSupported(
                "predefined type is not supported".to_string(),
            )),
        }
    }

    pub(crate) fn resolve_schema_path(
        &self,
        mut current: DirectoryOrSchema,
        path: &SchemaPath,
    ) -> BindResult<SchemaRef> {
        let path_str = schema_path_to_string(path);
        for (i, segment) in path.iter().enumerate() {
            match segment.value() {
                SchemaPathSegment::Parent => {
                    current = match &current {
                        DirectoryOrSchema::Directory(dir) => {
                            DirectoryOrSchema::Directory(dir.parent().expect("parent should exist"))
                        }
                        DirectoryOrSchema::Schema(schema) => DirectoryOrSchema::Directory(
                            schema.parent().expect("parent should exist"),
                        ),
                    };
                }

                SchemaPathSegment::Name(name) => {
                    let dir = match current {
                        DirectoryOrSchema::Directory(dir) => dir,
                        DirectoryOrSchema::Schema(_) => {
                            return Err(BindError::SchemaNotFound(path_str.clone()));
                        }
                    };
                    let child = dir
                        .get_child(name.as_str())
                        .map_err(|e| BindError::External(Box::new(e)))?
                        .ok_or_else(|| BindError::SchemaNotFound(path_str.clone()))?;
                    match &child {
                        DirectoryOrSchema::Schema(_) => {
                            if i != path.len() - 1 {
                                return Err(BindError::SchemaNotFound(path_str.clone()));
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
            DirectoryOrSchema::Directory(current) => Err(BindError::SchemaNotFound(path_str)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use gql_parser::ast::{SchemaPathSegment, SchemaRef as AstSchemaRef};
    use gql_parser::span::Spanned;
    use minigu_catalog::provider::CatalogRef;
    use smol_str::SmolStr;

    use crate::error::BindError;
    use crate::mock_catalog::MockCatalog;
    use crate::program::Binder;

    fn get_schema_ref(
        segments: impl IntoIterator<Item = impl AsRef<str>>,
        is_absolute: bool,
    ) -> AstSchemaRef {
        let mut offset = 0;
        let mut path = Vec::new();

        for segment in segments {
            let s = segment.as_ref();
            let len = s.len();
            let range = offset..(offset + len);
            offset += len + 1;

            let seg = if s == ".." {
                SchemaPathSegment::Parent
            } else {
                SchemaPathSegment::Name(SmolStr::new(s))
            };

            path.push(Spanned(seg, range));
        }
        if !is_absolute {
            AstSchemaRef::Relative(path)
        } else {
            AstSchemaRef::Absolute(path)
        }
    }

    #[test]
    fn test_resolve_schema_ref() {
        let catalog: CatalogRef = Arc::new(MockCatalog::default());
        let binder = Binder::new(catalog.clone(), None);
        let absolute_path = get_schema_ref(["root", "default", "a"], true);
        let schema_ref = binder.resolve_schema_ref(&absolute_path);
        assert!(matches!(schema_ref, Err(BindError::SchemaNotFound(_))));
        let absolute_path = get_schema_ref(["default", "a", "b"], true);
        let schema_ref = binder.resolve_schema_ref(&absolute_path);
        assert!(schema_ref.is_ok());

        let binder = Binder::new(catalog.clone(), Some(schema_ref.unwrap()));
        let relative_path = get_schema_ref(["..", "..", "a", "b"], false);
        let schema_ref = binder.resolve_schema_ref(&relative_path);
        assert!(schema_ref.is_ok());
        let relative_path = get_schema_ref(["..", "a", "b"], false);
        let schema_ref = binder.resolve_schema_ref(&relative_path);
        assert!(schema_ref.is_err());
    }
}
