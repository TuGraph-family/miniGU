pub mod config;
pub mod filter;
pub mod index;

#[cfg(not(target_family = "wasm"))]
pub mod in_mem_diskann;

#[cfg(target_family = "wasm")]
pub mod in_mem_naive;

#[cfg(not(target_family = "wasm"))]
pub use in_mem_diskann::InMemANNAdapter;
#[cfg(target_family = "wasm")]
pub use in_mem_naive::InMemANNAdapter;
pub use index::VectorIndex;
