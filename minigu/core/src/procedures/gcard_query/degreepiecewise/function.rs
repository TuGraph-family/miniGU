use std::fmt;
use serde::{Deserialize, Serialize};

use super::super::error::GCardResult;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PiecewiseConstantFunction {
    pub constants: Vec<f64>,
    pub right_interval_edges: Vec<f64>,
    pub cumulative_rows: Vec<f64>,
}

impl fmt::Display for PiecewiseConstantFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.constants.len();

        writeln!(f, "PiecewiseConstantFunction {{")?;
        writeln!(f, "  segments: {}", n)?;

        let mut left = f64::NEG_INFINITY;

        for i in 0..n {
            let right = self.right_interval_edges[i];
            let c = self.constants[i];
            let cum = self.cumulative_rows[i];

            writeln!(
                f,
                "  [{:>8.3}, {:>8.3})  value = {:>8.4},  cumulative = {:>8.4}",
                left, right, c, cum
            )?;

            left = right;
        }

        writeln!(f, "}}")
    }
}

impl PiecewiseConstantFunction {
    pub fn empty() -> Self {
        Self {
            right_interval_edges: vec![1.0],
            cumulative_rows: vec![1.0],
            constants: vec![0.0001],
        }
    }

    pub fn from_degree_sequence(
        data: &[u64],
        relative_error_per_segment: f64,
        model_cdf: bool,
    ) -> GCardResult<Self> {
        if data.is_empty() {
            return Ok(Self::empty());
        }

        let total_rows: f64 = data.iter().sum::<u64>() as f64;
        let bin_right_edges = calculate_bins_relative_error(data, relative_error_per_segment)?;

        let mut processed_edges = Vec::new();
        let data_reversed: Vec<u64> = data.iter().rev().copied().collect();
        for &edge in &bin_right_edges {
            if edge > 0 && edge <= data.len() {
                let value = data[edge - 1];
                let pos = bisect_left_long(&data_reversed, value as f64);
                let actual_edge = data.len() - pos;
                processed_edges.push(actual_edge);
            }
        }
        processed_edges.sort_unstable();
        processed_edges.dedup();
        if processed_edges.is_empty() {
            processed_edges.push(data.len());
        }

        let mut constants = Vec::new();
        let mut right_interval_edges = Vec::new();
        let mut cumulative_rows = Vec::new();

        let mut counter = 0;
        let mut cur_left_data = 0;
        let mut cur_right_edge = 0.0;
        let mut cur_row = 0.0;

        for &cur_right_data in &processed_edges {
            if cur_left_data == cur_right_data {
                continue;
            }

            if model_cdf {
                let rows_needed =
                    (cur_right_data - cur_left_data) as f64 * data[cur_left_data] as f64;
                if cur_row + rows_needed >= total_rows {
                    let remaining_rows = total_rows - cur_row;
                    let remaining_values = remaining_rows / data[cur_left_data] as f64;
                    cur_right_edge += remaining_values.ceil();
                    right_interval_edges.push(cur_right_edge);
                    cumulative_rows.push(total_rows);
                    constants.push(data[cur_left_data] as f64);
                    counter += 1;
                    break;
                }

                let rows_in_segment: u64 = data[cur_left_data..cur_right_data].iter().sum();
                cur_row += rows_in_segment as f64;
                cur_right_edge += rows_in_segment as f64 / data[cur_left_data] as f64;
                right_interval_edges.push(cur_right_edge);
                cumulative_rows.push(cur_row);
                constants.push(data[cur_left_data] as f64);
                cur_left_data = cur_right_data;
                counter += 1;
            } else {
                right_interval_edges.push(cur_right_data as f64);
                constants.push(data[cur_left_data] as f64);
                cur_row += data[cur_left_data] as f64 * (cur_right_data - cur_left_data) as f64;
                cumulative_rows.push(cur_row);
                cur_left_data = cur_right_data;
                counter += 1;
            }
        }

        right_interval_edges.truncate(counter);
        cumulative_rows.truncate(counter);
        constants.truncate(counter);

        Ok(Self {
            constants,
            right_interval_edges,
            cumulative_rows,
        })
    }

    pub fn multiplicity_solo(&self) -> bool {
        if self.constants.is_empty() {
            return true;
        }
        if self.constants[0] == 1.0 {
            return true
        }
        false
    }


    pub fn get_num_rows(&self) -> f64 {
        self.cumulative_rows.last().copied().unwrap_or(0.0)
    }

    pub fn truncate_by_ratio(&self, ratio: f64) -> Self {
        if ratio < 0.0 || ratio > 1.0 || self.constants.is_empty() {
            return Self::empty();
        }

        let total_rows = self.get_num_rows();
        let target_rows = ratio * total_rows;
        
        if target_rows <= 0.0 {
            return Self::empty();
        }

        let mut new_constants = Vec::new();
        let mut new_right_edges = Vec::new();
        let mut left = 0.0;
        let mut accumulated = 0.0;
        const TOL: f64 = 1e-9;

        for i in 0..self.constants.len() {
            let right = self.right_interval_edges[i];
            let width = (right - left).max(0.0);
            let contrib = self.constants[i] * width;

            if accumulated + contrib <= target_rows + TOL {
                new_constants.push(self.constants[i]);
                new_right_edges.push(right);
                accumulated += contrib;
                left = right;
                if (accumulated - target_rows).abs() < 1e-6 {
                    break;
                }
                continue;
            }

            let needed = target_rows - accumulated;
            if self.constants[i] != 0.0 && needed > TOL {
                let new_width = needed / self.constants[i];
                new_constants.push(self.constants[i]);
                new_right_edges.push(left + new_width);
            }
            break;
        }
        let mut new_cumulative_rows = Vec::new();
        let mut cur_row = 0.0;
        let mut cur_left = 0.0;
        for i in 0..new_right_edges.len() {
            let cur_right = new_right_edges[i];
            cur_row += new_constants[i] * (cur_right - cur_left);
            new_cumulative_rows.push(cur_row);
            cur_left = cur_right;
        }

        Self {
            constants: new_constants,
            right_interval_edges: new_right_edges,
            cumulative_rows: new_cumulative_rows,
        }
    }

    pub fn calculate_value_at_point(&self, x: f64) -> f64 {
        let segment = bisect_left_double(&self.right_interval_edges, x)
            .min(self.constants.len().saturating_sub(1));
        self.constants[segment]
    }

    pub fn calculate_rows_at_point(&self, x: f64) -> f64 {
        let x = x.min(*self.right_interval_edges.last().unwrap_or(&0.0));
        let segment = bisect_left_double(&self.right_interval_edges, x)
            .min(self.constants.len().saturating_sub(1));

        let mut cumulative_rows = 0.0;
        let mut left_val = 0.0;

        if segment > 0 {
            cumulative_rows = self.cumulative_rows[segment - 1];
            left_val = self.right_interval_edges[segment - 1];
        }

        let constant = self.constants[segment];
        cumulative_rows + (x - left_val) * constant
    }

    pub fn calculate_inverse(&self, y: f64) -> f64 {
        if self.cumulative_rows.is_empty() {
            return 0.0;
        }

        let segment = bisect_left_double(&self.cumulative_rows, y)
            .min(self.constants.len().saturating_sub(1));

        let mut rows_into_cur_segment = y;
        let mut left_val = 0.0;

        if segment > 0 {
            rows_into_cur_segment -= self.cumulative_rows[segment - 1];
            left_val = self.right_interval_edges[segment - 1];
        }

        let constant = self.constants[segment];
        if constant == 0.0 {
            return left_val;
        }

        left_val + rows_into_cur_segment / constant
    }

    pub fn get_self_join_size(&self) -> f64 {
        let mut self_join_size = 0.0;
        let mut left = 0.0;
        for i in 0..self.constants.len() {
            let right = self.right_interval_edges[i];
            self_join_size += (right - left) * self.constants[i] * self.constants[i];
            left = right;
        }
        self_join_size
    }

    pub fn compress_func(&mut self, num_segments: Option<usize>) {
        if let Some(num_seg) = num_segments {
            if num_seg > 0 {
                let max_edge = self.right_interval_edges.last().copied().unwrap_or(1.0);
                let bins = calculate_bins_exponential(max_edge.ceil() as usize, num_seg);
                let bins_f64: Vec<f64> = bins.iter().map(|&x| x as f64).collect();

                let mut new_constants = Vec::new();
                let mut new_right_edges = Vec::new();
                let mut new_cumulative_rows = Vec::new();

                let max_rows = self.cumulative_rows.last().copied().unwrap_or(0.0);
                let mut left = 0.0;
                let mut cumulative_rows = 0.0;

                for &right in &bins_f64 {
                    if right <= left {
                        continue;
                    }
                    let constant = self.calculate_value_at_point(left);
                    if cumulative_rows + constant * (right - left) > max_rows {
                        let adjusted_right = left + (max_rows - cumulative_rows) / constant;
                        new_cumulative_rows.push(max_rows);
                        new_right_edges.push(adjusted_right);
                        new_constants.push(constant);
                        break;
                    }
                    cumulative_rows += constant * (right - left);
                    new_cumulative_rows.push(cumulative_rows);
                    new_right_edges.push(right);
                    new_constants.push(constant);
                    left = right;
                }

                self.constants = new_constants;
                self.right_interval_edges = new_right_edges;
                self.cumulative_rows = new_cumulative_rows;
            }
        }

        let mut new_constants = Vec::new();
        let mut new_right_edges = Vec::new();
        let mut new_cumulative_rows = Vec::new();

        for i in 0..self.constants.len() {
            if i == 0 || ((self.constants[i] - self.constants[i - 1]).abs() > f64::EPSILON) {
                new_constants.push(self.constants[i]);
                new_right_edges.push(self.right_interval_edges[i]);
                new_cumulative_rows.push(self.cumulative_rows[i]);
            } else {
                if let Some(last) = new_right_edges.last_mut() {
                    *last = self.right_interval_edges[i];
                }
                if let Some(last) = new_cumulative_rows.last_mut() {
                    *last = self.cumulative_rows[i];
                }
            }
        }

        self.constants = new_constants;
        self.right_interval_edges = new_right_edges;
        self.cumulative_rows = new_cumulative_rows;
    }

    pub fn copy(&self) -> Self {
        Self {
            constants: self.constants.clone(),
            right_interval_edges: self.right_interval_edges.clone(),
            cumulative_rows: self.cumulative_rows.clone(),
        }
    }

    pub fn max_value(&self) -> f64 {
        if self.constants.is_empty() {
            return 0.0;
        }
        self.constants[0]
    }

    pub fn min_value(&self) -> f64 {
        if self.constants.is_empty() {
            return 0.0;
        }
        *self.constants.last().unwrap()
    }

    pub fn cap_constants(&self, max_constant: f64) -> Self {
        if max_constant <= 0.0 {
            return Self::empty();
        }
        let new_constants: Vec<f64> = self
            .constants
            .iter()
            .map(|&c| c.min(max_constant))
            .collect();

        let mut new_cumulative_rows = Vec::new();
        let mut cur_row = 0.0;
        let mut cur_left = 0.0;
        for i in 0..new_constants.len() {
            let cur_right = self.right_interval_edges[i];
            cur_row += new_constants[i] * (cur_right - cur_left);
            new_cumulative_rows.push(cur_row);
            cur_left = cur_right;
        }

        Self {
            constants: new_constants,
            right_interval_edges: self.right_interval_edges.clone(),
            cumulative_rows: new_cumulative_rows,
        }
    }

    pub fn crop_to_total(&self, target_total: f64) -> Self {
        if target_total <= 0.0 || self.constants.is_empty() {
            return Self::empty();
        }
        let target_total = target_total.min(self.get_num_rows());
        let mut new_constants = Vec::new();
        let mut new_right_edges = Vec::new();
        let mut left = 0.0;
        let mut accumulated = 0.0;
        const TOL: f64 = 1e-9;

        for i in 0..self.constants.len() {
            let right = self.right_interval_edges[i];
            let width = (right - left).max(0.0);
            let contrib = self.constants[i] * width;

            if accumulated + contrib <= target_total + TOL {
                new_constants.push(self.constants[i]);
                new_right_edges.push(right);
                accumulated += contrib;
                left = right;
                if (accumulated - target_total).abs() < 1e-6 {
                    break;
                }
                continue;
            }

            let needed = target_total - accumulated;
            if self.constants[i] != 0.0 && needed > TOL {
                let new_width = needed / self.constants[i];
                new_constants.push(self.constants[i]);
                new_right_edges.push(left + new_width);
            }
            break;
        }

        let mut new_cumulative_rows = Vec::new();
        let mut cur_row = 0.0;
        let mut cur_left = 0.0;
        for i in 0..new_right_edges.len() {
            let cur_right = new_right_edges[i];
            cur_row += new_constants[i] * (cur_right - cur_left);
            new_cumulative_rows.push(cur_row);
            cur_left = cur_right;
        }

        Self {
            constants: new_constants,
            right_interval_edges: new_right_edges,
            cumulative_rows: new_cumulative_rows,
        }
    }
}

pub fn pointwise_function_mult_refs(
    functions: &[&PiecewiseConstantFunction],
) -> PiecewiseConstantFunction {
    if functions.is_empty() {
        return PiecewiseConstantFunction::empty();
    }
    if functions.len() == 1 {
        return functions[0].copy();
    }

    let mut indices = vec![0usize; functions.len()];
    let mut function_has_space: Vec<bool> = functions
        .iter()
        .map(|f| !f.right_interval_edges.is_empty())
        .collect();

    let mut left = 0.0;
    let mut cur_cumulative_rows = 0.0;
    let mut right_edges = Vec::new();
    let mut constants = Vec::new();
    let mut cumulative_rows = Vec::new();

    while function_has_space.iter().all(|&has_space| has_space) {
        let mut min_right = f64::INFINITY;
        for (i, func) in functions.iter().enumerate() {
            if function_has_space[i] {
                let current_edge = func.right_interval_edges[indices[i]];
                min_right = min_right.min(current_edge);
            }
        }

        if min_right == f64::INFINITY {
            break;
        }

        let right = min_right;

        let mut constant = 1.0;
        for (i, func) in functions.iter().enumerate() {
            if function_has_space[i] {
                constant *= func.constants[indices[i]];
            }
        }

        if constant == 0.0 {
            constant = 0.0001;
        }

        cur_cumulative_rows += (right - left) * constant;
        right_edges.push(right);
        constants.push(constant);
        cumulative_rows.push(cur_cumulative_rows);

        for (i, func) in functions.iter().enumerate() {
            if function_has_space[i] && func.right_interval_edges[indices[i]] <= right {
                indices[i] += 1;
            }
        }

        for (i, func) in functions.iter().enumerate() {
            function_has_space[i] = indices[i] < func.right_interval_edges.len();
        }

        left = right;
    }

    let mut func = PiecewiseConstantFunction {
        constants,
        right_interval_edges: right_edges,
        cumulative_rows,
    };

    func.compress_func(None);
    func
}

pub fn pointwise_function_mult(
    functions: &[PiecewiseConstantFunction],
) -> PiecewiseConstantFunction {
    if functions.is_empty() {
        return PiecewiseConstantFunction::empty();
    }
    if functions.len() == 1 {
        return functions[0].copy();
    }

    let mut indices = vec![0usize; functions.len()];
    let mut function_has_space: Vec<bool> = functions
        .iter()
        .map(|f| !f.right_interval_edges.is_empty())
        .collect();

    let mut left = 0.0;
    let mut cur_cumulative_rows = 0.0;
    let mut right_edges = Vec::new();
    let mut constants = Vec::new();
    let mut cumulative_rows = Vec::new();

    while function_has_space.iter().all(|&has_space| has_space) {
        let mut min_right = f64::INFINITY;
        for (i, func) in functions.iter().enumerate() {
            if function_has_space[i] {
                let current_edge = func.right_interval_edges[indices[i]];
                min_right = min_right.min(current_edge);
            }
        }

        if min_right == f64::INFINITY {
            break;
        }

        let right = min_right;

        let mut constant = 1.0;
        for (i, func) in functions.iter().enumerate() {
            if function_has_space[i] {
                constant *= func.constants[indices[i]];
            }
        }

        if constant == 0.0 {
            constant = 0.0001;
        }

        cur_cumulative_rows += (right - left) * constant;
        right_edges.push(right);
        constants.push(constant);
        cumulative_rows.push(cur_cumulative_rows);

        for (i, func) in functions.iter().enumerate() {
            if function_has_space[i] && func.right_interval_edges[indices[i]] <= right {
                indices[i] += 1;
            }
        }

        for (i, func) in functions.iter().enumerate() {
            function_has_space[i] = indices[i] < func.right_interval_edges.len();
        }

        left = right;
    }

    let mut func = PiecewiseConstantFunction {
        constants,
        right_interval_edges: right_edges,
        cumulative_rows,
    };

    func.compress_func(None);
    func
}

pub fn pointwise_function_min(
    functions: &[PiecewiseConstantFunction],
) -> PiecewiseConstantFunction {
    if functions.is_empty() {
        return PiecewiseConstantFunction::empty();
    }
    if functions.len() == 1 {
        return functions[0].copy();
    }

    let mut indices = vec![0usize; functions.len()];
    let mut function_has_space: Vec<bool> = functions
        .iter()
        .map(|f| !f.right_interval_edges.is_empty())
        .collect();

    let mut left = 0.0;
    let mut prev_cumulative_rows = 0.0;
    let mut cumulative_rows_per_function = vec![0.0; functions.len()];
    let mut right_edges = Vec::new();
    let mut constants = Vec::new();
    let mut cumulative_rows = Vec::new();

    let min_row_count = functions
        .iter()
        .map(|f| f.get_num_rows())
        .fold(f64::INFINITY, f64::min);

    while function_has_space.iter().any(|&has_space| has_space) {
        let mut min_right = f64::INFINITY;
        for (i, func) in functions.iter().enumerate() {
            if function_has_space[i] {
                let current_edge = func.right_interval_edges[indices[i]];
                min_right = min_right.min(current_edge);
            }
        }

        if min_right == f64::INFINITY {
            break;
        }

        let right = min_right;

        if right > left {
            for (i, func) in functions.iter().enumerate() {
                if function_has_space[i] {
                    cumulative_rows_per_function[i] += (right - left) * func.constants[indices[i]];
                }
            }

            let cur_cumulative_rows = cumulative_rows_per_function
                .iter()
                .copied()
                .fold(f64::INFINITY, f64::min);

            let constant = if right > left {
                (cur_cumulative_rows - prev_cumulative_rows) / (right - left)
            } else {
                0.0
            };

            let constant = if constant == 0.0 { 0.0001 } else { constant };

            right_edges.push(right);
            constants.push(constant);
            cumulative_rows.push(cur_cumulative_rows);

            left = right;
            prev_cumulative_rows = cur_cumulative_rows;

            if (cur_cumulative_rows - min_row_count).abs() < 1e-6 {
                break;
            }
        }

        for (i, func) in functions.iter().enumerate() {
            if function_has_space[i] && func.right_interval_edges[indices[i]] <= right {
                indices[i] += 1;
            }
        }

        for (i, func) in functions.iter().enumerate() {
            function_has_space[i] = indices[i] < func.right_interval_edges.len();
        }
    }

    let mut func = PiecewiseConstantFunction {
        constants,
        right_interval_edges: right_edges,
        cumulative_rows,
    };

    func.compress_func(None);
    func
}

fn bisect_left_double(a: &[f64], x: f64) -> usize {
    let mut lo = 0;
    let mut hi = a.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if a[mid] < x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

fn bisect_left_long(a: &[u64], x: f64) -> usize {
    let mut lo = 0;
    let mut hi = a.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if (a[mid] as f64) < x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

fn calculate_bins_exponential(num_vals: usize, num_segments: usize) -> Vec<usize> {
    if num_segments == 0 || num_vals == 0 {
        return vec![num_vals];
    }

    let mut bins = Vec::with_capacity(num_segments);
    if num_segments == 1 {
        bins.push(num_vals);
        return bins;
    }

    let ratio = (num_vals as f64).powf(1.0 / (num_segments - 1) as f64);
    for i in 0..num_segments {
        let val = (ratio.powi(i as i32)).ceil() as usize;
        bins.push(val.min(num_vals));
    }
    bins[num_segments - 1] = num_vals;
    bins
}

fn calculate_bins_relative_error(data: &[u64], relative_error: f64) -> GCardResult<Vec<usize>> {
    if data.is_empty() {
        return Ok(vec![0]);
    }

    let mut right_edges = vec![0usize; 100];
    let mut current_constant = data[0];
    let mut current_approx_value = 0.0;
    let mut current_true_value = 0.0;
    let mut cur_right = 0.0;
    let mut num_edges = 0;

    let total_value: f64 = data.iter().map(|&x| (x * x) as f64).sum();

    for (i, &val) in data.iter().enumerate() {
        let idx = (cur_right as usize).min(data.len().saturating_sub(1));
        current_approx_value += data[idx] as f64 * val as f64;
        current_true_value += (val * val) as f64;
        cur_right += val as f64 / current_constant as f64;

        if current_approx_value - current_true_value > relative_error * total_value {
            current_approx_value = 0.0;
            current_true_value = 0.0;
            right_edges[num_edges] = i;
            current_constant = val;
            num_edges += 1;
        }

        if num_edges >= right_edges.len() - 1 {
            break;
        }
    }

    right_edges[num_edges] = data.len();
    num_edges += 1;

    Ok(right_edges[..num_edges].to_vec())
}
