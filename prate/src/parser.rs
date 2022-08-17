use crate::lexer::{Lexer, SyntaxKind};
use crate::syntax::{PrateLng, SyntaxNode};
use logos::Logos;
use rowan::{GreenNode, GreenNodeBuilder, Language};
use std::iter::Peekable;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    builder: GreenNodeBuilder<'static>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
            builder: GreenNodeBuilder::new(),
        }
    }

    pub fn parse(mut self) -> Parse{
        self.start_node(SyntaxKind::Root);

        match self.peek() {
            Some(T) => {
                match T {
                    _ => {
                        self.bump();
                    }
                }

            }
            None => {
                todo!();
            }
            _ => {
                todo!();
            }
        }
        
        self.finish_node();

        Parse { green_node: self.builder.finish() }
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(PrateLng::kind_to_raw(kind));
    }

    fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.lexer.peek().map(|(kind, _)| *kind)
    }

    fn bump(&mut self) {
        let (kind, text) = self.lexer.next().unwrap();

        self.builder
            .token(PrateLng::kind_to_raw(kind), text.into());
    }
}

pub struct Parse {
    green_node: GreenNode,
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        formatted[0..formatted.len()-1].to_string()
    }
}

///////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{expect, Expect};

    fn check(input: &str, expected_tree: Expect) {
        let parse = Parser::new(input).parse();
        expected_tree.assert_eq(&parse.debug_tree());
    }

    #[ignore]
    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }

    #[test]
    fn parse_number() {
        check("123",             
        expect![[r#"
        Root@0..3
          Number@0..3 "123""#
          ]],
                );
    }

    #[test]
    fn parse_binding_usage() {
        check(
            "counter",
            expect![[r#"
Root@0..7
  Identifier@0..7 "counter""#]],
        );
    }
}