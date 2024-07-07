use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;
use serde::{Deserialize, Serialize};
use crate::modules::ModuleId;

// Reference<T, R> type idea from https://thume.ca/2019/04/18/writing-a-compiler-in-rust/
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Reference<T, R> {
    pub raw: T,
    pub resolved: Option<R>,
}

impl<T: Debug, R: Debug> Debug for Reference<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(r) = &self.resolved {
            write!(f, "{:#?} => {:#?}", self.raw, r)
        } else {
            write!(f, "{:?}", self.raw) // only print the raw if not resolved yet
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct VarReference(Reference<String, (ModuleId, String)>);
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypeReference(Reference<String, (ModuleId, String)>);
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FunctionReference(Reference<String, (ModuleId, String)>);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Type {
    Void,
    Int,
    Float,
    Bool,
    String,
    Struct(TypeReference),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct VarDef {
    pub name: VarReference,
    pub ty: Type,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StructDef {
    pub name: TypeReference,
    pub field_types: HashMap<String, Type>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FnDef {
    pub return_type: Type,
    pub name: FunctionReference,
    pub args: Vec<VarDef>,
    // TODO: add body
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Definition {
    VarDef(VarDef),
    StructDef(StructDef),
    FnDef(FnDef),
}