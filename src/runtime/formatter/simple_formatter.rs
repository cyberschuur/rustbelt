use crate::commands::base::{
    CommandDTO,
    CommandResult::{self, Group, Simple},
};
use windows::Win32::System::Variant::{VariantToString, VARIANT};

use super::Formatter;

pub struct SimpleFormatter {}

impl Formatter for SimpleFormatter {
    fn parse_result(result: CommandResult) -> String {
        match result {
            Group(group_result) => format_group_result(group_result),
            Simple(simple_result) => format_command_dto(simple_result),
        }
    }
}

fn format_group_result(tables: Vec<CommandDTO>) -> String {
    let mut output = "".to_string();

    for table in tables {
        output = format!("{output}\n\n{}", format_command_dto(table));
    }
    output
}

fn format_command_dto(table: CommandDTO) -> String {
    let mut output = format!("==[{}]==", table.source);
    let mut count = 0;

    for row in table.data {
        output = format!("{output}\n [{:?}]", count);

        for (col, value) in row.iter() {
            let mut pszbuf = vec![0u16; 256];

            unsafe {
                let _ = VariantToString(value as *const VARIANT, pszbuf.as_mut_slice());
            }
            let null_pos = pszbuf.iter().position(|&c| c == 0).unwrap_or(pszbuf.len());
            let content = String::from_utf16_lossy(&pszbuf[..null_pos]);

            output = format!("{output}\n\t{col} : {content}")
        }
        count += 1;
    }
    output
}
