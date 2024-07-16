use std::collections::HashMap;
use crate::front::ast_types::{RawName, ResolvedName};
use crate::front::passes::name_resolution::{NameResolutionError, NameResolutionResult};
use crate::modules::ModuleId;

struct ScopeTableLayer {
    // maps the raw name to the resolved name
    symbols: HashMap<RawName, ResolvedName>,
    // contains the raw names that were referenced but were not bound. If they get bound later, they will move to the symbols map.
    unresolved: HashMap<RawName, ResolvedName>,
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
            stack: vec![ScopeTableLayer {
                symbols: HashMap::new(),
                unresolved: HashMap::new(),
            }],
            global_count: HashMap::new(),
        }
    }

    pub fn scope_enter(&mut self) {
        self.stack.push(ScopeTableLayer {
            symbols: HashMap::new(),
            unresolved: HashMap::new(),
        });
    }

    pub fn scope_exit(&mut self) {
        if self.stack.len() <= 1 {
            panic!("Cannot exit the global scope");
        }

        let layer = self.stack.pop().unwrap(); // can unwrap because we checked the length

        // move all unresolved symbols to the parent scope
        self.stack.last_mut().unwrap().unresolved.extend(layer.unresolved);
    }

    // forces binding. each raw name can only be bound once. Must be bound before any scope_bind calls.
    pub fn scope_bind_pre_made_name(&mut self, raw: RawName, resolved: ResolvedName) -> NameResolutionResult<()> {
        let node = &mut self.stack.last_mut().unwrap();

        if node.symbols.contains_key(&raw) {
            return Err(NameResolutionError::Redefinition(raw, resolved));
        }

        node.symbols.insert(raw, resolved);

        Ok(())
    }

    pub fn scope_bind(&mut self, raw: RawName) -> NameResolutionResult<ResolvedName> {
        let node = &mut self.stack.last_mut().unwrap();

        if let Some(resolved) = node.symbols.get(&raw) {
            return Err(NameResolutionError::Redefinition(raw, resolved.clone()));
        }

        if let Some(resolved) = node.unresolved.get(&raw) {
            return Err(NameResolutionError::Redefinition(raw, resolved.clone()));
        }

        let resolved = match self.global_count.get_mut(&raw) {
            Some(count) => {
                *count += 1;
                (self.module_id.clone(), format!("{name}:{count}", count = count, name = raw))
            }
            None => {
                (self.module_id.clone(), raw.clone())
            }
        };

        node.unresolved.insert(raw, resolved.clone());

        Ok(resolved)

    }
}