#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // import
    Use,
    DoubleColon,

    Ident(String),

    // literals
    LNull,
    LInt(i32),

    // type keyword
    TVoid,
    TInt,

    // definition declaration
    Static,
    Struct,
    Fn,

    // misc
    Colon,
    SemiColon,
    Comma,

    LBrace,
    RBrace,
    LParen,
    RParen,

    Arrow,

    Eof,
}

#[derive(Debug)]
pub enum TokenError {
    InvalidToken(String),
    MultipleDecimals,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}