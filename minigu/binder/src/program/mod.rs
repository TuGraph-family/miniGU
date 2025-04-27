pub mod bound_statement;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use gql_parser::ast::{Ident, Procedure, SchemaRef, Statement};
use minigu_catalog::schema::{CatalogInstance, Schema};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::program::bound_statement::common::{BoundNextStatement, BoundProcedure, BoundStatement};
use crate::program::bound_statement::object_ref::CanonicalSchemaPath;
use crate::resolver::resolve_schema_ref;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionContext {
    pub current_working_path: CanonicalSchemaPath,
    pub current_param_context: HashMap<Ident, SchemaRef>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct QuerySymbolTable {
    pub labels: Vec<String>,
    pub alias: Vec<String>,
    pub node_names: Vec<String>,
    pub edge_names: Vec<String>,
}

impl QuerySymbolTable {
    pub fn add_label(&mut self, label: String) -> bool {
        if self.labels.contains(&label) {
            return false;
        }
        self.labels.push(label.clone());
        true
    }

    pub fn check_label(&self, label: &str) -> bool {
        self.labels.contains(&label.to_owned())
    }

    pub fn add_alias(&mut self, alias: String) -> bool {
        if self.alias.contains(&alias) {
            return false;
        }
        self.alias.push(alias.clone());
        true
    }

    pub fn check_alias(&self, alias: &str) -> bool {
        self.alias.contains(&alias.to_owned())
    }

    pub fn add_node_names(&mut self, name: String) -> bool {
        if self.node_names.contains(&name) {
            return false;
        }
        self.node_names.push(name.clone());
        true
    }

    pub fn check_node_names(&self, name: &str) -> bool {
        self.node_names.contains(&name.to_owned())
    }

    pub fn add_edge_names(&mut self, name: String) -> bool {
        if self.edge_names.contains(&name) {
            return false;
        }
        self.edge_names.push(name.clone());
        true
    }

    pub fn check_edge_names(&self, name: &str) -> bool {
        self.edge_names.contains(&name.to_owned())
    }
}

pub struct Binder {
    /// By default, the `session context` carries the path or reference of the schema in use.
    /// However in some statements, a specific schema for the statement may need to be set.
    /// To avoid affecting the schema in use globally, a `working_schema` for the statement is
    /// introduced. During the binding process, `working_schema` takes priority over the
    /// schema in session_context.
    pub session_context: Arc<SessionContext>,
    pub working_schema: Option<Arc<Mutex<Schema>>>,
}

/// Bind a gql procedure into a bound procedure.
/// The `Binder` is responsible for semantic analysis of parsed AST nodes.
/// It transforms high-level syntactic structures (like `procedure` or `statement`)
/// into semantically enriched, validated forms such as `BoundProcedure`.

/// Binding stages
/// Binding in this system follows a **three-phase model**:
/// 1. **Resolve**: Identify and link all names(e.g. variables, labels, schema references) to
/// catalog objects or session context definitions.
/// 2. **TypeCheck**: Infer and validate types of expressions, ensuring correctness of operations,
/// casts, predicates and other typed constructs.
/// 3. **Validate**: Perform final logical checks.

/// These phases are applied in sequence to each bindable construct, ensuring correctness and
/// early error detection.

/// ### Construction
/// A `Binder` is constructed from a `SessionContext`, which provides access to:
/// - the current schema
/// - catalog definitions
/// - session variables and runtime context.
/// ``rust
///  let binder = Binder::new(session_context);
/// ``

impl Binder {
    pub fn new(session_context: SessionContext) -> Binder {
        Binder {
            session_context: Arc::new(session_context),
            working_schema: None,
        }
    }

    pub fn get_available_schema(&self) -> Result<Arc<Schema>, Error> {
        if let Some(working_schema) = &self.working_schema {
            let schema = working_schema.lock().unwrap();
            return Ok(Arc::new(schema.clone()));
        }
        if let Some(schema) = CatalogInstance::read().get_schema(
            self.session_context
                .current_working_path
                .segments
                .as_slice(),
        ) {
            return Ok(Arc::new(schema.clone()));
        }
        Err(Error::SchemaNotExists(
            self.session_context.current_working_path.to_string(),
        ))
    }

    pub fn bind_procedure(&mut self, procedure: &Procedure) -> Result<BoundProcedure, Error> {
        let next_statements = procedure.next_statements.clone();
        let mut bound_next_statements = Vec::new();
        for stmt in next_statements {
            let bound_statement = self.bind_statement(stmt.value().statement.value())?;
            bound_next_statements.push(BoundNextStatement {
                // TODO: Handle Yield clause.
                yield_clause: stmt.value().yield_clause.clone(),
                statement: bound_statement,
            })
        }
        let mut bound_schema_path = None;
        if let Some(schema) = procedure.at.as_ref() {
            let schema_path = resolve_schema_ref(self.session_context.as_ref(), schema.value());
            if let Some(schema) = CatalogInstance::read()
                .get_schema(schema_path.as_canonical().unwrap().segments.as_slice())
            {
                self.working_schema = Some(Arc::new(Mutex::new(schema.clone())));
            } else {
                return Err(Error::ErrorCur);
            }
            bound_schema_path = Some(schema_path);
        }

        Ok(BoundProcedure {
            at: bound_schema_path,
            // TODO: Handle binding_variable_defs in procedure body.
            binding_variable_def: procedure.binding_variable_defs.clone(),
            statement: self.bind_statement(procedure.statement.value())?,
            next_statement: bound_next_statements,
        })
    }

    pub fn bind_statement(&mut self, statement: &Statement) -> Result<BoundStatement, Error> {
        let mut resolved_statement = self.resolve_statement(&statement)?;
        self.type_check(&resolved_statement)?;
        self.validate_statement(&resolved_statement)?;
        Ok(resolved_statement)
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use std::collections::HashMap;
    use std::sync::Once;

    use gql_parser::ast::ProgramActivity;
    use insta::assert_yaml_snapshot;
    use matches::assert_matches;
    use smol_str::ToSmolStr;
    use minigu_catalog::graph_type::GraphType;
    use minigu_catalog::schema::{CatalogInstance, Graph, Schema};

    use crate::error::Error;
    use crate::program::bound_statement::common::BoundProcedure;
    use crate::program::bound_statement::object_ref::CanonicalSchemaPath;
    use crate::program::{Binder, SessionContext};

    static PREPARE: Once = Once::new();

    fn prepare_schema() {
        PREPARE.call_once(|| {
            let schema = Schema {
                id: 0,
                name: "default".to_string(),
                graph_id_map: HashMap::default(),
                graph_map: HashMap::default(),
                procedure: HashMap::default(),
                graph_type: HashMap::from([("test".to_string(), GraphType::default())]),
                next_schema: 1,
            };
            CatalogInstance::write().add_schema(&["default".to_smolstr()], schema);
        });
    }

    fn get_binder() -> Binder {
        Binder::new(SessionContext {
            current_working_path: CanonicalSchemaPath {
                segments: vec!["default".to_smolstr()],
            },
            current_param_context: HashMap::new(),
        })
    }

    fn get_bound_procedure(gql: &str) -> Result<BoundProcedure, Error> {
        let parsed = gql_parser::parse_gql(gql);
        let program_activity = parsed
            .unwrap()
            .value()
            .clone()
            .activity
            .unwrap()
            .value()
            .clone();
        let trans_activity = match program_activity {
            ProgramActivity::Session(session) => None,
            ProgramActivity::Transaction(transaction) => Some(transaction),
        };
        let procedure = trans_activity.unwrap().clone().procedure.unwrap().clone();
        get_binder().bind_procedure(procedure.value())
    }

    #[test]
    fn test_schema_create_and_drop() {
        let stmt = get_bound_procedure("create schema if not exists /a/b/c");
        assert_yaml_snapshot!(stmt);
        let stmt = get_bound_procedure("create schema /a");
        assert_yaml_snapshot!(stmt);
        let stmt = get_bound_procedure("drop schema if exists /a/b/c");
        assert_yaml_snapshot!(stmt);
        let stmt = get_bound_procedure("CREATE SCHEMA /foo NEXT CREATE SCHEMA /fee");
        assert_yaml_snapshot!(stmt);
    }

    #[test]
    fn test_create_and_drop_graph_type() {
        prepare_schema();
        let stmt = get_bound_procedure(
            "
     CREATE GRAPH TYPE fraud_GT {
     (customer :Customer => {id::STRING, name::STRING}),
     (account :Account => {no::STRING, type::STRING }),
     (customer)-[:HOLDS]->(account),
     (account)-[:TRANSFER {amount::INTEGER}]->(account)
     }",
        );
        assert_yaml_snapshot!(stmt);

        let stmt = get_bound_procedure(
            "
     CREATE GRAPH TYPE fraud_GT {
     (customer :Customer => {id::STRING, name::STRING}),
     (account :Account => {no::STRING, type::STRING }),
     (account_)-[:TRANSFER {amount::INTEGER}]->(account_)
     }",
        );
        assert_matches!(stmt, Err(Error::NodeTypeNotExists(_)));
        let stmt = get_bound_procedure(
            "
     CREATE GRAPH TYPE test {
     (customer :Customer => {id::STRING, name::STRING}),
     (account :Account => {no::STRING, type::STRING }),
     (account)-[:TRANSFER {amount::INTEGER}]->(account)
     }",
        );
        assert_matches!(stmt, Err(Error::GraphTypeAlreadyExists(_)));

        let stmt = get_bound_procedure("drop GRAPH TYPE test");
        assert_yaml_snapshot!(stmt);
        let stmt = get_bound_procedure("drop GRAPH TYPE test_not_exists");
        assert_matches!(stmt, Err(Error::GraphTypeNotExists(_)));
    }

    #[test]
    fn test_graph_create_and_drop() {
        prepare_schema();
        let stmt = get_bound_procedure(
            r"create graph if not exists myGraph ::any as copy of /default/test",
        );
        assert_yaml_snapshot!(stmt);

        // With two statement: create graph type and graph.
        let stmt = get_bound_procedure(
            r"create graph /a/b {
                (a:Person {id int, name string}),
                (account :Account => {no::STRING, type::STRING }),
                (account) ~[:knows {since int}]~ (a)
            }",
        );
        assert_yaml_snapshot!(stmt);

        let stmt = get_bound_procedure(r"drop graph /default/test");
        assert_yaml_snapshot!(stmt);
        let stmt = get_bound_procedure(r"drop graph /default/not_exists");
        assert_matches!(stmt, Err(Error::GraphNotExists(_)));
    }

    #[test]
    fn test_query() {
        prepare_schema();
        let stmt = get_bound_procedure(r"MATCH (n:Account{id:12}) RETURN n");
        assert_yaml_snapshot!(stmt);
        let stmt = get_bound_procedure(
            "MATCH (n:Account{id:12})
                    RETURN
                        n.createTime as createTime,
                        n.isBlocked as isBlocked,
                        n.type as type",
        );
        assert_yaml_snapshot!(stmt);
    }
}
