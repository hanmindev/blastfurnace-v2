use std::collections::{HashMap, HashSet};
use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};

pub type ModuleId = String;
pub fn module_id(package_name: &str, file_path: &Utf8PathBuf) -> ModuleId {
    format!("{}:{}", package_name, file_path.with_extension("").to_string())
}



// One file gets mapped to one or more modules.
pub struct ModuleNode {
    // immutable metadata: based on file location, does not change after initial creation
    pub id: ModuleId, // unique identifier
    pub origin_file_path: Utf8PathBuf, // the file where the module is defined
    pub children: HashSet<ModuleId>, // child modules

    // mutable metadata: changes when file content is modified
    pub body: Option<ModuleCachableData>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ModuleCachableData {
    pub read_on: u128, // when this data is from
    pub direct_deps: HashSet<ModuleId>, // direct dependencies, used for computing the dependency graph
    pub definitions: Vec<String>, // front-end objects
    pub object: Option<String>, // the object code created by the back-end
}

impl ModuleNode {
    fn new(id: ModuleId, origin_file_path: Utf8PathBuf) -> ModuleNode {
        ModuleNode {
            id,
            origin_file_path,
            children: HashSet::new(),

            body: None,
        }
    }
}

pub struct ModuleGraph {
    pub root: Option<ModuleId>,
    pub nodes: HashMap<ModuleId, ModuleNode>,
}

impl ModuleGraph {
    pub fn new() -> ModuleGraph {
        ModuleGraph {
            root: None,
            nodes: HashMap::new(),
        }
    }

    pub fn create_node(&mut self, id: ModuleId, file_path: &Utf8PathBuf, module_name: &str) {
        self.nodes.insert(id.clone(), ModuleNode::new(id, file_path.clone()));
    }
}