use crate::modules::ModuleId;

pub enum IRInstruction {
    MCommand(String),
}

pub struct IRFunction {
    pub name: String,
    pub instructions: Vec<IRInstruction>,
}

pub struct IRModule {
    pub id: ModuleId,
    pub functions: Vec<IRFunction>,
}
