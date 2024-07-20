mod ast_creator;
pub mod ast_types;
pub mod definition_table;
mod passes;

use crate::front::ast_creator::create_ast;
use crate::front::ast_types::FullItemPath;
use crate::front::definition_table::{collect_definitions, DefinitionTable};
use crate::front::passes::name_resolution::resolve_names;
use crate::modules::ModuleDependencies;
use std::collections::HashSet;

pub fn parse_file(
    module_path: FullItemPath,
    file_contents: &str,
) -> (ModuleDependencies, DefinitionTable) {
    let mut module = create_ast(&module_path.package_name, file_contents);

    // TODO: error handling
    resolve_names(module_path, &mut module).unwrap();

    // TODO: get this information
    let _module_dependencies: ModuleDependencies = HashSet::new();

    let definition_table = collect_definitions(&mut module);

    todo!()
}
