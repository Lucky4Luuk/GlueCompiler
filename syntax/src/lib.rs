use num_traits::{FromPrimitive, ToPrimitive};
use num_derive::{FromPrimitive, ToPrimitive};

use lexer::TokenKind;

pub type SyntaxNode = rowan::SyntaxNode<GlueLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<GlueLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<GlueLanguage>;

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum SyntaxKind {
    Whitespace,

    KeywordFunc,
    KeywordReturn,

    Colon,
    Semicolon,
    Comma,

    Identifier,

    Plus,
    Minus,
    Asterix,
    Slash,
    Equals,
    LParen,
    RParen,
    LBrace,
    RBrace,
    CPPComment,

    Error,
    Root,

    InfixExpr,
    Literal,
    ParenExpr,
    PrefixExpr,

    VariableDef,
    VariableRef,
    VariableAssignment,

    FunctionDeclaration,
    FunctionDeclarationArgs,
    FunctionDeclarationReturnArgs,
    FunctionCall,
    FunctionCallArgs,
    CodeBlock,

    Return,
}

impl From<TokenKind> for SyntaxKind {
    fn from(token_kind: TokenKind) -> Self {
        match token_kind {
            TokenKind::Error => Self::Error,
            TokenKind::Whitespace => Self::Whitespace,
            TokenKind::KeywordFunc => Self::KeywordFunc,
            TokenKind::KeywordReturn => Self::KeywordReturn,
            TokenKind::Colon => Self::Colon,
            TokenKind::Semicolon => Self::Semicolon,
            TokenKind::Comma => Self::Comma,
            TokenKind::Identifier => Self::Identifier,
            TokenKind::Literal => Self::Literal,
            TokenKind::Plus => Self::Plus,
            TokenKind::Minus => Self::Minus,
            TokenKind::Asterix => Self::Asterix,
            TokenKind::Slash => Self::Slash,
            TokenKind::Equals => Self::Equals,
            TokenKind::LParen => Self::LParen,
            TokenKind::RParen => Self::RParen,
            TokenKind::LBrace => Self::LBrace,
            TokenKind::RBrace => Self::RBrace,
            TokenKind::CPPComment => Self::CPPComment,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum GlueLanguage {}

impl rowan::Language for GlueLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}
