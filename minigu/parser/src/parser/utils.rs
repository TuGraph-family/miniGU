#[cfg(all(test, feature = "serde"))]
macro_rules! parse {
    ($parser:expr, $input:literal) => {{
        let input = $input;
        crate::parser::token::tokenize(input)
            .ok()
            .and_then(|tokens| {
                let stream = crate::parser::token::build_token_stream(&tokens);
                $parser.parse(stream).ok()
            })
    }};
}

#[cfg(all(test, feature = "serde"))]
pub(super) use parse;
