use std::fmt;

/// The fragment of user command
#[derive(Debug, PartialEq)]
pub enum CommandFragment {
    /// Select subcommand
    Select(String),

    /// Execute command with arguments provided
    Execute(Vec<CommandArgument>),
}

/// Command argument
#[derive(Debug, PartialEq)]
pub enum CommandArgument {
    /// Named argument
    Named(String, CommandArgumentValue),

    /// Positional argument
    Positioned(usize, CommandArgumentValue),
}

/// The actual argument value
#[derive(Clone, Debug, PartialEq)]
pub enum CommandArgumentValue {
    /// The string type.
    String(String),

    /// The i64 type.
    I64(i64),

    /// The f64 type.
    F64(f64),
}

#[derive(Debug)]
pub enum CommandArgumentValueType {
    String,
    I64,
    F64,
}

impl From<&'_ CommandArgumentValue> for CommandArgumentValueType {
    fn from(value: &CommandArgumentValue) -> Self {
        match value {
            CommandArgumentValue::String(_) => CommandArgumentValueType::String,
            CommandArgumentValue::I64(_) => CommandArgumentValueType::I64,
            CommandArgumentValue::F64(_) => CommandArgumentValueType::F64,
        }
    }
}

impl fmt::Display for CommandArgumentValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandArgumentValueType::String => write!(f, "String"),
            CommandArgumentValueType::I64 => write!(f, "i64"),
            CommandArgumentValueType::F64 => write!(f, "f64"),
        }
    }
}

/// Failed to convert [`CommandArgumentValue`] to the type expected.
#[derive(Debug)]
pub struct CommandArgumentValueTypeMismatchError {
    /// The type expected.
    pub expected_type: CommandArgumentValueType,

    /// The actual value accepted.
    pub actual_value: CommandArgumentValue,
}

impl core::fmt::Display for CommandArgumentValueTypeMismatchError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
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

impl std::error::Error for CommandArgumentValueTypeMismatchError {}

pub trait TryFromArgumentValue: Sized {
    fn try_from_argument_value(
        value: CommandArgumentValue,
        strict: bool,
    ) -> Result<Self, CommandArgumentValueTypeMismatchError>;
}

impl TryFromArgumentValue for String {
    fn try_from_argument_value(
        value: CommandArgumentValue,
        strict: bool,
    ) -> Result<Self, CommandArgumentValueTypeMismatchError> {
        match value {
            CommandArgumentValue::String(v) => Ok(v),
            CommandArgumentValue::I64(ref inner) => {
                if strict {
                    Err(CommandArgumentValueTypeMismatchError {
                        expected_type: CommandArgumentValueType::String,
                        actual_value: value,
                    })
                } else {
                    Ok(inner.to_string())
                }
            }
            CommandArgumentValue::F64(ref inner) => {
                if strict {
                    Err(CommandArgumentValueTypeMismatchError {
                        expected_type: CommandArgumentValueType::String,
                        actual_value: value,
                    })
                } else {
                    Ok(inner.to_string())
                }
            }
        }
    }
}
impl TryFromArgumentValue for i64 {
    fn try_from_argument_value(
        value: CommandArgumentValue,
        strict: bool,
    ) -> Result<Self, CommandArgumentValueTypeMismatchError> {
        match value {
            CommandArgumentValue::String(ref inner) => {
                let parsed = if strict { None } else { inner.parse().ok() };
                if let Some(parsed) = parsed {
                    Ok(parsed)
                } else {
                    Err(CommandArgumentValueTypeMismatchError {
                        expected_type: CommandArgumentValueType::I64,
                        actual_value: value,
                    })
                }
            }
            CommandArgumentValue::I64(v) => Ok(v),
            value => Err(CommandArgumentValueTypeMismatchError {
                expected_type: CommandArgumentValueType::I64,
                actual_value: value,
            }),
        }
    }
}
impl TryFromArgumentValue for f64 {
    fn try_from_argument_value(
        value: CommandArgumentValue,
        strict: bool,
    ) -> Result<Self, CommandArgumentValueTypeMismatchError> {
        match value {
            CommandArgumentValue::String(ref inner) => {
                let parsed = if strict { None } else { inner.parse().ok() };
                if let Some(parsed) = parsed {
                    Ok(parsed)
                } else {
                    Err(CommandArgumentValueTypeMismatchError {
                        expected_type: CommandArgumentValueType::I64,
                        actual_value: value,
                    })
                }
            }
            CommandArgumentValue::F64(v) => Ok(v),
            value => Err(CommandArgumentValueTypeMismatchError {
                expected_type: CommandArgumentValueType::F64,
                actual_value: value,
            }),
        }
    }
}

impl<T: TryFromArgumentValue> TryFromArgumentValue for Option<T> {
    fn try_from_argument_value(
        value: CommandArgumentValue,
        strict: bool,
    ) -> Result<Self, CommandArgumentValueTypeMismatchError> {
        Ok(T::try_from_argument_value(value, strict).ok())
    }
}

impl<T: TryFromArgumentValue> TryFromArgumentValue for Vec<T> {
    fn try_from_argument_value(
        value: CommandArgumentValue,
        strict: bool,
    ) -> Result<Self, CommandArgumentValueTypeMismatchError> {
        match value {
            CommandArgumentValue::String(s) => s
                .split_ascii_whitespace()
                .map(|s| {
                    T::try_from_argument_value(CommandArgumentValue::String(s.to_string()), false)
                })
                .collect::<Result<_, _>>(),
            value => Ok(vec![T::try_from_argument_value(value, strict)?]),
        }
    }
}
