use core::fmt;

use crate::{CommandArgumentValue, CommandArgumentValueType};

mod impls;

/// Failed to convert [`CommandArgumentValue`] to the type expected.
#[derive(Debug)]
pub struct TryFromArgumentValueError {
    /// The type expected.
    pub expected_type: CommandArgumentValueType,

    /// The actual value accepted.
    pub actual_value: CommandArgumentValue,
}

impl fmt::Display for TryFromArgumentValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Expected type {} but actual type is {} with value {}",
            self.expected_type,
            CommandArgumentValueType::from(&self.actual_value),
            match self.actual_value {
                CommandArgumentValue::String(ref s) => s.to_string(),
                CommandArgumentValue::I64(i) => i.to_string(),
                CommandArgumentValue::F64(f) => f.to_string(),
            }
        )
    }
}

/// The trait to convert [`CommandArgumentValue`] to the type expected.
pub trait TryFromArgumentValue: Sized {
    /// Try to convert [`CommandArgumentValue`] to the type expected.
    fn try_from_argument_value(
        value: CommandArgumentValue,
        strict: bool,
    ) -> Result<Self, TryFromArgumentValueError>;
}
