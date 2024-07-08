use crate::front::ast_creator::lexer::get_tokens;
use crate::front::ast_types::Module;

mod lexer;
mod token_types;
mod parser;

fn create_ast(src: &str) -> Module {
    // TODO: error handling
    let tokens = get_tokens(src).unwrap();
    let ast = parser::parse_tokens(tokens).unwrap();

    return ast;
}

#[cfg(test)]
mod tests {
    use crate::front::ast_creator::create_ast;
    use crate::front::ast_types::{Definition, FnDef, FunctionReference, Module, StructDef, Type, TypeReference, UseMap, VarDef, VarReference};
    use std::collections::HashMap;
    use camino::Utf8PathBuf;

    #[test]
    fn test_create_ast_use() {
        let src = r#"
        use root::struct_a;
        use root::path::path2::{struct_b, struct_c};
        use package_a::path::path2::{struct_d, struct_e};
        "#;

        let expected_ast = Module {
            use_map: UseMap {
                uses: HashMap::from([
                    ("struct_a".to_string(), ("root".to_string(), Utf8PathBuf::from(""), "struct_a".to_string())),
                    ("struct_b".to_string(), ("root".to_string(), Utf8PathBuf::from("path\\path2"), "struct_b".to_string())),
                    ("struct_c".to_string(), ("root".to_string(), Utf8PathBuf::from("path\\path2"), "struct_c".to_string())),
                    ("struct_d".to_string(), ("package_a".to_string(), Utf8PathBuf::from("path\\path2"), "struct_d".to_string())),
                    ("struct_e".to_string(), ("package_a".to_string(), Utf8PathBuf::from("path\\path2"), "struct_e".to_string())),
                ])
            },
            definitions: vec![],
        };

        let ast = create_ast(src);
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn test_create_ast_struct() {
        let src = r#"
        struct struct_a {
            field_a: int,
            field_b: struct_b,
        }
        "#;

        let expected_ast = Module {
            use_map: UseMap {
                uses: HashMap::new(),
            },
            definitions: vec![
                Definition::StructDef(StructDef {
                    name: TypeReference::new("struct_a".to_string()),
                    field_types: {
                        let mut field_types = HashMap::new();
                        field_types.insert("field_a".to_string(), Type::Int);
                        field_types.insert("field_b".to_string(), Type::Struct(TypeReference::new("struct_b".to_string())));
                        field_types
                    },
                }),
            ],
        };

        let ast = create_ast(src);
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn test_create_ast_static() {
        let src = r#"
        static val: int;
        "#;

        let expected_ast = Module {
            use_map: UseMap {
                uses: HashMap::new(),
            },
            definitions: vec![
                Definition::VarDef(VarDef {
                    name: VarReference::new("val".to_string()),
                    ty: Type::Int,
                }),
            ],
        };
    }

    #[test]
    fn test_create_ast_fn() {
        let src = r#"
        fn fn_a(arg_a: int, arg_b: struct_b) -> struct_c {
        }
        "#;

        let expected_ast = Module {
            use_map: UseMap {
                uses: HashMap::new(),
            },
            definitions: vec![
                Definition::FnDef(FnDef {
                    return_type: Type::Struct(TypeReference::new("struct_c".to_string())),
                    name: FunctionReference::new("fn_a".to_string()),
                    args: vec![
                        VarDef {
                            name: VarReference::new("arg_a".to_string()),
                            ty: Type::Int,
                        },
                        VarDef {
                            name: VarReference::new("arg_b".to_string()),
                            ty: Type::Struct(TypeReference::new("struct_b".to_string())),
                        },
                    ],
                }),
            ],
        };

        let ast = create_ast(src);
        assert_eq!(ast, expected_ast);
    }
}