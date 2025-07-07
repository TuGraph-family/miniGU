use gql_parser::ast::{
    GraphRef, PredefinedSchemaRef, ProcedureRef as AstProcedureRef, SchemaPath, SchemaPathSegment,
    SchemaRef as AstSchemaRef,
};
use minigu_catalog::provider::{CatalogProvider, DirectoryOrSchema, SchemaRef};
use minigu_common::error::not_implemented;
use minigu_ir::named_ref::{NamedGraphRef, NamedProcedureRef};

use super::Binder;
use crate::error::{BindError, BindResult};

impl Binder<'_> {
    pub fn bind_schema_ref(&self, schema_ref: &AstSchemaRef) -> BindResult<SchemaRef> {
        match schema_ref {
            AstSchemaRef::Absolute(path) => self.bind_absolute_schema_path(path),
            AstSchemaRef::Relative(path) => self.bind_relative_schema_path(path),
            AstSchemaRef::Predefined(predefined) => self.bind_predefined_schema_ref(predefined),
            AstSchemaRef::Parameter(_) => {
                not_implemented("schema reference parameter".to_string(), None)
            }
        }
    }

    pub fn bind_absolute_schema_path(&self, schema_path: &SchemaPath) -> BindResult<SchemaRef> {
        bind_absolute_schema_path(self.catalog, schema_path)
    }

    pub fn bind_relative_schema_path(&self, schema_path: &SchemaPath) -> BindResult<SchemaRef> {
        let current_schema = self
            .current_schema
            .clone()
            .ok_or(BindError::CurrentSchemaNotSpecified)?;
        bind_relative_schema_path(current_schema, schema_path)
    }

    pub fn bind_predefined_schema_ref(
        &self,
        predefined: &PredefinedSchemaRef,
    ) -> BindResult<SchemaRef> {
        match predefined {
            PredefinedSchemaRef::Home => self
                .home_schema
                .clone()
                .ok_or(BindError::HomeSchemaNotSpecified),
            PredefinedSchemaRef::Current => self
                .current_schema
                .clone()
                .ok_or(BindError::CurrentSchemaNotSpecified),
        }
    }

    pub fn bind_procedure_ref(
        &self,
        procedure_ref: &AstProcedureRef,
    ) -> BindResult<NamedProcedureRef> {
        match procedure_ref {
            AstProcedureRef::Ref(procedure) => {
                let schema = if let Some(schema) = &procedure.schema {
                    self.bind_schema_ref(schema.value())?
                } else {
                    self.current_schema
                        .clone()
                        .ok_or(BindError::CurrentSchemaNotSpecified)?
                };
                match procedure.objects.as_slice() {
                    [] => unreachable!(),
                    [name] => {
                        let name = name.value();
                        let procedure = schema
                            .get_procedure(name)?
                            .ok_or_else(|| BindError::ProcedureNotFound(name.clone()))?;
                        Ok(NamedProcedureRef::new(name.clone(), procedure))
                    }
                    objects => Err(BindError::InvalidObjectReference(
                        objects.iter().map(|o| o.value().clone()).collect(),
                    )),
                }
            }
            AstProcedureRef::Parameter(_) => {
                not_implemented("procedure reference parameter".to_string(), None)
            }
        }
    }

    pub fn bind_graph_ref(&self, graph_ref: &GraphRef) -> BindResult<NamedGraphRef> {
        match graph_ref {
            GraphRef::Name(name) => {
                let schema = self
                    .current_schema
                    .as_ref()
                    .ok_or(BindError::CurrentSchemaNotSpecified)?;
                let graph = schema
                    .get_graph(name)?
                    .ok_or_else(|| BindError::GraphNotFound(name.clone()))?;
                Ok(NamedGraphRef::new(name.clone(), graph))
            }
            GraphRef::Parameter(_) => {
                not_implemented("graph reference parameter".to_string(), None)
            }
            GraphRef::Ref(catalog_object_ref) => {
                let schema = if let Some(schema) = &catalog_object_ref.schema {
                    self.bind_schema_ref(schema.value())?
                } else {
                    self.current_schema
                        .clone()
                        .ok_or(BindError::CurrentSchemaNotSpecified)?
                };
                match catalog_object_ref.objects.as_slice() {
                    [] => unreachable!(),
                    [name] => {
                        let name = name.value();
                        let graph = schema
                            .get_graph(name)?
                            .ok_or_else(|| BindError::GraphNotFound(name.clone()))?;
                        Ok(NamedGraphRef::new(name.clone(), graph))
                    }
                    objects => Err(BindError::InvalidObjectReference(
                        objects.iter().map(|o| o.value().clone()).collect(),
                    )),
                }
            }
            GraphRef::Home => self
                .home_graph
                .clone()
                .ok_or(BindError::HomeGraphNotSpecified),
        }
    }
}

pub fn bind_absolute_schema_path(
    catalog: &dyn CatalogProvider,
    path: &SchemaPath,
) -> BindResult<SchemaRef> {
    let mut current = catalog.get_root()?;
    let mut current_path = vec![];
    for segment in path {
        let name = match segment.value() {
            SchemaPathSegment::Name(name) => name,
            SchemaPathSegment::Parent => unreachable!(),
        };
        let current_dir = current
            .into_directory()
            .ok_or_else(|| BindError::NotDirectory(path_to_string::<true>(&current_path)))?;
        current_path.push(segment.value());
        let child = current_dir.get_child(name)?.ok_or_else(|| {
            BindError::DirectoryOrSchemaNotFound(path_to_string::<true>(&current_path))
        })?;
        current = child;
    }
    current
        .into_schema()
        .ok_or_else(|| BindError::NotSchema(path_to_string::<true>(&current_path)))
}

pub fn bind_relative_schema_path(
    current_schema: SchemaRef,
    schema_path: &SchemaPath,
) -> BindResult<SchemaRef> {
    let mut current = DirectoryOrSchema::Schema(current_schema);
    let mut current_path = vec![];
    for segment in schema_path {
        match segment.value() {
            SchemaPathSegment::Name(name) => {
                let current_dir = current.into_directory().ok_or_else(|| {
                    BindError::NotDirectory(path_to_string::<false>(&current_path))
                })?;
                current_path.push(segment.value());
                let child = current_dir.get_child(name)?.ok_or_else(|| {
                    BindError::DirectoryOrSchemaNotFound(path_to_string::<false>(&current_path))
                })?;
                current = child;
            }
            SchemaPathSegment::Parent => {
                current_path.push(segment.value());
                if let Some(parent_dir) = current.parent() {
                    current = DirectoryOrSchema::Directory(parent_dir);
                }
            }
        }
    }
    current
        .into_schema()
        .ok_or_else(|| BindError::NotSchema(path_to_string::<false>(&current_path)))
}

fn path_to_string<const ABS: bool>(path: &[&SchemaPathSegment]) -> String {
    let path: Vec<_> = path
        .iter()
        .map(|s| match s {
            SchemaPathSegment::Name(name) => name,
            SchemaPathSegment::Parent => "..",
        })
        .collect();
    if ABS {
        format!("/{}", path.join("/"))
    } else {
        path.join("/")
    }
}
