//! # kal
//!
//! Command Abstraction Layer for bot libraries
#![deny(missing_docs)]

pub use command::Command;
pub use command_fragment::{
    CommandArgument, CommandArgumentValue, CommandArgumentValueTypeMismatchError, CommandFragment,
};
pub use command_spec::{CommandOption, CommandOptionValueKind, CommandOptionValueTy, CommandSpec};
pub use kal_derive::Command;

mod command;
mod command_fragment;
mod command_group;
mod command_spec;
