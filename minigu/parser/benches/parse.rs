use std::hint::black_box;

use gql_parser::ast::Program;
use gql_parser::span::Spanned;
use paste::paste;

fn main() {
    divan::main();
}

#[cfg(feature = "bench-antlr4")]
mod antlr4 {
    use std::ffi::c_char;

    unsafe extern "C" {
        pub fn parse_gql(input: *const c_char);
    }
}

macro_rules! add_parser_bench {
    ($dataset:expr, [ $($query:expr),* ]) => {
        paste! {
            $(
                #[divan::bench]
                fn [<parse_ $dataset _ $query>]() -> Spanned<Program> {
                    let input = include_str!(concat!("../../../resources/gql/", $dataset, "/", $query, ".gql"));
                    black_box(gql_parser::parse_gql(input).unwrap())
                }

                #[cfg(feature = "bench-antlr4")]
                #[divan::bench]
                fn [<parse_ $dataset _ $query _antlr>](b: divan::Bencher) {
                    use std::ffi::CString;
                    let input = include_str!(concat!("../../../resources/gql/", $dataset, "/", $query, ".gql"));
                    b.with_inputs(|| CString::new(input).unwrap())
                        .bench_values(|input| unsafe {
                            black_box(antlr4::parse_gql(input.as_ptr()));
                        });
                }
            )*
        }
    }
}

// add_parser_bench!("finbench", ["tsr1", "tsr2", "tsr3", "tsr4", "tsr5", "tsr6"]);
add_parser_bench!("finbench", ["tsr1", "tsr4", "tsr5", "tsr6"]);
// add_parser_bench!("snb", ["is1", "is2", "is3", "is4", "is5", "is6", "is7"]);
add_parser_bench!("snb", ["is1", "is2", "is3", "is4", "is6"]);
// add_parser_bench!("opengql", [
//     "create_graph",
//     "create_schema",
//     "insert",
//     "match_and_insert",
//     "match",
//     "session_set"
// ]);
add_parser_bench!("opengql", ["create_graph", "create_schema"]);
// add_parser_bench!("gql_on_one_page", ["gql_on_one_page"]);
