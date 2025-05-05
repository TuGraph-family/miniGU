/// A macro that simulates the behavior of `?` operator in gen blocks,
/// conforming to the specification in [RFC 3513](https://rust-lang.github.io/rfcs/3513-gen-blocks.html#error-handling-1).
///
/// Currently, the `?` operator is not usable in gen blocks
/// (see related issue for more details: https://github.com/rust-lang/rust/issues/117486).
/// Once the issue is resolved, this macro can be removed.
macro_rules! gen_try {
    ($a:expr) => {{
        use std::ops::{ControlFlow, FromResidual, Try};
        match $a.branch() {
            ControlFlow::Continue(val) => val,
            ControlFlow::Break(e) => {
                yield <_ as FromResidual>::from_residual(e.map_err(Into::into));
                return;
            }
        }
    }};
}

pub(crate) use gen_try;
