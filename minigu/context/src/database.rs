use minigu_catalog::memory::MemoryCatalog;
use rayon::ThreadPool;

pub struct DatabaseContext {
    catalog: MemoryCatalog,
    runtime: ThreadPool,
}

impl DatabaseContext {
    pub fn new(catalog: MemoryCatalog, runtime: ThreadPool) -> Self {
        Self { catalog, runtime }
    }

    #[inline]
    pub fn catalog(&self) -> &MemoryCatalog {
        &self.catalog
    }

    #[inline]
    pub fn runtime(&self) -> &ThreadPool {
        &self.runtime
    }
}
