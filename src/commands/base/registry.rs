use crate::commands::base::Command;
use clap::Command as ClapCommand;

/// Struct representing a command registration.
///
/// # Fields
///
/// * `name` - The name of the command.
/// * `factory` - A function that returns a boxed instance of the command.
/// * `clap_command` - A function that returns the Clap command.
pub struct CommandRegistration {
    pub name: &'static str,
    pub factory: fn() -> Box<dyn Command>,
    pub clap_command: fn() -> ClapCommand,
}

// Collect all command registrations.
inventory::collect!(CommandRegistration);

/// Retrieves a command by its name.
///
/// # Arguments
///
/// * `name` - The name of the command to retrieve.
///
/// # Returns
///
/// An `Option` containing the boxed command if found, or `None` if not found.
pub fn get_command(name: &str) -> Option<Box<dyn Command>> {
    inventory::iter::<CommandRegistration>
        .into_iter()
        .find(|registration| registration.name == name)
        .map(|registration| (registration.factory)())
}
