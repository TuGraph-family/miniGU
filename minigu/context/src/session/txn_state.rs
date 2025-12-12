use std::sync::Arc;

use minigu_catalog::error::CatalogError;
use minigu_catalog::txn::catalog_txn::CatalogTxn;
use minigu_catalog::txn::error::CatalogTxnResult;
use minigu_catalog::txn::manager::CatalogTxnManager;
use minigu_transaction::{GraphTxnManager, IsolationLevel, Transaction};

/// Session-scoped transaction state machine handling explicit and implicit (auto-commit) txns.
#[derive(Clone, Debug)]
pub struct SessionTxnState {
    pub(crate) catalog_txn_mgr: CatalogTxnManager,
    explicit_txn: Option<Arc<CatalogTxn>>,
    implicit_txn: Option<Arc<CatalogTxn>>,
    isolation_level: Option<IsolationLevel>,
}

impl SessionTxnState {
    pub fn new() -> Self {
        Self {
            catalog_txn_mgr: CatalogTxnManager::new(),
            explicit_txn: None,
            implicit_txn: None,
            isolation_level: None,
        }
    }

    /// Set default isolation level for implicitly created transactions.
    #[inline]
    pub fn set_default_isolation(&mut self, iso: IsolationLevel) {
        self.isolation_level = Some(iso);
    }

    /// Returns the active transaction without creating a new one (explicit preferred).
    pub fn current_txn(&self) -> Option<Arc<CatalogTxn>> {
        if let Some(t) = &self.explicit_txn {
            return Some(t.clone());
        }
        self.implicit_txn.clone()
    }

    /// Begin an explicit, session-scoped transaction. Errors if another transaction is active.
    pub fn begin_explicit_txn(&mut self) -> CatalogTxnResult<()> {
        if self.explicit_txn.is_some() || self.implicit_txn.is_some() {
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
        let (txn_arc, created_here) = self.implicit_or_new_txn()?;

        // Execute the function within the transaction.
        let result = f(txn_arc.as_ref());
        match result {
            Ok(v) => {
                if created_here {
                    txn_arc.commit()?;
                    self.clear_implicit_txn();
                }
                Ok(v)
            }
            Err(e) => {
                if created_here {
                    let _ = txn_arc.abort();
                    self.clear_implicit_txn();
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
        let (txn_arc, created_here) = self
            .implicit_or_new_txn()
            .map_err(|e| CatalogError::External(Box::new(e)))
            .map_err(E::from)?;

        let result = f(txn_arc.as_ref());
        match result {
            Ok(v) => {
                if created_here {
                    txn_arc
                        .commit()
                        .map_err(|e| CatalogError::External(Box::new(e)))
                        .map_err(E::from)?;
                    self.clear_implicit_txn();
                }
                Ok(v)
            }
            Err(e) => {
                if created_here {
                    let _ = txn_arc.abort();
                    self.clear_implicit_txn();
                }
                Err(e)
            }
        }
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

    fn implicit_or_new_txn(&mut self) -> CatalogTxnResult<(Arc<CatalogTxn>, bool)> {
        if let Some(t) = &self.implicit_txn {
            return Ok((t.clone(), false));
        }
        let t = self.begin_txn()?;
        self.implicit_txn = Some(t.clone());
        Ok((t, true))
    }

    /// Clear the current implicit transaction reference (without committing or rolling back).
    /// This is used in auto-commit mode, where the transaction is committed successfully,
    /// and the `implicit_txn` is removed to avoid mistakenly reusing the committed transaction.
    #[inline]
    fn clear_implicit_txn(&mut self) {
        self.implicit_txn = None;
    }
}
