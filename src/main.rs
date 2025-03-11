use clap::{arg, Command as ClapCommand};
use windows::core::*;
mod commands;
mod runtime;
mod utils;

use commands::base::{
    registry::{get_command, CommandRegistration},
    Command,
};
use runtime::{
    formatter::{simple_formatter::SimpleFormatter, Formatter},
    writer::{console_writer::ConsoleWriter, Writer},
    Runtime,
};

/// The main entry point of the Rustbelt CLI application.
///
/// This function sets up the CLI application using the `clap` crate, registers
/// available commands, parses the command-line arguments, and executes the
/// corresponding command.
///
/// # Returns
///
/// * `Result<()>` - Returns `Ok(())` if the application runs successfully, or an error if it fails.
fn main() -> Result<()> {
    // Create the base CLI app.
    let mut app = ClapCommand::new("rustbelt")
        .about("An oxidized seatbelt: safety off..")
        .long_about("Rustbelt is a Rust implementation of the well-known Seatbelt tool. \
            Rusty enumeration.. what's not to love!")
        .version("1.0")
        .args([
            arg!(-u --username <USERNAME> "Optional username of the user. Uses the current user by default.")
                .required(false)
                .help("Specify the username for the operation."),
            arg!(-p --password <PASSWORD> "Optional password of the user")
                .required(false)
                .help("Specify the password for the operation."),
            arg!(-c --computername <COMPUTER_NAME> "Optional computer name")
                .required(false)
                .help("Specify the computer in case of remote operations."),
        ]);

    let mut commands: Vec<Box<dyn Command>> = vec![];

    // Add each registered command as a subcommand.
    for reg in inventory::iter::<CommandRegistration> {
        app = app.subcommand((reg.clap_command)());
        commands.push((reg.factory)());
    }

    // Parse the matched arguments and execute the corresponding command.
    let matches = app.get_matches();
    let username = matches.get_one::<String>("username");

    // Initialize the runtime with the provided username.
    let runtime: Runtime = Runtime::new(username.cloned(), None, None)?;

    // Check if a subcommand was provided and execute the corresponding command.
    if let Some((subcommand_name, _sub_matches)) = matches.subcommand() {
        if let Some(command) = get_command(subcommand_name) {
            // Collect the arguments and execute the command.
            let args = std::env::args().collect::<Vec<_>>();
            let result = command.execute(&runtime, &args)?;

            // Write the result to the console.
            ConsoleWriter::write_line(SimpleFormatter::parse_result(result));
        } else {
            eprintln!("Command '{}' not found.", subcommand_name);
        }
    }
    Ok(())
}
