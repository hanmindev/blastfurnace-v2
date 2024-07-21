use crate::back::hmasm_types::HmasmFile;
use crate::front::definition_table::DefinitionTable;
use crate::modules::{ModuleDependencies, ModuleId};
use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// One file gets mapped to one or more modules.
pub struct ModuleNode {
    // immutable metadata: based on file location, does not change after initial creation
    pub package_name: String,  // the package name this module belongs to
    pub rel_path: Utf8PathBuf, // the file where the module is defined relative to package root
    pub children: HashSet<ModuleId>, // child modules

    // mutable metadata: changes when file content is modified
    pub body: Option<ModuleCachableData>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ModuleCachableData {
    pub read_on: u128,                   // when this data is from
    pub direct_deps: ModuleDependencies, // direct dependencies, used for computing the dependency graph
    pub definitions: DefinitionTable,    // front-end objects
    pub object: Option<HmasmFile>,       // the object code created by the back-end
}

impl ModuleNode {
    fn new(package_name: &str, rel_path: Utf8PathBuf) -> ModuleNode {
        ModuleNode {
            package_name: package_name.to_string(),
            rel_path,
            children: HashSet::new(),

            body: None,
        }
    }
}

pub struct ModuleGraph {
    pub root: Option<ModuleId>,                    // entrypoint module
    pub package_map: HashMap<String, Utf8PathBuf>, // maps package name to package root directory
    pub nodes: HashMap<ModuleId, ModuleNode>,      // all modules
}

impl ModuleGraph {
    pub fn new() -> ModuleGraph {
        ModuleGraph {
            root: None,
            package_map: HashMap::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn create_node(&mut self, id: ModuleId, package_name: &str, rel_path: &Utf8PathBuf) {
        self.nodes
            .insert(id.clone(), ModuleNode::new(package_name, rel_path.clone()));
    }
}
