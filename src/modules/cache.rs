use crate::file_system::FileSystem;
use crate::modules::types::ModuleCachableData;
use crate::modules::ModuleId;
use camino::Utf8Path;
use std::collections::HashMap;
use std::io::Write;

pub struct BuildCacheLayer<'p, T: FileSystem> {
    pub file_system: &'p mut T,
    cache_location: Option<Box<Utf8Path>>,
    cache: Option<HashMap<ModuleId, ModuleCachableData>>,
}

impl<T: FileSystem> BuildCacheLayer<'_, T> {
    pub fn new(file_system: &mut T, cache_location: Option<Box<Utf8Path>>) -> BuildCacheLayer<T> {
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
                Err(_) => {
                    self.cache = Some(HashMap::new());
                }
            }
        }
    }

    pub fn save_cache(&mut self) {
        // save cache to disk

        if let Some(cache_location) = &self.cache_location {
            let cache = self.cache.as_ref().unwrap();
            // TODO: add proper error handling
            let j = serde_json::to_string(cache).unwrap();
            // TODO: add proper error handling
            let mut file = self
                .file_system
                .get_writer(&cache_location.to_path_buf())
                .unwrap();
            // TODO: add proper error handling
            file.write_all(j.as_bytes()).unwrap();
        }
    }

    pub fn get_module(&self, id: &ModuleId) -> Option<&ModuleCachableData> {
        if let Some(cache) = &self.cache {
            return cache.get(id);
        }
        return None;
    }
}
