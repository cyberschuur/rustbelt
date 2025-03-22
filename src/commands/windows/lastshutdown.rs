use std::collections::HashMap;

use clap::Command as ClapCommand;
use windows::{core::*, Win32::System::Variant::VARIANT};

use bitconv::{endian::Little, to_uint64};
use chrono::DateTime;

use crate::{
    commands::base::registry::CommandRegistration,
    commands::base::{
        Command, CommandDTO, CommandData,
        CommandResult::{self, Simple},
    },
    runtime::Runtime,
    utils::registry::{get_binary_value, RegistryHive},
};

pub struct LastShutdownCommand {
    data: CommandData,
}

inventory::submit! {
    CommandRegistration {
        name: "lastshutdown",
        factory: || Box::new(LastShutdownCommand::default()),
        clap_command: || ClapCommand
            ::new("lastshutdown")
            .version("1.0")
            .about("A command that check the last shutdown of the local computer")
    }
}

impl Command for LastShutdownCommand {
    fn execute(&self, runtime: &Runtime, _: &[String]) -> Result<CommandResult> {

        // Get the shutdown bytes form the registry. These bytes represent the Windows FileTime.
        let shutdown_bytes = get_binary_value(
            RegistryHive::LocalMachine,
            "SYSTEM\\ControlSet001\\Control\\Windows",
            "ShutdownTime"
        )?;

        // Difference between Windows FileTime Epoch (January 1, 1601 UTC) and Unix Time Epoch (January 1, 1970 UTC)
        const EPOCH_DIFFERENCE: u64 = 116444736000000000;

        // Convert the bytes to u64 and then to Unix Timestamp (in microseconds)
        let unix_time: u64 = (to_uint64::<Little>(
            &shutdown_bytes, 
            0
        ) - EPOCH_DIFFERENCE) / 10;
        
        let mut result: Vec<HashMap<String, VARIANT>> = Vec::new();

        match DateTime::from_timestamp_micros(unix_time as i64) {
            Some(time) => {
                let mut hashmap: HashMap<String, VARIANT> = HashMap::new();
                hashmap.insert(
                    "Last Shutdown".to_string(),
                    VARIANT::from(time.to_string().as_str())
                );
                result.insert(0, hashmap);
            },
            None => panic!("Error in Datetime")
        };

        Ok(Simple(CommandDTO {
            source: "Last Shutdown".to_string(),
            data: result,
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
