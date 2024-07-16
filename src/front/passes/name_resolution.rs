mod scope_table;
mod visitor;

use crate::front::ast_types::{ASTFile, Definition, RawName, ResolvedName};
use crate::front::passes::name_resolution::scope_table::{ScopeTable};
use crate::front::passes::visitor::Visitable;
use crate::modules::{ModuleId};

#[derive(Debug, PartialEq)]
pub enum NameResolutionError {
    UndefinedVariable(String),
    Redefinition(RawName, ResolvedName),
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


    // load the "use" statements into the scope table. There should not be any duplicates
    for (raw_name, resolved_name) in astfile.uses.drain(0..) {
        scope_table.scope_bind_pre_made_name(raw_name, resolved_name)?;
    }

    // then we visit each definition in the ASTFile
    for definition in astfile.definitions.iter_mut() {
        definition.visit(&mut scope_table)?;
    }

    Ok(astfile.definitions)
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::front::ast_types::{Definition, StructDef, Type, TypeReference, VarDef, VarReference};
    use crate::modules::ModuleId;

    use super::*;

    #[test]
    fn test_name_resolution() {
        let module_id = ModuleId::from("module_a");
        let mut ast_file: ASTFile = ASTFile {
            uses: vec![],
            definitions: vec![
                Definition::StructDef(StructDef {
                    name: TypeReference::new("struct_a".to_string()),
                    field_types: HashMap::new(),
                }),
                Definition::StructDef(StructDef {
                    name: TypeReference::new("struct_b".to_string()),
                    field_types: {
                        let mut field_types = HashMap::new();
                        field_types.insert(
                            "field_a".to_string(),
                            Type::Struct(TypeReference::new("struct_a".to_string())),
                        );
                        field_types.insert("field_b".to_string(), Type::Int);
                        field_types
                    },
                }),
                Definition::VarDef(VarDef {
                    name: VarReference::new("var_a".to_string()),
                    ty: Type::Struct(TypeReference::new("struct_b".to_string())),
                }),
            ],
        };

        let result = resolve_names(module_id.clone(), ast_file);
        assert_eq!(result.is_ok(), true);
        let definitions = result.unwrap();

        match definitions[0] {
            Definition::StructDef(ref struct_def) => {
                assert_eq!(
                    struct_def.name.resolved,
                    Some((module_id.clone(), "0:struct_a".to_string()))
                );
            }
            _ => panic!("Expected StructDef"),
        }

        match definitions[1] {
            Definition::StructDef(ref struct_def) => {
                assert_eq!(
                    struct_def.name.resolved,
                    Some((module_id.clone(), "0:struct_b".to_string()))
                );
                match struct_def.field_types["field_a"] {
                    Type::Struct(ref type_ref) => {
                        assert_eq!(
                            type_ref.resolved,
                            Some((module_id.clone(), "0:struct_a".to_string()))
                        );
                    }
                    _ => panic!("Expected Struct"),
                }
            }
            _ => panic!("Expected StructDef"),
        }

        match definitions[2] {
            Definition::VarDef(ref var_def) => {
                assert_eq!(
                    var_def.name.resolved,
                    Some((module_id.clone(), "0:var_a".to_string()))
                );
                match var_def.ty {
                    Type::Struct(ref type_ref) => {
                        assert_eq!(
                            type_ref.resolved,
                            Some((module_id.clone(), "0:struct_b".to_string()))
                        );
                    }
                    _ => panic!("Expected Struct"),
                }
            }
            _ => panic!("Expected VarDef"),
        }
    }
}
