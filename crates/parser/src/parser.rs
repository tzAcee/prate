pub(crate) mod marker;

use crate::event::Event;
use crate::grammar;
use crate::source::Source;
use marker::Marker;
use syntax::SyntaxKind;

pub(crate) struct Parser<'t, 'input> {
    source: Source<'t, 'input>,
    events: Vec<Event>,
}

impl<'t, 'input> Parser<'t, 'input> {
    pub(crate) fn new(source: Source<'t, 'input>) -> Self {
        Self {
            source,
            events: Vec::new(),
        }
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

    pub(crate) fn peek(&mut self) -> Option<SyntaxKind> {
        self.source.peek_kind()
    }

    pub(crate) fn bump(&mut self) {
        self.source.next_token().unwrap();
        self.events.push(Event::AddToken);
    }

    pub(crate) fn at(&mut self, kind: SyntaxKind) -> bool {
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