use crate::modules::ModuleId;
use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;

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

pub type RawName = String;
pub type ResolvedName = (ModuleId, String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct VarDummy;
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypeDummy;
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FunctionDummy;

pub type VarReference = Reference<RawName, ResolvedName, VarDummy>;
pub type TypeReference = Reference<RawName, ResolvedName, TypeDummy>;
pub type FunctionReference = Reference<RawName, ResolvedName, FunctionDummy>;

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
pub struct StaticVarDef {
    pub name: VarReference,
    pub ty: Type,
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
    StaticVarDef(StaticVarDef),
    VarDef(VarDef),
    StructDef(StructDef),
    FnDef(FnDef),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ASTFile {
    pub uses: Vec<(RawName, ResolvedName)>,
    pub definitions: Vec<Definition>,
}
