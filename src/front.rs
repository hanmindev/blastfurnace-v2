mod ast_creator;
pub mod ast_types;
pub mod definition_table;
mod passes;

use crate::front::ast_creator::create_ast;
use crate::front::ast_types::FullItemPath;
use crate::front::definition_table::DefinitionTable;
use crate::front::passes::collect_definitions::collect_definitions;
use crate::front::passes::collect_dependencies::collect_dependencies;
use crate::front::passes::name_resolution::resolve_names;
use crate::modules::{module_id_from_local, ModuleDependencies};

pub fn parse_file(
    module_path: FullItemPath,
    file_contents: &str,
) -> (ModuleDependencies, DefinitionTable) {
    let module_id = module_id_from_local(&module_path.package_name, &module_path.item_path);
    let mut module = create_ast(&module_path.package_name, file_contents);

    // TODO: error handling
    resolve_names(module_path, &mut module).unwrap();
    let module_dependencies = collect_dependencies(module_id, &mut module);
    let definition_table = collect_definitions(&mut module);

    (module_dependencies, definition_table)
}
