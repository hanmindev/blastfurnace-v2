use crate::front::ast_types::Type;
use crate::front::passes::name_resolution::scope_table::ScopeTable;
use crate::front::passes::name_resolution::{NameResolutionError, NameResolutionResult};
use crate::front::passes::visitor::{ASTNodeEnum, GenericVisitApplyResult, Visitable, Visitor};

pub type ResolveResult<T> = GenericVisitApplyResult<T, NameResolutionError>;
impl Visitor<(), NameResolutionError> for ScopeTable {
    fn apply(&mut self, ast_node: &mut ASTNodeEnum) -> ResolveResult<()> {
        Ok((
            match ast_node {
                ASTNodeEnum::VarReference(_)
                | ASTNodeEnum::TypeReference(_)
                | ASTNodeEnum::FunctionReference(_) => {
                    panic!("Reference should not be visited directly")
                }

                ASTNodeEnum::Type(ty) => {
                    if let Type::Struct(struct_name) = ty {
                        struct_name.resolved = Some(self.scope_lookup(&struct_name.raw, true)?);
                    }
                    false
                }

                ASTNodeEnum::VarDef(def) => {
                    def.name.resolved = Some(self.scope_bind(&def.name.raw, false, None)?);
                    def.ty.visit(self)?;
                    false
                }
                ASTNodeEnum::FnDef(def) => {
                    def.name.resolved = Some(self.scope_bind(&def.name.raw, false, None)?);
                    for mut var_def in def.args.iter_mut() {
                        var_def.visit(self)?;
                    }
                    def.return_type.visit(self)?;
                    false
                }
                ASTNodeEnum::StructDef(def) => {
                    def.name.resolved = Some(self.scope_bind(&def.name.raw, true, None)?);
                    for (_, field_type) in def.field_types.iter_mut() {
                        field_type.visit(self)?;
                    }
                    false
                }
                ASTNodeEnum::Definition(_) => true,
            },
            None,
        ))
    }
}
