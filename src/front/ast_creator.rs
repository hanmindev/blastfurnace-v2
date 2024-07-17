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
        Definition, FnDef, FunctionReference, Module, StaticVarDef, StructDef, Type, TypeReference,
        VarDef, VarReference,
    };
    use camino::Utf8PathBuf;
    use std::collections::HashMap;

    #[test]
    fn test_create_ast_use() {
        let current_package = "package_a";
        let src = r#"
        use root::struct_a;
        use root::path::path2::{struct_b, struct_c};
        use package_b::path::path2::{struct_d, struct_e};
        "#;

        let uses = vec![
            (
                "struct_a".to_string(),
                ("package_a:".to_string(), "struct_a".to_string()),
            ),
            (
                "struct_b".to_string(),
                ("package_a:path\\path2".to_string(), "struct_b".to_string()),
            ),
            (
                "struct_c".to_string(),
                ("package_a:path\\path2".to_string(), "struct_c".to_string()),
            ),
            (
                "struct_d".to_string(),
                ("package_b:path\\path2".to_string(), "struct_d".to_string()),
            ),
            (
                "struct_e".to_string(),
                ("package_b:path\\path2".to_string(), "struct_e".to_string()),
            ),
        ];

        let ast = create_ast(current_package, src);
        assert_eq!(uses, ast.uses.unwrap());
    }

    #[test]
    fn test_create_ast_struct() {
        let current_package = "package_a";
        let current_module_path = Utf8PathBuf::from("foo\\bar");
        let src = r#"
        struct struct_a {
            field_a: int,
            field_b: struct_b,
        }
        "#;

        let expected = Module {
            uses: Some(vec![]),
            definitions: vec![Definition::StructDef(StructDef {
                name: TypeReference::new("struct_a".to_string()),
                field_types: {
                    let mut field_types = HashMap::new();
                    field_types.insert("field_a".to_string(), Type::Int);
                    field_types.insert(
                        "field_b".to_string(),
                        Type::Struct(TypeReference::new("struct_b".to_string())),
                    );
                    field_types
                },
            })],
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
            definitions: vec![Definition::StaticVarDef(StaticVarDef {
                name: VarReference::new("val".to_string()),
                ty: Type::Int,
            })],
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
            definitions: vec![Definition::FnDef(FnDef {
                return_type: Type::Void,
                name: FunctionReference::new("fn_a".to_string()),
                args: vec![],
                body: Module {
                    uses: Some(vec![]),
                    definitions: vec![],
                },
            })],
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
            definitions: vec![Definition::FnDef(FnDef {
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
                body: Module {
                    uses: Some(vec![]),
                    definitions: vec![],
                },
            })],
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
            definitions: vec![Definition::FnDef(FnDef {
                return_type: Type::Void,
                name: FunctionReference::new("fn_a".to_string()),
                args: vec![],
                body: Module {
                    uses: Some(vec![]),
                    definitions: vec![Definition::Scope(Module {
                        uses: Some(vec![]),
                        definitions: vec![],
                    })],
                },
            })],
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
            definitions: vec![Definition::FnDef(FnDef {
                return_type: Type::Void,
                name: FunctionReference::new("fn_a".to_string()),
                args: vec![],
                body: Module {
                    uses: Some(vec![]),
                    definitions: vec![Definition::Scope(Module {
                        uses: Some(vec![]),
                        definitions: vec![Definition::StructDef(StructDef {
                            name: TypeReference::new("struct_a".to_string()),
                            field_types: {
                                let mut field_types = HashMap::new();
                                field_types.insert("field_a".to_string(), Type::Int);
                                field_types.insert(
                                    "field_b".to_string(),
                                    Type::Struct(TypeReference::new("struct_b".to_string())),
                                );
                                field_types
                            },
                        })],
                    })],
                },
            })],
        };

        let ast = create_ast(current_package, src);
        assert_eq!(expected_ast, ast);
    }
}
