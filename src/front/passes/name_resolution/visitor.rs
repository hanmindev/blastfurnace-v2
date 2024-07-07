use crate::front::ast_types::{Type};
use crate::front::passes::name_resolution::{NameResolutionError, NameResolver};
use crate::front::passes::name_resolution::scope_table::SymbolType;
use crate::front::passes::visitor::{ASTNodeEnum, GenericVisitApplyResult, Visitable, Visitor};

// TODO: less generic name
type NameResolutionVisitApplyResult<T> = GenericVisitApplyResult<T, NameResolutionError>;

impl Visitor<(), NameResolutionError> for NameResolver {
    fn apply(&mut self, ast_node: &mut ASTNodeEnum) -> NameResolutionVisitApplyResult<()> {
        match ast_node {
            ASTNodeEnum::VarReference(_) | ASTNodeEnum::TypeReference(_) | ASTNodeEnum::FunctionReference(_) => {
                panic!("References should not be visited directly")
            }

            ASTNodeEnum::VarDef(var_def) => {
                if let Type::Struct(struct_name) = &mut var_def.ty {
                    struct_name.0.resolved = Some(self.scope_table.scope_lookup_or_create(&self.module_id, &struct_name.0.raw, SymbolType::Struct));
                }

                var_def.name.0.resolved =
                    Some(self.scope_table.scope_bind(&self.module_id, &var_def.name.0.raw, SymbolType::Var)?);
            }
            ASTNodeEnum::FnDef(fn_def) => {
                self.scope_table.scope_enter();

                fn_def.name.0.resolved =
                    Some(self.scope_table.scope_bind(&self.module_id, &fn_def.name.0.raw, SymbolType::Fn)?);
                for arg in &mut fn_def.args {
                    arg.visit(self)?;
                }

                // fn_def.body.visit(self)?;
                self.scope_table.scope_exit();
            }
            ASTNodeEnum::StructDef(struct_def) => {
                struct_def.name.0.resolved = Some(self.scope_table.scope_bind(&self.module_id, &struct_def.name.0.raw, SymbolType::Struct)?);

                for v in &mut struct_def.field_types.values_mut() {
                    if let Type::Struct(struct_name) = v {
                        struct_name.0.resolved =
                            Some(self.scope_table.scope_lookup_or_create(&self.module_id, &struct_name.0.raw, SymbolType::Struct));
                    }
                }
            }

            ASTNodeEnum::Definition(_) => return Ok((true, None)),
            _ => return Ok((true, None)),
        };
        return Ok((false, None));
    }
}
