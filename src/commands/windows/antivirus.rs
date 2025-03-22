use clap::Command as ClapCommand;
use windows::{core::*, Win32::System::Wmi::IEnumWbemClassObject};

use crate::{
    commands::base::registry::CommandRegistration,
    commands::base::{
        Command, CommandDTO, CommandData,
        CommandResult::{self, Simple},
    },
    runtime::Runtime,
    utils::wmi::WbemIterator,
};

pub struct AntivirusCommand {
    data: CommandData,
}

inventory::submit! {
    CommandRegistration {
        name: "antivirus",
        factory: || Box::new(AntivirusCommand::default()),
        clap_command: || ClapCommand
            ::new("antivirus")
            .version("1.0")
            .about("Returns information about antivirus providers.")
    }
}

// Implement the Command trait for ExampleCommand.
impl Command for AntivirusCommand {
    fn execute(&self, runtime: &Runtime, _: &[String]) -> Result<CommandResult> {
        let results: IEnumWbemClassObject =
            runtime.wmi_query("root\\SecurityCenter2", "SELECT * FROM AntiVirusProduct")?;

        let results_iterator: WbemIterator = WbemIterator::from(
            &results,
            vec![
                "displayName".to_string(),
                "pathToSignedProductExe".to_string(),
                "pathToSignedReportingExe".to_string(),
            ],
        );

        let mut results_checked = Vec::new();

        for result in results_iterator {
            if let Some(result_checked) = result.ok() {
                results_checked.push(result_checked)
            }
        }
        
        Ok(Simple(CommandDTO {
            source: "Antivirus".to_string(),
            data: results_checked,
        }))
    }
}

impl Default for AntivirusCommand {
    fn default() -> Self {
        return AntivirusCommand {
            data: CommandData {
                support_remote: false,
            },
        };
    }
}
