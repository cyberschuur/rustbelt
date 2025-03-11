use clap::Command as ClapCommand;

use crate::commands::base::registry::CommandRegistration;

use super::CommandGroup;

pub struct MiscGroup {}

inventory::submit! {
    CommandRegistration {
        name: "group:misc",
        factory: || Box::new(MiscGroup::default()),
        clap_command: || ClapCommand
            ::new("group:misc")
            .version("1.0")
            .about("Executes all the commands in the 'misc' group.")
    }
}

impl CommandGroup for MiscGroup {
    fn commands(&self) -> Vec<String> {
        return vec!["antivirus".to_string(), "amsiproviders".to_string()];
    }
}

impl Default for MiscGroup {
    fn default() -> Self {
        MiscGroup {}
    }
}
