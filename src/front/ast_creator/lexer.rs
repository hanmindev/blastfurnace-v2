use std::collections::VecDeque;
use std::io::Read;
use std::str::CharIndices;
use crate::front::ast_creator::token_types::{Span, Token, TokenError, TokenKind};

fn get_tokens(src: &str) -> Result<Vec<Token>, Vec<TokenError>> {
    let mut lexer = Lexer::new(src);
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    loop {
        match lexer.get_token() {
            Ok(token) => {
                if token.kind == TokenKind::Eof {
                    break;
                }
                tokens.push(token);
            }
            Err(err) => {
                errors.push(err);
            }
        }
    }

    if errors.is_empty() {
        Ok(tokens)
    } else {
        Err(errors)
    }
}

struct Lexer<'src> {
    src: &'src str,
    chars: CharIndices<'src>,
    curr: char,
    pos: usize,

    peeked_chars: VecDeque<(usize, char)>,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Self {
        let mut lexer = Self {
            src,
            chars: src.char_indices(),
            curr: '\0',
            pos: 0,
            peeked_chars: Default::default(),
        };
        lexer.eat();
        lexer
    }

    fn eat(&mut self) -> char {
        let prev = self.curr;
        if let Some((pos, ch)) = self.peeked_chars.pop_front() {
            (self.pos, self.curr) = (pos, ch);
        } else {
            (self.pos, self.curr) = self.chars.next().unwrap_or((self.src.len(), '\0'));
        }
        prev
    }

    fn peek(&mut self, offset: usize) -> char {
        if offset == 0 {
            return self.curr;
        }
        while offset > self.peeked_chars.len() {
            self.peeked_chars.push_back(self.chars.next().unwrap_or((self.src.len(), '\0')));
        }
        self.peeked_chars[offset - 1].1
    }

    pub fn get_token(&mut self) -> Result<Token, TokenError> {
        let lo = self.pos;
        return Ok(Token { kind: self.parse_token()?, span: Span { lo, hi: self.pos - 1 } });
    }

    fn parse_token(&mut self) -> Result<TokenKind, TokenError> {
        // check for EOF
        if self.curr == '\0' {
            return Ok(TokenKind::Eof);
        }

        // skip whitespace and comments
        loop {
            let is_whitespace = self.curr.is_whitespace();
            while self.curr.is_whitespace() {
                self.eat();
            }
            let mut is_comment_start = false;
            if self.curr == '/' && self.peek(1) == '/' {
                is_comment_start = true;
                // comment until end of line
                loop {
                    self.eat();
                    if self.curr == '\n' || self.curr == '\r' {
                        self.eat();
                        break;
                    }
                }
            }

            if !is_whitespace && !is_comment_start {
                break;
            }
        }

        // check for identifier
        // identifier: [a-zA-Z_][a-zA-Z0-9_]*
        if self.curr.is_alphabetic() {
            let mut ident = String::new();

            // read word and set to ident
            while self.curr.is_alphanumeric() || self.curr == '_' || self.curr == '-' {
                ident.push(self.eat());
            }

            return Ok(match ident.as_str() {
                "use" => TokenKind::Use,

                "void" => TokenKind::TVoid,
                "int" => TokenKind::TInt,

                "static" => TokenKind::Static,
                "struct" => TokenKind::Struct,
                "fn" => TokenKind::Fn,
                _ => TokenKind::Ident(ident),
            });
        }

        let prev = self.eat();

        match (prev, self.curr) {
            (':', ':') => {
                self.eat();
                return Ok(TokenKind::DoubleColon);
            }
            _ => {}
        }

        // match singletons
        Ok(match prev {
            ':' => TokenKind::Colon,
            ';' => TokenKind::SemiColon,
            ',' => TokenKind::Comma,

            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            _ => return Err(TokenError::InvalidToken(format!("{}", prev))),
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eat() {
        let src = "hello 안녕😎하세요 world";
        let mut lexer = Lexer::new(src);

        assert_eq!(lexer.eat(), 'h');
        assert_eq!(lexer.eat(), 'e');
        assert_eq!(lexer.eat(), 'l');
        assert_eq!(lexer.eat(), 'l');
        assert_eq!(lexer.eat(), 'o');
        assert_eq!(lexer.eat(), ' ');
        assert_eq!(lexer.eat(), '안');
        assert_eq!(lexer.eat(), '녕');
        assert_eq!(lexer.eat(), '😎');
        assert_eq!(lexer.eat(), '하');
        assert_eq!(lexer.eat(), '세');
        assert_eq!(lexer.eat(), '요');
        assert_eq!(lexer.eat(), ' ');
        assert_eq!(lexer.eat(), 'w');
        assert_eq!(lexer.eat(), 'o');
        assert_eq!(lexer.eat(), 'r');
        assert_eq!(lexer.eat(), 'l');
        assert_eq!(lexer.eat(), 'd');
        assert_eq!(lexer.eat(), '\0');
        assert_eq!(lexer.eat(), '\0');
    }

    #[test]
    fn test_peek() {
        let src = "hello 안녕😎하세요 world";
        let mut lexer = Lexer::new(src);

        assert_eq!(lexer.eat(), 'h');
        assert_eq!(lexer.eat(), 'e');
        assert_eq!(lexer.eat(), 'l');
        assert_eq!(lexer.eat(), 'l');
        assert_eq!(lexer.eat(), 'o');
        assert_eq!(lexer.eat(), ' ');
        assert_eq!(lexer.eat(), '안');
        assert_eq!(lexer.eat(), '녕');
        assert_eq!(lexer.peek(3), '요');
        assert_eq!(lexer.eat(), '😎');
        assert_eq!(lexer.eat(), '하');
        assert_eq!(lexer.eat(), '세');
        assert_eq!(lexer.eat(), '요');
        assert_eq!(lexer.eat(), ' ');
        assert_eq!(lexer.eat(), 'w');
        assert_eq!(lexer.eat(), 'o');
        assert_eq!(lexer.eat(), 'r');
        assert_eq!(lexer.eat(), 'l');
        assert_eq!(lexer.eat(), 'd');
        assert_eq!(lexer.eat(), '\0');
        assert_eq!(lexer.eat(), '\0');
    }
}