//! Generate command framgents from String command.

use std::{fmt, iter::Peekable, str::CharIndices};

use super::{CommandToken, RawStringPattern};

#[derive(Debug, PartialEq, Eq)]
pub enum CommandLexError<'a> {
    UnclosedQuote(usize, &'a str),
    NamedProhibitsWhitespace(usize, &'a str),
    NamedCannotContainNamed(usize, &'a str),
}

impl fmt::Display for CommandLexError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandLexError::UnclosedQuote(pos, src_part) => {
                write!(
                    f,
                    "Unclosed quote at position {} in command: {}",
                    pos, src_part
                )
            }
            CommandLexError::NamedProhibitsWhitespace(pos, src_part) => write!(
                f,
                "Named argument prohibits whitespace at position {} in command: {}",
                pos, src_part
            ),
            CommandLexError::NamedCannotContainNamed(pos, src_part) => write!(
                f,
                "Named argument cannot contain named argument at position {} in command: {}",
                pos, src_part
            ),
        }
    }
}

impl std::error::Error for CommandLexError<'_> {}
pub struct CommandLexer<'a> {
    source: &'a str,
    iter: Peekable<CharIndices<'a>>,
    failed: bool,
}

impl<'a> CommandLexer<'a> {
    pub fn new(source: &'a str) -> Self {
        CommandLexer {
            source,
            iter: source.char_indices().peekable(),
            failed: false,
        }
    }
}

impl<'a> CommandLexer<'a> {
    fn next_whitespace(&mut self) -> Option<Result<CommandToken<'a>, CommandLexError<'a>>> {
        let (first, _) = self.iter.peek()?;
        let first = *first;

        let last = loop {
            match self.iter.peek() {
                Some((_, ch)) if ch.is_whitespace() => {
                    self.iter.next();
                }
                Some((i, _)) => break *i,
                None => break self.source.len(),
            }
        };
        Some(Ok(CommandToken::Whitespace(&self.source[first..last])))
    }
    fn next_quote(&mut self) -> Option<Result<CommandToken<'a>, CommandLexError<'a>>> {
        let (first, ch) = self.iter.next()?;
        let mut src = String::new();

        let src_first = match self.iter.next() {
            None => {
                return Some(Err(CommandLexError::UnclosedQuote(
                    first,
                    &self.source[first..],
                )))
            }
            Some((last, c)) if c == ch => {
                return Some(Ok(CommandToken::QuotedString(
                    &self.source[first..=last],
                    "".to_string(),
                    &self.source[last..=self
                        .iter
                        .peek()
                        .map(|(next_i, _)| *next_i)
                        .unwrap_or(self.source.len())],
                )))
            }
            Some((src_first, ch)) => {
                src.push(ch);
                src_first
            }
        };

        let mut previous_backslash = false;
        let src_last = loop {
            match self.iter.next() {
                Some((src_last, c)) if c == ch => {
                    if previous_backslash {
                        src.push(c);
                        previous_backslash = false;
                    } else {
                        break src_last;
                    }
                }
                Some((_, '\\')) => {
                    if previous_backslash {
                        src.push('\\');
                        previous_backslash = false;
                    } else {
                        previous_backslash = true;
                    }
                }
                Some((_, ch)) => {
                    src.push(ch);
                    if previous_backslash {
                        previous_backslash = false;
                    }
                }
                None => {
                    return Some(Err(CommandLexError::UnclosedQuote(
                        first,
                        &self.source[first..],
                    )))
                }
            }
        };

        Some(Ok(CommandToken::QuotedString(
            &self.source[first..src_first],
            src,
            &self.source[src_last
                ..self
                    .iter
                    .peek()
                    .map(|(next_i, _)| *next_i)
                    .unwrap_or(self.source.len())],
        )))
    }
    fn next_raw_string_or_named(
        &mut self,
    ) -> Option<Result<CommandToken<'a>, CommandLexError<'a>>> {
        let (first, _) = self.iter.peek()?;
        let first = *first;

        let mut is_numeric = true;
        let mut met_float_dot = false;
        let last = loop {
            match self.iter.peek() {
                Some((i, ch)) if ch.is_whitespace() => break *i,
                Some((i, ch)) => {
                    let i = *i;
                    match ch {
                        '=' => {
                            let name = &self.source[first..i];
                            if name.is_empty() {
                                return Some(Err(CommandLexError::NamedProhibitsWhitespace(
                                    first,
                                    &self.source[first..],
                                )));
                            }
                            self.iter.next();
                            let token = match self.next() {
                                Some(Ok(CommandToken::Whitespace(_))) | None => {
                                    return Some(Err(CommandLexError::NamedProhibitsWhitespace(
                                        first,
                                        &self.source[first..],
                                    )))
                                }
                                Some(Ok(CommandToken::Named(..))) => {
                                    return Some(Err(CommandLexError::NamedCannotContainNamed(
                                        first,
                                        &self.source[first..],
                                    )))
                                }
                                Some(Ok(expr)) => expr,
                                otherwise => return otherwise,
                            };
                            return Some(Ok(CommandToken::Named(name, Box::new(token))));
                        }
                        '0'..='9' => {
                            self.iter.next();
                        }
                        '.' => {
                            if met_float_dot {
                                is_numeric = false;
                            } else {
                                met_float_dot = true;
                            }
                            self.iter.next();
                        }
                        _ => {
                            is_numeric = false;
                            met_float_dot = false;
                            self.iter.next();
                        }
                    }
                }
                None => break self.source.len(),
            }
        };

        let pattern = match (is_numeric, met_float_dot) {
            (false, _) => RawStringPattern::Unrecognized,
            (true, true) => RawStringPattern::Float,
            (true, false) => RawStringPattern::Integer,
        };

        Some(Ok(CommandToken::RawString(
            &self.source[first..last],
            pattern,
        )))
    }
}

impl<'a> Iterator for CommandLexer<'a> {
    type Item = Result<CommandToken<'a>, CommandLexError<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.failed {
            return None;
        }

        let (_, ch) = self.iter.peek()?;

        let res = match ch {
            ch if ch.is_whitespace() => self.next_whitespace(),
            '"' | '\'' => self.next_quote(),
            _ => self.next_raw_string_or_named(),
        };

        if res.as_ref().map(|res| res.is_err()).unwrap_or(false) {
            self.failed = true;
        }

        res
    }
}
