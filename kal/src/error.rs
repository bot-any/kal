/// An error made while parsing command from [`CommandFragment`](`super::CommandFragment`).
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
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

#[cfg(feature = "lex")]
impl<'a> From< crate::lex::TokenTransformError<'a>> for CommandParseError<'a> {
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
