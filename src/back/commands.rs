use crate::back::hmasm_types::{HmasmFile, HmasmFunction, HmasmInstruction, HmasmScope};

type MCFContent = String;
pub struct CommandContext {
    pub additional_functions: i32,
}

pub trait CommandConvertable {
    fn to_commands(&self, ctx: &mut CommandContext) -> (MCFContent, Vec<MCFContent>) {
        (String::from("say NOT IMPLEMENTED"), vec![])
    }
}

impl CommandConvertable for HmasmInstruction {
    fn to_commands(&self, ctx: &mut CommandContext) -> (MCFContent, Vec<MCFContent>) {
        match self {
            HmasmInstruction::MCommand(command) => (command.clone(), vec![]),
            HmasmInstruction::Scope(scope) => {
                // TODO: proper path
                let command = format!(
                    "function root:path/path/custom_created{}",
                    ctx.additional_functions
                );
                ctx.additional_functions += 1;

                let (scope_commands, extra) = scope.to_commands(ctx);
                let mut additional_functions = vec![scope_commands];
                additional_functions.extend(extra);

                (command, additional_functions)
            }
        }
    }
}

impl CommandConvertable for HmasmScope {
    fn to_commands(&self, ctx: &mut CommandContext) -> (MCFContent, Vec<MCFContent>) {
        let mut additional_functions = vec![];
        let mut scoped_command = String::new();

        for instruction in self.instructions.iter() {
            let (command, extra) = instruction.to_commands(ctx);
            scoped_command.push_str(&command);
            additional_functions.extend(extra);
        }

        (scoped_command, additional_functions)
    }
}

impl CommandConvertable for HmasmFunction {
    fn to_commands(&self, ctx: &mut CommandContext) -> (MCFContent, Vec<MCFContent>) {
        let (command, mut additional_functions) = self.scope.to_commands(ctx);
        (command, additional_functions)
    }
}
