use winnow::combinator::opt;
use winnow::error::{ErrMode, ParserError};
use winnow::stream::{Stream, StreamIsPartial};
use winnow::{ModalParser, ModalResult, Parser};

/// Specifies the associativity of an (infix) operator.
#[derive(Debug, Clone, Copy)]
pub(super) enum Assoc {
    Left,
    Right,
    Neither,
}

type Precedence = i64;
type UnaryCallback<I, Operand, E> = fn(&mut I, Operand) -> ModalResult<Operand, E>;
type BinaryCallback<I, Operand, E> = fn(&mut I, Operand, Operand) -> ModalResult<Operand, E>;

/// An operator precedence parser based on the [Pratt algorithm](https://en.wikipedia.org/wiki/Operator-precedence_parser).
/// The implementation is adapted from https://github.com/winnow-rs/winnow/pull/614.
pub(super) fn precedence<I, ParseOperand, ParseInfix, ParsePrefix, ParseSuffix, Operand, E>(
    init: Precedence,
    mut operand: ParseOperand,
    mut prefix: ParsePrefix,
    mut suffix: ParseSuffix,
    mut infix: ParseInfix,
) -> impl ModalParser<I, Operand, E>
where
    I: Stream + StreamIsPartial,
    ParseOperand: ModalParser<I, Operand, E>,
    ParseInfix: ModalParser<I, (Assoc, Precedence, BinaryCallback<I, Operand, E>), E>,
    ParsePrefix: ModalParser<I, (Precedence, UnaryCallback<I, Operand, E>), E>,
    ParseSuffix: ModalParser<I, (Precedence, UnaryCallback<I, Operand, E>), E>,
    E: ParserError<I>,
{
    move |i: &mut I| precedence_impl(i, &mut operand, &mut prefix, &mut suffix, &mut infix, init)
}

fn precedence_impl<I, ParseOperand, ParseInfix, ParsePrefix, ParseSuffix, Operand, E>(
    i: &mut I,
    parse_operand: &mut ParseOperand,
    prefix: &mut ParsePrefix,
    suffix: &mut ParseSuffix,
    infix: &mut ParseInfix,
    min_power: Precedence,
) -> ModalResult<Operand, E>
where
    I: Stream + StreamIsPartial,
    ParseOperand: ModalParser<I, Operand, E>,
    ParseInfix: ModalParser<I, (Assoc, Precedence, BinaryCallback<I, Operand, E>), E>,
    ParsePrefix: ModalParser<I, (Precedence, UnaryCallback<I, Operand, E>), E>,
    ParseSuffix: ModalParser<I, (Precedence, UnaryCallback<I, Operand, E>), E>,
    E: ParserError<I>,
{
    let operand = opt(parse_operand.by_ref()).parse_next(i)?;
    let mut operand = if let Some(operand) = operand {
        operand
    } else {
        let len = i.eof_offset();
        let (power, fold_prefix) = prefix.parse_next(i)?;
        if i.eof_offset() == len {
            return Err(ErrMode::assert(i, "`prefix` parsers must always consume"));
        }
        let operand = precedence_impl(i, parse_operand, prefix, suffix, infix, power)?;
        fold_prefix(i, operand)?
    };

    let mut prev_op_is_neither = None;
    while i.eof_offset() > 0 {
        let start = i.checkpoint();
        if let Some((power, fold_suffix)) = opt(suffix.by_ref()).parse_next(i)? {
            if power < min_power {
                i.reset(&start);
                break;
            }
            operand = fold_suffix(i, operand)?;
            continue;
        }

        let start = i.checkpoint();
        let parse_result = opt(infix.by_ref()).parse_next(i)?;
        if let Some((assoc, power, fold_infix)) = parse_result {
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
            let rhs = precedence_impl(i, parse_operand, prefix, suffix, infix, rpower)?;
            operand = fold_infix(i, operand, rhs)?;
            continue;
        }
        break;
    }
    Ok(operand)
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;

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
            delimited(multispace0, dispatch!{
                peek(any);
                '(' => parenthesized,
                '0'..='9' => operand,
                _ => fail
            }, multispace0),
            delimited(
                multispace0,
                dispatch! {
                    any;
                    '+' => empty.value((2, (|_: &mut _, a: i64| Ok(a)) as _)),
                    '-' => empty.value((2, (|_: &mut _, a: i64| Ok(-a)) as _)),
                    _ => fail
                },
                multispace0,
            ),
            delimited(
                multispace0,
                dispatch! {
                    any;
                    '!' => empty.value((3, (|_: &mut _, a: i64| Ok((1..=a).product())) as _)),
                    _ => fail
                },
                multispace0,
            ),
            delimited(multispace0, dispatch! {
                any;
                '+' => empty.value((Assoc::Left, 0, (|_: &mut _, a: i64, b: i64| Ok(a + b)) as _)),
                '-' => empty.value((Assoc::Left, 0, (|_: &mut _, a: i64, b: i64| Ok(a - b)) as _)),
                '*' => empty.value((Assoc::Left, 1, (|_: &mut _, a: i64, b: i64| Ok(a * b)) as _)),
                '/' => empty.value((Assoc::Left, 1, (|_: &mut _, a: i64, b: i64| Ok(a / b)) as _)),
                _ => fail
            }, multispace0),
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
