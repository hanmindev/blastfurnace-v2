mod visitor;

use crate::front::ast_types::Module;
use crate::front::passes::collect_dependencies::visitor::DependencyVisitor;
use crate::front::passes::visitor::{Visitable, Visitor};
use crate::modules::{ModuleDependencies, ModuleId};

pub fn collect_dependencies(module_id: ModuleId, module: &mut Module) -> ModuleDependencies {
    let mut dependencies = ModuleDependencies::new();
    module
        .visit(&mut DependencyVisitor::new(module_id, &mut dependencies))
        .unwrap();
    dependencies
}
