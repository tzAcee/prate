use super::*;

enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinaryOp {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
        }
    }
}

enum UnaryOp {
    Neg,
}

impl UnaryOp {
    fn binding_power(&self) -> ((), u8) {
        match self {
            Self::Neg => ((), 5),
        }
    }
}

pub(super) fn expr(p: &mut Parser) -> Option<CompletedMarker> {
    expr_binding_power(p, 0)
}

fn expr_binding_power(p: &mut Parser, minimum_binding_power: u8) -> Option<CompletedMarker> {
    let mut lhs = lhs(p)?;

    loop {
        let op = if p.at(TokenKind::Plus) {
            BinaryOp::Add
        } else if p.at(TokenKind::Minus) {
            BinaryOp::Sub
        } else if p.at(TokenKind::Star) {
            BinaryOp::Mul
        } else if p.at(TokenKind::Slash) {
            BinaryOp::Div
        } else {
            break;
        };

        let (left_binding_power, right_binding_power) = op.binding_power();

        if left_binding_power < minimum_binding_power {
            break;
        }

        // Eat the operator’s token.
        p.bump();

        let m = lhs.precede(p);
        let parsed_rhs = expr_binding_power(p, right_binding_power).is_some();
        lhs = m.complete(p, SyntaxKind::InfixExpression);

        if !parsed_rhs {
            break;
        }
    }
    Some(lhs)
}

fn lhs(p: &mut Parser) -> Option<CompletedMarker> {
    let cm = if p.at(TokenKind::Number) {
        literal(p)
    } else if p.at(TokenKind::Identifier) {
        variable_ref(p)
    } else if p.at(TokenKind::Minus) {
        prefix_expr(p)
    } else if p.at(TokenKind::LBrace) {
        paren_expr(p)
    } else if p.at(TokenKind::Quotation) {
      string_expr(p)
    } else {
        p.error();
        return None;
    };

    Some(cm)
}

fn literal(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::Number));

    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::Literal)
}

fn variable_ref(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::Identifier));

    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::VariableRef)
}

fn string_expr(p: &mut Parser) -> CompletedMarker {
  assert!(p.at(TokenKind::Quotation));
  let m = p.start();
  p.bump();
  p.expect(TokenKind::Identifier);
  p.expect(TokenKind::Quotation);

  m.complete(p, SyntaxKind::String)
}

fn prefix_expr(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::Minus));

    let m = p.start();

    let op = UnaryOp::Neg;
    let ((), right_binding_power) = op.binding_power();

    // Eat the operator’s token.
    p.bump();

    expr_binding_power(p, right_binding_power);

    m.complete(p, SyntaxKind::PrefixExpression)
}

fn paren_expr(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::LBrace));

    let m = p.start();
    p.bump();
    expr_binding_power(p, 0);
    p.expect(TokenKind::RBrace);

    m.complete(p, SyntaxKind::ParenExpression)
}
///////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_number() {
        check(
            "123",
            expect![[r#"
    Root@0..3
      Literal@0..3
        Number@0..3 "123""#]],
        );
    }

    #[test]
    fn parse_variable_ref() {
        check(
            "counter",
            expect![[r#"
        Root@0..7
          VariableRef@0..7
            Identifier@0..7 "counter""#]],
        );
    }

    #[test]
    fn parse_simple_binary_expression() {
        check(
            "1+2",
            expect![[r#"
Root@0..3
  InfixExpression@0..3
    Literal@0..1
      Number@0..1 "1"
    Plus@1..2 "+"
    Literal@2..3
      Number@2..3 "2""#]],
        );
    }

    #[test]
    fn parse_left_associative_binary_expression() {
        check(
            "1+2+3+4",
            expect![[r#"
        Root@0..7
          InfixExpression@0..7
            InfixExpression@0..5
              InfixExpression@0..3
                Literal@0..1
                  Number@0..1 "1"
                Plus@1..2 "+"
                Literal@2..3
                  Number@2..3 "2"
              Plus@3..4 "+"
              Literal@4..5
                Number@4..5 "3"
            Plus@5..6 "+"
            Literal@6..7
              Number@6..7 "4""#]],
        );
    }

    #[test]
    fn parse_binary_expression_with_mixed_binding_power() {
        check(
            "1+2*3-4",
            expect![[r#"
Root@0..7
  InfixExpression@0..7
    InfixExpression@0..5
      Literal@0..1
        Number@0..1 "1"
      Plus@1..2 "+"
      InfixExpression@2..5
        Literal@2..3
          Number@2..3 "2"
        Star@3..4 "*"
        Literal@4..5
          Number@4..5 "3"
    Minus@5..6 "-"
    Literal@6..7
      Number@6..7 "4""#]],
        );
    }

    #[test]
    fn parse_negation() {
        check(
            "-10",
            expect![[r#"
    Root@0..3
      PrefixExpression@0..3
        Minus@0..1 "-"
        Literal@1..3
          Number@1..3 "10""#]],
        );
    }

    #[test]
    fn negation_has_higher_binding_power_than_infix_operators() {
        check(
            "-20+20",
            expect![[r#"
Root@0..6
  InfixExpression@0..6
    PrefixExpression@0..3
      Minus@0..1 "-"
      Literal@1..3
        Number@1..3 "20"
    Plus@3..4 "+"
    Literal@4..6
      Number@4..6 "20""#]],
        );
    }

    #[test]
    fn parse_nested_parentheses() {
        check(
            "((((((10))))))",
            expect![[r#"
        Root@0..14
          ParenExpression@0..14
            LBrace@0..1 "("
            ParenExpression@1..13
              LBrace@1..2 "("
              ParenExpression@2..12
                LBrace@2..3 "("
                ParenExpression@3..11
                  LBrace@3..4 "("
                  ParenExpression@4..10
                    LBrace@4..5 "("
                    ParenExpression@5..9
                      LBrace@5..6 "("
                      Literal@6..8
                        Number@6..8 "10"
                      RBrace@8..9 ")"
                    RBrace@9..10 ")"
                  RBrace@10..11 ")"
                RBrace@11..12 ")"
              RBrace@12..13 ")"
            RBrace@13..14 ")""#]],
        );
    }

    #[test]
    fn parentheses_affect_precedence() {
        check(
            "5*(2+1)",
            expect![[r#"
Root@0..7
  InfixExpression@0..7
    Literal@0..1
      Number@0..1 "5"
    Star@1..2 "*"
    ParenExpression@2..7
      LBrace@2..3 "("
      InfixExpression@3..6
        Literal@3..4
          Number@3..4 "2"
        Plus@4..5 "+"
        Literal@5..6
          Number@5..6 "1"
      RBrace@6..7 ")""#]],
        );
    }

    #[test]
    fn parse_number_preceded_by_whitespace() {
        check(
            "   9876",
            expect![[r#"
Root@0..7
  Whitespace@0..3 "   "
  Literal@3..7
    Number@3..7 "9876""#]],
        );
    }

    #[test]
    fn parse_number_followed_by_whitespace() {
        check(
            "999   ",
            expect![[r#"
Root@0..6
  Literal@0..6
    Number@0..3 "999"
    Whitespace@3..6 "   ""#]],
        );
    }

    #[test]
    fn parse_number_surrounded_by_whitespace() {
        check(
            " 123     ",
            expect![[r#"
        Root@0..9
          Whitespace@0..1 " "
          Literal@1..9
            Number@1..4 "123"
            Whitespace@4..9 "     ""#]],
        );
    }

    #[test]
    fn parse_binary_expression_with_whitespace() {
        check(
            " 1 +   2* 3 ",
            expect![[r#"
Root@0..12
  Whitespace@0..1 " "
  InfixExpression@1..12
    Literal@1..3
      Number@1..2 "1"
      Whitespace@2..3 " "
    Plus@3..4 "+"
    Whitespace@4..7 "   "
    InfixExpression@7..12
      Literal@7..8
        Number@7..8 "2"
      Star@8..9 "*"
      Whitespace@9..10 " "
      Literal@10..12
        Number@10..11 "3"
        Whitespace@11..12 " ""#]],
        );
    }

    #[test]
    fn do_not_parse_operator_if_gettting_rhs_failed() {
        check(
            "(1+",
            expect![[r#"
Root@0..3
  ParenExpression@0..3
    LBrace@0..1 "("
    InfixExpression@1..3
      Literal@1..2
        Number@1..2 "1"
      Plus@2..3 "+"
error at 2..3: expected number, identifier, ‘-’, ‘(’ or ‘"’
error at 2..3: expected ‘)’"#]],
        );
    }
}
