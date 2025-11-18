use serde::{Deserialize, Serialize};

use super::super::error::GCardResult;

use super::function::{pointwise_function_min, pointwise_function_mult, PiecewiseConstantFunction};


pub type Pcf = PiecewiseConstantFunction;

pub fn alpha(pieces: &[Pcf]) -> Pcf {
    pointwise_function_mult(pieces)
}

pub fn alpha_refs(pieces: &[&Pcf]) -> Pcf {
    use super::function::pointwise_function_mult_refs;
    pointwise_function_mult_refs(pieces)
}

pub fn beta_left(rx: &Pcf, ry: &Pcf, sy: &Pcf) -> Pcf {
    calculate_non_joining_column_frequency(sy, ry, rx)
}

pub fn beta_right(rx: &Pcf, sx: &Pcf, sy: &Pcf) -> Pcf {
    calculate_non_joining_column_frequency(rx, sx, sy)
}

pub fn beta(rx: &Pcf, ry: &Pcf, sy: &Pcf, sz: &Pcf) -> (Pcf, Pcf) {
    (beta_left(rx, ry, sy), beta_right(ry, sy, sz))
}

fn project_child_via_parent(
    child: &PiecewiseConstantFunction,
    parent_from: &PiecewiseConstantFunction,
    parent_to: &PiecewiseConstantFunction,
) -> PiecewiseConstantFunction {
    if child.constants.is_empty()
        || parent_from.constants.is_empty()
        || parent_to.constants.is_empty()
    {
        return PiecewiseConstantFunction::empty();
    }

    let mut res_constants = Vec::new();
    let mut res_right_edges = Vec::new();
    let mut child_idx = 0;
    let mut child_const = child.constants[child_idx];
    let mut child_bound = child.right_interval_edges[child_idx];
    let mut x = 0.0;
    let mut cum = 0.0;

    for seg_idx in 0..parent_to.constants.len() {
        let seg_right = parent_to.right_interval_edges[seg_idx];
        let seg_const = parent_to.constants[seg_idx];
        let seg_cum_end = parent_to.cumulative_rows[seg_idx];

        while cum < seg_cum_end - 1e-9 {
            let target_rows = if child_idx < child.constants.len() {
                parent_from.calculate_rows_at_point(child_bound)
            } else {
                f64::INFINITY
            };

            let next_cum = seg_cum_end.min(target_rows);
            let next_cum = if next_cum.is_infinite() {
                seg_cum_end
            } else {
                next_cum
            };

            let width = if seg_const != 0.0 {
                (next_cum - cum) / seg_const
            } else {
                0.0
            };
            let next_x = seg_right.min(x + width);

            if next_x <= x + 1e-9 {
                res_constants.push(child_const);
                res_right_edges.push(seg_right);
                break;
            }

            res_constants.push(child_const);
            res_right_edges.push(next_x);
            x = next_x;
            cum = next_cum;

            if (cum - target_rows).abs() < 1e-6 {
                child_idx += 1;
                if child_idx < child.constants.len() {
                    child_const = child.constants[child_idx];
                    child_bound = child.right_interval_edges[child_idx];
                } else {
                    child_const = 0.0;
                    child_bound = f64::INFINITY;
                }
            }

            if (cum - seg_cum_end).abs() < 1e-6 {
                break;
            }
        }

        x = seg_right;
        cum = seg_cum_end;
    }

    let mut res_cumulative_rows = Vec::new();
    let mut cur_row = 0.0;
    let mut cur_left = 0.0;
    for i in 0..res_right_edges.len() {
        let cur_right = res_right_edges[i];
        cur_row += res_constants[i] * (cur_right - cur_left);
        res_cumulative_rows.push(cur_row);
        cur_left = cur_right;
    }

    PiecewiseConstantFunction {
        constants: res_constants,
        right_interval_edges: res_right_edges,
        cumulative_rows: res_cumulative_rows,
    }
}

fn maxreduce_edge(edge_a: &Pcf, edge_b: &Pcf) -> (Pcf, Pcf) {
    let max_a = edge_a.max_value();
    let max_b = edge_b.max_value();

    let mut new_a = edge_a.copy();
    let mut new_b = edge_b.copy();

    if max_a < max_b - 1e-9 {
        new_b = new_b.cap_constants(max_a);
    }
    if max_b < max_a - 1e-9 {
        new_a = new_a.cap_constants(max_b);
    }

    let total_a = new_a.get_num_rows();
    let total_b = new_b.get_num_rows();
    let aligned_total = total_a.min(total_b);

    let aligned_a = new_a.crop_to_total(aligned_total);
    let aligned_b = new_b.crop_to_total(aligned_total);

    (aligned_a, aligned_b)
}

pub fn calculate_non_joining_column_frequency(
    rx: &PiecewiseConstantFunction,
    sx: &PiecewiseConstantFunction,
    sy: &PiecewiseConstantFunction,
) -> PiecewiseConstantFunction {
    let mut sy_to_x_right_x = Vec::new();
    let mut sy_to_x_right_y = Vec::new();
    let mut sx_to_y_slope = Vec::new();

    let mut idx0 = 0;
    let mut idx1 = 0;
    let mut finished = false;

    while !finished {
        let new_row = sx.cumulative_rows[idx0].min(sy.cumulative_rows[idx1]);

        sy_to_x_right_x.push(sx.calculate_inverse(new_row));
        sy_to_x_right_y.push(sy.calculate_inverse(new_row));

        let slope = if sy.constants[idx1] != 0.0 {
            sx.constants[idx0] / sy.constants[idx1]
        } else {
            0.0
        };
        sx_to_y_slope.push(slope);

        if new_row >= sx.cumulative_rows[idx0] {
            idx0 += 1;
        }
        if new_row >= sy.cumulative_rows[idx1] {
            idx1 += 1;
        }
        if idx0 >= sx.cumulative_rows.len() || idx1 >= sy.cumulative_rows.len() {
            finished = true;
        }
    }

    if sy_to_x_right_x.is_empty() {
        return PiecewiseConstantFunction::empty();
    }

    let mut final_constants = Vec::new();
    let mut final_right_interval_edges = Vec::new();

    idx0 = 0;
    idx1 = 0;
    finished = false;

    while !finished {
        let new_x_val = rx.right_interval_edges[idx0].min(sy_to_x_right_x[idx1]);

        let mut left_y = 0.0;
        let mut left_x = 0.0;
        if idx1 > 0 {
            left_y = sy_to_x_right_y[idx1 - 1];
            left_x = sy_to_x_right_x[idx1 - 1];
        }

        let right_edge = left_y + (new_x_val - left_x) * sx_to_y_slope[idx1];
        final_right_interval_edges.push(right_edge);
        final_constants.push(rx.constants[idx0]);

        if new_x_val >= rx.right_interval_edges[idx0] {
            idx0 += 1;
        }
        if new_x_val >= sy_to_x_right_x[idx1] {
            idx1 += 1;
        }
        if idx0 >= rx.right_interval_edges.len() || idx1 >= sy_to_x_right_x.len() {
            finished = true;
        }
    }

    if final_constants.is_empty() {
        return PiecewiseConstantFunction::empty();
    }

    let mut final_cumulative_rows = Vec::new();
    let mut cur_row = 0.0;
    let mut cur_left = 0.0;

    for i in 0..final_right_interval_edges.len() {
        let cur_right = final_right_interval_edges[i];
        cur_row += final_constants[i] * (cur_right - cur_left);
        final_cumulative_rows.push(cur_row);
        cur_left = cur_right;
    }

    PiecewiseConstantFunction {
        constants: final_constants,
        right_interval_edges: final_right_interval_edges,
        cumulative_rows: final_cumulative_rows,
    }
}
