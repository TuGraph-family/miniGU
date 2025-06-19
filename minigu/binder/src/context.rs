use indexmap::IndexMap;
use minigu_ir::bound::BoundExpr;
use smol_str::SmolStr;

#[derive(Debug, Default)]
pub struct BindContext {
    name_to_expr: IndexMap<SmolStr, BoundExpr>,
}

impl BindContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn exprs(&self) -> impl Iterator<Item = &BoundExpr> {
        self.name_to_expr.values()
    }

    pub fn contains(&self, name: &str) -> bool {
        self.name_to_expr.contains_key(name)
    }

    pub fn insert(&mut self, name: SmolStr, expr: BoundExpr) {
        self.name_to_expr.insert(name, expr);
    }

    pub fn get(&self, name: &str) -> Option<&BoundExpr> {
        self.name_to_expr.get(name)
    }
}
