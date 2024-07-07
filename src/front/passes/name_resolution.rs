mod visitor;
mod scope_table;

use crate::front::ast_types::Definition;
use crate::front::passes::name_resolution::scope_table::ScopeTable;
use crate::front::passes::visitor::Visitable;
use crate::modules::ModuleId;

#[derive(Debug, PartialEq)]
pub enum NameResolutionError {
    UndefinedVariable(String),
    Redefinition(String),
}

pub struct NameResolver {
    module_id: ModuleId,
    scope_table: ScopeTable,
}
impl NameResolver {
    pub fn run(module_id: ModuleId, definitions: &mut Vec<Definition>) -> Result<(), NameResolutionError> {
        let mut name_resolver = NameResolver {
            module_id,
            scope_table: ScopeTable::new(),
        };

        for definition in definitions.iter_mut() {
            definition.visit(&mut name_resolver)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    use crate::front::ast_types::{Definition, Reference, StructDef, Type, TypeReference, VarDef, VarReference};
    use crate::modules::ModuleId;

    #[test]
    fn test_name_resolution() {
        let module_id = ModuleId::from("module_a");
        let mut definitions = vec![
            Definition::StructDef(StructDef {
                name: TypeReference::new("struct_a".to_string()),
                field_types: HashMap::new(),
            }),
            Definition::StructDef(StructDef {
                name: TypeReference::new("struct_b".to_string()),
                field_types: {
                    let mut field_types = HashMap::new();
                    field_types.insert("field_a".to_string(), Type::Struct(TypeReference::new("struct_a".to_string())));
                    field_types.insert("field_b".to_string(), Type::Int);
                    field_types
                },
            }),
            Definition::VarDef(VarDef {
                name: VarReference::new("var_a".to_string()),
                ty: Type::Struct(TypeReference::new("struct_b".to_string())),
            }),
        ];

        let result = NameResolver::run(module_id.clone(), &mut definitions);
        assert_eq!(result, Ok(()));

        match definitions[0] {
            Definition::StructDef(ref struct_def) => {
                assert_eq!(struct_def.name.resolved, Some((module_id.clone(), "0:struct_a".to_string())));
            }
            _ => panic!("Expected StructDef"),
        }

        match definitions[1] {
            Definition::StructDef(ref struct_def) => {
                assert_eq!(struct_def.name.resolved, Some((module_id.clone(), "0:struct_b".to_string())));
                match struct_def.field_types["field_a"] {
                    Type::Struct(ref type_ref) => {
                        assert_eq!(type_ref.resolved, Some((module_id.clone(), "0:struct_a".to_string())));
                    }
                    _ => panic!("Expected Struct"),
                }
            }
            _ => panic!("Expected StructDef"),
        }

        match definitions[2] {
            Definition::VarDef(ref var_def) => {
                assert_eq!(var_def.name.resolved, Some((module_id.clone(), "0:var_a".to_string())));
                match var_def.ty {
                    Type::Struct(ref type_ref) => {
                        assert_eq!(type_ref.resolved, Some((module_id.clone(), "0:struct_b".to_string())));
                    }
                    _ => panic!("Expected Struct"),
                }
            }
            _ => panic!("Expected VarDef"),
        }
    }
}