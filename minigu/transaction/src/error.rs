use thiserror::Error;

#[derive(Error, Debug)]
pub enum TimestampError {
    #[error("expected commit-ts, but got txn-id ({0})")]
    WrongDomainCommit(u64),

    #[error("expected txn-id, but got commit-ts ({0})")]
    WrongDomainTxnId(u64),

    #[error("commit-ts overflow, reached {0}")]
    CommitTsOverflow(u64),

    #[error("txn-id overflow, reached {0}")]
    TxnIdOverflow(u64),
}
