use std::collections::HashMap;

use crate::CommandOptionValueKind;

/// The kind of [`TransformHintPart`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransformHintPart {
    /// Take integer if possible.
    Integer,

    /// Take float if possible.
    Float,

    /// Take string.
    String,

    /// Make transformer to eat all tokens until the end of the input and make as a single string.
    StringGreedy,
}

impl TransformHintPart {
    /// Make itself greedy if possible.
    pub fn make_greedy(self) -> Self {
        match self {
            TransformHintPart::String => TransformHintPart::StringGreedy,
            itself => itself,
        }
    }
}

impl From<CommandOptionValueKind> for TransformHintPart {
    fn from(kind: CommandOptionValueKind) -> Self {
        match kind {
            CommandOptionValueKind::Optional(v) => TransformHintPart::from(*v),
            CommandOptionValueKind::String => TransformHintPart::String,
            CommandOptionValueKind::Integer => TransformHintPart::Integer,
            CommandOptionValueKind::Double => TransformHintPart::Float,
            CommandOptionValueKind::Multiple(v) => TransformHintPart::from(*v),
        }
    }
}

/// The hint for [`TokenTransformer`](`super::TokenTransformer`) to produce [`CommandFragment`](`crate::CommandFragment`) well.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransformHint {
    /// It should select a command specified in a map.
    Select(HashMap<&'static str, TransformHint>),

    /// It should execute command with arguments format hint specified.
    Execute(Vec<TransformHintPart>),

    /// It should select a command specified in a map or execute the command with arguments format hint specified.
    SelectOrExecute(HashMap<&'static str, TransformHint>, Vec<TransformHintPart>),
}

/// A type can provide a [`TransformHint`].
pub trait TransformHintProvider {
    /// Provides a [`TransformHint`] for [`TokenTransformer`](`super::TokenTransformer`).
    fn hint() -> TransformHint;
}
