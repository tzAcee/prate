use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Copy, Clone, PartialEq, Logos, FromPrimitive, ToPrimitive, Hash, Ord, PartialOrd, Eq)]
pub(crate) enum SyntaxKind {
    #[regex(" +")]
    Whitespace,

    #[regex("callable")]
    Function,

    #[regex("def")]
    Define,

    #[regex("[A-Za-z][A-Za-z0-9]*")]
    Identifier,

    #[regex("[0-9]+")]
    Number,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("=")]
    Equals,

    #[token("{")]
    LCurlyBrace,

    #[token("}")]
    RCurlyBrace,

    #[token("(")]
    LBrace,

    #[token(")")]
    RBrace,

    #[token("]")]
    RSquareBrace,

    #[token("[")]
    LSquareBrace,

    #[error]
    Undefined,

    Root,

    BinExpression,

    PrefixExpression
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

pub(crate) struct Lexer<'a> {
    inner: logos::Lexer<'a, SyntaxKind>,
}

impl <'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            inner: SyntaxKind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Lexeme<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();

        Some(Self::Item { kind, text })
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Lexeme<'a> {
    pub(crate) kind: SyntaxKind,
    pub(crate) text: &'a str,
}

///////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    fn check_lex(input: &str, kind: SyntaxKind) {
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some(Lexeme { kind, text: input }));
    }

    #[test]
    fn lex_ws() {
        check_lex("   ", SyntaxKind::Whitespace);
    }

    #[test]
    fn lex_function_keyword() {
        check_lex("callable", SyntaxKind::Function);
    }

    #[test]
    fn lex_def_keyword() {
        check_lex("def", SyntaxKind::Define);
    }

    #[test]
    fn lex_identifier_all() {
        check_lex("asdyxyS123aAB11", SyntaxKind::Identifier);
    }

    #[test]
    fn lex_identifier_letters() {
        check_lex("asdyxAsdgqQW", SyntaxKind::Identifier);
    }

    #[test]
    fn lex_nr() {
        check_lex("123111204895", SyntaxKind::Number);
    }

    #[test]
    fn lex_plus() {
        check_lex("+", SyntaxKind::Plus);
    }

    #[test]
    fn lex_minus() {
        check_lex("-", SyntaxKind::Minus);
    }

    #[test]
    fn lex_star() {
        check_lex("*", SyntaxKind::Star);
    }

    #[test]
    fn lex_slash() {
        check_lex("/", SyntaxKind::Slash);
    }

    #[test]
    fn lex_equal() {
        check_lex("=", SyntaxKind::Equals);
    }

    #[test]
    fn lex_left_brace() {
        check_lex("(", SyntaxKind::LBrace);
    }

    #[test]
    fn lex_right_brace() {
        check_lex(")", SyntaxKind::RBrace);
    }

    #[test]
    fn lex_curly_right_brace() {
        check_lex("}", SyntaxKind::RCurlyBrace);
    }

    #[test]
    fn lex_curly_left_brace() {
        check_lex("{", SyntaxKind::LCurlyBrace);
    }

    #[test]
    fn lex_square_left_brace() {
        check_lex("[", SyntaxKind::LSquareBrace);
    }

    #[test]
    fn lex_square_right_brace() {
        check_lex("]", SyntaxKind::RSquareBrace);
    }

    #[test]
    fn lex_single_char_identifier() {
        check_lex("x", SyntaxKind::Identifier);
    }
}