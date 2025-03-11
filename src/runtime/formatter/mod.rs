pub mod simple_formatter;

use crate::commands::base::CommandResult;

pub trait Formatter {
    fn parse_result(result: CommandResult) -> String;
}
