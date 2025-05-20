use winnow::Parser;
use winnow::stream::{Location, Stream};

use crate::span::Spanned;

/// A helper macro for executing a parser on an input string. **This should only be used in unit
/// tests!**
#[cfg(all(test, feature = "serde"))]
macro_rules! parse {
    ($parser:expr, $input:literal) => {{
        let input = $input;
        $crate::parser::token::tokenize(input)
            .ok()
            .and_then(|tokens| {
                let stream = crate::parser::token::build_token_stream(&tokens, Default::default());
                $parser.parse(stream).ok()
            })
    }};
}

#[cfg(all(test, feature = "serde"))]
pub(super) use parse;

/// A helper trait for wrapping the output in [`Spanned`].
pub(super) trait ToSpanned<I, O, E>: Parser<I, O, E> {
    /// Wraps the output in [`Spanned`].
    #[inline(always)]
    fn spanned(self) -> impl Parser<I, Spanned<O>, E>
    where
        Self: Sized,
        I: Stream + Location,
    {
        self.with_span().map(|(inner, span)| Spanned(inner, span))
    }
}

/// A helper trait for operating on the value wrapped in [`Spanned`].
pub(super) trait SpannedParserExt<I, O, E>: Parser<I, Spanned<O>, E> {
    /// Unwraps the output from [`Spanned`].
    #[inline(always)]
    fn unspanned(self) -> impl Parser<I, O, E>
    where
        Self: Sized,
    {
        self.map(|spanned| spanned.0)
    }

    /// Maps the inner value of the output while keeping the span unchanged.
    #[inline(always)]
    fn map_inner<F, O2>(self, mut f: F) -> impl Parser<I, Spanned<O2>, E>
    where
        Self: Sized,
        F: FnMut(O) -> O2,
    {
        self.map(move |Spanned(inner, span)| Spanned(f(inner), span))
    }

    /// Updates the span of the output while keeping the inner value unchanged.
    #[inline(always)]
    fn update_span(self) -> impl Parser<I, Spanned<O>, E>
    where
        Self: Sized,
        I: Stream + Location,
    {
        self.with_span().map(|(mut output, new_span)| {
            output.1 = new_span;
            output
        })
    }
}

impl<I, O, E, P> ToSpanned<I, O, E> for P where P: Parser<I, O, E> {}

impl<I, O, E, P> SpannedParserExt<I, O, E> for P where P: Parser<I, Spanned<O>, E> {}

/// A helper macro for defining parser aliases.
macro_rules! def_parser_alias {
    ($name:ident, $parser:ident, $output:ty) => {
        #[inline(always)]
        pub fn $name(
            input: &mut $crate::parser::token::TokenStream,
        ) -> winnow::ModalResult<$output> {
            $parser(input)
        }
    };
}

pub(super) use def_parser_alias;
