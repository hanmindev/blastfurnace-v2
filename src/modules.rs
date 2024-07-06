use std::collections::VecDeque;
use camino::Utf8PathBuf;
use crate::modules::file_system::FileSystem;
use crate::modules::types::{module_id, ModuleGraph};

mod types;
mod file_system;

enum ModuleBuildError {
    NoMainInRoot,
}

type ModuleBuildResult<T> = Result<T, ModuleBuildError>;


pub struct ModuleBuilder<T: FileSystem> {
    module_graph: ModuleGraph,
    file_system: T,
}

impl<T: FileSystem> ModuleBuilder<T> {
    pub fn new(file_system: T) -> Self {
        Self {
            module_graph: ModuleGraph::new(),
            file_system,
        }
    }

    pub fn add_fs_package(&mut self, package_name: &str, path: &Utf8PathBuf, is_root: bool) -> ModuleBuildResult<()> {
        let mut queue = VecDeque::from([path.clone()]);

        let mut find_root = is_root;

        while let Some(current_path) = queue.pop_front() {
            let file_paths = self.file_system.list_files_with_extension(&current_path, "ing");

            for file_path in file_paths {
                if let Some(module_name) = file_path.file_name() {
                    queue.push_back(file_path.with_extension(""));
                    let id = module_id(package_name, &file_path);

                    if find_root && module_name == "main"{
                        self.module_graph.root = Some(id.clone());
                        find_root = false;
                    }

                    self.module_graph.create_node(id, &file_path, module_name);
                }
            }

            if find_root {
                self.module_graph.nodes.clear();
                return Err(ModuleBuildError::NoMainInRoot);
            }
        }

        return Ok(());
    }
}