use gql_parser::ast::{EdgeType, GraphElementType, Ident, NodeOrEdgeTypeFiller, NodeTypeRef};
use crate::binder::binder::Binder;
use crate::bound_statement::catalog::BoundCatalogModifyingStatement;
use crate::bound_statement::object_ref::BoundGraphTypeSource;
use crate::bound_statement::procedure_spec::BoundStatement;
use crate::error::BindError;

pub fn check_internal_words(word: &Ident) -> bool {
    true // TODO: Add actual checks for internal/reserved words.
}

pub fn check_node_or_edge_filler(filler: &NodeOrEdgeTypeFiller) -> Result<(), BindError> {
    let mut field_names = vec![];
    if let Some(filler_types) = &filler.property_types {
        for field in filler_types {
            let field_name = field.value().name.value();
            // Check if it will conflict with the predefined internal fields.
            if !check_internal_words(field_name) {
                return Err(BindError::InvalidField(field_name.to_string()));
            }
            // Check for duplicate fields.
            if field_names.contains(field_name) {
                return Err(BindError::DuplicateField(field_name.to_string()));
            }
            field_names.push(field_name.clone());
        }
    }
    Ok(())
}

pub fn check_node_type_ref(table: &QuerySymbolTable, type_ref: &NodeTypeRef, ) -> Result<(), Error> {
    match type_ref {
        NodeTypeRef::Alias(alias) => {
            if !table.check_alias(alias) {
                return Err(Error::NodeTypeNotExists(alias.to_string()));
            }
        }
        NodeTypeRef::Filler(filler) => {
            check_node_or_edge_filler(filler)?;
        }
        NodeTypeRef::Empty => {}
    }
    Ok(())
}

impl Binder {
    
    pub(crate) fn check_nested_graph_element(&self, element: &GraphElementType, table: &mut QuerySymbolTable) -> Result<(), Error> {
            match element {
                GraphElementType::Node(node) => {
                    if let Some(name) = &node.name {
                        if !table.add_node_names(name.value().to_string()) {
                            return Err(Error::NotSupported("invalid name".to_string()));
                        }
                    }

                    if let Some(alias) = &node.alias {
                        if !table.add_alias(alias.value().to_string()) {
                            return Err(Error::NotSupported("invalid alias".to_string()));
                        }
                    }

                    if let Some(filler) = &node.filler {
                        check_node_or_edge_filler(filler.value())?
                    }
                },
                GraphElementType::Edge(edge) => match edge.as_ref() {
                    EdgeType::Pattern(pattern) => {
                        if let Some(name) = &pattern.name {
                            if !table.add_edge_names(name.value().to_string()) {
                                return Err(Error::NotSupported("invalid edge name".to_string()));
                            }
                        }
                        check_node_type_ref(&table, &pattern.left.value())?;
                        check_node_type_ref(&table, &pattern.right.value())?;
                        check_node_or_edge_filler(&pattern.filler.value())?;
                    }
                    EdgeType::Phrase(phrase) => {
                        if let Some(name) = &phrase.name {
                            if !table.add_edge_names(name.value().to_string()) {
                                return Err(Error::NotSupported("invalid edge name".to_string()));
                            }
                        }

                        if !table.check_label(phrase.left.value())
                            || !table.check_label(phrase.right.value())
                        {
                            return Err(Error::NotSupported( "invalid label".to_string()));
                        }

                        if let Some(filler) = &phrase.filler {
                            check_node_or_edge_filler(filler.value())?;
                        }
                    }
                }
            }
        Ok(())
    }
    pub(crate) fn type_check(&self, statement: &BoundStatement) -> Result<(), Error> {
        match statement {
            BoundStatement::Catalog(catalog) => {
                for stmt in catalog {
                    if let BoundCatalogModifyingStatement::CreateGraphType(create) = stmt {
                        if let BoundGraphTypeSource::Nested(elements) = &create.source {
                            let mut table = QuerySymbolTable::default();
                            for element in elements {
                                self.check_nested_graph_element(element, &mut table)?;
                            }
                        }
                    }
                }
                Ok(())
            }
            BoundStatement::Query(query) => Ok(()),
            BoundStatement::Data(data) => Ok(()),
        }
    }
}
