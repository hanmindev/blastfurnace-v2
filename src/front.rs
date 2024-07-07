pub mod ast_types;
mod passes;

use std::collections::{HashMap, HashSet};
use std::io::Read;
use crate::front::ast_types::Definition;
use crate::modules::{ModuleDependencies, ModuleId};

pub fn parse_file(file_reader: Box<dyn Read>) -> (ModuleDependencies, DefinitionMap) {
    let module_dependencies: ModuleDependencies = HashSet::new();
    let definitions: DefinitionMap = HashMap::new();

    todo!()
}

pub type DefinitionMap = HashMap<String, Definition>;