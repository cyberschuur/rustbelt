/// This module defines the base structures and traits for commands.
pub mod registry;

use crate::runtime::Runtime;
use std::collections::HashMap;
use windows::{core::*, Win32::System::Variant::VARIANT};

/// Data Transfer Object for commands.
///
/// # Fields
/// - `source`: The source of the command.
/// - `data`: A vector of hash maps containing command data.
#[derive(Clone)]
pub struct CommandDTO {
    pub source: String,
    pub data: Vec<HashMap<String, VARIANT>>,
}

/// Enum representing the result of a command execution.
///
/// # Variants
/// - `Simple`: A single command result.
/// - `Group`: A group of command results.
pub enum CommandResult {
    Simple(CommandDTO),
    Group(Vec<CommandDTO>),
}

/// Struct containing data for commands.
///
/// # Fields
/// - `support_remote`: A boolean indicating if the command supports remote execution.
pub struct CommandData {
    pub support_remote: bool,
}

/// Trait defining the behavior of a command.
pub trait Command {
    /// Executes the command.
    ///
    /// # Arguments
    /// - `runtime`: A reference to the runtime environment.
    /// - `args`: A slice of strings representing the arguments for the command.
    ///
    /// # Returns
    /// A result containing a `CommandResult` or an error.
    fn execute(&self, runtime: &Runtime, args: &[String]) -> Result<CommandResult>;
}
