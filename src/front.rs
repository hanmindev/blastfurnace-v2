mod ast_creator;
pub mod ast_types;
mod passes;

use crate::front::ast_creator::create_ast;
use crate::front::ast_types::{Definition, FullItemPath};
use crate::front::passes::name_resolution::resolve_names;
use crate::modules::{ModuleDependencies, ModuleId};
use std::collections::{HashMap, HashSet};
use std::io::Read;

pub fn parse_file(
    package_name: &str,
    module_path: FullItemPath,
    file_contents: &str,
) -> (ModuleDependencies, DefinitionMap) {
    let mut module = create_ast(package_name, file_contents);

    // TODO: error handling
    let definitions = resolve_names(module_path, &mut module).unwrap();

    let module_dependencies: ModuleDependencies = HashSet::new();
    let definitions: DefinitionMap;

    todo!()
}

pub type DefinitionMap = HashMap<String, Definition>;
