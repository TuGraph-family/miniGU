use core::marker::PhantomData;

use winnow::Parser;
use winnow::combinator::opt;
use winnow::error::ParserError;
use winnow::stream::{Stream, StreamIsPartial};

/// Specifies the associativity of an (infix) operator.
#[derive(Debug, Clone, Copy)]
pub(super) enum Assoc {
    Left,
    Right,
    Neither,
}

pub(super) type Precedence = i64;

/// An operator precedence parser based on the [Pratt algorithm](https://en.wikipedia.org/wiki/Operator-precedence_parser).
/// The implementation is adapted from https://github.com/winnow-rs/winnow/pull/614.
#[allow(clippy::too_many_arguments)]
pub(super) fn precedence<
    I,
    E,
    Operand,
    PrefixOp,
    SuffixOp,
    InfixOp,
    ParseOperand,
    ParsePrefixOp,
    ParseSuffixOp,
    ParseInfixOp,
    FoldPrefix,
    FoldSuffix,
    FoldInfix,
>(
    init: Precedence,
    parse_operand: ParseOperand,
    parse_prefix: ParsePrefixOp,
    parse_suffix: ParseSuffixOp,
    parse_infix: ParseInfixOp,
    fold_prefix: FoldPrefix,
    fold_suffix: FoldSuffix,
    fold_infix: FoldInfix,
) -> impl Parser<I, Operand, E>
where
    I: Stream + StreamIsPartial,
    ParseOperand: Parser<I, Operand, E>,
    ParsePrefixOp: Parser<I, (Precedence, PrefixOp), E>,
    ParseSuffixOp: Parser<I, (Precedence, SuffixOp), E>,
    ParseInfixOp: Parser<I, (Assoc, Precedence, InfixOp), E>,
    FoldPrefix: FnMut(PrefixOp, Operand) -> Result<Operand, E>,
    FoldSuffix: FnMut(Operand, SuffixOp) -> Result<Operand, E>,
    FoldInfix: FnMut(Operand, InfixOp, Operand) -> Result<Operand, E>,
    E: ParserError<I>,
{
    let mut config = PrattConfig {
        parse_operand,
        parse_prefix,
        parse_suffix,
        parse_infix,
        fold_prefix,
        fold_suffix,
        fold_infix,
        _marker: PhantomData,
    };
    move |i: &mut _| config.precedence_impl(i, init)
}

struct PrattConfig<
    I,
    E,
    Operand,
    PrefixOp,
    SuffixOp,
    InfixOp,
    ParseOperand,
    ParsePrefixOp,
    ParseSuffixOp,
    ParseInfixOp,
    FoldPrefix,
    FoldSuffix,
    FoldInfix,
> {
    parse_operand: ParseOperand,
    parse_prefix: ParsePrefixOp,
    parse_suffix: ParseSuffixOp,
    parse_infix: ParseInfixOp,
    fold_prefix: FoldPrefix,
    fold_suffix: FoldSuffix,
    fold_infix: FoldInfix,
    _marker: PhantomData<(I, E, Operand, PrefixOp, SuffixOp, InfixOp)>,
}

impl<
    I,
    E,
    Operand,
    PrefixOp,
    SuffixOp,
    InfixOp,
    ParseOperand,
    ParsePrefixOp,
    ParseSuffixOp,
    ParseInfixOp,
    FoldPrefix,
    FoldSuffix,
    FoldInfix,
>
    PrattConfig<
        I,
        E,
        Operand,
        PrefixOp,
        SuffixOp,
        InfixOp,
        ParseOperand,
        ParsePrefixOp,
        ParseSuffixOp,
        ParseInfixOp,
        FoldPrefix,
        FoldSuffix,
        FoldInfix,
    >
where
    I: Stream + StreamIsPartial,
    ParseOperand: Parser<I, Operand, E>,
    ParsePrefixOp: Parser<I, (Precedence, PrefixOp), E>,
    ParseSuffixOp: Parser<I, (Precedence, SuffixOp), E>,
    ParseInfixOp: Parser<I, (Assoc, Precedence, InfixOp), E>,
    FoldPrefix: FnMut(PrefixOp, Operand) -> Result<Operand, E>,
    FoldSuffix: FnMut(Operand, SuffixOp) -> Result<Operand, E>,
    FoldInfix: FnMut(Operand, InfixOp, Operand) -> Result<Operand, E>,
    E: ParserError<I>,
{
    fn precedence_impl(&mut self, i: &mut I, min_power: Precedence) -> Result<Operand, E> {
        let operand = opt(self.parse_operand.by_ref()).parse_next(i)?;
        let mut operand = if let Some(operand) = operand {
            operand
        } else {
            let len = i.eof_offset();
            let (power, prefix_op) = self.parse_prefix.parse_next(i)?;
            if i.eof_offset() == len {
                return Err(E::assert(i, "`prefix` parsers must always consume"));
            }
            let operand = self.precedence_impl(i, power)?;
            (self.fold_prefix)(prefix_op, operand)?
        };

        let mut prev_op_is_neither = None;
        while i.eof_offset() > 0 {
            let start = i.checkpoint();
            if let Some((power, suffix_op)) = opt(self.parse_suffix.by_ref()).parse_next(i)? {
                if power < min_power {
                    i.reset(&start);
                    break;
                }
                operand = (self.fold_suffix)(operand, suffix_op)?;
                continue;
            }

            let start = i.checkpoint();
            let parse_result = opt(self.parse_infix.by_ref()).parse_next(i)?;
            if let Some((assoc, power, infix_op)) = parse_result {
                let mut is_neither = None;
                let (lpower, rpower) = match assoc {
                    Assoc::Left => (power, power + 1),
                    Assoc::Right => (power, power - 1),
                    Assoc::Neither => {
                        is_neither = Some(power);
                        (power, power + 1)
                    }
                };
                if lpower < min_power || prev_op_is_neither.is_some_and(|p| lpower == p) {
                    i.reset(&start);
                    break;
                }
                prev_op_is_neither = is_neither;
                let rhs = self.precedence_impl(i, rpower)?;
                operand = (self.fold_infix)(operand, infix_op, rhs)?;
                continue;
            }
            break;
        }
        Ok(operand)
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use winnow::ModalResult;
    use winnow::ascii::{digit1, multispace0};
    use winnow::combinator::{delimited, dispatch, empty, fail, peek};
    use winnow::token::any;

    use super::*;

    fn operand(i: &mut &str) -> ModalResult<i64> {
        digit1.try_map(i64::from_str).parse_next(i)
    }

    fn parenthesized(i: &mut &str) -> ModalResult<i64> {
        delimited('(', expr, ')').parse_next(i)
    }

    fn expr(i: &mut &str) -> ModalResult<i64> {
        precedence(
            0,
            delimited(
                multispace0,
                dispatch! {peek(any);
                    '(' => parenthesized,
                    '0'..='9' => operand,
                    _ => fail
                },
                multispace0,
            ),
            delimited(
                multispace0,
                dispatch! {any;
                    '+' => empty.value((2, '+')),
                    '-' => empty.value((2, '-')),
                    _ => fail
                },
                multispace0,
            ),
            delimited(
                multispace0,
                dispatch! {any;
                    '!' => empty.value((3, '!')),
                    _ => fail
                },
                multispace0,
            ),
            delimited(
                multispace0,
                dispatch! {any;
                    '+' => empty.value((Assoc::Left, 0, '+')),
                    '-' => empty.value((Assoc::Left, 0, '-')),
                    '*' => empty.value((Assoc::Left, 1, '*')),
                    '/' => empty.value((Assoc::Left, 1, '/')),
                    _ => fail
                },
                multispace0,
            ),
            |op, a| match op {
                '+' => Ok(a),
                '-' => Ok(-a),
                _ => unreachable!(),
            },
            |a, op| match op {
                '!' => Ok((1..=a).product()),
                _ => unreachable!(),
            },
            |a, op, b| match op {
                '+' => Ok(a + b),
                '-' => Ok(a - b),
                '*' => Ok(a * b),
                '/' => Ok(a / b),
                _ => unreachable!(),
            },
        )
        .parse_next(i)
    }

    #[test]
    fn test_parse_operand() {
        assert_eq!(operand.parse("123").unwrap(), 123);
    }

    #[test]
    fn test_simple_expr() {
        assert_eq!(expr.parse("1 +  3* 4 /2 ").unwrap(), 7);
    }

    #[test]
    fn test_complex_expr() {
        assert_eq!(expr.parse("3- (+ 1 * - (3 +4) + 10)!").unwrap(), -3);
    }
}
