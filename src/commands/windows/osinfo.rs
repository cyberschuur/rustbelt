use std::{collections::HashMap, env};

use clap::Command as ClapCommand;
use windows::{
    core::*, 
    Win32::System::{
        Time::{
            GetTimeZoneInformation,
            TIME_ZONE_INFORMATION
        },
        SystemInformation::GetTickCount64,
        Variant::VARIANT,
    }
};
use chrono::prelude::*;

use crate::{
    commands::base::registry::CommandRegistration,
    commands::base::{
        Command, CommandDTO, CommandData,
        CommandResult::{self, Simple},
    },
    runtime::Runtime,
    utils::registry::{get_value, RegistryHive},
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

fn is_vm() -> bool {
    todo!()
}

impl Command for OSInfoCommand {
    fn execute(&self, runtime: &Runtime, _: &[String]) -> Result<CommandResult> {
        let names = vec![
            "ProductName",
            "EditionID",
            "ReleaseId",
            "BuildBranch",
            "CurrentMajorVersionNumber",
            "CurrentVersion",
            "CurrentBuildNumber",
            "UBR",
        ];

        let mut values: HashMap<String, VARIANT> = names
            .iter()
            .filter_map(| name | {
                match get_value(
                        RegistryHive::LocalMachine, 
                        "Software\\Microsoft\\Windows NT\\CurrentVersion", 
                        name
                    ) { 
                        Ok(value) => Some((name.to_string(), VARIANT::from(value.as_str()))),
                        Err(_) => None
                    }
            })
            .collect();

        if runtime.is_remote() {
            todo!()
        } else {
            let env_names = vec![
                "PROCESSOR_ARCHITECTURE",
                "NUMBER_OF_PROCESSORS",
                "COMPUTERNAME",
            ];

            let env_values: HashMap<String, VARIANT> = env_names
                .iter()
                .filter_map(|env_variable | {
                    match env::var_os(env_variable) {
                        Some(value) => Some((env_variable.to_string(), VARIANT::from(value.to_str().unwrap()))),
                        None => None
                    }
                })
                .collect();

            values.extend(env_values);
            
            let boot_time_utc = DateTime::from_timestamp_millis(
                Utc::now().timestamp_millis() - (unsafe {GetTickCount64()} as i64)
            ).unwrap();

            values.insert(
                "BootTime".to_string(), 
                VARIANT::from(boot_time_utc.to_string().as_str())
            );

            unsafe {
                let mut tz_info = TIME_ZONE_INFORMATION::default();
                GetTimeZoneInformation(&mut tz_info);
                let tz_name = String::from_utf16_lossy(&tz_info.StandardName);

                values.insert(
                    "TimeZone".to_string(),
                    VARIANT::from(tz_name.as_str())
                );    
            }

            values.insert(
                "MachineGuid".to_string(), 
                VARIANT::from(
                    get_value(
                        RegistryHive::LocalMachine, 
                        "SOFTWARE\\Microsoft\\Cryptography", 
                        "MachineGuid"
                    )?.as_str()
                )
            );
        }

        Ok(Simple(CommandDTO {
            source: "OSInfo".to_string(),
            data: vec![values],
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
