use crate::modules::ModuleId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;
use camino::Utf8PathBuf;

// Reference<T, R> type idea from https://thume.ca/2019/04/18/writing-a-compiler-in-rust/
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Reference<T, R, D> {
    pub raw: T,
    pub resolved: Option<R>,
    phantom: PhantomData<D>, // this dummy type is used to make the type unique
}

impl<T, R, D> Reference<T, R, D> {
    pub fn new(raw: T) -> Reference<T, R, D> {
        Reference {
            raw,
            resolved: None,
            phantom: PhantomData,
        }
    }
}

impl<T: Debug, R: Debug, D> Debug for Reference<T, R, D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(r) = &self.resolved {
            write!(f, "{:#?} => {:#?}", self.raw, r)
        } else {
            write!(f, "{:?}", self.raw) // only print the raw if not resolved yet
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct VarDummy;
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypeDummy;
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FunctionDummy;

pub type VarReference = Reference<String, (ModuleId, String), VarDummy>;
pub type TypeReference = Reference<String, (ModuleId, String), TypeDummy>;
pub type FunctionReference = Reference<String, (ModuleId, String), FunctionDummy>;

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

pub struct UseMap {
    // map from alias to (package name, path, name in module)
    pub uses: HashMap<String, (String, Utf8PathBuf, String)>,
}

pub struct Module {
    pub id: ModuleId,

    pub use_map: UseMap,
    pub definitions: Vec<Definition>,
}