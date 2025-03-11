use super::base::{
    registry::get_command,
    Command, CommandDTO,
    CommandResult::{self, Group, Simple},
};

pub mod misc;

pub trait CommandGroup: Command {
    fn commands(&self) -> Vec<String>;
}

impl<T: CommandGroup + Default> Command for T {
    fn execute(
        &self,
        runtime: &crate::runtime::Runtime,
        _: &[String],
    ) -> windows::core::Result<CommandResult> {
        let mut results: Vec<CommandDTO> = vec![];

        for command_name in self.commands() {
            let command: Option<Box<dyn Command>> = get_command(command_name.as_str());

            let command_result = match command {
                Some(command) => command.execute(runtime, &[])?,
                None => {
                    panic!(
                        "Could not find command {command_name}... Who added this in the source code?!"
                    );
                }
            };
            match command_result {
                Simple(simple_result) => results = [results, vec![simple_result]].concat(),
                Group(group_result) => results = [results, group_result].concat(),
            }
        }
        Ok(Group(results))
    }
}
