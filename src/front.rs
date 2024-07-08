mod ast_creator;
pub mod ast_types;
mod passes;

use crate::front::ast_types::Definition;
use crate::modules::{ModuleDependencies, ModuleId};
use std::collections::{HashMap, HashSet};
use std::io::Read;
use crate::front::ast_creator::create_ast;
use crate::front::passes::name_resolution::NameResolver;

pub fn parse_file(module_id: ModuleId, file_contents: &str) -> (ModuleDependencies, DefinitionMap) {
    let mut ast_file = create_ast(file_contents);

    // TODO: error handling
    NameResolver::run(module_id, &mut ast_file).unwrap();

    let module_dependencies: ModuleDependencies = HashSet::new();
    let definitions: DefinitionMap;



    todo!()
}

pub type DefinitionMap = HashMap<String, Definition>;
