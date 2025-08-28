// Vector index modules - Linux only with vector-support feature
#[cfg(all(target_os = "linux", feature = "vector-support"))]
pub mod filter;
#[cfg(all(target_os = "linux", feature = "vector-support"))]
pub mod in_mem_diskann;
#[cfg(all(target_os = "linux", feature = "vector-support"))]
pub mod index;

#[cfg(all(target_os = "linux", feature = "vector-support"))]
pub use in_mem_diskann::InMemANNAdapter;
#[cfg(all(target_os = "linux", feature = "vector-support"))]
pub use index::VectorIndex;
