use std::collections::{HashSet, VecDeque};
use camino::{Utf8Path, Utf8PathBuf};
use crate::modules::cache::BuildCacheLayer;
use crate::file_system::FileSystem;
use crate::modules::types::{module_id, ModuleCachableData, ModuleGraph};

mod types;
mod cache;

enum ModuleBuildError {
    NoMainInRoot,
    FileNoLongerExists,
}

type ModuleBuildResult<T> = Result<T, ModuleBuildError>;


pub struct ModuleBuilder<'p, T: FileSystem> {
    module_graph: ModuleGraph,
    build_cache: BuildCacheLayer<'p, T>
}

impl<'p, T: FileSystem> ModuleBuilder<'p, T> {
    pub fn new(file_system: &'p mut T, cache: Option<Box<Utf8Path>>) -> Self {

        Self {
            module_graph: ModuleGraph::new(),
            build_cache: BuildCacheLayer::new(file_system, cache)
        }
    }

    pub fn add_fs_package(&mut self, package_name: &str, path: &Utf8PathBuf, is_root: bool) -> ModuleBuildResult<()> {
        let mut queue = VecDeque::from([path.clone()]);

        let mut find_root = is_root;

        while let Some(current_path) = queue.pop_front() {
            let file_paths = self.build_cache.file_system.list_files_with_extension(&current_path, "ing");

            for file_path in file_paths {
                if let Some(module_name) = file_path.file_name() {
                    queue.push_back(file_path.with_extension(""));
                    let id = module_id(package_name, &file_path);

                    if find_root && module_name == "main" {
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

    // can be multithreaded easily
    pub fn load_module_bodies(&mut self) -> ModuleBuildResult<()> {
        for node in self.module_graph.nodes.values_mut() {
            let file_path = node.origin_file_path.clone();

            let age = self.build_cache.file_system.get_file_age(&file_path).or(Err(ModuleBuildError::FileNoLongerExists))?;

            if let Some(&ref body) = self.build_cache.get_module(&node.id) {
                if body.read_on == age {
                    node.body = Some(body.clone());
                    continue;
                }
            }

            node.body = {
                Some(ModuleCachableData {
                    read_on: age,
                    direct_deps: HashSet::new(), // TODO: actually read the files and such
                    definitions: Vec::new(),
                    object: None
                })
            };
        }

        Ok(())
    }
}