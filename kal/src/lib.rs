//! # kal
//!
//! Command Abstraction Layer for bot libraries

// Deny missing_docs only on release mode.
// The lint must not interfere our development.
// But in release mode, we should take care of.
#![cfg_attr(not(debug_assertions), deny(missing_docs))]
#![cfg_attr(debug_assertions, warn(missing_docs))]

pub use command::Command;
pub use command_fragment::{
    CommaSeparated, CommandArgument, CommandArgumentValue, CommandArgumentValueType,
    CommandFragment, SpaceSeparated, TryFromArgumentValue, TryFromArgumentValueError,
};
pub use command_spec::{CommandOption, CommandOptionValueKind, CommandOptionValueTy, CommandSpec};
pub use error::CommandParseError;
pub use kal_derive::Command;

mod command;
mod command_fragment;
mod command_group;
mod command_spec;
mod error;

pub mod lex;
