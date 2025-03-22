
use clap::Command as ClapCommand;
use windows::core::*;

use crate::{
    commands::base::registry::CommandRegistration,
    commands::base::{
        Command, CommandDTO, CommandData,
        CommandResult::{self, Simple},
    },
    runtime::Runtime,
};

pub struct LastShutdownCommand {
    data: CommandData,
}

inventory::submit! {
    CommandRegistration {
        name: "example",
        factory: || Box::new(LastShutdownCommand::default()),
        clap_command: || ClapCommand
            ::new("example")
            .version("1.0")
            .about("An example command to show how a command is implemented.")
    }
}

impl Command for LastShutdownCommand {
    fn execute(&self, runtime: &Runtime, _: &[String]) -> Result<CommandResult> {
        Ok(Simple(CommandDTO {
            source: "Example".to_string(),
            data: vec![],
        }))
    }
}

impl Default for LastShutdownCommand {
    fn default() -> Self {
        LastShutdownCommand {
            data: CommandData {
                support_remote: false,
            },
        }
    }
}
