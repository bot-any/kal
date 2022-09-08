pub enum CommandFragment {
    Select(String),
    Execute(Vec<(String, CommandArgumentValue)>),
}

#[derive(Clone)]
pub enum CommandArgumentValue {
    String(String),
    I64(i64),
    F64(f64),
}

#[derive(Debug)]
pub struct CommandArgumentValueTypeMismatchError {
    pub expected: String,
    pub actual: String,
}

impl core::fmt::Display for CommandArgumentValueTypeMismatchError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Expected {} but actual type is {}", self.expected, self.actual)
    }
}

impl std::error::Error for CommandArgumentValueTypeMismatchError {}

impl TryFrom<CommandArgumentValue> for String {
    type Error = CommandArgumentValueTypeMismatchError;

    fn try_from(value: CommandArgumentValue) -> Result<Self, Self::Error> {
        match value {
            CommandArgumentValue::String(v) => Ok(v),
            CommandArgumentValue::I64(_) => Err(CommandArgumentValueTypeMismatchError {
                expected: "String".to_string(),
                actual: "i64".to_string(),
            }),
            CommandArgumentValue::F64(_) => Err(CommandArgumentValueTypeMismatchError {
                expected: "String".to_string(),
                actual: "f64".to_string(),
            }),
        }
    }
}
impl TryFrom<CommandArgumentValue> for i64 {
    type Error = CommandArgumentValueTypeMismatchError;

    fn try_from(value: CommandArgumentValue) -> Result<Self, Self::Error> {
        match value {
            CommandArgumentValue::String(_) => Err(CommandArgumentValueTypeMismatchError {
                expected: "i64".to_string(),
                actual: "String".to_string(),
            }),
            CommandArgumentValue::I64(v) => Ok(v),
            CommandArgumentValue::F64(_) => Err(CommandArgumentValueTypeMismatchError {
                expected: "i64".to_string(),
                actual: "f64".to_string(),
            }),
        }
    }
}
impl TryFrom<CommandArgumentValue> for f64 {
    type Error = CommandArgumentValueTypeMismatchError;

    fn try_from(value: CommandArgumentValue) -> Result<Self, Self::Error> {
        match value {
            CommandArgumentValue::String(_) => Err(CommandArgumentValueTypeMismatchError {
                expected: "f64".to_string(),
                actual: "String".to_string(),
            }),
            CommandArgumentValue::I64(_) => Err(CommandArgumentValueTypeMismatchError {
                expected: "f64".to_string(),
                actual: "i64".to_string(),
            }),
            CommandArgumentValue::F64(v) => Ok(v),
        }
    }
}

impl TryFrom<CommandArgumentValue> for Option<String> {
    type Error = CommandArgumentValueTypeMismatchError;

    fn try_from(value: CommandArgumentValue) -> Result<Self, Self::Error> {
        Ok(String::try_from(value).ok())
    }
}

impl TryFrom<CommandArgumentValue> for Option<i64> {
    type Error = CommandArgumentValueTypeMismatchError;

    fn try_from(value: CommandArgumentValue) -> Result<Self, Self::Error> {
        Ok(i64::try_from(value).ok())
    }
}

impl TryFrom<CommandArgumentValue> for Option<f64> {
    type Error = CommandArgumentValueTypeMismatchError;

    fn try_from(value: CommandArgumentValue) -> Result<Self, Self::Error> {
        Ok(f64::try_from(value).ok())
    }
}
