use crate::error::StorageResult;

/// Trait defining basic transaction operations.
pub trait StorageTransaction {
    type CommitTimestamp;

    /// Commit the current transaction, returning a commit timestamp on success.
    fn commit(&self) -> StorageResult<Self::CommitTimestamp>;

    /// Abort (rollback) the current transaction, discarding all changes.
    fn abort(&self) -> StorageResult<()>;
}
