use std::io::Read;
use std::str::{CharIndices};

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
    SemiColon,
    Eof,
}

#[derive(Debug)]
pub enum TokenError {
    InvalidToken(String),
    MultipleDecimals,
}

pub struct Span {
    pub lo: u64,
    pub hi: u64,
}

pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

pub struct Lexer<'src> {
    src: &'src str,
    chars: CharIndices<'src>,
    curr: char,
    pos: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Self {
        let mut lexer = Self {
            src,
            chars: src.char_indices(),
            curr: '\0',
            pos: 0,
        };
        lexer.eat();
        lexer
    }

    fn eat(&mut self) -> (usize, char) {
        let prev = (self.pos, self.curr);
        (self.pos, self.curr) = self.chars.next().unwrap_or((self.src.len(), '\0'));
        prev
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let src = "hello 안녕😎하세요 world";
        let mut lexer = Lexer::new(src);

        assert_eq!(lexer.eat(), (0, 'h'));
        assert_eq!(lexer.eat(), (1, 'e'));
        assert_eq!(lexer.eat(), (2, 'l'));
        assert_eq!(lexer.eat(), (3, 'l'));
        assert_eq!(lexer.eat(), (4, 'o'));
        assert_eq!(lexer.eat(), (5, ' '));
        assert_eq!(lexer.eat(), (6, '안'));
        assert_eq!(lexer.eat(), (9, '녕'));
        assert_eq!(lexer.eat(), (12, '😎'));
        assert_eq!(lexer.eat(), (16, '하'));
        assert_eq!(lexer.eat(), (19, '세'));
        assert_eq!(lexer.eat(), (22, '요'));
        assert_eq!(lexer.eat(), (25, ' '));
        assert_eq!(lexer.eat(), (26, 'w'));
        assert_eq!(lexer.eat(), (27, 'o'));
        assert_eq!(lexer.eat(), (28, 'r'));
        assert_eq!(lexer.eat(), (29, 'l'));
        assert_eq!(lexer.eat(), (30, 'd'));
        assert_eq!(lexer.eat(), (31, '\0'));
        assert_eq!(lexer.eat(), (31, '\0'));
    }
}