use std::sync::Arc;

use gql_parser::ast::{
    EdgePatternKind, ElementPattern, ElementPatternFiller, GraphPattern, GraphPatternBindingTable,
    LabelExpr, MatchMode, PathMode, PathPattern, PathPatternExpr, PathPatternPrefix,
};
use minigu_common::data_type::{DataField, DataSchema, LogicalType};
use minigu_common::error::{NotImplemented, not_implemented};
use minigu_common::types::LabelId;
use smol_str::ToSmolStr;

use super::error::{BindError, BindResult};
use crate::binder::Binder;
use crate::bound::BoundLabelExpr::Any;
use crate::bound::{
    BoundEdgePattern, BoundEdgePatternKind, BoundElementPattern, BoundExpr, BoundGraphPattern,
    BoundGraphPatternBindingTable, BoundLabelExpr, BoundMatchMode, BoundPathMode, BoundPathPattern,
    BoundPathPatternExpr, BoundVertexPattern,
};

pub fn lower_label_expr_to_specs(expr: &BoundLabelExpr) -> Vec<Vec<LabelId>> {
    use BoundLabelExpr::*;
    match expr {
        Any => vec![vec![]],
        Label(id) => vec![vec![*id]],

        // Disjunction => concatenate routes
        Disjunction(lhs, rhs) => {
            let mut a = lower_label_expr_to_specs(lhs);
            let mut b = lower_label_expr_to_specs(rhs);
            a.append(&mut b);
            a
        }

        // Conjunction => distributive product of routes, merging inner AND sets
        Conjunction(lhs, rhs) => {
            let left = lower_label_expr_to_specs(lhs);
            let right = lower_label_expr_to_specs(rhs);
            let mut out: Vec<Vec<LabelId>> = Vec::with_capacity(left.len() * right.len());
            for l in &left {
                for r in &right {
                    let mut merged = Vec::with_capacity(l.len() + r.len());
                    merged.extend_from_slice(l);
                    merged.extend_from_slice(r);
                    merged.sort_unstable();
                    merged.dedup();
                    out.push(merged);
                }
            }
            out
        }
        Negation(_) => not_implemented("Label negation is not supported in lower_label_expr_to_specs"),
    }
}

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
            ElementPattern::Edge { kind, filler } => match kind {
                EdgePatternKind::Any => not_implemented("any edge pattern", None),
                EdgePatternKind::Left => not_implemented("left edge pattern", None),
                EdgePatternKind::LeftRight => not_implemented("left-right edge pattern", None),
                EdgePatternKind::LeftUndirected => {
                    not_implemented("left undirected edge pattern", None)
                }
                EdgePatternKind::Right => {
                    let edge = self.bind_edge_filler(filler, kind)?;
                    Ok(BoundElementPattern::Edge(Arc::new(edge)))
                }
                EdgePatternKind::RightUndirected => {
                    not_implemented("right undirected edge pattern", None)
                }
                EdgePatternKind::Undirected => not_implemented("undirected edge pattern", None),
            },
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
                    .ok_or_else(|| BindError::CurrentGraphNotSpecified)?;
                let id = graph
                    .graph_type()
                    .get_label_id(name)?
                    .ok_or_else(|| BindError::LabelNotFound(name.to_smolstr()))?;
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

    fn bind_edge_filler(
        &mut self,
        f: &ElementPatternFiller,
        kind: &EdgePatternKind,
    ) -> BindResult<BoundEdgePattern> {
        let var = match &f.variable {
            Some(var) => var.value().to_string(),
            None => {
                let idx = self
                    .active_data_schema
                    .as_ref()
                    .map(|s| s.size())
                    .unwrap_or(0);
                format!("__e{idx}")
            }
        };

        if f.predicate.is_some() {
            return Err(BindError::NotImplemented(NotImplemented::new(
                "predicate".to_string(),
                None.into(),
            )));
        }
        let edge_ty =
            LogicalType::Edge(vec![DataField::new("id".into(), LogicalType::Int64, false)]);
        self.register_variable(var.as_str(), edge_ty, false)?;
        let label = match &f.label {
            Some(sp) => Some(self.bind_label_expr(sp.value())?),
            None => None,
        };

        let label_set = if let Some(label_val) = label.as_ref() {
            lower_label_expr_to_specs(label_val)
        } else {
            vec![vec![]]
        };

        self.register_variable_labels(var.as_str(), &label_set);

        let kind: BoundEdgePatternKind = BoundEdgePatternKind::from(kind);
        Ok(BoundEdgePattern {
            var: Some(var),
            kind,
            label: label_set,
            predicate: None,
        })
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
        let label_set = if let Some(label_val) = label.as_ref() {
            lower_label_expr_to_specs(label_val)
        } else {
            vec![vec![]]
        };

        self.register_variable_labels(var.as_str(), &label_set);

        if f.predicate.is_some() {
            return Err(BindError::NotImplemented(NotImplemented::new(
                "predicate".to_string(),
                None.into(),
            )));
        }

        let predicate = match &f.predicate {
            None => None,
            Some(sp) => None,
        };

        Ok(BoundVertexPattern {
            var,
            label: label_set,
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

    pub fn register_variable_labels(
        &mut self,
        name: &str,
        labels: &[Vec<LabelId>],
    ) -> BindResult<()> {
        if self.active_data_schema.is_none() {
            return Err(BindError::Unexpected);
        }
        let mut schema = self
            .active_data_schema
            .clone()
            .expect("schema should be initialized");
        schema.set_var_label(name.to_string(), labels.to_owned());
        Ok(())
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
