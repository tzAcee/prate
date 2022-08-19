pub(crate) mod marker;

mod parser_error;
pub(crate) use parser_error::ParseError;

use crate::event::Event;
use crate::grammar;
use crate::source::Source;
use lexer::{Token, TokenKind};
use marker::Marker;
use std::mem;
use syntax::SyntaxKind;

const RECOVERY_SET: [TokenKind; 1] = [TokenKind::Define];

pub(crate) struct Parser<'t, 'input> {
    source: Source<'t, 'input>,
    events: Vec<Event>,
    expected_kinds: Vec<TokenKind>,
}

impl<'t, 'input> Parser<'t, 'input> {
    pub(crate) fn new(source: Source<'t, 'input>) -> Self {
        Self {
            source,
            events: Vec::new(),
            expected_kinds: Vec::new(),
        }
    }

    pub(crate) fn expect(&mut self, kind: TokenKind) {
        if self.at(kind) {
            self.bump();
        } else {
            self.error();
        }
    }

    fn at_set(&mut self, set: &[TokenKind]) -> bool {
        self.peek().map_or(false, |k| set.contains(&k))
    }

    pub(crate) fn error(&mut self) {
        let current_token = self.source.peek_token();

        let (found, range) = if let Some(Token { kind, range, .. }) = current_token {
            (Some(*kind), *range)
        } else {
            // If we’re at the end of the input we use the range of the very last token in the
            // input.
            (None, self.source.last_token_range().unwrap())
        };

        self.events.push(Event::Error(ParseError {
            expected: mem::take(&mut self.expected_kinds),
            found,
            range,
        }));

        if !self.at_set(&RECOVERY_SET) && !self.at_end() {
            let m = self.start();
            self.bump();
            m.complete(self, SyntaxKind::Undefined);
        }
    }

    pub(crate) fn at_end(&mut self) -> bool {
        self.peek().is_none()
    }

    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);

        Marker::new(pos)
    }

    pub(crate) fn parse(mut self) -> Vec<Event> {
        grammar::root(&mut self);
        self.events
    }

    fn peek(&mut self) -> Option<TokenKind> {
        self.source.peek_kind()
    }

    pub(crate) fn bump(&mut self) {
        self.expected_kinds.clear();
        self.source.next_token().unwrap();
        self.events.push(Event::AddToken);
    }

    pub(crate) fn at(&mut self, kind: TokenKind) -> bool {
        self.expected_kinds.push(kind);
        self.peek() == Some(kind)
    }
}

///////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

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
              InfixExpression@1..37
                InfixExpression@1..22
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
