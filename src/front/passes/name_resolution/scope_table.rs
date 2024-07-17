use std::collections::{HashMap, HashSet};
use crate::front::ast_types::{RawName, ResolvedName};
use crate::front::passes::name_resolution::{NameResolutionError, NameResolutionResult};
use crate::front::passes::name_resolution::NameResolutionError::UndefinedLookup;
use crate::modules::ModuleId;

struct ScopeTableLayer {
    // maps the raw name to the resolved name
    symbols: HashMap<RawName, ResolvedName>,
    // contains the raw names that were referenced but were not bound. If they get bound later, they will move to the symbols map.
    unresolved: HashSet<RawName>,
}

pub struct ScopeTable {
    module_id: ModuleId,
    stack: Vec<ScopeTableLayer>,

    global_count: HashMap<RawName, i32>,
}

impl ScopeTable {
    pub fn new(module_id: ModuleId) -> ScopeTable {
        ScopeTable {
            module_id,
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
    pub fn scope_bind(&mut self, raw_name: &RawName, first_in_scope: bool, force_name: Option<ResolvedName>) -> NameResolutionResult<ResolvedName> {
        let mut force_name = force_name;
        let layer = self.stack.last_mut().unwrap();

        if first_in_scope {
            if layer.symbols.contains_key(raw_name) {
                if !layer.unresolved.remove(raw_name) {
                    return Err(NameResolutionError::Redefinition(raw_name.clone()));
                }
            }

            if force_name.is_none() {
                force_name = Some((self.module_id.clone(), raw_name.clone()));
            }
        }

        let resolved_name = match force_name {
            Some(name) => name,
            None => {
                let new_count = if let Some(val) = self.global_count.get_mut(raw_name) {
                    *val += 1;
                    *val
                } else {
                    self.global_count.insert(raw_name.clone(), 1);
                    1
                };

                (self.module_id.clone(), format!("{}:{}", new_count, raw_name))
            }
        };

        layer.symbols.insert(raw_name.clone(), resolved_name.clone());
        Ok(resolved_name)
    }

    /*
    * Looks for a name in the current and previous scopes and returns the resolved name. This is used for references.

    If allow_future_binding is true, the name can be unresolved in the current scope but bound later. This can be useful for structs with recursive definitions.
     */
    pub fn scope_lookup(&mut self, raw_name: &RawName, allow_future_binding: bool) -> NameResolutionResult<ResolvedName> {
        for layer in self.stack.iter().rev() {
            if let Some(resolved_name) = layer.symbols.get(raw_name) {
                return Ok(resolved_name.clone());
            }
        }

        if allow_future_binding {
            let layer = self.stack.last_mut().unwrap();
            layer.unresolved.insert(raw_name.clone());
            Ok(self.scope_bind(raw_name, true, None)?)

        } else {
            Err(UndefinedLookup(raw_name.clone()))
        }
    }
}