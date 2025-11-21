use std::sync::Arc;

use gql_parser::ast::{Ident, SchemaPathSegment, SchemaRef};
use minigu_catalog::error::CatalogError;
use minigu_catalog::memory::schema::MemorySchemaCatalog;
use minigu_catalog::named_ref::NamedGraphRef;
use minigu_catalog::provider::{CatalogProvider, SchemaProvider};
use minigu_catalog::txn::catalog_txn::CatalogTxn;
use minigu_catalog::txn::error::CatalogTxnResult;
use minigu_catalog::txn::manager::CatalogTxnManager;
use minigu_transaction::{GraphTxnManager, IsolationLevel, Transaction};

use crate::database::DatabaseContext;
use crate::error::{Error, SessionResult};

#[derive(Clone, Debug)]
pub struct SessionContext {
    database: Arc<DatabaseContext>,
    pub home_schema: Option<Arc<MemorySchemaCatalog>>,
    pub current_schema: Option<Arc<MemorySchemaCatalog>>,
    // currently home_graph and current_graph is same.
    // In the future, home_graph is a default graph named default.
    pub home_graph: Option<NamedGraphRef>,
    pub current_graph: Option<NamedGraphRef>,
    current_txn: Option<Arc<CatalogTxn>>,
    /// Explicit, session-scoped transaction started by user (`BEGIN`). When present,
    /// statement execution should reuse this and never auto-commit.
    explicit_txn: Option<Arc<CatalogTxn>>,
    pub catalog_txn_mgr: CatalogTxnManager,
    isolation_level: Option<IsolationLevel>,
}

impl SessionContext {
    pub fn new(database: Arc<DatabaseContext>) -> Self {
        Self {
            database,
            home_schema: None,
            current_schema: None,
            home_graph: None,
            current_graph: None,
            current_txn: None,
            explicit_txn: None,
            catalog_txn_mgr: CatalogTxnManager::new(),
            isolation_level: None,
        }
    }

    pub fn database(&self) -> &DatabaseContext {
        &self.database
    }

    /// Set default isolation level for implicitly created transactions.
    #[inline]
    pub fn set_default_isolation(&mut self, iso: IsolationLevel) {
        self.isolation_level = Some(iso);
    }

    /// Begin a new transaction using the default isolation level.
    fn begin_txn(&self) -> CatalogTxnResult<Arc<CatalogTxn>> {
        if let Some(iso) = self.isolation_level {
            let txn_arc = self.catalog_txn_mgr.begin_transaction(iso)?;
            return Ok(txn_arc);
        }
        let txn_arc = self
            .catalog_txn_mgr
            .begin_transaction(IsolationLevel::Serializable)?;
        Ok(txn_arc)
    }

    /// Clear the current transaction reference (without committing or rolling back).
    /// This is used in auto-commit mode, where the transaction is committed successfully,
    /// and the `current_txn` is removed to avoid mistakenly reusing the committed transaction.
    /// Note: This method only clears the session-level reference, does not affect the lifecycle of
    /// the transaction itself.
    #[inline]
    pub fn clear_current_txn(&mut self) {
        self.current_txn = None;
    }

    /// Returns the active transaction without creating a new one (explicit preferred).
    pub fn current_txn(&self) -> Option<Arc<CatalogTxn>> {
        if let Some(t) = &self.explicit_txn {
            return Some(t.clone());
        }
        self.current_txn.clone()
    }

    /// Begin an explicit, session-scoped transaction. Errors if another transaction is active.
    pub fn begin_explicit_txn(&mut self) -> CatalogTxnResult<()> {
        if self.explicit_txn.is_some() || self.current_txn.is_some() {
            return Err(minigu_catalog::txn::error::CatalogTxnError::IllegalState {
                reason: "transaction already active".into(),
            });
        }
        let txn = self.begin_txn()?;
        self.explicit_txn = Some(txn);
        Ok(())
    }

    /// Commit the explicit transaction; error if none exists.
    pub fn commit_explicit_txn(&mut self) -> CatalogTxnResult<()> {
        let Some(txn) = self.explicit_txn.take() else {
            return Err(minigu_catalog::txn::error::CatalogTxnError::IllegalState {
                reason: "no explicit transaction to commit".into(),
            });
        };
        txn.commit()?;
        Ok(())
    }

    /// Roll back the explicit transaction; error if none exists.
    pub fn rollback_explicit_txn(&mut self) -> CatalogTxnResult<()> {
        let Some(txn) = self.explicit_txn.take() else {
            return Err(minigu_catalog::txn::error::CatalogTxnError::IllegalState {
                reason: "no explicit transaction to roll back".into(),
            });
        };
        txn.abort()?;
        Ok(())
    }

    /// Execute within a statement-scoped transaction.
    /// - If an explicit txn exists, reuses it without committing.
    /// - Otherwise, creates an implicit txn, commits on Ok, aborts on Err.
    pub fn with_statement_txn<F, R>(&mut self, f: F) -> CatalogTxnResult<R>
    where
        F: FnOnce(&CatalogTxn) -> CatalogTxnResult<R>,
    {
        if let Some(txn) = &self.explicit_txn {
            return f(txn.as_ref());
        }
        let created_here: bool;
        let txn_arc = if let Some(t) = &self.current_txn {
            created_here = false;
            t.clone()
        } else {
            let t = self.begin_txn()?;
            self.current_txn = Some(t.clone());
            created_here = true;
            t
        };

        // Execute the function within the transaction.
        let result = f(txn_arc.as_ref());
        match result {
            Ok(v) => {
                if created_here {
                    txn_arc.commit()?;
                    self.clear_current_txn();
                }
                Ok(v)
            }
            Err(e) => {
                if created_here {
                    let _ = txn_arc.abort();
                    self.clear_current_txn();
                }
                Err(e)
            }
        }
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
        if let Some(txn) = &self.explicit_txn {
            return f(txn.as_ref());
        }
        let created_here: bool;
        let txn_arc = if let Some(t) = &self.current_txn {
            created_here = false;
            t.clone()
        } else {
            let t = self
                .begin_txn()
                .map_err(|e| CatalogError::External(Box::new(e)))
                .map_err(E::from)?;
            self.current_txn = Some(t.clone());
            created_here = true;
            t
        };

        let result = f(txn_arc.as_ref());
        match result {
            Ok(v) => {
                if created_here {
                    txn_arc
                        .commit()
                        .map_err(|e| CatalogError::External(Box::new(e)))
                        .map_err(E::from)?;
                    self.clear_current_txn();
                }
                Ok(v)
            }
            Err(e) => {
                if created_here {
                    let _ = txn_arc.abort();
                    self.clear_current_txn();
                }
                Err(e)
            }
        }
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
