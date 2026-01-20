use miette::Diagnostic;
#[cfg(not(target_family = "wasm"))]
use rayon::ThreadPool;
#[cfg(not(target_family = "wasm"))]
use rayon::ThreadPoolBuilder;
use thiserror::Error;

#[cfg(target_family = "wasm")]
#[derive(Debug, Clone, Default)]
pub struct DatabaseRuntime;

#[cfg(not(target_family = "wasm"))]
#[derive(Debug)]
pub struct DatabaseRuntime {
    pool: ThreadPool,
}

#[cfg(target_family = "wasm")]
impl DatabaseRuntime {
    #[inline]
    pub fn new(_num_threads: usize) -> Result<Self, RuntimeError> {
        Ok(Self)
    }

    #[inline]
    pub fn install<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        f()
    }
}

#[cfg(not(target_family = "wasm"))]
impl DatabaseRuntime {
    pub fn new(num_threads: usize) -> Result<Self, RuntimeError> {
        let pool = ThreadPoolBuilder::new().num_threads(num_threads).build()?;
        Ok(Self { pool })
    }

    #[inline]
    pub fn install<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R + Send,
        R: Send,
    {
        self.pool.install(f)
    }
}

#[cfg(target_family = "wasm")]
#[derive(Debug, Error, Diagnostic)]
#[error("runtime error")]
pub struct RuntimeError;

#[cfg(not(target_family = "wasm"))]
#[derive(Debug, Error, Diagnostic)]
pub enum RuntimeError {
    #[error("rayon error")]
    Rayon(#[from] rayon::ThreadPoolBuildError),
}
