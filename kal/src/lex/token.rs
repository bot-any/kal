#[derive(Debug, PartialEq, Eq)]
pub enum RawStringPattern {
    Unrecognized,
    Integer,
    Float,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CommandToken<'a> {
    RawString(&'a str, RawStringPattern),
    Whitespace(&'a str),
    QuotedString(&'a str, String, &'a str),
    Named(&'a str, Box<CommandToken<'a>>),
}
