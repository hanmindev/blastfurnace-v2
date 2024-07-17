use crate::front::ast_creator::token_types::{Span, Token, TokenKind};
use crate::front::ast_types::{
    Definition, FnDef, FunctionReference, Module, RawName, ResolvedName, StaticVarDef, StructDef,
    Type, TypeReference, VarDef, VarReference,
};
use crate::modules::module_id_from_local;
use camino::Utf8PathBuf;
use std::cmp::min;
use std::collections::HashMap;
use std::mem;

pub fn parse_tokens(package_name: &str, tokens: Vec<Token>) -> ParseResult<Module> {
    let mut parser = Parser::new(tokens);
    parser.parse_top_level(package_name)
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Unexpected(Token, String),
    Unknown,
}

pub type ParseResult<T> = Result<T, ParseError>;

struct Parser {
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

    fn get_token(&self) -> &Token {
        &self.tokens[self.curr_index]
    }

    fn eat_any(&mut self) -> &TokenKind {
        let old_token = &self.tokens[self.curr_index].kind;
        if self.curr_index < self.tokens.len() - 1 {
            self.curr_index += 1;
        }
        old_token
    }

    fn eat(&mut self, type_: &TokenKind) -> ParseResult<&TokenKind> {
        let old_token = &self.tokens[self.curr_index];
        // return old token kind, set new token
        if mem::discriminant(&old_token.kind) == mem::discriminant(&type_) {
            if self.curr_index < self.tokens.len() - 1 {
                self.curr_index += 1;
            }
            return Ok(&old_token.kind);
        } else {
            Err(ParseError::Unexpected(
                old_token.clone(),
                format!("Tried to eat {:?}, ate {:?}", type_, old_token),
            ))
        }
    }

    fn peek(&self, offset: usize) -> &TokenKind {
        &self.tokens[min(self.curr_index + offset, self.tokens.len() - 1)].kind
    }

    fn parse_top_level(&mut self, package_name: &str) -> ParseResult<Module> {
        let mut module = Module {
            uses: Some(Default::default()),
            definitions: Default::default(),
        };

        loop {
            match self.peek(0) {
                TokenKind::Use => {
                    module
                        .uses
                        .as_mut()
                        .unwrap()
                        .extend(self.parse_use(package_name)?);
                }
                TokenKind::Fn => {
                    let definition = self.parse_fn_definition()?;
                    module.definitions.push(Definition::FnDef(definition));
                }
                TokenKind::Struct => {
                    let definition = self.parse_struct_definition()?;
                    module.definitions.push(Definition::StructDef(definition));
                }
                TokenKind::Static => {
                    let definition = self.parse_static_var_definition()?;
                    module
                        .definitions
                        .push(Definition::StaticVarDef(definition));
                }
                TokenKind::LBrace => {
                    let definition = self.parse_intermediate_level(package_name)?;
                    module.definitions.push(Definition::Scope(definition));
                }
                TokenKind::Eof => {
                    break;
                }
                TokenKind::Let | _ => {
                    // this is added to explicitly show that we are ignoring Let s
                    return Err(ParseError::Unexpected(
                        self.get_token().clone(),
                        "Cannot be used for top level".to_string(),
                    ));
                }
            }
        }

        Ok(module)
    }

    fn parse_intermediate_level(&mut self, package_name: &str) -> ParseResult<Module> {
        let mut module = Module {
            uses: Some(Default::default()),
            definitions: Default::default(),
        };
        self.eat(&TokenKind::LBrace)?;
        loop {
            match self.peek(0) {
                TokenKind::Use => {
                    module
                        .uses
                        .as_mut()
                        .unwrap()
                        .extend(self.parse_use(package_name)?);
                }
                TokenKind::Fn => {
                    let definition = self.parse_fn_definition()?;
                    module.definitions.push(Definition::FnDef(definition));
                }
                TokenKind::Struct => {
                    let definition = self.parse_struct_definition()?;
                    module.definitions.push(Definition::StructDef(definition));
                }
                TokenKind::Let => {
                    let definition = self.parse_var_definition()?;
                    module.definitions.push(Definition::VarDef(definition));
                }
                TokenKind::RBrace => {
                    break;
                }
                TokenKind::Static | _ => {
                    // this is added to explicitly show that we are ignoring Static s
                    return Err(ParseError::Unexpected(
                        self.get_token().clone(),
                        "Cannot be used for intermediate level".to_string(),
                    ));
                }
            }
        }
        self.eat(&TokenKind::RBrace)?;

        Ok(module)
    }

    fn parse_type(&mut self) -> ParseResult<Type> {
        Ok(match self.eat_any() {
            TokenKind::TVoid => Type::Void,
            TokenKind::TInt => Type::Int,
            TokenKind::Ident(ident) => Type::Struct(TypeReference::new(ident.clone())),
            _ => {
                return Err(ParseError::Unexpected(
                    self.get_token().clone(),
                    "Expected ident".to_string(),
                ))
            }
        })
    }

    fn parse_fn_definition(&mut self) -> ParseResult<FnDef> {
        self.eat(&TokenKind::Fn)?;
        if let TokenKind::Ident(fn_name) = self.eat_any() {
            let fn_name = fn_name.clone();

            self.eat(&TokenKind::LParen)?;
            let mut args = vec![];
            loop {
                if self.peek(0) == &TokenKind::RParen {
                    break;
                }

                if let TokenKind::Ident(arg_name) = self.eat_any() {
                    let arg_name = arg_name.clone();

                    self.eat(&TokenKind::Colon)?;
                    let ty = self.parse_type()?;

                    args.push(VarDef {
                        name: VarReference::new(arg_name),
                        ty,
                    });

                    if self.eat(&TokenKind::Comma).is_err() {
                        break;
                    }
                } else {
                    return Err(ParseError::Unexpected(
                        self.get_token().clone(),
                        "Expected ident".to_string(),
                    ));
                }
            }

            self.eat(&TokenKind::RParen)?;
            self.eat(&TokenKind::Arrow)?;
            let return_type = self.parse_type()?;

            // TODO: parse body
            self.eat(&TokenKind::LBrace)?;
            self.eat(&TokenKind::RBrace)?;

            Ok(FnDef {
                return_type,
                name: FunctionReference::new(fn_name),
                args,
            })
        } else {
            Err(ParseError::Unexpected(
                self.get_token().clone(),
                "Expected ident".to_string(),
            ))
        }
    }

    fn parse_struct_definition(&mut self) -> ParseResult<StructDef> {
        self.eat(&TokenKind::Struct)?;
        if let TokenKind::Ident(struct_name) = self.eat_any() {
            let struct_name = struct_name.clone();

            let mut field_types = HashMap::new();

            self.eat(&TokenKind::LBrace)?;
            loop {
                if self.peek(0) == &TokenKind::RBrace {
                    break;
                }

                if let TokenKind::Ident(field_name) = self.eat_any() {
                    let field_name = field_name.clone();

                    self.eat(&TokenKind::Colon)?;
                    let ty = self.parse_type()?;

                    field_types.insert(field_name, ty);

                    if self.eat(&TokenKind::Comma).is_err() {
                        break;
                    }
                } else {
                    return Err(ParseError::Unexpected(
                        self.get_token().clone(),
                        "Expected ident".to_string(),
                    ));
                }
            }

            self.eat(&TokenKind::RBrace)?;
            Ok(StructDef {
                name: TypeReference::new(struct_name),
                field_types,
            })
        } else {
            Err(ParseError::Unexpected(
                self.get_token().clone(),
                "Expected ident".to_string(),
            ))
        }
    }

    fn parse_var_definition_helper(&mut self) -> ParseResult<(VarReference, Type)> {
        if let TokenKind::Ident(variable_name) = self.eat_any() {
            let variable_name = variable_name.clone();

            // TODO: currently no type inference, so type is required
            self.eat(&TokenKind::Colon)?;
            let ty = self.parse_type()?;

            self.eat(&TokenKind::SemiColon)?;
            Ok((VarReference::new(variable_name), ty))
        } else {
            Err(ParseError::Unexpected(
                self.get_token().clone(),
                "Expected ident".to_string(),
            ))
        }
    }

    fn parse_var_definition(&mut self) -> ParseResult<VarDef> {
        self.eat(&TokenKind::Let)?;
        let var_def = self.parse_var_definition_helper()?;
        Ok(VarDef {
            name: var_def.0,
            ty: var_def.1,
        })
    }

    fn parse_static_var_definition(&mut self) -> ParseResult<StaticVarDef> {
        self.eat(&TokenKind::Static)?;
        let var_def = self.parse_var_definition_helper()?;
        Ok(StaticVarDef {
            name: var_def.0,
            ty: var_def.1,
        })
    }

    // maps
    fn parse_use(&mut self, package_name: &str) -> ParseResult<Vec<(RawName, ResolvedName)>> {
        self.eat(&TokenKind::Use)?;

        if let TokenKind::Ident(use_package_name) = self.eat_any() {
            let package_name = if use_package_name == "root" {
                package_name.to_string()
            } else {
                use_package_name.clone()
            };

            let mut res = vec![];

            let mut path = Utf8PathBuf::new();
            self.eat(&TokenKind::DoubleColon)?;

            loop {
                match self.eat_any() {
                    TokenKind::Ident(ident) => {
                        let ident = ident.clone();
                        if self.peek(0) == &TokenKind::SemiColon {
                            res.push((
                                ident.clone(),
                                (module_id_from_local(&package_name, &path), ident.clone()),
                            ));
                            break;
                        } else {
                            path.push(ident);
                            self.eat(&TokenKind::DoubleColon)?;
                        }
                    }
                    TokenKind::LBrace => {
                        loop {
                            if let TokenKind::Ident(ident) =
                                self.eat(&TokenKind::Ident("".to_string()))?
                            {
                                res.push((
                                    ident.clone(),
                                    (module_id_from_local(&package_name, &path), ident.clone()),
                                ));
                                if self.eat(&TokenKind::Comma).is_err() {
                                    break;
                                }
                            } else {
                                panic!("Can't happen");
                            }
                        }
                        self.eat(&TokenKind::RBrace)?;
                        break;
                    }
                    _ => {
                        return Err(ParseError::Unexpected(
                            self.get_token().clone(),
                            "Expected ident or ::".to_string(),
                        ));
                    }
                }
            }

            self.eat(&TokenKind::SemiColon)?;
            Ok(res)
        } else {
            Err(ParseError::Unexpected(
                self.get_token().clone(),
                "Expected ident".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            parser.eat(&TokenKind::Ident("a".to_string())).unwrap(),
            &TokenKind::Ident("a".to_string())
        );

        assert_eq!(
            parser.eat(&TokenKind::Ident("b".to_string())).unwrap(),
            &TokenKind::Ident("b".to_string())
        );

        assert_eq!(
            parser.eat(&TokenKind::Ident("c".to_string())).unwrap(),
            &TokenKind::Ident("c".to_string())
        );

        assert_eq!(parser.eat(&TokenKind::Eof).unwrap(), &TokenKind::Eof);

        assert_eq!(parser.eat(&TokenKind::Eof).unwrap(), &TokenKind::Eof);
    }
}
