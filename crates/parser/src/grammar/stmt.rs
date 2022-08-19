use super::*;

pub(super) fn stmt(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(TokenKind::Define) {
        Some(variable_def(p))
    } else {
        expr::expr(p)
    }
}

fn variable_def(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::Define));
    let m = p.start();
    p.bump();

    p.expect(TokenKind::Identifier);
    p.expect(TokenKind::Equals);

    expr::expr(p);

    return m.complete(p, SyntaxKind::VariableDef);
}
///////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_variable_definition() {
        check(
            "def foo = abc123",
            expect![[r#"
Root@0..16
  VariableDef@0..16
    Define@0..3 "def"
    Whitespace@3..4 " "
    Identifier@4..7 "foo"
    Whitespace@7..8 " "
    Equals@8..9 "="
    Whitespace@9..10 " "
    VariableRef@10..16
      Identifier@10..16 "abc123""#]],
        );
    }

    #[test]
    fn recover_on_def_token() {
        check(
            "def a =\ndef b = a",
            expect![[r#"
Root@0..17
  VariableDef@0..8
    Define@0..3 "def"
    Whitespace@3..4 " "
    Identifier@4..5 "a"
    Whitespace@5..6 " "
    Equals@6..7 "="
    Whitespace@7..8 "\n"
  VariableDef@8..17
    Define@8..11 "def"
    Whitespace@11..12 " "
    Identifier@12..13 "b"
    Whitespace@13..14 " "
    Equals@14..15 "="
    Whitespace@15..16 " "
    VariableRef@16..17
      Identifier@16..17 "a"
error at 8..11: expected number, identifier, ‘-’ or ‘(’, but found def"#]],
        );
    }

    #[test]
    fn parse_multiple_statements() {
        check(
            "def a = 1\na",
            expect![[r#"
Root@0..11
  VariableDef@0..10
    Define@0..3 "def"
    Whitespace@3..4 " "
    Identifier@4..5 "a"
    Whitespace@5..6 " "
    Equals@6..7 "="
    Whitespace@7..8 " "
    Literal@8..10
      Number@8..9 "1"
      Whitespace@9..10 "\n"
  VariableRef@10..11
    Identifier@10..11 "a""#]],
        );
    }

    #[test]
    fn parse_unclosed_parentheses() {
        check(
            "(foo",
            expect![[r#"
Root@0..4
  ParenExpression@0..4
    LBrace@0..1 "("
    VariableRef@1..4
      Identifier@1..4 "foo"
error at 1..4: expected ‘+’, ‘-’, ‘*’, ‘/’ or ‘)’"#]],
        );
    }
}
