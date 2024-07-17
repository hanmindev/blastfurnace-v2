mod scope_table;
mod visitor;

use crate::front::ast_types::{Definition, Module, RawName};
use crate::front::passes::name_resolution::scope_table::ScopeTable;
use crate::front::passes::visitor::Visitable;
use crate::modules::ModuleId;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum NameResolutionError {
    UndefinedVariable(String),
    Redefinition(RawName),
    UnresolvedNames(HashSet<RawName>),
    UndefinedLookup(RawName),
}

type NameResolutionResult<T> = Result<T, NameResolutionError>;

/* the following function is the entry point for the name resolution pass
* Whenever it sees a raw name referring to a type, variable, or function, it will give it a fully qualified name
* The fully qualified name is a tuple of the module id and the name of the definition
* It will also give the proper name for imported names
 */
pub fn resolve_names(
    module_id: ModuleId, // id of the module we are resolving names for
    module: &mut Module, // the ASTFile containing the definitions
) -> NameResolutionResult<()> {
    let mut scope_table = ScopeTable::new(module_id);
    module.visit(&mut scope_table)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::front::ast_creator::create_ast;
    use crate::front::ast_types::{Definition, Type};
    use crate::modules::ModuleId;

    use super::*;

    #[test]
    fn test_static_name_collision() {
        let current_package = "package_a";
        let src = r#"
        static var_a: int;
        static var_a: int;
        "#;
        let mut ast_file = create_ast(current_package, src);

        let module_id = ModuleId::from("module_a");

        let err = resolve_names(module_id.clone(), &mut ast_file);

        assert_eq!(
            err,
            Err(NameResolutionError::Redefinition(RawName::from("var_a")))
        );
    }

    #[test]
    fn test_struct_name_collision() {
        let current_package = "package_a";
        let src = r#"
        struct struct_a {
            field_a: int,
        }

        struct struct_a {
            field_a: int,
        }
        "#;
        let mut ast_file = create_ast(current_package, src);

        let module_id = ModuleId::from("module_a");

        let err = resolve_names(module_id.clone(), &mut ast_file);

        assert_eq!(
            err,
            Err(NameResolutionError::Redefinition(RawName::from("struct_a")))
        );
    }

    #[test]
    fn test_function_name_collision() {
        let current_package = "package_a";
        let src = r#"
        fn fn_a() -> int {
        }
        fn fn_a() -> int {
        }
        "#;
        let mut ast_file = create_ast(current_package, src);

        let module_id = ModuleId::from("module_a");

        let err = resolve_names(module_id.clone(), &mut ast_file);

        assert_eq!(
            err,
            Err(NameResolutionError::Redefinition(RawName::from("fn_a")))
        );
    }

    #[test]
    fn test_circular_struct_name_resolution() {
        let current_package = "package_a";
        let src = r#"
        struct struct_a {
            field_a: struct_b,
        }

        struct struct_b {
            field_a: struct_a,
        }
        "#;
        let mut ast_file = create_ast(current_package, src);

        let module_id = ModuleId::from("module_a");

        resolve_names(module_id.clone(), &mut ast_file).unwrap();

        let definitions = ast_file.definitions;

        match definitions[0] {
            Definition::StructDef(ref struct_def) => {
                assert_eq!(
                    Some((module_id.clone(), "0:0:struct_a".to_string())),
                    struct_def.name.resolved
                );
                match struct_def.field_types["field_a"] {
                    Type::Struct(ref type_ref) => {
                        assert_eq!(
                            type_ref.resolved,
                            Some((module_id.clone(), "0:0:struct_b".to_string()))
                        );
                    }
                    _ => panic!("Expected Struct"),
                }
            }
            _ => panic!("Expected StructDef"),
        }

        match definitions[1] {
            Definition::StructDef(ref struct_def) => {
                assert_eq!(
                    Some((module_id.clone(), "0:0:struct_b".to_string())),
                    struct_def.name.resolved
                );
                match struct_def.field_types["field_a"] {
                    Type::Struct(ref type_ref) => {
                        assert_eq!(
                            type_ref.resolved,
                            Some((module_id.clone(), "0:0:struct_a".to_string()))
                        );
                    }
                    _ => panic!("Expected Struct"),
                }
            }
            _ => panic!("Expected StructDef"),
        }
    }
    #[test]
    fn test_unresolved_struct_name_resolution() {
        let current_package = "package_a";
        let src = r#"
        struct struct_a {
            field_a: struct_b,
        }
        "#;
        let mut ast_file = create_ast(current_package, src);

        let module_id = ModuleId::from("module_a");

        let err = resolve_names(module_id.clone(), &mut ast_file);

        assert_eq!(
            err,
            Err(NameResolutionError::UnresolvedNames(HashSet::from_iter(
                vec!["struct_b".to_string()]
            )))
        );
    }
    #[test]
    fn test_scoped_circular_struct_error() {
        let current_package = "package_a";
        let src = r#"
        struct struct_a {
            field_a: struct_b,
        }
        fn fn_a() {
            struct struct_b {
                field_a: struct_a,
            }
        }
        "#;
        let mut ast_file = create_ast(current_package, src);

        let module_id = ModuleId::from("module_a");

        let err = resolve_names(module_id.clone(), &mut ast_file);

        assert_eq!(
            err,
            Err(NameResolutionError::UnresolvedNames(HashSet::from_iter(
                vec!["struct_b".to_string()]
            )))
        );
    }
    #[test]
    fn test_scoped_non_circular_struct() {
        let current_package = "package_a";
        let src = r#"
        struct struct_a {
            field_a: int,
        }
        fn fn_a() {
            struct struct_b {
                field_a: struct_a,
            }
        }
        "#;
        let mut ast_file = create_ast(current_package, src);

        let module_id = ModuleId::from("module_a");

        resolve_names(module_id.clone(), &mut ast_file).unwrap();

        let definitions = ast_file.definitions;

        match definitions[0] {
            Definition::StructDef(ref struct_def) => {
                assert_eq!(
                    Some((module_id.clone(), "0:0:struct_a".to_string())),
                    struct_def.name.resolved
                );
            }
            _ => panic!("Expected StructDef"),
        }

        match definitions[1] {
            Definition::FnDef(ref fn_def) => {
                match fn_def.body.definitions[0] {
                    Definition::StructDef(ref struct_def) => {
                        assert_eq!(
                            Some((module_id.clone(), "1:0:struct_b".to_string())),
                            struct_def.name.resolved
                        );
                        match struct_def.field_types["field_a"] {
                            Type::Struct(ref type_ref) => {
                                assert_eq!(
                                    type_ref.resolved,
                                    Some((module_id.clone(), "0:0:struct_a".to_string()))
                                );
                            }
                            _ => panic!("Expected Struct"),
                        }
                    }
                    _ => panic!("Expected StructDef"),
                }
            }
            _ => panic!("Expected FunctionDef"),
        }
    }
}
