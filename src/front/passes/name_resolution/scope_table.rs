use crate::front::ast_types::{FullItemPath, RawName, RawNameRoot, RawNameTailNode, ResolvedName};
use crate::front::passes::name_resolution::NameResolutionError::UndefinedLookup;
use crate::front::passes::name_resolution::{NameResolutionError, NameResolutionResult};
use crate::modules::module_id_from_local;
use std::collections::{HashMap, HashSet};

fn stitch_path(
    mut full_item_path: FullItemPath,
    mut tail: &Option<Vec<RawNameTailNode>>,
) -> ResolvedName {
    if let Some(tail_unwrap) = tail {
        if tail_unwrap.len() != 0 {
            for i in 0..tail_unwrap.len() - 1 {
                full_item_path.item_path.push(tail_unwrap[i].clone());
            }

            return (
                module_id_from_local(&full_item_path.package_name, &full_item_path.item_path),
                tail_unwrap.last().unwrap().clone(),
            );
        }
    }

    let item = full_item_path.item_path.pop().unwrap(); // TODO: error handling
    return (
        module_id_from_local(&full_item_path.package_name, &full_item_path.item_path),
        item,
    );
}

struct ScopeTableLayer {
    // maps the raw name to the full item path
    symbols: HashMap<RawNameRoot, FullItemPath>,
    // contains the raw names that were referenced but were not bound. If they get bound later, they will move to the symbols map.
    unresolved: HashSet<RawNameRoot>,
}

pub struct ScopeTable {
    module_path: FullItemPath,
    stack: Vec<ScopeTableLayer>,

    global_count: HashMap<RawNameRoot, i32>,
}

impl ScopeTable {
    pub fn new(module_path: FullItemPath) -> ScopeTable {
        ScopeTable {
            module_path,
            stack: vec![],
            global_count: HashMap::new(),
        }
    }

    pub fn scope_enter(&mut self) {
        self.stack.push(ScopeTableLayer {
            symbols: HashMap::new(),
            unresolved: HashSet::new(),
        });
    }

    pub fn scope_exit(&mut self) -> NameResolutionResult<()> {
        let layer = self.stack.pop().unwrap();

        if !layer.unresolved.is_empty() {
            return Err(NameResolutionError::UnresolvedNames(layer.unresolved));
        }
        Ok(())
    }

    /*
    * Binds the name to the scope and returns a new resolved name. This is used for declarations.

    If first_in_scope is true, there must not be any existing bindings for the name in the current scope. This is useful if you want to ensure that a name is not redefined in the same scope.

    If force_name is Some, the name will be bound to that name. This is used for publicly exposed names.
     */
    pub fn scope_bind(
        &mut self,
        raw_name: &RawNameRoot,
        first_in_scope: bool,
        force_name: Option<FullItemPath>,
    ) -> NameResolutionResult<ResolvedName> {
        let mut force_name = force_name;
        let size = self.stack.len();
        let layer = self.stack.last_mut().unwrap();

        if first_in_scope {
            if let Some(resolved_name) = layer.symbols.get(raw_name) {
                if layer.unresolved.remove(raw_name) {
                    if force_name.is_none() {
                        force_name = Some(resolved_name.clone());
                    }
                } else {
                    return Err(NameResolutionError::Redefinition(raw_name.clone()));
                }
            }
        }

        let full_path = match force_name {
            Some(name) => name,
            None => {
                let new_count = if let Some(val) = self.global_count.get_mut(raw_name) {
                    *val += 1;
                    *val
                } else {
                    self.global_count.insert(raw_name.clone(), 1);
                    0
                };

                let mut path = self.module_path.clone();
                path.item_path
                    .push(format!("{}:{}:{}", size - 1, new_count, raw_name));

                path
            }
        };

        layer.symbols.insert(raw_name.clone(), full_path.clone());
        Ok(stitch_path(full_path, &None))
    }

    /*
    * Looks for a name in the current and previous scopes and returns the resolved name. This is used for references.

    If allow_future_binding is true, the name can be unresolved in the current scope but bound later. This can be useful for structs with recursive definitions.
     */
    pub fn scope_lookup(
        &mut self,
        raw_name: &RawName,
        allow_future_binding: bool,
    ) -> NameResolutionResult<ResolvedName> {
        let raw_name_root: RawNameRoot = raw_name.0.clone();

        for layer in self.stack.iter().rev() {
            if let Some(full_item_path) = layer.symbols.get(&raw_name_root) {
                return Ok(stitch_path(full_item_path.clone(), &raw_name.1));
            }
        }

        if allow_future_binding {
            let layer = self.stack.last_mut().unwrap();
            layer.unresolved.insert(raw_name_root.clone());
            Ok(self.scope_bind(&raw_name_root, true, None)?)
        } else {
            Err(UndefinedLookup(raw_name_root.clone()))
        }
    }
}
