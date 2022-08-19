use lexer::TokenKind;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive, Hash, Ord, Eq, PartialOrd)]
pub enum SyntaxKind {
    Whitespace,
    Function,
    Define,
    Identifier,
    Number,
    Plus,
    Minus,
    Star,
    Slash,
    Equals,
    LBrace,
    RBrace,
    LCurlyBrace,
    RCurlyBrace,
    LSquareBrace,
    RSquareBrace,
    Comment,
    Undefined,
    Root,
    InfixExpression,
    Literal,
    ParenExpression,
    PrefixExpression,
    VariableRef,
    VariableDef,
}

impl From<TokenKind> for SyntaxKind {
    fn from(token_kind: TokenKind) -> Self {
        match token_kind {
            TokenKind::Whitespace => Self::Whitespace,
            TokenKind::Function => Self::Function,
            TokenKind::Define => Self::Define,
            TokenKind::Identifier => Self::Identifier,
            TokenKind::Number => Self::Number,
            TokenKind::Plus => Self::Plus,
            TokenKind::Minus => Self::Minus,
            TokenKind::Star => Self::Star,
            TokenKind::Slash => Self::Slash,
            TokenKind::Equals => Self::Equals,
            TokenKind::LBrace => Self::LBrace,
            TokenKind::RBrace => Self::RBrace,
            TokenKind::LCurlyBrace => Self::LCurlyBrace,
            TokenKind::RCurlyBrace => Self::RCurlyBrace,
            TokenKind::LSquareBrace => Self::LSquareBrace,
            TokenKind::RSquareBrace => Self::RSquareBrace,
            TokenKind::Comment => Self::Comment,
            TokenKind::Undefined => Self::Undefined,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PrateLng {}

pub type SyntaxNode = rowan::SyntaxNode<PrateLng>;
pub type SyntaxElement = rowan::SyntaxElement<PrateLng>;
pub type SyntaxToken = rowan::SyntaxToken<PrateLng>;

impl rowan::Language for PrateLng {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}
