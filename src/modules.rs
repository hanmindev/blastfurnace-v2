use crate::file_system::FileSystem;
use crate::front::parse_file;
use crate::modules::cache::BuildCacheLayer;
use crate::modules::types::{ModuleCachableData, ModuleGraph};
use camino::{Utf8Path, Utf8PathBuf};
use std::collections::{HashSet, VecDeque};
use std::io::Read;

mod cache;
mod types;

enum ModuleBuildError {
    NoMainInRoot,
    FileNoLongerExists,
    FileReadError,
}

type ModuleBuildResult<T> = Result<T, ModuleBuildError>;

pub struct ModuleBuilder<'p, T: FileSystem> {
    module_graph: ModuleGraph,
    build_cache: BuildCacheLayer<'p, T>,
}

impl<'p, T: FileSystem> ModuleBuilder<'p, T> {
    pub fn new(file_system: &'p mut T, cache: Option<Box<Utf8Path>>) -> Self {
        Self {
            module_graph: ModuleGraph::new(),
            build_cache: BuildCacheLayer::new(file_system, cache),
        }
    }

    pub fn add_fs_package(
        &mut self,
        package_name: &str,
        path: &Utf8PathBuf,
        is_root: bool,
    ) -> ModuleBuildResult<()> {
        self.module_graph
            .package_map
            .insert(package_name.to_string(), path.clone());

        let mut queue = VecDeque::from([path.clone()]);

        let mut find_root = is_root;

        while let Some(current_path) = queue.pop_front() {
            let file_paths = self
                .build_cache
                .file_system
                .list_files_with_extension(&current_path, "ing");

            for file_path in file_paths {
                if let Some(module_name) = file_path.file_name() {
                    queue.push_back(file_path.with_extension(""));
                    let rel_path = &create_rel_path(&file_path, &path);
                    let id = module_id_from_local(package_name, rel_path);

                    if find_root && module_name == "main" {
                        self.module_graph.root = Some(id.clone());
                        find_root = false;
                    }

                    self.module_graph.create_node(id, &package_name, &rel_path);
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
            let rel_path = node.rel_path.clone();
            let abs_path = self
                .module_graph
                .package_map
                .get(&node.package_name)
                .unwrap()
                .join(&rel_path);

            let age = self
                .build_cache
                .file_system
                .get_file_age(&abs_path)
                .or(Err(ModuleBuildError::FileNoLongerExists))?;

            let module_id = module_id_from_local(&node.package_name, &rel_path);
            if let Some(&ref body) = self.build_cache.get_module(&module_id) {
                if body.read_on == age {
                    node.body = Some(body.clone());
                    continue;
                }
            }

            node.body = {
                let mut reader = self
                    .build_cache
                    .file_system
                    .get_reader(&abs_path)
                    .or(Err(ModuleBuildError::FileNoLongerExists))?;

                let mut file_content = String::new();
                reader
                    .read_to_string(&mut file_content)
                    .or(Err(ModuleBuildError::FileReadError))?;

                let (direct_deps, definitions) =
                    parse_file(&node.package_name, module_id, ("pkg".to_string(), vec![], "module_a".to_string()), &file_content);
                // TODO:

                Some(ModuleCachableData {
                    read_on: age,
                    direct_deps,
                    definitions,
                    object: None,
                })
            };
        }

        Ok(())
    }
}

fn create_rel_path(file_path: &Utf8PathBuf, package_path: &Utf8PathBuf) -> Utf8PathBuf {
    file_path.strip_prefix(package_path).unwrap().to_path_buf()
}

// the module id contains the package name and the relative path to module file from the package root.
pub type ModuleId = String;

pub fn module_id_from_local(package_name: &str, file_path: &Utf8PathBuf) -> ModuleId {
    file_path.with_extension("").iter().fold(package_name.to_string(), |a, b| a + "::" + b)
}

pub type ModuleDependencies = HashSet<ModuleId>;
