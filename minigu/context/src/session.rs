use std::sync::Arc;

use gql_parser::ast::{Ident, SchemaPathSegment, SchemaRef};
use minigu_catalog::error::CatalogError;
use minigu_catalog::memory::schema::MemorySchemaCatalog;
use minigu_catalog::named_ref::NamedGraphRef;
use minigu_catalog::provider::{CatalogProvider, SchemaProvider};
use minigu_catalog::txn::catalog_txn::CatalogTxn;
use minigu_catalog::txn::error::CatalogTxnResult;
use minigu_catalog::txn::manager::CatalogTxnManager;
use minigu_transaction::IsolationLevel;

use crate::database::DatabaseContext;
use crate::error::{Error, SessionResult};

mod txn_state;
use txn_state::SessionTxnState;

#[derive(Clone, Debug)]
pub struct SessionContext {
    database: Arc<DatabaseContext>,
    pub home_schema: Option<Arc<MemorySchemaCatalog>>,
    pub current_schema: Option<Arc<MemorySchemaCatalog>>,
    // currently home_graph and current_graph is same.
    // In the future, home_graph is a default graph named default.
    pub home_graph: Option<NamedGraphRef>,
    pub current_graph: Option<NamedGraphRef>,
    // Transaction state for this session.
    txn_state: SessionTxnState,
}

impl SessionContext {
    pub fn new(database: Arc<DatabaseContext>) -> Self {
        Self {
            database,
            home_schema: None,
            current_schema: None,
            home_graph: None,
            current_graph: None,
            txn_state: SessionTxnState::new(),
        }
    }

    pub fn database(&self) -> &DatabaseContext {
        &self.database
    }

    pub fn database_arc(&self) -> Arc<DatabaseContext> {
        self.database.clone()
    }

    pub fn catalog_txn_mgr(&self) -> &CatalogTxnManager {
        &self.txn_state.catalog_txn_mgr
    }

    /// Set default isolation level for implicitly created transactions.
    #[inline]
    pub fn set_default_isolation(&mut self, iso: IsolationLevel) {
        self.txn_state.set_default_isolation(iso);
    }

    /// Returns the active transaction without creating a new one (explicit preferred).
    pub fn current_txn(&self) -> Option<Arc<CatalogTxn>> {
        self.txn_state.current_txn()
    }

    /// Begin an explicit, session-scoped transaction. Errors if another transaction is active.
    pub fn begin_explicit_txn(&mut self) -> CatalogTxnResult<()> {
        self.txn_state.begin_explicit_txn()
    }

    /// Commit the explicit transaction; error if none exists.
    pub fn commit_explicit_txn(&mut self) -> CatalogTxnResult<()> {
        self.txn_state.commit_explicit_txn()
    }

    /// Roll back the explicit transaction; error if none exists.
    pub fn rollback_explicit_txn(&mut self) -> CatalogTxnResult<()> {
        self.txn_state.rollback_explicit_txn()
    }

    /// Execute within a statement-scoped transaction.
    /// - If an explicit txn exists, reuses it without committing.
    /// - Otherwise, creates an implicit txn, commits on Ok, aborts on Err.
    pub fn with_statement_txn<F, R>(&mut self, f: F) -> CatalogTxnResult<R>
    where
        F: FnOnce(&CatalogTxn) -> CatalogTxnResult<R>,
    {
        self.txn_state.with_statement_txn(f)
    }

    /// Execute within a statement-scoped transaction, letting the caller control the error type.
    /// - If an explicit txn exists, reuses it without committing.
    /// - Otherwise, creates an implicit txn, commits on Ok, aborts on Err.
    /// - Any transaction manager errors are converted into `CatalogError` and then into `E`.
    pub fn with_statement_result<F, R, E>(&mut self, f: F) -> Result<R, E>
    where
        F: FnOnce(&CatalogTxn) -> Result<R, E>,
        E: From<CatalogError>,
    {
        self.txn_state.with_statement_result(f)
    }

    pub fn set_current_schema(&mut self, schema: SchemaRef) -> SessionResult<()> {
        match schema {
            SchemaRef::Absolute(schema_path) => {
                let current = self
                    .database
                    .catalog()
                    .get_root()
                    .map_err(|e| minigu_catalog::error::CatalogError::External(Box::new(e)))?;
                let msc: Arc<MemorySchemaCatalog> = self
                    .with_statement_txn(|txn| {
                        let mut current_local = current.clone();
                        let mut current_path = vec![];
                        for segment in schema_path {
                            let name = match segment.value() {
                                SchemaPathSegment::Name(name) => name,
                                SchemaPathSegment::Parent => unreachable!(),
                            };
                            let current_dir = current_local.into_directory().ok_or_else(|| {
                                minigu_catalog::txn::error::CatalogTxnError::IllegalState {
                                    reason: "schema path invalid".into(),
                                }
                            })?;
                            current_path.push(segment.value().clone());
                            let child = current_dir.get_child(name, txn).map_err(|e| {
                                minigu_catalog::txn::error::CatalogTxnError::External(Box::new(e))
                            })?;
                            let child = child.ok_or_else(|| {
                                minigu_catalog::txn::error::CatalogTxnError::IllegalState {
                                    reason: "schema path invalid".into(),
                                }
                            })?;
                            current_local = child;
                        }
                        let schema_arc: minigu_catalog::provider::SchemaRef =
                            current_local.into_schema().ok_or_else(|| {
                                minigu_catalog::txn::error::CatalogTxnError::IllegalState {
                                    reason: "schema path invalid".into(),
                                }
                            })?;

                        let msc: Arc<MemorySchemaCatalog> = schema_arc
                            .downcast_arc::<MemorySchemaCatalog>()
                            .map_err(|_| {
                                minigu_catalog::txn::error::CatalogTxnError::IllegalState {
                                    reason: "schema path invalid".into(),
                                }
                            })?;
                        Ok(msc)
                    })
                    .map_err(|e| minigu_catalog::error::CatalogError::External(Box::new(e)))?;
                self.current_schema = Some(msc);
                Ok(())
            }
            _ => {
                todo!()
            }
        }
    }

    pub fn set_current_graph(&mut self, graph_name: String) -> SessionResult<()> {
        if self.current_schema.is_none() {
            return Err(Error::CurrentSchemaNotSet);
        };
        let schema = self
            .current_schema
            .as_ref()
            .ok_or_else(|| Error::CurrentSchemaNotSet)?
            .clone();
        let graph = self
            .with_statement_txn(|txn| {
                schema
                    .as_ref()
                    .get_graph(graph_name.as_str(), txn)
                    .map_err(|e| {
                        minigu_catalog::txn::error::CatalogTxnError::External(Box::new(e))
                    })?
                    .ok_or_else(|| minigu_catalog::txn::error::CatalogTxnError::NotFound {
                        key: graph_name.clone(),
                    })
            })
            .map_err(|e| minigu_catalog::error::CatalogError::External(Box::new(e)))?;
        self.current_graph = Some(NamedGraphRef::new(Ident::new(graph_name), graph));
        Ok(())
    }

    pub fn reset_current_graph(&mut self) {
        self.current_graph = self.home_graph.clone();
    }

    pub fn reset_current_schema(&mut self) {
        self.current_schema = self.home_schema.clone();
    }
}
