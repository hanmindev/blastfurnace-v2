pub mod ast_types;

use std::collections::{HashMap, HashSet};
use std::io::Read;
use crate::front::ast_types::Definition;
use crate::modules::{ModuleDependencies, ModuleId};

pub fn parse_file(file_reader: Box<dyn Read>) -> (ModuleDependencies, Definitions) {
    let module_dependencies: ModuleDependencies = HashSet::new();
    let definitions: Definitions = HashMap::new();

    todo!()
}

pub type Definitions = HashMap<String, Definition>;