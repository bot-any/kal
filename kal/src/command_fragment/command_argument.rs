use core::fmt;

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

/// The type that command argument could be
#[derive(Debug)]
pub enum CommandArgumentValueType {
    /// The `String` type
    String,
    /// The `i64` type
    I64,
    /// The `f64` type
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
