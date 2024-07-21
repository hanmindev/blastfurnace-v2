use crate::modules::ModuleId;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
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

pub type PackageName = String;
pub type ItemPath = Vec<String>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FullItemPath {
    pub package_name: PackageName,
    pub item_path: ItemPath,
}

impl FullItemPath {
    pub fn new(package_name: PackageName, item_path: ItemPath) -> FullItemPath {
        FullItemPath {
            package_name,
            item_path,
        }
    }
}

pub type RawNameRoot = String;
pub type RawNameTailNode = String;
pub type RawName = (RawNameRoot, Option<Vec<RawNameTailNode>>);
pub type ItemName = String;
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct ResolvedName {
    pub module_id: ModuleId,
    pub item_name: ItemName,
}

impl ResolvedName {
    pub fn new(module_id: ModuleId, item_name: ItemName) -> ResolvedName {
        ResolvedName {
            module_id,
            item_name,
        }
    }
}
impl Serialize for ResolvedName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}+{}", self.module_id, self.item_name))
    }
}

impl<'de> Deserialize<'de> for ResolvedName {
    fn deserialize<D>(deserializer: D) -> Result<ResolvedName, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split('+').collect();
        if parts.len() != 2 {
            return Err(serde::de::Error::custom("Invalid ResolvedName format"));
        }
        Ok(ResolvedName {
            module_id: parts[0].parse().unwrap(),
            item_name: parts[1].to_string(),
        })
    }
}

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
    pub body: Module,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Definition {
    StaticVarDef(StaticVarDef),
    VarDef(VarDef),
    StructDef(StructDef),
    FnDef(FnDef),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Statement {
    VarAssign(String), // TODO
    FnCall(String),    // TODO
    Return,
    Module(Module),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Module {
    pub uses: Option<Vec<(RawName, FullItemPath)>>,
    pub definitions: Option<Vec<Definition>>,
    pub statements: Vec<Statement>,
}
