mod event;
mod expr;
mod marker;
mod sink;
mod source;

use crate::lexer::{Lexer, SyntaxKind, Token};
use crate::syntax::SyntaxNode;
use event::Event;
use expr::expr;
use marker::Marker;
use rowan::GreenNode;
use sink::Sink;
use source::Source;

pub fn parse(input: &str) -> Parse {
    let tokens: Vec<_> = Lexer::new(input).collect();
    let parser = Parser::new(&tokens);
    let events = parser.parse();
    let sink = Sink::new(&tokens, events);

    Parse {
        green_node: sink.finish(),
    }
}

struct Parser<'t, 'input> {
    source: Source<'t, 'input>,
    events: Vec<Event>,
}

impl<'t, 'input> Parser<'t, 'input> {
    fn new(tokens: &'t [Token<'input>]) -> Self {
        Self {
            source: Source::new(tokens),
            events: Vec::new(),
        }
    }

    fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);

        Marker::new(pos)
    }

    fn parse(mut self) -> Vec<Event> {
        let m = self.start();
        expr(&mut self);
        m.complete(&mut self, SyntaxKind::Root);

        self.events
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.source.peek_kind()
    }

    fn bump(&mut self) {
        let Token { kind, text } = self.source.next_token().unwrap();

        self.events.push(Event::AddToken {
            kind: *kind,
            text: (*text).into(),
        });
    }

    fn at(&mut self, kind: SyntaxKind) -> bool {
        self.peek() == Some(kind)
    }
}

pub struct Parse {
    green_node: GreenNode,
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        formatted[0..formatted.len() - 1].to_string()
    }
}

///////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{expect, Expect};

    pub fn check(input: &str, expected_tree: Expect) {
        let parse = parse(input);
        expected_tree.assert_eq(&parse.debug_tree());
    }

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }

    #[test]
    fn parse_whitespace() {
        check(
            "   ",
            expect![[r#"
Root@0..3
  Whitespace@0..3 "   ""#]],
        );
    }

    #[test]
    fn parse_whitespace_with_id() {
        check(
            "\r\n\r",
            expect![[r#"
            Root@0..3
              Whitespace@0..1 "\r"
              Whitespace@1..2 "\n"
              Whitespace@2..3 "\r""#]],
        );
    }

    #[test]
    fn parse_comment() {
        check(
            "// hello!",
            expect![[r##"
Root@0..9
  Comment@0..9 "// hello!""##]],
        );
    }

    #[test]
    fn parse_binary_expression_interspersed_with_comments() {
        check(
            "
1
  + 1 // Add one
  + 10 // Add ten",
            expect![[r##"
            Root@0..37
              Whitespace@0..1 "\n"
              BinExpression@1..37
                BinExpression@1..22
                  Literal@1..5
                    Number@1..2 "1"
                    Whitespace@2..5 "\n  "
                  Plus@5..6 "+"
                  Whitespace@6..7 " "
                  Literal@7..22
                    Number@7..8 "1"
                    Whitespace@8..9 " "
                    Comment@9..19 "// Add one"
                    Whitespace@19..22 "\n  "
                Plus@22..23 "+"
                Whitespace@23..24 " "
                Literal@24..37
                  Number@24..26 "10"
                  Whitespace@26..27 " "
                  Comment@27..37 "// Add ten""##]],
        );
    }
}
