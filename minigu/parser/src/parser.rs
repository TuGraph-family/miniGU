use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused_imports)]
    #[rustfmt::skip]
    gql
);

#[cfg(all(test, feature = "serde", feature = "std"))]
mod tests {
    use insta::assert_yaml_snapshot;
    use paste::paste;

    use super::gql::*;
    use crate::lexer::Lexer;

    macro_rules! parse {
        ($input:literal, $t:ty) => {{
            let input = $input;
            let lexer = Lexer::new(input);
            paste! {
              [<$t Parser>]::new().parse(input, lexer).ok()
            }
        }};
    }

    #[test]
    fn test_identifier() {
        assert_yaml_snapshot!(parse!("_abcd", Identifier), @r"
        name: _abcd
        span:
          start: 0
          end: 5
        ");
        assert_yaml_snapshot!(parse!("AcYcLic", Identifier), @r"
        name: AcYcLic
        span:
          start: 0
          end: 7
        ");
        assert_yaml_snapshot!(parse!(r#""abc\n""#, Identifier), @r#"
        name: "abc\n"
        span:
          start: 0
          end: 7
        "#);
        assert_yaml_snapshot!(parse!(r"@`ab``c\n`", Identifier), @r#"
        name: "ab`c\\n"
        span:
          start: 0
          end: 10
        "#);
    }

    #[test]
    fn test_label_expression() {
        assert_yaml_snapshot!(parse!("a | b & !(c | d)", LabelExpression), @r"
        Disjunction:
          - Label:
              name: a
              span:
                start: 0
                end: 1
          - Conjunction:
              - Label:
                  name: b
                  span:
                    start: 4
                    end: 5
              - Negation:
                  Disjunction:
                    - Label:
                        name: c
                        span:
                          start: 10
                          end: 11
                    - Label:
                        name: d
                        span:
                          start: 14
                          end: 15
        ");
    }

    #[test]
    fn test_is_label_expression() {
        assert_yaml_snapshot!(parse!("is a & b", IsLabelExpression), @r"
        Conjunction:
          - Label:
              name: a
              span:
                start: 3
                end: 4
          - Label:
              name: b
              span:
                start: 7
                end: 8
        ");
        assert_yaml_snapshot!(parse!(": a & b", IsLabelExpression), @r"
        Conjunction:
          - Label:
              name: a
              span:
                start: 2
                end: 3
          - Label:
              name: b
              span:
                start: 6
                end: 7
        ");
    }

    #[test]
    fn test_element_variable_declaration() {
        assert_yaml_snapshot!(parse!("temp abc_", ElementVariableDeclaration), @r"
        variable:
          name: abc_
          span:
            start: 5
            end: 9
        temp: true
        ");
    }

    #[test]
    fn test_boolean_literal() {
        assert_yaml_snapshot!(parse!("true", UnsignedLiteral), @r#"Boolean: "True""#);
        assert_yaml_snapshot!(parse!("False", UnsignedLiteral), @r#"Boolean: "False""#);
        assert_yaml_snapshot!(parse!("UnknoWn", UnsignedLiteral), @"Boolean: Unknown");
    }

    #[test]
    fn test_list_value_constructor() {
        assert_yaml_snapshot!(parse!("LIST []", ListValueConstructor), @r"
        type_name:
          group: false
          synonym: List
        values: []
        ");
    }

    #[test]
    fn test_record_constructor() {
        assert_yaml_snapshot!(parse!("RECORD {}", RecordConstructor), @"[]");
    }

    #[test]
    fn test_unsigned_integer() {
        assert_yaml_snapshot!(parse!("123_456", UnsignedInteger), @r"
        kind: Decimal
        integer: 123_456
        ");
        assert_yaml_snapshot!(parse!("0x123a_bCeF", UnsignedInteger), @r#"
        kind: Hex
        integer: "0x123a_bCeF"
        "#);
        assert_yaml_snapshot!(parse!("0o123_67", UnsignedInteger), @r"
        kind: Octal
        integer: 0o123_67
        ");
        assert_yaml_snapshot!(parse!("0b0000_1111", UnsignedInteger), @r"
        kind: Binary
        integer: 0b0000_1111
        ");
    }

    #[test]
    fn test_comment() {
        assert_yaml_snapshot!(parse!(r"
        // This is a comment introduced by double solidus.
        -- This is a comment introduced by double minus.
        0b0000_1111", UnsignedInteger), @r"
        kind: Binary
        integer: 0b0000_1111
        ");

        assert_yaml_snapshot!(parse!(r"
        /*
          This is a bracketed comment.
        */
        /*** This is a bracketed comment. ***/
        /**/
        /***/
        0b0000_1111", UnsignedInteger), @r"
        kind: Binary
        integer: 0b0000_1111
        ");
    }

    #[test]
    fn test_value_expression() {
        assert_yaml_snapshot!(parse!("c = (a + 1) * b", ValueExpression), @r#"
        Binary:
          op: Eq
          left:
            Variable:
              name: c
              span:
                start: 0
                end: 1
          right:
            Binary:
              op: Mul
              left:
                Binary:
                  op: Add
                  left:
                    Variable:
                      name: a
                      span:
                        start: 5
                        end: 6
                  right:
                    Value:
                      Literal:
                        Numeric:
                          Integer:
                            kind: Decimal
                            integer: "1"
              right:
                Variable:
                  name: b
                  span:
                    start: 14
                    end: 15
        "#);
    }

    #[test]
    fn test_catalog_object_reference() {
        assert_yaml_snapshot!(parse!("/a/b/c.d.e", CatalogObjectReference), @r"
        schema:
          Path:
            - Root
            - Name:
                name: a
                span:
                  start: 1
                  end: 2
            - Name:
                name: b
                span:
                  start: 3
                  end: 4
        objects:
          - name: c
            span:
              start: 5
              end: 6
          - name: d
            span:
              start: 7
              end: 8
          - name: e
            span:
              start: 9
              end: 10
        ");
        assert_yaml_snapshot!(parse!("../../a/`b`/c.`d`.e", CatalogObjectReference), @r"
        schema:
          Path:
            - Parent
            - Parent
            - Name:
                name: a
                span:
                  start: 6
                  end: 7
            - Name:
                name: b
                span:
                  start: 8
                  end: 11
        objects:
          - name: c
            span:
              start: 12
              end: 13
          - name: d
            span:
              start: 14
              end: 17
          - name: e
            span:
              start: 18
              end: 19
        ");
        assert_yaml_snapshot!(parse!("./a.b.c", CatalogObjectReference), @r"
        schema:
          Predefined: Current
        objects:
          - name: a
            span:
              start: 2
              end: 3
          - name: b
            span:
              start: 4
              end: 5
          - name: c
            span:
              start: 6
              end: 7
        ");
        assert_yaml_snapshot!(parse!("a.b.c", CatalogObjectReference), @r"
        schema: ~
        objects:
          - name: a
            span:
              start: 0
              end: 1
          - name: b
            span:
              start: 2
              end: 3
          - name: c
            span:
              start: 4
              end: 5
        ");
        assert_yaml_snapshot!(parse!("a", CatalogObjectReferenceOptional), @r"
        schema: ~
        objects:
          - name: a
            span:
              start: 0
              end: 1
        ");
        assert_yaml_snapshot!(parse!("/a b", CatalogObjectReference), @"~");
        assert_yaml_snapshot!(parse!("//", CatalogObjectReference), @"~");
    }

    #[test]
    fn test_graph_reference() {
        assert_yaml_snapshot!(parse!("`this is a graph`", GraphReference), @r"
        Name:
          name: this is a graph
          span:
            start: 0
            end: 17
        ");
        assert_yaml_snapshot!(parse!("$$g", GraphReference), @r"
        Parameter:
          name: g
          span:
            start: 0
            end: 3
        ");
        assert_yaml_snapshot!(parse!("/my_graph", GraphReference), @r"
        Ref:
          schema:
            Path:
              - Root
          objects:
            - name: my_graph
              span:
                start: 1
                end: 9
        ");
        assert_yaml_snapshot!(parse!("/my_schema/my_graph", GraphReference), @r"
        Ref:
          schema:
            Path:
              - Root
              - Name:
                  name: my_schema
                  span:
                    start: 1
                    end: 10
          objects:
            - name: my_graph
              span:
                start: 11
                end: 19
        ");
        assert_yaml_snapshot!(parse!("home_graph", GraphReference), @"Home");
    }
}
