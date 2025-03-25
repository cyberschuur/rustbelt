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

pub struct OSInfoCommand {
    data: CommandData,
}

inventory::submit! {
    CommandRegistration {
        name: "osinfo",
        factory: || Box::new(OSInfoCommand::default()),
        clap_command: || ClapCommand
            ::new("osinfo")
            .version("1.0")
            .about("A command to retrieve basic info about a computer. (i.e. architecture, OS Version etc.)")
    }
}

impl Command for OSInfoCommand {
    fn execute(&self, runtime: &Runtime, _: &[String]) -> Result<CommandResult> {
        Ok(Simple(CommandDTO {
            source: "Example".to_string(),
            data: vec![],
        }))
    }
}

impl Default for OSInfoCommand {
    fn default() -> Self {
        OSInfoCommand {
            data: CommandData {
                support_remote: true,
            },
        }
    }
}
