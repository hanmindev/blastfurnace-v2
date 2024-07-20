use crate::front::passes::visitor::{ASTNodeEnum, GenericVisitApplyResult, Visitor};
use crate::modules::{ModuleDependencies, ModuleId};

#[derive(Debug)]
pub enum DependencyError {
    Unknown,
}

pub type ResolveResult<T> = GenericVisitApplyResult<T, DependencyError>;

pub struct DependencyVisitor<'a> {
    module_id: ModuleId,
    dependencies: &'a mut ModuleDependencies,
}

impl DependencyVisitor<'_> {
    pub fn new(module_id: ModuleId, dependencies: &mut ModuleDependencies) -> DependencyVisitor {
        DependencyVisitor {
            module_id,
            dependencies,
        }
    }
}

impl Visitor<(), DependencyError> for DependencyVisitor<'_> {
    fn apply(&mut self, ast_node: &mut ASTNodeEnum) -> ResolveResult<()> {
        let ref_module_id = match ast_node {
            ASTNodeEnum::VarReference(name) => Some(&name.resolved.as_ref().unwrap().0),
            ASTNodeEnum::TypeReference(name) => Some(&name.resolved.as_ref().unwrap().0),
            ASTNodeEnum::FunctionReference(name) => Some(&name.resolved.as_ref().unwrap().0),
            _ => None,
        };

        if let Some(module_id) = ref_module_id {
            if &self.module_id != module_id {
                self.dependencies.insert(module_id.clone());
            }
        }

        Ok((true, None))
    }
}
