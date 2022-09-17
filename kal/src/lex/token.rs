/// The pattern matched from raw string.
/// Useful for analyze how the token should be treated.
#[derive(Debug, PartialEq, Eq)]
pub enum RawStringPattern {
    /// Do not match to any pattern.
    Unrecognized,

    /// Matches to sequence of digits with optional plus or minus sign.
    Integer,

    /// Simillar to integer, but there's one dot between digits.
    Float,
}

/// The token made out of command string.
#[derive(Debug, PartialEq, Eq)]
pub enum CommandToken<'a> {
    /// raw string with value and matched pattern.
    RawString(&'a str, RawStringPattern),

    /// Whitespaces.
    Whitespace(&'a str),

    /// Quoted string with opening quote, value, closing quote.
    QuotedString(&'a str, String, &'a str),

    /// A named command token. It should not have a named token inside.
    Named(&'a str, Box<CommandToken<'a>>),
}
