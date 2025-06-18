use minigu_catalog::memory::MemoryCatalog;

pub struct DatabaseContext {
    catalog: MemoryCatalog,
}

impl DatabaseContext {
    pub fn new(catalog: MemoryCatalog) -> Self {
        Self { catalog }
    }

    pub fn catalog(&self) -> &MemoryCatalog {
        &self.catalog
    }
}
