use std::sync::Arc;

use gql_parser::ast::{
    ElementPattern, ElementPatternFiller, GraphPattern, GraphPatternBindingTable, LabelExpr,
    MatchMode, PathMode, PathPattern, PathPatternExpr, PathPatternPrefix,
};
use minigu_common::data_type::{DataField, DataSchema, LogicalType};
use minigu_common::error::not_implemented;

use super::error::{BindError, BindResult};
use crate::binder::Binder;
use crate::bound::{
    BoundElementPattern, BoundExpr, BoundGraphPattern, BoundGraphPatternBindingTable,
    BoundLabelExpr, BoundMatchMode, BoundPathMode, BoundPathPattern, BoundPathPatternExpr,
    BoundVertexPattern,
};

impl Binder<'_> {
    pub fn bind_graph_pattern_binding_table(
        &mut self,
        table: &GraphPatternBindingTable,
    ) -> BindResult<BoundGraphPatternBindingTable> {
        let bound_pattern = self.bind_graph_pattern(table.pattern.value())?;
        let cur_schema = self
            .active_data_schema
            .as_ref()
            .ok_or_else(|| BindError::Unexpected)?;
        let (outputs, output_schemas) = if table.yield_clause.is_empty() {
            let outs: Vec<BoundExpr> = cur_schema
                .fields()
                .iter()
                .map(|f| BoundExpr::variable(f.name().to_string(), f.ty().clone(), f.is_nullable()))
                .collect();
            (outs, cur_schema.clone())
        } else {
            let mut outs = Vec::with_capacity(table.yield_clause.len());
            let mut out_schema = DataSchema::new(Vec::new());
            for id_sp in &table.yield_clause {
                let name = id_sp.value().as_str();
                let f = cur_schema
                    .get_field_by_name(name)
                    .ok_or_else(|| BindError::Unexpected)?;
                outs.push(BoundExpr::variable(
                    name.to_string(),
                    f.ty().clone(),
                    f.is_nullable(),
                ));
                let field = DataField::new(name.to_string(), f.ty().clone(), f.is_nullable());
                out_schema.push_back(&field)
            }
            (outs, out_schema)
        };
        Ok(BoundGraphPatternBindingTable {
            pattern: bound_pattern,
            yield_clause: outputs,
            output_schema: output_schemas,
        })
    }

    pub fn bind_graph_pattern(&mut self, pattern: &GraphPattern) -> BindResult<BoundGraphPattern> {
        if pattern.keep.is_some() {
            return not_implemented("keep clause in graph pattern", None);
        }
        let match_mode = pattern
            .match_mode
            .as_ref()
            .map(|m| bind_match_mode(m.value()));

        let mut paths: Vec<Arc<BoundPathPattern>> = Vec::with_capacity(pattern.patterns.len());
        for path in pattern.patterns.iter() {
            let bound = self.bind_path_pattern(path.value())?;
            paths.push(bound);
        }

        let predicate: Option<BoundExpr> = match pattern.where_clause.as_ref() {
            Some(expr) => Some(self.bind_value_expression(expr.value())?),
            None => None,
        };

        Ok(BoundGraphPattern {
            match_mode,
            paths,
            predicate,
        })
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
            Concat(items) => {
                let mut bound_parts: Vec<BoundPathPatternExpr> = Vec::with_capacity(items.len());
                for it in items.iter() {
                    let child = self.bind_path_pattern_expr(it.value())?;
                    match child {
                        BoundPathPatternExpr::Concat(mut v) => bound_parts.extend(v),
                        other => bound_parts.push(other),
                    }
                }
                if bound_parts.is_empty() {
                    return Err(BindError::Unexpected);
                }

                if bound_parts.len() == 1 {
                    return Ok(bound_parts.pop().unwrap());
                }

                Ok(BoundPathPatternExpr::Concat(bound_parts))
            }
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
            ElementPattern::Edge { .. } => not_implemented("edge pattern", None),
        }
    }

    pub fn bind_label_expr(&mut self, expr: &LabelExpr) -> BindResult<BoundLabelExpr> {
        match expr {
            LabelExpr::Wildcard => Ok(BoundLabelExpr::Any),
            LabelExpr::Label(ident) => {
                let name = ident.as_str();
                let graph = self
                    .current_graph
                    .as_ref()
                    .ok_or_else(|| BindError::Unexpected)?;
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

    fn bind_vertex_filler(&mut self, f: &ElementPatternFiller) -> BindResult<BoundVertexPattern> {
        let var = match &f.variable {
            Some(var) => var.value().to_string(),
            // If the user didn't give a name, we will generate a name.
            None => {
                let idx = self
                    .active_data_schema
                    .as_ref()
                    .map(|s| s.size())
                    .unwrap_or(0);
                format!("__n{idx}")
            }
        };

        let vertex_ty =
            LogicalType::Vertex(vec![DataField::new("id".into(), LogicalType::Int64, false)]);
        self.register_variable(var.as_str(), vertex_ty, false)?;

        let label = match &f.label {
            Some(sp) => Some(self.bind_label_expr(sp.value())?),
            None => None,
        };
        let predicate = match &f.predicate {
            None => None,
            Some(sp) => None,
        };
        Ok(BoundVertexPattern {
            var,
            label,
            predicate,
        })
    }

    pub fn register_variable(
        &mut self,
        name: &str,
        ty: LogicalType,
        nullable: bool,
    ) -> BindResult<()> {
        if self.active_data_schema.is_none() {
            let schema = DataSchema::new(vec![DataField::new(name.to_string(), ty, nullable)]);
            self.active_data_schema = Some(schema);
            return Ok(());
        }
        let schema = self
            .active_data_schema
            .as_mut()
            .ok_or_else(|| BindError::Unexpected)?;
        if let Some(f) = schema.get_field_by_name(name) {
            if f.ty() != &ty {
                return Err(BindError::Unexpected);
            }
            if f.is_nullable() && !nullable {
                return Err(BindError::Unexpected);
            }
            Ok(())
        } else {
            let data_schema = DataSchema::new(vec![DataField::new(name.to_string(), ty, nullable)]);
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
