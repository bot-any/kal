use core::fmt;

/// An error made while parsing command from [`CommandFragment`](`super::CommandFragment`).
#[derive(Debug, PartialEq, Eq)]
pub enum CommandParseError<'a> {
    /// The command name cannot be matched.
    UnknownCommand(&'a String),

    /// There are missing arguments
    MissingArguments(Vec<&'a str>),

    /// The command fragments are incomplete for parsing.
    IncompleteCommand,

    /// Tried to execute too early.
    ExecuteTooEarly,

    /// The error happen while transforming tokens
    #[cfg(feature = "lex")]
    TokenTransformError(crate::lex::TokenTransformError<'a>),

    /// The error happen while transforming tokens
    #[cfg(feature = "lex")]
    TokenTransformErrorRef(&'a crate::lex::TokenTransformError<'a>),
}

impl fmt::Display for CommandParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandParseError::UnknownCommand(command) => {
                write!(f, "Unknown command: {}", command)
            }
            CommandParseError::MissingArguments(missing) => {
                write!(f, "Missing arguments: {}", missing.join(", "))
            }
            CommandParseError::IncompleteCommand => write!(f, "Incomplete command"),
            CommandParseError::ExecuteTooEarly => write!(f, "Execute too early"),
            #[cfg(feature = "lex")]
            CommandParseError::TokenTransformError(error) => {
                write!(f, "Token transform error: {}", error)
            }
            #[cfg(feature = "lex")]
            CommandParseError::TokenTransformErrorRef(error) => {
                write!(f, "Token transform error: {}", error)
            }
        }
    }
}

impl std::error::Error for CommandParseError<'_> {}

#[cfg(feature = "lex")]
impl<'a> From<crate::lex::TokenTransformError<'a>> for CommandParseError<'a> {
    fn from(err: crate::lex::TokenTransformError<'a>) -> Self {
        Self::TokenTransformError(err)
    }
}

#[cfg(feature = "lex")]
impl<'a> From<&'a crate::lex::TokenTransformError<'a>> for CommandParseError<'a> {
    fn from(err: &'a crate::lex::TokenTransformError<'a>) -> Self {
        Self::TokenTransformErrorRef(err)
    }
}
