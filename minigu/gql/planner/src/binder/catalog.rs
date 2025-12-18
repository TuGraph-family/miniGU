use gql_parser::ast::{
    CatalogModifyingStatement, CreateGraphStatement, CreateGraphTypeStatement,
    CreateSchemaStatement, CreateVectorIndexStatement, DropGraphStatement, DropGraphTypeStatement,
    DropSchemaStatement, DropVectorIndexStatement,
};
use minigu_catalog::label_set::LabelSet;
use minigu_common::data_type::LogicalType;
use minigu_common::error::not_implemented;
use minigu_common::types::{VectorIndexKey, VectorMetric};

use super::Binder;
use super::error::{BindError, BindResult};
use crate::bound::{
    BoundCatalogModifyingStatement, BoundCreateGraphStatement, BoundCreateGraphTypeStatement,
    BoundCreateSchemaStatement, BoundCreateVectorIndexStatement, BoundDropGraphStatement,
    BoundDropGraphTypeStatement, BoundDropSchemaStatement, BoundDropVectorIndexStatement,
};

impl Binder<'_> {
    pub fn bind_catalog_modifying_statement(
        &mut self,
        statement: &CatalogModifyingStatement,
    ) -> BindResult<BoundCatalogModifyingStatement> {
        match statement {
            CatalogModifyingStatement::Call(statement) => {
                let statement = self.bind_call_procedure_statement(statement)?;
                if statement.optional {
                    return not_implemented("optional catalog modifying statements", None);
                }
                if statement.schema().is_some() {
                    return Err(BindError::NotCatalogProcedure(statement.name()));
                }
                Ok(BoundCatalogModifyingStatement::Call(statement))
            }
            CatalogModifyingStatement::CreateSchema(statement) => self
                .bind_create_schema_statement(statement)
                .map(BoundCatalogModifyingStatement::CreateSchema),
            CatalogModifyingStatement::DropSchema(statement) => self
                .bind_drop_schema_statement(statement)
                .map(BoundCatalogModifyingStatement::DropSchema),
            CatalogModifyingStatement::CreateGraph(statement) => self
                .bind_create_graph_statement(statement)
                .map(BoundCatalogModifyingStatement::CreateGraph),
            CatalogModifyingStatement::DropGraph(statement) => self
                .bind_drop_graph_statement(statement)
                .map(BoundCatalogModifyingStatement::DropGraph),
            CatalogModifyingStatement::CreateVectorIndex(statement) => self
                .bind_create_vector_index_statement(statement)
                .map(BoundCatalogModifyingStatement::CreateVectorIndex),
            CatalogModifyingStatement::DropVectorIndex(statement) => self
                .bind_drop_vector_index_statement(statement)
                .map(BoundCatalogModifyingStatement::DropVectorIndex),
            CatalogModifyingStatement::CreateGraphType(statement) => self
                .bind_create_graph_type_statement(statement)
                .map(BoundCatalogModifyingStatement::CreateGraphType),
            CatalogModifyingStatement::DropGraphType(statement) => self
                .bind_drop_graph_type_statement(statement)
                .map(BoundCatalogModifyingStatement::DropGraphType),
        }
    }

    pub fn bind_create_schema_statement(
        &mut self,
        statement: &CreateSchemaStatement,
    ) -> BindResult<BoundCreateSchemaStatement> {
        not_implemented("create schema statement", None)
    }

    pub fn bind_drop_schema_statement(
        &mut self,
        statement: &DropSchemaStatement,
    ) -> BindResult<BoundDropSchemaStatement> {
        not_implemented("drop schema statement", None)
    }

    pub fn bind_create_vector_index_statement(
        &mut self,
        statement: &CreateVectorIndexStatement,
    ) -> BindResult<BoundCreateVectorIndexStatement> {
        if statement.binding.value() != statement.property_binding.value() {
            return Err(BindError::CreateVectorIndexBindingMismatch {
                binding: statement.binding.value().clone(),
                property_binding: statement.property_binding.value().clone(),
            });
        }

        let graph = self
            .current_graph
            .clone()
            .ok_or(BindError::CurrentGraphNotSpecified)?;
        let graph_type = graph.graph_type();

        let label_name = statement.label.value().clone();
        let label_id = graph_type
            .get_label_id(label_name.as_str())?
            .ok_or_else(|| BindError::LabelNotFound(label_name.clone()))?;

        let vertex_type = graph_type
            .get_vertex_type(&LabelSet::from_iter([label_id]))?
            .ok_or_else(|| BindError::LabelNotFound(label_name.clone()))?;

        let property_name = statement.property.value().clone();
        let (property_id, property) = vertex_type
            .get_property(property_name.as_str())?
            .ok_or_else(|| BindError::PropertyNotFound {
                label: label_name.clone(),
                property: property_name.clone(),
            })?;

        let dimension = match property.logical_type() {
            LogicalType::Vector(dimension) => *dimension,
            ty => {
                return Err(BindError::PropertyNotVector {
                    label: label_name.clone(),
                    property: property_name.clone(),
                    ty: ty.clone(),
                });
            }
        };

        let index_key = VectorIndexKey::new(label_id, property_id);

        // Consult the graph's index metadata via index_catalog.
        let existing = graph
            .index_catalog()
            .map(|c| c.get_vector_index(index_key))
            .transpose()?
            .flatten();
        let name = statement.name.value().clone();
        let existing_by_name = graph
            .index_catalog()
            .map(|c| c.get_vector_index_by_name(name.as_str()))
            .transpose()?
            .flatten();
        if let Some(meta) = existing_by_name
            && meta.key != index_key
        {
            return Err(BindError::VectorIndexNameAlreadyExists { name });
        }
        let no_op = existing.is_some() && statement.if_not_exists;
        if existing.is_some() && !statement.if_not_exists {
            return Err(BindError::VectorIndexAlreadyExists {
                label: label_name.clone(),
                property: property_name.clone(),
            });
        }

        Ok(BoundCreateVectorIndexStatement {
            name,
            if_not_exists: statement.if_not_exists,
            index_key,
            // Currently default to L2; parser/AST does not expose custom metrics yet.
            metric: VectorMetric::L2,
            dimension,
            label: label_name,
            property: property_name,
            no_op,
        })
    }

    pub fn bind_drop_vector_index_statement(
        &mut self,
        statement: &DropVectorIndexStatement,
    ) -> BindResult<BoundDropVectorIndexStatement> {
        let graph = self
            .current_graph
            .clone()
            .ok_or(BindError::CurrentGraphNotSpecified)?;
        let name = statement.name.value().clone();
        let found = graph
            .index_catalog()
            .map(|c| c.get_vector_index_by_name(name.as_str()))
            .transpose()?
            .flatten();
        match found {
            Some(meta) => Ok(BoundDropVectorIndexStatement {
                name,
                if_exists: statement.if_exists,
                index_key: Some(meta.key),
                metadata: Some(meta),
                no_op: false,
            }),
            None if statement.if_exists => Ok(BoundDropVectorIndexStatement {
                name,
                if_exists: true,
                index_key: None,
                metadata: None,
                no_op: true,
            }),
            None => Err(BindError::VectorIndexNotFound { name }),
        }
    }

    pub fn bind_create_graph_statement(
        &mut self,
        statement: &CreateGraphStatement,
    ) -> BindResult<BoundCreateGraphStatement> {
        not_implemented("create graph statement", None)
    }

    pub fn bind_drop_graph_statement(
        &mut self,
        statement: &DropGraphStatement,
    ) -> BindResult<BoundDropGraphStatement> {
        not_implemented("drop graph statement", None)
    }

    pub fn bind_create_graph_type_statement(
        &mut self,
        statement: &CreateGraphTypeStatement,
    ) -> BindResult<BoundCreateGraphTypeStatement> {
        not_implemented("create graph type statement", None)
    }

    pub fn bind_drop_graph_type_statement(
        &mut self,
        statement: &DropGraphTypeStatement,
    ) -> BindResult<BoundDropGraphTypeStatement> {
        not_implemented("drop graph type statement", None)
    }
}
