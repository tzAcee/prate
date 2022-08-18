use logos::Logos;

#[derive(Debug, Copy, Clone, PartialEq, Logos)]
pub enum TokenKind {
    #[regex("\r+")]
    #[regex("[ \n]+")]
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

    #[token("/*", |lex| {
        let len = lex.remainder().find("*/")?;
        lex.bump(len + 2); // include len of `*/`
    
        Some(())
    })]
    #[regex("//.*")]
    Comment,

    #[error]
    Undefined,
}

///////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Lexer, Token};

    fn check_lex(input: &str, kind: TokenKind) {
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some(Token { kind, text: input }));
    }

    #[test]
    fn lex_ws() {
        check_lex("   ", TokenKind::Whitespace);
    }

    #[test]
    fn lex_function_keyword() {
        check_lex("callable", TokenKind::Function);
    }

    #[test]
    fn lex_def_keyword() {
        check_lex("def", TokenKind::Define);
    }

    #[test]
    fn lex_identifier_all() {
        check_lex("asdyxyS123aAB11", TokenKind::Identifier);
    }

    #[test]
    fn lex_identifier_letters() {
        check_lex("asdyxAsdgqQW", TokenKind::Identifier);
    }

    #[test]
    fn lex_nr() {
        check_lex("123111204895", TokenKind::Number);
    }

    #[test]
    fn lex_plus() {
        check_lex("+", TokenKind::Plus);
    }

    #[test]
    fn lex_minus() {
        check_lex("-", TokenKind::Minus);
    }

    #[test]
    fn lex_star() {
        check_lex("*", TokenKind::Star);
    }

    #[test]
    fn lex_slash() {
        check_lex("/", TokenKind::Slash);
    }

    #[test]
    fn lex_equal() {
        check_lex("=", TokenKind::Equals);
    }

    #[test]
    fn lex_left_brace() {
        check_lex("(", TokenKind::LBrace);
    }

    #[test]
    fn lex_right_brace() {
        check_lex(")", TokenKind::RBrace);
    }

    #[test]
    fn lex_curly_right_brace() {
        check_lex("}", TokenKind::RCurlyBrace);
    }

    #[test]
    fn lex_curly_left_brace() {
        check_lex("{", TokenKind::LCurlyBrace);
    }

    #[test]
    fn lex_square_left_brace() {
        check_lex("[", TokenKind::LSquareBrace);
    }

    #[test]
    fn lex_square_right_brace() {
        check_lex("]", TokenKind::RSquareBrace);
    }

    #[test]
    fn lex_single_char_identifier() {
        check_lex("x", TokenKind::Identifier);
    }

    #[test]
    fn lex_comment_one_line() {
        check_lex("// foo", TokenKind::Comment);
    }

    #[test]
    fn lex_comment_multi_line() {
        check_lex(
            r"/* abc
        long cmd 
        */",
            TokenKind::Comment,
        );
    }

    #[test]
    fn lex_spaces_and_newlines() {
        check_lex("  \n ", TokenKind::Whitespace);
    }

    #[test]
    fn lex_comment_multi_line1() {
        check_lex("/*1*/", TokenKind::Comment);
    }
}
