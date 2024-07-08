use crate::front::ast_types::{ASTFile, Definition};
use crate::front::DefinitionMap;
use crate::front::passes::name_resolution::scope_table::{ScopeTable, SymbolType};
use crate::front::passes::visitor::Visitable;
use crate::modules::{module_id_from_local, ModuleId};

mod scope_table;
mod visitor;

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
    pub fn run(
        module_id: ModuleId,
        mut astfile: &mut ASTFile,
    ) -> Result<(), NameResolutionError> {
        let mut name_resolver = NameResolver {
            module_id,
            scope_table: ScopeTable::new(),
        };

        for (key, (package_name, path, name)) in astfile.use_map.uses.iter() {
            let import_module_id = module_id_from_local(package_name, path);

            // TODO: need to handle errors, also I don't like how we are binding all three
            name_resolver.scope_table.scope_bind(&import_module_id, key, SymbolType::Var).unwrap();
            name_resolver.scope_table.scope_bind(&import_module_id, key, SymbolType::Fn).unwrap();
            name_resolver.scope_table.scope_bind(&import_module_id, key, SymbolType::Struct).unwrap();
        }


        for definition in astfile.definitions.iter_mut() {
            definition.visit(&mut name_resolver)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::front::ast_types::{Definition, StructDef, Type, TypeReference, UseMap, VarDef, VarReference};
    use crate::modules::ModuleId;

    use super::*;

    #[test]
    fn test_name_resolution() {
        let module_id = ModuleId::from("module_a");
        let mut ast_file: ASTFile = ASTFile {
            use_map: UseMap {
                uses: HashMap::new(),
            },
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

        let result = NameResolver::run(module_id.clone(), &mut ast_file);
        assert_eq!(result, Ok(()));

        match ast_file.definitions[0] {
            Definition::StructDef(ref struct_def) => {
                assert_eq!(
                    struct_def.name.resolved,
                    Some((module_id.clone(), "0:struct_a".to_string()))
                );
            }
            _ => panic!("Expected StructDef"),
        }

        match ast_file.definitions[1] {
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

        match ast_file.definitions[2] {
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
