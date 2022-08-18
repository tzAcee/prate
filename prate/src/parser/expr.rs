
use super::{Parser, marker::CompletedMarker};
use crate::lexer::SyntaxKind;
enum InfixOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl InfixOp {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1,2),
            Self::Mul | Self::Div => (3,4),
        }
    }
}

enum PrefixOp {
    Neg
}

impl PrefixOp {
    fn binding_power(&self) -> ((), u8) {
        match self {
            Self::Neg => ((), 5),
        }
    }
}




pub(super) fn expr(p: &mut Parser) {
    expr_binding_power(p, 0);
}

fn expr_binding_power(p: &mut Parser, minimum_binding_power: u8) {
    let checkpoint = p.checkpoint();

    let mut lhs = match p.peek() {
        Some(SyntaxKind::Number) => {
            let m = p.start();
            p.bump();
            m.complete(p, SyntaxKind::Literal)
        }
        Some(SyntaxKind::Identifier) => {
            let m = p.start();
            p.bump();
            m.complete(p, SyntaxKind::VariableRef)
        }
        Some(SyntaxKind::Minus) => {
            let m = p.start();

            let op = PrefixOp::Neg;
            let ((), right_binding_power) = op.binding_power();

            // Eat the operator’s token.
            p.bump();

            expr_binding_power(p, right_binding_power);

            m.complete(p, SyntaxKind::PrefixExpression)
        }
        Some(SyntaxKind::LBrace) => {
            let m = p.start();

            p.bump();
            expr_binding_power(p, 0);

            assert_eq!(p.peek(), Some(SyntaxKind::RBrace));
            p.bump();

            m.complete(p, SyntaxKind::ParenExpr)
        }
        _ => return, // we’ll handle errors later.
    };

    loop {
        let op = match p.peek() {
            Some(SyntaxKind::Plus) => InfixOp::Add,
            Some(SyntaxKind::Minus) => InfixOp::Sub,
            Some(SyntaxKind::Star) => InfixOp::Mul,
            Some(SyntaxKind::Slash) => InfixOp::Div,
            _ => return, // we’ll handle errors later.
        };

        let (left_binding_power, right_binding_power) = op.binding_power();

        if left_binding_power < minimum_binding_power {
            return;
        }

       // Eat the operator’s token.
        p.bump();

        let m = lhs.precede(p);
        expr_binding_power(p, right_binding_power);
        lhs = m.complete(p, SyntaxKind::BinExpression);
    }
}

///////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::parser::parse;

    use expect_test::{expect, Expect};

    fn check(input: &str, expected_tree: Expect) {
        let parse = parse(input);
        expected_tree.assert_eq(&parse.debug_tree());
    }

#[test]
fn parse_number() {
    check("123",             
    expect![[r#"
    Root@0..3
      Literal@0..3
        Number@0..3 "123""#
      ]],
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
  BinExpression@0..3
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
          BinExpression@0..7
            BinExpression@0..5
              BinExpression@0..3
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
  BinExpression@0..7
    BinExpression@0..5
      Literal@0..1
        Number@0..1 "1"
      Plus@1..2 "+"
      BinExpression@2..5
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
    check("-10",             expect![[r#"
    Root@0..3
      PrefixExpression@0..3
        Minus@0..1 "-"
        Literal@1..3
          Number@1..3 "10""#]]);
}

#[test]
fn negation_has_higher_binding_power_than_infix_operators() {
    check(
        "-20+20",
        expect![[r#"
Root@0..6
  BinExpression@0..6
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
          ParenExpr@0..14
            LBrace@0..1 "("
            ParenExpr@1..13
              LBrace@1..2 "("
              ParenExpr@2..12
                LBrace@2..3 "("
                ParenExpr@3..11
                  LBrace@3..4 "("
                  ParenExpr@4..10
                    LBrace@4..5 "("
                    ParenExpr@5..9
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
  BinExpression@0..7
    Literal@0..1
      Number@0..1 "5"
    Star@1..2 "*"
    ParenExpr@2..7
      LBrace@2..3 "("
      BinExpression@3..6
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
  BinExpression@1..12
    Literal@1..3
      Number@1..2 "1"
      Whitespace@2..3 " "
    Plus@3..4 "+"
    Whitespace@4..7 "   "
    BinExpression@7..12
      Literal@7..8
        Number@7..8 "2"
      Star@8..9 "*"
      Whitespace@9..10 " "
      Literal@10..12
        Number@10..11 "3"
        Whitespace@11..12 " ""#]],
    );
}
}