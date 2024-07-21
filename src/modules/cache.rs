use crate::file_system::{FileSystem, FileSystemError};
use crate::front::ast_types::{FullItemPath, ItemPath, PackageName};
use crate::front::parse_file;
use crate::modules::types::ModuleCachableData;
use crate::modules::{module_id_from_local, ModuleBuildError, ModuleBuildResult, ModuleId};
use camino::Utf8PathBuf;
use std::collections::HashMap;

pub struct BuildCacheLayer<'p, T: FileSystem> {
    pub file_system: &'p mut T,
    cache_location: Option<Utf8PathBuf>,
    cache: Option<HashMap<ModuleId, ModuleCachableData>>,
}

impl<T: FileSystem> BuildCacheLayer<'_, T> {
    pub fn new(file_system: &mut T, cache_location: Option<Utf8PathBuf>) -> BuildCacheLayer<T> {
        BuildCacheLayer {
            file_system,
            cache_location,
            cache: None,
        }
    }

    pub fn load_cache(&mut self) {
        // load cache from disk

        if let Some(cache_location) = &self.cache_location {
            match self.file_system.get_reader(&cache_location.to_path_buf()) {
                Ok(reader) => {
                    // TODO: add proper error handling
                    let cache: HashMap<ModuleId, ModuleCachableData> =
                        serde_json::from_reader(reader).unwrap();
                    self.cache = Some(cache);
                }
                Err(FileSystemError::FileNotFound) => {
                    self.cache = Some(HashMap::new());
                }
                _ => {
                    panic!("Unexpected error while reading cache file");
                }
            }
        }
    }

    pub fn save_cache(&mut self, cache: &HashMap<ModuleId, ModuleCachableData>) {
        // save cache to disk

        if let Some(cache_location) = &self.cache_location {
            // TODO: add proper error handling
            let file = self
                .file_system
                .get_writer(&cache_location.to_path_buf())
                .unwrap();
            // TODO: add proper error handling
            serde_json::to_writer(file, cache).unwrap();
        }
    }

    pub fn take_module(
        &mut self,
        package_name: &PackageName,
        item_path: &ItemPath,
        abs_path: &Utf8PathBuf,
    ) -> ModuleBuildResult<ModuleCachableData> {
        let id = module_id_from_local(package_name, item_path);
        let age = self
            .file_system
            .get_file_age(&abs_path)
            .or(Err(ModuleBuildError::FileNoLongerExists))?;

        if let Some(cache) = &mut self.cache {
            if let Some(cached_data) = cache.remove(&id) {
                if cached_data.read_on == age {
                    return Ok(cached_data);
                }
            }
        }

        let mut reader = self
            .file_system
            .get_reader(&abs_path)
            .or(Err(ModuleBuildError::FileNoLongerExists))?;

        let mut file_content = String::new();
        reader
            .read_to_string(&mut file_content)
            .or(Err(ModuleBuildError::FileReadError))?;

        let (direct_deps, definitions) = parse_file(
            FullItemPath::new(package_name.clone(), item_path.clone()),
            &file_content,
        );

        return Ok(ModuleCachableData {
            read_on: age,
            direct_deps,
            definitions,
            object: None,
        });
    }
}
