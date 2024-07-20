use crate::front::ast_types::Type;
use crate::front::passes::name_resolution::scope_table::ScopeTable;
use crate::front::passes::name_resolution::NameResolutionError;
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
                ASTNodeEnum::StaticVarDef(def) => {
                    def.name.resolved = Some(self.scope_bind(&def.name.raw.0, true, None)?);
                    def.ty.visit(self)?;
                    false
                }
                ASTNodeEnum::VarDef(def) => {
                    def.name.resolved = Some(self.scope_bind(&def.name.raw.0, false, None)?);
                    def.ty.visit(self)?;
                    false
                }
                ASTNodeEnum::FnDef(def) => {
                    def.name.resolved = Some(self.scope_bind(&def.name.raw.0, true, None)?);
                    for var_def in def.args.iter_mut() {
                        var_def.visit(self)?;
                    }
                    def.return_type.visit(self)?;
                    def.body.visit(self)?;
                    false
                }
                ASTNodeEnum::StructDef(def) => {
                    def.name.resolved = Some(self.scope_bind(&def.name.raw.0, true, None)?);
                    for (_, field_type) in def.field_types.iter_mut() {
                        field_type.visit(self)?;
                    }
                    false
                }
                ASTNodeEnum::Definition(_) => true,
                ASTNodeEnum::Statement(_) => true,
                ASTNodeEnum::Module(module) => {
                    self.scope_enter();
                    // load the "use" statements into the scope table. There should not be any duplicates
                    for (raw_name, resolved_name) in module.uses.take().unwrap() {
                        self.scope_bind(&raw_name.0, true, Some(resolved_name))?;
                    }
                    // then we visit each definition in the Module
                    for definition in module.definitions.iter_mut().flatten() {
                        definition.visit(self)?;
                    }
                    // then we visit each definition in the Module
                    for statement in module.statements.iter_mut() {
                        statement.visit(self)?;
                    }
                    self.scope_exit()?;
                    false
                }
            },
            None,
        ))
    }
}
