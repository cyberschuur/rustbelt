//! This module defines the `ExampleCommand` which is an example implementation of a command
//! using the `Command` trait. It demonstrates how to register a command and implement its
//! execution logic.

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

/// `ExampleCommand` struct holds the data required for the command.
pub struct ExampleCommand {
    data: CommandData,
}

// Register the `ExampleCommand` with the command registry.
inventory::submit! {
    CommandRegistration {
        name: "example",
        factory: || Box::new(ExampleCommand::default()),
        clap_command: || ClapCommand
            ::new("example")
            .version("1.0")
            .about("An example command to show how a command is implemented.")
    }
}

// Implement the `Command` trait for `ExampleCommand`.
impl Command for ExampleCommand {
    /// Executes the `ExampleCommand`.
    ///
    /// # Arguments
    ///
    /// * `runtime` - A reference to the `Runtime` instance.
    /// * `_` - A slice of strings representing the command arguments.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `CommandResult`.
    fn execute(&self, runtime: &Runtime, _: &[String]) -> Result<CommandResult> {
        Ok(Simple(CommandDTO {
            source: "Example".to_string(),
            data: vec![],
        }))
    }
}

// Provide a default implementation for `ExampleCommand`.
impl Default for ExampleCommand {
    /// Creates a default instance of `ExampleCommand`.
    ///
    /// # Returns
    ///
    /// A new `ExampleCommand` instance with default `CommandData`.
    fn default() -> Self {
        ExampleCommand {
            data: CommandData {
                support_remote: false,
            },
        }
    }
}
