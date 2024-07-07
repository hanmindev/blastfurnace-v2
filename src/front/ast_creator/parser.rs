use std::mem;
use crate::front::ast_creator::token_types::{Span, Token, TokenKind};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ParseError {
    Unexpected(Token, String),
    Unknown,
}

pub type ParseResult<T> = Result<T, ParseError>;

pub struct Parser {
    tokens: Vec<Token>,

    curr_index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Self {
            tokens,
            curr_index: 0,
        };
        parser
    }

    fn eat(&mut self, type_: &TokenKind) -> ParseResult<&Token> {
        let old_token = &self.tokens[self.curr_index];
        // return old token kind, set new token
        if mem::discriminant(&old_token.kind) == mem::discriminant(&type_) {
            if self.curr_index < self.tokens.len() - 1 {
                self.curr_index += 1;
            }
            return Ok(old_token);
        } else {
            Err(ParseError::Unexpected(
                old_token.clone(),
                format!("Tried to eat {:?}, ate {:?}", type_, old_token),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::front::ast_creator::token_types::{Span, Token, TokenKind};

    #[test]
    fn test_parser_eat() {
        let tokens = vec![
            Token {
                kind: TokenKind::Ident("a".to_string()),
                span: Span { lo: 0, hi: 0 },
            },
            Token {
                kind: TokenKind::Ident("b".to_string()),
                span: Span { lo: 1, hi: 1 },
            },
            Token {
                kind: TokenKind::Ident("c".to_string()),
                span: Span { lo: 2, hi: 2 },
            },
            Token {
                kind: TokenKind::Eof,
                span: Span { lo: 3, hi: 3 },
            },
        ];

        let mut parser = Parser::new(tokens);

        assert_eq!(
            parser.eat(&TokenKind::Ident("a".to_string())).unwrap().kind,
            TokenKind::Ident("a".to_string())
        );

        assert_eq!(
            parser.eat(&TokenKind::Ident("b".to_string())).unwrap().kind,
            TokenKind::Ident("b".to_string())
        );

        assert_eq!(
            parser.eat(&TokenKind::Ident("c".to_string())).unwrap().kind,
            TokenKind::Ident("c".to_string())
        );

        assert_eq!(
            parser.eat(&TokenKind::Eof).unwrap().kind,
            TokenKind::Eof
        );

        assert_eq!(
            parser.eat(&TokenKind::Eof).unwrap().kind,
            TokenKind::Eof
        );
    }
}