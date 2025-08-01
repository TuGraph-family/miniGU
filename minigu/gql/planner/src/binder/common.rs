use std::sync::Arc;

use gql_parser::ast::{
    GraphPattern, GraphPatternBindingTable, MatchMode, PathMode, PathPattern, PathPatternExpr,
    PathPatternPrefix,
};
use minigu_common::error::not_implemented;

use super::error::BindResult;
use crate::binder::Binder;
use crate::bound::{
    BoundGraphPattern, BoundMatchMode, BoundPathMode, BoundPathPattern, BoundPathPatternExpr,
};

impl Binder<'_> {
    pub fn bind_graph_pattern_binding_table(
        &mut self,
        table: &GraphPatternBindingTable,
    ) -> BindResult<BoundGraphPattern> {
        let graph = self.bind_graph_pattern(table.pattern.value())?;

        todo!()
    }

    pub fn bind_graph_pattern(&mut self, pattern: &GraphPattern) -> BindResult<BoundGraphPattern> {
        if pattern.keep.is_some() {
            return not_implemented("keep clause in graph pattern", None);
        }
        let match_mode = pattern
            .match_mode
            .as_ref()
            .map(|m| bind_match_mode(m.value()));
        todo!()
    }

    pub fn bind_path_pattern(
        &mut self,
        pattern: &PathPattern,
    ) -> BindResult<Arc<BoundPathPattern>> {
        let mode = pattern
            .prefix
            .as_ref()
            .map(|p| bind_path_pattern_prefix(p.value()))
            .transpose()?;
        let expr = self.bind_path_pattern_expr(pattern.expr.value())?;
        let path = Arc::new(BoundPathPattern { mode, expr });
        Ok(path)
    }

    pub fn bind_path_pattern_expr(
        &mut self,
        expr: &PathPatternExpr,
    ) -> BindResult<BoundPathPatternExpr> {
        todo!()
    }
}

pub fn bind_path_pattern_prefix(prefix: &PathPatternPrefix) -> BindResult<BoundPathMode> {
    match prefix {
        PathPatternPrefix::PathMode(mode) => Ok(bind_path_mode(mode)),
        PathPatternPrefix::PathSearch(_) => not_implemented("path search prefix", None),
    }
}

pub fn bind_path_mode(mode: &PathMode) -> BoundPathMode {
    match mode {
        PathMode::Walk => BoundPathMode::Walk,
        PathMode::Trail => BoundPathMode::Trail,
        PathMode::Simple => BoundPathMode::Simple,
        PathMode::Acyclic => BoundPathMode::Acyclic,
    }
}

pub fn bind_match_mode(mode: &MatchMode) -> BoundMatchMode {
    match mode {
        MatchMode::Repeatable => BoundMatchMode::Repeatable,
        MatchMode::Different => BoundMatchMode::Different,
    }
}
