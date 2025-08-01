use std::hint::black_box;

use divan::{AllocProfiler, Bencher, black_box_drop};
use gql_parser::ast::{BinaryOp, Expr};
use gql_parser::span::Spanned;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

fn build_expr(depth: usize) -> Expr {
    if depth == 0 {
        return Expr::Variable("a".into());
    }
    let left = build_expr(depth - 1);
    let right = Expr::Variable("a".into());
    Expr::Binary {
        op: Spanned(BinaryOp::Add, 0..0),
        left: Box::new(Spanned(left, 0..0)),
        right: Box::new(Spanned(right, 0..0)),
    }
}

const DEPTHS: &[usize] = &[20, 40, 60, 80, 100];

#[divan::bench(args = DEPTHS)]
fn alloc_ast(depth: usize) -> Expr {
    black_box(build_expr(depth))
}

#[divan::bench(args = DEPTHS)]
fn drop_ast(bencher: Bencher, depth: usize) {
    bencher
        .with_inputs(|| build_expr(depth))
        .bench_local_values(black_box_drop);
}
