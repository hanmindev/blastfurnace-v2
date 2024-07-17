mod scope_table;
mod visitor;

use std::collections::{HashMap, HashSet};
use crate::front::ast_types::{ASTFile, Definition, RawName, ResolvedName};
use crate::front::passes::name_resolution::scope_table::{ScopeTable};
use crate::front::passes::visitor::Visitable;
use crate::modules::{ModuleId};

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
    mut astfile: ASTFile, // the ASTFile containing the definitions
) -> Result<Vec<Definition>, NameResolutionError> {
    let mut scope_table = ScopeTable::new(module_id);
    scope_table.scope_enter();

    // load the "use" statements into the scope table. There should not be any duplicates
    for (raw_name, resolved_name) in astfile.uses.drain(0..) {
        scope_table.scope_bind(&raw_name, true, Some(resolved_name))?;
    }

    // then we visit each definition in the ASTFile
    for definition in astfile.definitions.iter_mut() {
        definition.visit(&mut scope_table)?;
    }
    scope_table.scope_exit()?;

    Ok(astfile.definitions)
}


#[cfg(test)]
mod tests {
    use crate::front::ast_creator::create_ast;
    use crate::front::ast_types::{Definition, Type};
    use crate::modules::ModuleId;

    use super::*;

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
        let ast_file = create_ast(current_package, src);

        let module_id = ModuleId::from("module_a");

        let definitions = resolve_names(module_id.clone(), ast_file).unwrap();

        match definitions[0] {
            Definition::StructDef(ref struct_def) => {
                assert_eq!(
                    Some((module_id.clone(), "struct_a".to_string())),
                    struct_def.name.resolved
                );
                match struct_def.field_types["field_a"] {
                    Type::Struct(ref type_ref) => {
                        assert_eq!(
                            type_ref.resolved,
                            Some((module_id.clone(), "struct_b".to_string()))
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
                    Some((module_id.clone(), "struct_b".to_string())),
                    struct_def.name.resolved
                );
                match struct_def.field_types["field_a"] {
                    Type::Struct(ref type_ref) => {
                        assert_eq!(
                            type_ref.resolved,
                            Some((module_id.clone(), "struct_a".to_string()))
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
        let ast_file = create_ast(current_package, src);

        let module_id = ModuleId::from("module_a");

        let err = resolve_names(module_id.clone(), ast_file);

        assert_eq!(
            err,
            Err(NameResolutionError::UnresolvedNames(HashSet::from_iter(vec!["struct_b".to_string()])))
        );
    }
}
