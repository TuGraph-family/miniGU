//! Filter mask trait for DiskANN search filtering

/// Filter mask trait for vector search filtering
/// Provides a simple interface for checking if a vector should be included in search results
pub trait FilterIndex: Send + Sync {
    /// Check if a vector ID should be included in the search results
    fn contains_vector(&self, vector_id: u32) -> bool;
}
