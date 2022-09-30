pub use command_argument::{CommandArgument, CommandArgumentValue, CommandArgumentValueType};
pub use try_from_argument_value::{TryFromArgumentValue, TryFromArgumentValueError};
pub use wrapper::{CommaSeparated, SpaceSeparated};

mod command_argument;
mod try_from_argument_value;
mod wrapper;

/// The fragment of user command
#[derive(Debug, PartialEq)]
pub enum CommandFragment {
    /// Select subcommand
    Select(String),

    /// Execute command with arguments provided
    Execute(Vec<CommandArgument>),
}

impl std::error::Error for TryFromArgumentValueError {}
