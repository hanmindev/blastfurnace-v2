mod ast_creator;
pub mod ast_types;
mod passes;

use crate::front::ast_creator::create_ast;
use crate::front::ast_types::{Definition, FullItemPath};
use crate::front::passes::name_resolution::resolve_names;
use crate::modules::ModuleDependencies;
use std::collections::{HashMap, HashSet};

pub fn parse_file(
    module_path: FullItemPath,
    file_contents: &str,
) -> (ModuleDependencies, DefinitionMap) {
    let mut module = create_ast(&module_path.package_name, file_contents);

    // TODO: error handling
    let _definitions = resolve_names(module_path, &mut module).unwrap();

    let _module_dependencies: ModuleDependencies = HashSet::new();
    let _definitions: DefinitionMap;

    todo!()
}

pub type DefinitionMap = HashMap<String, Definition>;
