use crate::front::ast_types::{Definition, FnDef, FunctionReference, StructDef, Type, TypeReference, VarDef, VarReference};
/*
The current file sets up the infrastructure for the visitor pattern.

The visitor will recursively visit every node in the AST. A developer may implement
 */

pub enum ASTNodeEnum<'a> {
    Type(&'a mut Type),

    VarReference(&'a mut VarReference),
    TypeReference(&'a mut TypeReference),
    FunctionReference(&'a mut FunctionReference),

    VarDef(&'a mut VarDef),
    FnDef(&'a mut FnDef),
    StructDef(&'a mut StructDef),

    Definition(&'a mut Definition),
}

pub type GenericVisitApplyResult<K, V> = Result<(bool, Option<K>), V>;

pub trait Visitor<K, V> {
    /* The default implementation of apply will return true and None.
    This means that the function will not return anything to the parent, and will visit the children.

    The apply method can be overridden to have different behaviors when visiting a node.
     */
    fn apply(&mut self, _ast_node: &mut ASTNodeEnum) -> GenericVisitApplyResult<K, V> {
        return Ok((true, None));
    }
}

pub trait Visitable<T: Visitor<K, V>, K, V> {
    // needs to be implemented for every node in the AST. Allows the visitor to automatically visit every node in the AST.
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V>;
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for VarReference {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        Ok(visitor.apply(&mut ASTNodeEnum::VarReference(self))?.1)
    }
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for TypeReference {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        Ok(visitor.apply(&mut ASTNodeEnum::TypeReference(self))?.1)
    }
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for FunctionReference {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        Ok(visitor.apply(&mut ASTNodeEnum::FunctionReference(self))?.1)
    }
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for Type {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        let (visit_result, res) = visitor.apply(&mut ASTNodeEnum::Type(self))?;
        if visit_result {
            if let Type::Struct(struct_name) = self {
                struct_name.visit(visitor)?;
            }
        }
        Ok(res)
    }
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for VarDef {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        let (visit_result, res) = visitor.apply(&mut ASTNodeEnum::VarDef(self))?;
        if visit_result {
            self.ty.visit(visitor)?;
            self.name.visit(visitor)?;
        }
        Ok(res)
    }
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for FnDef {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        let (visit_result, res) = visitor.apply(&mut ASTNodeEnum::FnDef(self))?;
        if visit_result {
            self.return_type.visit(visitor)?;
            self.name.visit(visitor)?;
            // self.body.visit(visitor)?;
        }
        Ok(res)
    }
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for StructDef {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        let (visit_result, res) = visitor.apply(&mut ASTNodeEnum::StructDef(self))?;
        if visit_result {
            self.name.visit(visitor)?;
        }
        Ok(res)
    }
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for Definition {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        let (visit_result, res) = visitor.apply(&mut ASTNodeEnum::Definition(self))?;
        if visit_result {
            match self {
                Definition::VarDef(x) => x.visit(visitor)?,
                Definition::StructDef(x) => x.visit(visitor)?,
                Definition::FnDef(x) => x.visit(visitor)?,
            };
        }
        Ok(res)
    }
}