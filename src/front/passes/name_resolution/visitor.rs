use crate::front::ast_types::Type;
use crate::front::passes::name_resolution::{NameResolutionError, NameResolutionResult};
use crate::front::passes::name_resolution::scope_table::ScopeTable;
use crate::front::passes::visitor::{ASTNodeEnum, GenericVisitApplyResult, Visitor};

pub type ResolveResult<T> = GenericVisitApplyResult<T, NameResolutionError>;
impl Visitor<(), NameResolutionError> for ScopeTable {
    fn apply(&mut self, ast_node: &mut ASTNodeEnum) -> ResolveResult<()> {
        match ast_node {
            ASTNodeEnum::VarReference(_) | ASTNodeEnum::TypeReference(_) | ASTNodeEnum::FunctionReference(_) => {
                panic!("Reference should not be visited directly")
            }

            ASTNodeEnum::Type(_) => {
                panic!("Type should not be visited directly")
            }

            ASTNodeEnum::VarDef(_) => {
            }
            ASTNodeEnum::FnDef(_) => {}
            ASTNodeEnum::StructDef(_) => {}


            ASTNodeEnum::Definition(_) => {}
        };

        return Ok((true, None));
    }
}