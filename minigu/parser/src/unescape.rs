use itertools::Itertools;

use crate::Cow;

pub fn unescape<const Q: char, const NO_ESCAPE: bool>(mut input: &str) -> Option<Cow<str>> {
    let mut ret = Cow::default();
    let pat = if NO_ESCAPE { &[Q][..] } else { &['\\', Q][..] };
    while let Some(pos) = input.find(pat) {
        let (first, last) = input.split_at(pos);
        ret += first;
        input = last;
        let mut char_indices = input.char_indices();
        let ((_, delim), (_, next)) = char_indices
            .next_tuple()
            .expect("`input` should have at least two chars");
        if !NO_ESCAPE && delim == '\\' {
            let unescaped_char = match next {
                '\\' | '\'' | '"' | '`' => next,
                't' => '\t',
                'b' => '\x08',
                'n' => '\n',
                'r' => '\r',
                'f' => '\x0c',
                'u' => {
                    let ((start_idx, _), _, _, (end_idx, end)) = char_indices
                        .next_tuple()
                        .expect("`input` should have at least 4 chars");
                    let num =
                        u32::from_str_radix(&input[start_idx..(end_idx + end.len_utf8())], 16)
                            .expect("hex digits should be valid");
                    char::from_u32(num)?
                }
                'U' => {
                    let ((start_idx, _), _, _, _, _, (end_idx, end)) = char_indices
                        .next_tuple()
                        .expect("`input` should have at least 6 chars");
                    let num =
                        u32::from_str_radix(&input[start_idx..(end_idx + end.len_utf8())], 16)
                            .expect("hex digits should be valid");
                    char::from_u32(num)?
                }
                _ => unreachable!(),
            };
            ret.to_mut().push(unescaped_char);
        } else if delim == Q {
            assert_eq!(next, delim);
            ret.to_mut().push(Q);
        } else {
            unreachable!("`delim` should be \\ or {Q}");
        }
        if let Some((idx, _)) = char_indices.next() {
            input = &input[idx..];
        } else {
            input = "";
        }
    }
    ret += input;
    Some(ret)
}

#[cfg(all(test, feature = "serde", feature = "std"))]
mod tests {
    use super::unescape;

    #[test]
    fn test_unescape_1() {
        let unescaped = unescape::<'"', false>("abc").unwrap();
        assert_eq!(unescaped, "abc");
    }

    #[test]
    fn test_unescape_2() {
        let unescaped = unescape::<'"', false>(r#"a\nb""\uabcd\U0abcdec"#).unwrap();
        assert_eq!(unescaped, "a\nb\"\u{abcd}\u{0abcde}c");
    }

    #[test]
    fn test_unescape_3() {
        let unescaped = unescape::<'`', true>(r#"a\nb""``\uabcd\U0abcdec"#).unwrap();
        assert_eq!(unescaped, "a\\nb\"\"`\\uabcd\\U0abcdec");
    }

    #[test]
    fn test_unescape_4() {
        let unescaped = unescape::<'\'', false>(r#"''这是一个UTF8字符串\n''"#).unwrap();
        assert_eq!(unescaped, "'这是一个UTF8字符串\n'");
    }
}
