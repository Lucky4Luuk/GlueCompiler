use std::fmt;

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
pub enum TokenKind {
    #[token("func")]
    KeywordFunc,
    #[token("return")]
    KeywordReturn,

    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,

    // #[regex(r#"/[*](~(.*[*]/.*))[*]/"#)]
    // CComment,
    #[regex(r#"//[^\n]*"#)]
    CPPComment,

    #[regex("[0-9]+")]
    Number,

    #[regex(r#"\+"#)]
    Plus,
    #[regex(r#"\-"#)]
    Minus,
    #[regex(r#"\*"#)]
    Asterix,
    #[regex(r#"/"#r)]
    Slash,
    #[regex(r#"="#)]
    Equals,

    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    #[regex(r#"[A-Za-z_][A-Za-z0-9_]*"#)]
    Identifier,

    #[regex(r#"[ \t\n\r\f]+"#)]
    Whitespace,

    #[error]
    Error,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            TokenKind::Whitespace => "whitespace",
            TokenKind::KeywordFunc => "`func`",
            TokenKind::KeywordReturn => "`return`",
            TokenKind::Colon => "`:`",
            TokenKind::Semicolon => "`;`",
            TokenKind::Identifier => "identifier",
            TokenKind::Number => "number",
            TokenKind::Plus => "`+`",
            TokenKind::Minus => "`-`",
            TokenKind::Asterix => "`*`",
            TokenKind::Slash => "`/`",
            TokenKind::Equals => "`=`",
            TokenKind::LParen => "`(`",
            TokenKind::RParen => "`)`",
            TokenKind::LBrace => "`{`",
            TokenKind::RBrace => "`}`",
            TokenKind::CPPComment => "comment",
            _ => unreachable!(),
        })
    }
}

impl TokenKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::CPPComment)
    }
}
