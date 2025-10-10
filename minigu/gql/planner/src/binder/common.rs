use std::sync::Arc;

use gql_parser::ast::{ElementPattern, ElementPatternFiller, GraphPattern, GraphPatternBindingTable, LabelExpr, MatchMode, PathMode, PathPattern, PathPatternExpr, PathPatternPrefix};
use minigu_common::data_type::{DataField, DataSchema, LogicalType};
use minigu_common::error::not_implemented;

use super::error::{BindError, BindResult};
use crate::binder::Binder;
use crate::bound::{BoundElementPattern, BoundExpr, BoundGraphPattern, BoundLabelExpr, BoundMatchMode, BoundPathMode, BoundPathPattern, BoundPathPatternExpr, BoundVertexPattern};

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
        use PathPatternExpr::*;
        match expr {
            Union(_) => not_implemented("union expression", None),
            Alternation(_) => not_implemented("alternate expression", None),
            Concat(_) => not_implemented("concat expression", None),
            Quantified { .. } => not_implemented("quantified expression", None),
            Optional(_) => not_implemented("optional expression", None),
            Grouped(_) => not_implemented("grouped expression", None),
            Pattern(elem) => {
                let p = self.bind_element_pattern(elem)?;
                Ok(BoundPathPatternExpr::Pattern(p))
            }
        }
    }

    pub fn bind_element_pattern(
        &mut self,
        elem: &ElementPattern,
    ) -> BindResult<BoundElementPattern> {
        match elem {
            ElementPattern::Node(filler) => {
                let v = self.bind_vertex_filler(filler)?;
                Ok(BoundElementPattern::Vertex(Arc::new(v)))
            }
            ElementPattern::Edge { .. } => {
                not_implemented("edge pattern", None)
            }
        }
    }
    pub fn bind_label_expr(
        &mut self,
        expr: &LabelExpr,
    ) -> BindResult<BoundLabelExpr> {
        match expr {
            LabelExpr::Wildcard => Ok(BoundLabelExpr::Any),
            LabelExpr::Label(ident) => {
                let name = ident.as_str();
                let graph = self.current_graph.as_ref().ok_or_else(
                    || BindError::Unexpected
                )?;
                // To handle. 
                let id = graph.graph_type().get_label_id(name)?.unwrap();
                Ok(BoundLabelExpr::Label(id))
            }
            LabelExpr::Negation(inner) => {
                let child = self.bind_label_expr(inner.value())?;
                Ok(BoundLabelExpr::Negation(Box::new(child)))
            }
            LabelExpr::Conjunction(lhs, rhs) => {
                let l = self.bind_label_expr(lhs.value())?;
                let r = self.bind_label_expr(rhs.value())?;
                Ok(BoundLabelExpr::Conjunction(Box::new(l), Box::new(r)))
            }
            LabelExpr::Disjunction(lhs, rhs) => {
                let l = self.bind_label_expr(lhs.value())?;
                let r = self.bind_label_expr(rhs.value())?;
                Ok(BoundLabelExpr::Disjunction(Box::new(l), Box::new(r)))
            }
        }
    }

    fn bind_vertex_filler(
        &mut self,
        f: &ElementPatternFiller,
    ) -> BindResult<BoundVertexPattern> {
        if let Some(var) = &f.variable {
            let name = var.value().to_string();
            let vertex_ty = LogicalType::Vertex(
                vec![DataField::new("id".into(), LogicalType::Int64, false)],
            );
            self.register_variable(name.as_str(), vertex_ty, false)?;
        }
        let label = match &f.label {
            Some(sp) => Some(self.bind_label_expr(sp.value())?),
            None => None,
        };
        let predicate = match &f.predicate {
            None => None,
            Some(sp) => None
        };
        Ok(BoundVertexPattern {
            label,
            predicate
        })
    }

    pub fn register_variable(
        &mut self,
        name: &str,
        ty: LogicalType,
        nullable: bool,
    ) -> BindResult<()> {
        let schema = self.active_data_schema.as_mut().ok_or_else(|| BindError::Unexpected)?;
        if let Some(f) = schema.get_field_by_name(name) {
            if f.ty() != &ty {
                return Err(BindError::Unexpected);
            }
            if f.is_nullable() && !nullable {
                return Err(BindError::Unexpected);
            }
            Ok(())
        } else {
            let data_schema = DataSchema::new(
                vec![DataField::new(name.to_string(), ty, nullable)]
            );
            schema.append(&data_schema);
            Ok(())
        }
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
