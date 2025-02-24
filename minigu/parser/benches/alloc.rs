use std::hint::black_box;

use divan::{AllocProfiler, Bencher, black_box_drop};
use gql_parser::ast::{BinaryOp, Expr, ExprKind, Ident};
use gql_parser::span::Span;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

fn build_expr(depth: usize) -> Expr {
    if depth == 0 {
        return Expr {
            kind: ExprKind::Variable(Ident {
                name: "a".into(),
                span: Span::default(),
            }),
            span: Span::default(),
        };
    }
    let left = build_expr(depth - 1);
    let right = Expr {
        kind: ExprKind::Variable(Ident {
            name: "a".into(),
            span: Span::default(),
        }),
        span: Span::default(),
    };
    Expr {
        kind: ExprKind::Binary {
            op: BinaryOp::Add,
            left: Box::new(left),
            right: Box::new(right),
        },
        span: Span::default(),
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
