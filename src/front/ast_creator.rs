use crate::front::ast_creator::lexer::get_tokens;
use crate::front::ast_types::Module;

mod lexer;
mod parser;
mod token_types;

pub fn create_ast(file_root_package_name: &str, src: &str) -> Module {
    // TODO: error handling
    let tokens = get_tokens(src).unwrap();
    let ast = parser::parse_tokens(file_root_package_name, tokens).unwrap();

    return ast;
}

#[cfg(test)]
mod tests {
    use crate::front::ast_creator::create_ast;
    use crate::front::ast_types::{
        Definition, FnDef, FullItemPath, FunctionReference, Module, RawName, Statement,
        StaticVarDef, StructDef, Type, TypeReference, VarDef, VarReference,
    };
    use std::collections::HashMap;

    #[test]
    fn test_create_ast_use() {
        let current_package = "package_a";
        let src = r#"
        use root::struct_a;
        use root::path::path2::{struct_b, struct_c};
        use package_b::path::path2::{struct_d, struct_e};
        "#;

        let uses: Vec<(RawName, FullItemPath)> = vec![
            (
                ("struct_a".to_string(), None),
                FullItemPath::new(
                    "package_a".to_string(),
                    vec!["struct_a"]
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>(),
                ),
            ),
            (
                ("struct_b".to_string(), None),
                FullItemPath::new(
                    "package_a".to_string(),
                    vec!["path", "path2", "struct_b"]
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>(),
                ),
            ),
            (
                ("struct_c".to_string(), None),
                FullItemPath::new(
                    "package_a".to_string(),
                    vec!["path", "path2", "struct_c"]
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>(),
                ),
            ),
            (
                ("struct_d".to_string(), None),
                FullItemPath::new(
                    "package_b".to_string(),
                    vec!["path", "path2", "struct_d"]
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>(),
                ),
            ),
            (
                ("struct_e".to_string(), None),
                FullItemPath::new(
                    "package_b".to_string(),
                    vec!["path", "path2", "struct_e"]
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>(),
                ),
            ),
        ];

        let ast = create_ast(current_package, src);
        assert_eq!(uses, ast.uses.unwrap());
    }

    #[test]
    fn test_create_ast_struct() {
        let current_package = "package_a";
        let src = r#"
        struct struct_a {
            field_a: int,
            field_b: struct_b,
        }
        "#;

        let expected = Module {
            uses: Some(vec![]),
            definitions: Some(vec![
                (Definition::StructDef(StructDef {
                    name: TypeReference::new(("struct_a".to_string(), None)),
                    field_types: {
                        let mut field_types = HashMap::new();
                        field_types.insert("field_a".to_string(), Type::Int);
                        field_types.insert(
                            "field_b".to_string(),
                            Type::Struct(TypeReference::new(("struct_b".to_string(), None))),
                        );
                        field_types
                    },
                })),
            ]),
            statements: vec![],
        };

        let ast = create_ast(current_package, src);
        assert_eq!(expected, ast);
    }

    #[test]
    fn test_create_ast_static() {
        let current_package = "package_a";
        let src = r#"
        static val: int;
        "#;

        let expected_ast = Module {
            uses: Some(vec![]),
            definitions: Some(vec![
                (Definition::StaticVarDef(StaticVarDef {
                    name: VarReference::new(("val".to_string(), None)),
                    ty: Type::Int,
                })),
            ]),
            statements: vec![],
        };

        let ast = create_ast(current_package, src);
        assert_eq!(expected_ast, ast);
    }

    #[test]
    fn test_create_ast_void_fn() {
        let current_package = "package_a";
        let src = r#"
        fn fn_a() {
        }
        "#;

        let expected_ast = Module {
            uses: Some(vec![]),
            definitions: Some(vec![
                (Definition::FnDef(FnDef {
                    return_type: Type::Void,
                    name: FunctionReference::new(("fn_a".to_string(), None)),
                    args: vec![],
                    body: Module {
                        uses: Some(vec![]),
                        definitions: Some(vec![]),
                        statements: vec![],
                    },
                })),
            ]),
            statements: vec![],
        };

        let ast = create_ast(current_package, src);
        assert_eq!(expected_ast, ast);
    }

    #[test]
    fn test_create_ast_let() {
        let current_package = "package_a";
        let src = r#"
        fn fn_a() {
            let val: int;
        }
        "#;

        let expected_ast = Module {
            uses: Some(vec![]),
            definitions: Some(vec![
                (Definition::FnDef(FnDef {
                    return_type: Type::Void,
                    name: FunctionReference::new(("fn_a".to_string(), None)),
                    args: vec![],
                    body: Module {
                        uses: Some(vec![]),
                        definitions: Some(vec![
                            (Definition::VarDef(VarDef {
                                name: VarReference::new(("val".to_string(), None)),
                                ty: Type::Int,
                            })),
                        ]),
                        statements: vec![],
                    },
                })),
            ]),
            statements: vec![],
        };

        let ast = create_ast(current_package, src);
        assert_eq!(expected_ast, ast);
    }

    #[test]
    fn test_create_ast_fn() {
        let current_package = "package_a";
        let src = r#"
        fn fn_a(arg_a: int, arg_b: struct_b) -> struct_c {
        }
        "#;

        let expected_ast = Module {
            uses: Some(vec![]),
            definitions: Some(vec![
                (Definition::FnDef(FnDef {
                    return_type: Type::Struct(TypeReference::new(("struct_c".to_string(), None))),
                    name: FunctionReference::new(("fn_a".to_string(), None)),
                    args: vec![
                        VarDef {
                            name: VarReference::new(("arg_a".to_string(), None)),
                            ty: Type::Int,
                        },
                        VarDef {
                            name: VarReference::new(("arg_b".to_string(), None)),
                            ty: Type::Struct(TypeReference::new(("struct_b".to_string(), None))),
                        },
                    ],
                    body: Module {
                        uses: Some(vec![]),
                        definitions: Some(vec![]),
                        statements: vec![],
                    },
                })),
            ]),
            statements: vec![],
        };

        let ast = create_ast(current_package, src);
        assert_eq!(expected_ast, ast);
    }

    #[test]
    fn test_create_ast_scope_intermediate() {
        let current_package = "package_a";
        let src = r#"
        fn fn_a() {
        {}
        }
        "#;

        let expected_ast = Module {
            uses: Some(vec![]),
            definitions: Some(vec![
                (Definition::FnDef(FnDef {
                    return_type: Type::Void,
                    name: FunctionReference::new(("fn_a".to_string(), None)),
                    args: vec![],
                    body: Module {
                        uses: Some(vec![]),
                        definitions: Some(vec![]),
                        statements: vec![
                            (Statement::Module(Module {
                                uses: Some(vec![]),
                                definitions: Some(vec![]),
                                statements: vec![],
                            })),
                        ],
                    },
                })),
            ]),
            statements: vec![],
        };

        let ast = create_ast(current_package, src);
        assert_eq!(expected_ast, ast);
    }

    #[test]
    fn test_create_ast_layered_definition() {
        let current_package = "package_a";
        let src = r#"
        fn fn_a() {
        {
        struct struct_a {
            field_a: int,
            field_b: struct_b,
        }
        }
        }
        "#;

        let expected_ast = Module {
            uses: Some(vec![]),
            definitions: Some(vec![
                (Definition::FnDef(FnDef {
                    return_type: Type::Void,
                    name: FunctionReference::new(("fn_a".to_string(), None)),
                    args: vec![],
                    body: Module {
                        uses: Some(vec![]),
                        definitions: Some(vec![]),
                        statements: vec![
                            (Statement::Module(Module {
                                uses: Some(vec![]),
                                definitions: Some(vec![
                                    (Definition::StructDef(StructDef {
                                        name: TypeReference::new(("struct_a".to_string(), None)),
                                        field_types: {
                                            let mut field_types = HashMap::new();
                                            field_types.insert("field_a".to_string(), Type::Int);
                                            field_types.insert(
                                                "field_b".to_string(),
                                                Type::Struct(TypeReference::new((
                                                    "struct_b".to_string(),
                                                    None,
                                                ))),
                                            );
                                            field_types
                                        },
                                    })),
                                ]),
                                statements: vec![],
                            })),
                        ],
                    },
                })),
            ]),
            statements: vec![],
        };

        let ast = create_ast(current_package, src);
        assert_eq!(expected_ast, ast);
    }
}
