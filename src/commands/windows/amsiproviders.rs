use std::collections::HashMap;

use clap::Command as ClapCommand;
use windows::{core::*, Win32::System::Variant::VARIANT};

use crate::{
    commands::base::{
        registry::CommandRegistration,
        Command, CommandDTO, CommandData,
        CommandResult::{self, Simple},
    },
    runtime::Runtime,
    utils::registry::{get_string_value, get_sub_key_names, RegistryHive},
};

pub struct AmsiProvidersCommand {
    data: CommandData,
}

inventory::submit! {
    CommandRegistration {
        name: "amsiproviders",
        factory: || Box::new(AmsiProvidersCommand::default()),
        clap_command: || ClapCommand
            ::new("amsiproviders")
            .version("1.0")
            .about("Providers registered for AMSI")
    }
}

// Implement the Command trait for ExampleCommand.
impl Command for AmsiProvidersCommand {
    fn execute(&self, _: &Runtime, _: &[String]) -> Result<CommandResult> {
        // TODO: as long as the runtime does not implement remote access,
        // there is no sense in using it here...
        let provider_ids = get_sub_key_names(
            RegistryHive::LocalMachine,
            "SOFTWARE\\Microsoft\\AMSI\\Providers",
        )?;
        let providers: Vec<String> = provider_ids
            .iter()
            .filter_map(|provider| {
                match get_string_value(
                    RegistryHive::LocalMachine,
                    format!("SOFTWARE\\Classes\\CLSID\\{}\\InprocServer32", provider).as_str(),
                    "",
                ) {
                    Ok(value) => Some(value),
                    Err(_) => None,
                }
            })
            .collect();

        let providers_formatted: Vec<HashMap<String, VARIANT>> = providers
            .iter()
            .map(|provider| {
                let mut result = HashMap::new();
                result.insert(
                    "AMSI Provider".to_string(),
                    VARIANT::from(provider.as_str()),
                );
                return result;
            })
            .collect();

        return Ok(Simple(CommandDTO {
            source: "Amsi Providers".to_string(),
            data: providers_formatted,
        }));
    }
}

impl Default for AmsiProvidersCommand {
    fn default() -> Self {
        return AmsiProvidersCommand {
            data: CommandData {
                support_remote: false,
            },
        };
    }
}
