use crate::file_system::FileSystem;
use crate::modules::cache::BuildCacheLayer;
use crate::modules::types::ModuleGraph;
use crate::modules::utf8buf_utils::utf8path_buf_to_vec;
use camino::Utf8PathBuf;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Read;

mod cache;
mod types;
mod utf8buf_utils;

#[derive(Debug)]
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
    pub fn new(file_system: &'p mut T, cache: Option<Utf8PathBuf>) -> Self {
        Self {
            module_graph: ModuleGraph::new(),
            build_cache: BuildCacheLayer::new(file_system, cache),
        }
    }

    pub fn add_fs_package(
        &mut self,
        package_name: &str,
        path: &Utf8PathBuf,
        is_root_package: bool,
    ) -> ModuleBuildResult<()> {
        self.module_graph
            .package_map
            .insert(package_name.to_string(), path.clone());

        let mut queue = VecDeque::from([path.clone()]);

        let mut is_root_dir = true;
        let mut found_root_module = false;

        while let Some(current_path) = queue.pop_front() {
            let file_paths = self
                .build_cache
                .file_system
                .list_files_with_extension(&current_path, "ing");

            for file_path in file_paths {
                if let Some(module_name) = file_path.file_name() {
                    let rel_path = &create_rel_path(&file_path, &path);
                    let id = module_id_from_local(
                        package_name,
                        &utf8path_buf_to_vec(&rel_path.with_extension("")),
                    );

                    if is_root_dir && module_name == "main.ing" && is_root_package {
                        self.module_graph.root = Some(id.clone());
                        found_root_module = true;
                    }

                    queue.push_back(file_path.with_extension(""));
                    self.module_graph.create_node(id, &package_name, &rel_path);
                }
            }
            is_root_dir = false;

            if !found_root_module && is_root_package {
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

            node.body = Some(self.build_cache.take_module(
                &node.package_name,
                &utf8path_buf_to_vec(&rel_path),
                &abs_path,
            )?);
        }

        Ok(())
    }

    pub fn save_cache(&mut self) {
        let cache =
            self.module_graph
                .nodes
                .drain()
                .fold(HashMap::new(), |mut acc, (id, mut node)| {
                    if let Some(body) = node.body.take() {
                        acc.insert(id, body);
                    }
                    acc
                });

        self.build_cache.save_cache(&cache);
    }

    pub fn load_cache(&mut self) {
        self.build_cache.load_cache();
    }

    pub fn get_module_graph(&self) -> &ModuleGraph {
        &self.module_graph
    }
}

fn create_rel_path(file_path: &Utf8PathBuf, package_path: &Utf8PathBuf) -> Utf8PathBuf {
    file_path.strip_prefix(package_path).unwrap().to_path_buf()
}

// the module id contains the package name and the relative path to module file from the package root.
pub type ModuleId = String;

pub fn module_id_from_local(package_name: &str, file_path: &Vec<String>) -> ModuleId {
    file_path
        .iter()
        .fold(package_name.to_string(), |a, b| a + "::" + b)
}

pub type ModuleDependencies = HashSet<ModuleId>;

#[cfg(test)]
mod tests {
    use crate::file_system::concrete::mock_fs::MockFileSystem;
    use crate::front::ast_types::Type;
    use crate::modules::ModuleBuilder;
    use camino::Utf8PathBuf;

    #[test]
    fn test_module_id_from_local() {
        let package_name = "package_a";
        let file_path = vec!["module_a".to_string(), "module_b".to_string()];
        let module_id = crate::modules::module_id_from_local(package_name, &file_path);
        assert_eq!(module_id, "package_a::module_a::module_b");
    }

    #[test]
    fn test_build_modules() {
        let mut mock_fs = MockFileSystem::new();
        mock_fs.insert_dir(Utf8PathBuf::from("pkg/package_a"));
        mock_fs.insert_file(Utf8PathBuf::from("pkg/package_a/main.ing"), "fn main() {}");
        mock_fs.insert_file(
            Utf8PathBuf::from("pkg/package_a/module_a.ing"),
            "static a: int;",
        );

        mock_fs.insert_dir(Utf8PathBuf::from("pkg/package_b"));
        mock_fs.insert_file(
            Utf8PathBuf::from("pkg/package_b/module_b.ing"),
            "static b: int;",
        );

        let mut module_builder = ModuleBuilder::new(&mut mock_fs, None);

        module_builder
            .add_fs_package("package_a", &Utf8PathBuf::from("pkg/package_a"), true)
            .unwrap();
        module_builder
            .add_fs_package("package_b", &Utf8PathBuf::from("pkg/package_b"), false)
            .unwrap();

        module_builder.load_module_bodies().unwrap();

        let module_graph = module_builder.get_module_graph();

        assert_eq!(module_graph.root, Some("package_a::main".to_string()));
        assert_eq!(module_graph.nodes.len(), 3);

        assert_eq!(
            module_graph.package_map.get("package_a").unwrap(),
            &Utf8PathBuf::from("pkg/package_a")
        );

        let main_definition_table = &module_graph
            .nodes
            .get("package_a::main")
            .as_ref()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .definitions;
        let module_a_definition_table = &module_graph
            .nodes
            .get("package_a::module_a")
            .as_ref()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .definitions;
        let module_b_definition_table = &module_graph
            .nodes
            .get("package_b::module_b")
            .as_ref()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .definitions;

        assert_eq!(
            main_definition_table
                .fn_map
                .get(&("package_a::main".to_string(), "0:0:main".to_string()))
                .unwrap()
                .return_type,
            Type::Void
        );
        assert_eq!(
            module_a_definition_table
                .static_var_map
                .get(&("package_a::module_a".to_string(), "0:0:a".to_string()))
                .unwrap()
                .ty,
            Type::Int
        );

        assert_eq!(
            module_b_definition_table
                .static_var_map
                .get(&("package_b::module_b".to_string(), "0:0:b".to_string()))
                .unwrap()
                .ty,
            Type::Int
        );
    }

    #[test]
    fn test_irregular_package_name() {
        let mut mock_fs = MockFileSystem::new();
        mock_fs.insert_dir(Utf8PathBuf::from("pkg/package_a"));
        mock_fs.insert_file(Utf8PathBuf::from("pkg/package_a/main.ing"), "fn main() {}");

        let mut module_builder = ModuleBuilder::new(&mut mock_fs, None);

        module_builder
            .add_fs_package("pack", &Utf8PathBuf::from("pkg/package_a"), true)
            .unwrap();

        module_builder.load_module_bodies().unwrap();

        let module_graph = module_builder.get_module_graph();
        assert_eq!(module_graph.root, Some("pack::main".to_string()));
    }

    #[test]
    fn test_caching() {
        let mut mock_fs = MockFileSystem::new();
        mock_fs.insert_dir(Utf8PathBuf::from("pkg/package_a"));
        mock_fs.insert_file(Utf8PathBuf::from("pkg/package_a/main.ing"), "fn main() {}");
        mock_fs.insert_file(
            Utf8PathBuf::from("pkg/package_a/module_a.ing"),
            "static a: int;",
        );

        let mut module_builder = ModuleBuilder::new(&mut mock_fs, Some(Utf8PathBuf::from("cache")));

        module_builder
            .add_fs_package("package_a", &Utf8PathBuf::from("pkg/package_a"), true)
            .unwrap();

        module_builder.load_module_bodies().unwrap();

        // save cache
        module_builder.save_cache();

        // load cache
        let mut module_builder = ModuleBuilder::new(&mut mock_fs, Some(Utf8PathBuf::from("cache")));
        module_builder.load_cache();

        module_builder
            .add_fs_package("package_a", &Utf8PathBuf::from("pkg/package_a"), true)
            .unwrap();

        module_builder.load_module_bodies().unwrap();

        let module_graph = module_builder.get_module_graph();
        assert_eq!(module_graph.root, Some("package_a::main".to_string()));
    }
}
