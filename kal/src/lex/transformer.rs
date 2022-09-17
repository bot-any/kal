use crate::{CommandArgument, CommandArgumentValue, CommandFragment};

use super::{CommandLexError, CommandToken, RawStringPattern, TransformHint, TransformHintKind};

/// An error that can appear while transform the token produced into a command fragment.
#[derive(Debug, PartialEq, Eq)]
pub enum TokenTransformError<'a> {
    /// The error happen from lex step.
    LexError(CommandLexError<'a>),

    /// The command label is invalid.
    InvalidCommandLabel,
    /// Positioned argument cannot appear after named argument accepted.
    PositionedAfterNamed,
}

impl<'a> From<CommandLexError<'a>> for TokenTransformError<'a> {
    fn from(e: CommandLexError<'a>) -> Self {
        TokenTransformError::LexError(e)
    }
}

/// This struct provide a way to transform [`CommandToken`] into [`CommandFragment`] with hint provided
pub struct TokenTransformer<F>
where
    F: Fn(&str) -> Result<&str, TokenTransformError>,
{
    label_stripper: Option<F>,
    hint: TransformHint,
}

impl TokenTransformer<fn(&str) -> Result<&str, TokenTransformError>> {
    /// Make a new [`TokenTransformer`] with the hint from a command directly deriving [`TransformHintProvider`](`super::transform_hint::TransformHintProvider`).
    pub fn command_args(hint: TransformHint) -> Self {
        TokenTransformer {
            label_stripper: None,
            hint,
        }
    }
}

impl<F> TokenTransformer<F>
where
    F: Fn(&str) -> Result<&str, TokenTransformError>,
{
    /// Make a new [`TokenTransformer`] with a label stripper and the hint from [`command_group!`].
    pub fn command_group(label_stripper: F, hint: TransformHint) -> Self {
        TokenTransformer {
            label_stripper: Some(label_stripper),
            hint,
        }
    }

    /// Transform `Iterator` yielding [`CommandToken`] into an `Iterator` yielding [`CommandFragment`].
    pub fn transform<'a, 'b: 'a>(
        &'b self,
        tokens: impl Iterator<Item = Result<CommandToken<'a>, CommandLexError<'a>>> + 'a,
    ) -> impl Iterator<Item = Result<CommandFragment, TokenTransformError<'a>>> + 'a {
        TokenTransformerHandle {
            transformer: self,
            state: if self.label_stripper.is_some() {
                TokenTransformerHandleState::Label
            } else {
                TokenTransformerHandleState::Subcommand
            },
            tokens,
            hint: Some(self.hint.clone()),
            named_produced: false,
        }
    }
}

enum TokenTransformerHandleState {
    Label,
    Subcommand,
    Done,
}
struct TokenTransformerHandle<'a, I, F>
where
    I: Iterator<Item = Result<CommandToken<'a>, CommandLexError<'a>>>,
    F: Fn(&str) -> Result<&str, TokenTransformError>,
{
    transformer: &'a TokenTransformer<F>,
    state: TokenTransformerHandleState,
    tokens: I,
    hint: Option<TransformHint>,
    named_produced: bool,
}

impl<'a, I, F> TokenTransformerHandle<'a, I, F>
where
    I: Iterator<Item = Result<CommandToken<'a>, CommandLexError<'a>>>,
    F: Fn(&str) -> Result<&str, TokenTransformError>,
{
    fn next_label(&mut self) -> Option<Result<CommandFragment, TokenTransformError<'a>>> {
        self.state = TokenTransformerHandleState::Subcommand;
        loop {
            match self.tokens.next() {
                Some(Ok(CommandToken::Whitespace(_))) => continue,
                Some(Ok(CommandToken::RawString(label, _))) => {
                    let transformed = self
                        .transformer
                        .label_stripper
                        .as_ref()
                        .map(|f| f(label))
                        .unwrap_or(Ok(label));
                    break match transformed {
                        Ok(transformed) => {
                            self.hint = self.hint.as_ref().and_then(|hint| match hint {
                                TransformHint::Select(map) => map.get(transformed).cloned(),
                                _ => None,
                            });
                            Some(Ok(CommandFragment::Select(transformed.to_string())))
                        }
                        Err(e) => Some(Err(e)),
                    };
                }
                Some(Ok(_)) => break Some(Err(TokenTransformError::InvalidCommandLabel)),
                Some(Err(e)) => break Some(Err(TokenTransformError::from(e))),
                None => break None,
            }
        }
    }

    fn next_subcommand_or_args(
        &mut self,
    ) -> Option<Result<CommandFragment, TokenTransformError<'a>>> {
        if let Some(TransformHint::Execute(_)) = self.hint {
            let current = self.tokens.next();
            return self.next_args(current);
        }
        loop {
            match self.tokens.next() {
                Some(Ok(CommandToken::Whitespace(_))) => {
                    continue;
                }
                Some(Ok(CommandToken::RawString(subcommand, _))) => {
                    self.hint = self.hint.as_ref().and_then(|hint| match hint {
                        TransformHint::Select(map) => map.get(subcommand).cloned(),
                        _ => None,
                    });
                    break Some(Ok(CommandFragment::Select(subcommand.to_string())));
                }
                current @ Some(Ok(_)) => {
                    let res = self.next_args(current);
                    break res;
                }
                Some(Err(e)) => break Some(Err(TokenTransformError::from(e))),
                None => {
                    break self.next_args(None);
                }
            }
        }
    }
    fn next_args(
        &mut self,
        mut current: Option<Result<CommandToken<'a>, CommandLexError<'a>>>,
    ) -> Option<Result<CommandFragment, TokenTransformError<'a>>> {
        let hint_seq = self
            .hint
            .clone()
            .and_then(|hint| match hint {
                TransformHint::Execute(seq) => Some(seq),
                _ => None,
            })
            .unwrap_or_default();
        let mut hint_seq = hint_seq.iter();
        let mut args = Vec::new();
        let mut pos = 0;
        let mut greedy_string = None;

        let mut hint = hint_seq.next();
        loop {
            // TODO: handle multiple elements
            let multiple = hint.map(|hint| hint.multiple).unwrap_or(false);
            let hint_kind = hint.map(|hint| &hint.kind);
            let is_greedy = hint
                .map(|hint| matches!(hint.kind, TransformHintKind::StringGreedy))
                .unwrap_or(false);

            if is_greedy {
                let greedy = greedy_string.get_or_insert("".to_string());
                match current {
                    Some(Ok(mut token)) => loop {
                        match token {
                            CommandToken::RawString(s, _) => greedy.push_str(s),
                            CommandToken::Whitespace(s) => {
                                if !greedy.is_empty() {
                                    greedy.push_str(s);
                                }
                            }
                            CommandToken::QuotedString(open, value, close) => {
                                greedy.push_str(open);
                                greedy.push_str(&value);
                                greedy.push_str(close);
                            }
                            CommandToken::Named(name, next_token) => {
                                greedy.push_str(name);
                                greedy.push('=');
                                token = *next_token;
                                continue;
                            }
                        }
                        break;
                    },
                    Some(Err(e)) => match e {
                        CommandLexError::UnclosedQuote(_, s) => greedy.push_str(s),
                        CommandLexError::NamedProhibitsWhitespace(_, s) => greedy.push_str(s),
                        CommandLexError::NamedCannotContainNamed(_, s) => greedy.push_str(s),
                    },
                    None => break,
                }
            } else {
                match current {
                    Some(Ok(token)) => {
                        fn into_command_argument_value(
                            hint_kind: Option<&TransformHintKind>,
                            token: CommandToken,
                        ) -> Option<(Option<String>, CommandArgumentValue)>
                        {
                            match token {
                                CommandToken::Whitespace(_) => (None),
                                CommandToken::RawString(value, pat) => {
                                    let value = match (hint_kind, pat) {
                                        (
                                            Some(TransformHintKind::Float),
                                            RawStringPattern::Float | RawStringPattern::Integer,
                                        ) => CommandArgumentValue::F64(value.parse().unwrap()),
                                        (
                                            Some(TransformHintKind::Integer),
                                            RawStringPattern::Integer,
                                        ) => CommandArgumentValue::I64(value.parse().unwrap()),
                                        _ => CommandArgumentValue::String(value.to_string()),
                                    };
                                    Some((None, value))
                                }
                                CommandToken::QuotedString(_, value, _) => {
                                    Some((None, CommandArgumentValue::String(value)))
                                }
                                CommandToken::Named(name, value) => Some((
                                    Some(name.to_string()),
                                    into_command_argument_value(hint_kind, *value).unwrap().1,
                                )),
                            }
                        }

                        let value = into_command_argument_value(hint_kind, token);

                        if let Some((name, value)) = value {
                            let is_named = name.is_some();

                            let arg = if let Some(name) = name {
                                CommandArgument::Named(name, value)
                            } else {
                                CommandArgument::Positioned(pos, value)
                            };

                            args.push(arg);
                            if is_named {
                                self.named_produced = true;
                            } else {
                                if self.named_produced {
                                    return Some(Err(TokenTransformError::PositionedAfterNamed));
                                }
                                pos += 1;
                            }
                            hint = hint_seq.next();
                        }
                    }
                    Some(Err(e)) => return Some(Err(TokenTransformError::from(e))),
                    None => break,
                }
            }

            current = self.tokens.next();
        }
        if let Some(s) = greedy_string {
            if !s.is_empty() {
                args.push(CommandArgument::Positioned(
                    pos,
                    CommandArgumentValue::String(s),
                ));
            }
        }
        self.state = TokenTransformerHandleState::Done;
        Some(Ok(CommandFragment::Execute(args)))
    }
}
impl<'a, I, F> Iterator for TokenTransformerHandle<'a, I, F>
where
    I: Iterator<Item = Result<CommandToken<'a>, CommandLexError<'a>>>,
    F: Fn(&str) -> Result<&str, TokenTransformError>,
{
    type Item = Result<CommandFragment, TokenTransformError<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            TokenTransformerHandleState::Label => self.next_label(),
            TokenTransformerHandleState::Subcommand => self.next_subcommand_or_args(),
            TokenTransformerHandleState::Done => None,
        }
    }
}
