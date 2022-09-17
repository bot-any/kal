use std::collections::HashMap;

use crate::CommandOptionValueKind;

/// The kind of [`TransformHintPart`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransformHintPartKind {
    /// Take integer if possible.
    Integer,

    /// Take float if possible.
    Float,

    /// Take string.
    String,

    /// Make transformer to eat all tokens until the end of the input and make as a single string.
    StringGreedy,
}

impl TransformHintPartKind {
    /// Make itself greedy if possible.
    pub fn make_greedy(self) -> Self {
        match self {
            TransformHintPartKind::String => TransformHintPartKind::StringGreedy,
            itself => itself,
        }
    }
}

/// The part of [`TransformHint`] on [`TransformHint::Execute`] or [`TransformHint::SelectOrExecute`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransformHintPart {
    /// The kind of the part.
    pub kind: TransformHintPartKind,

    /// Whether the part can be multiple.
    /// If this is true, it will try to eat all tokens until it cannot take anymore.
    pub multiple: bool,
}

impl TransformHintPart {
    /// Make itself greedy if possible.
    pub fn make_greedy(self) -> Self {
        Self {
            multiple: self.multiple,
            kind: self.kind.make_greedy(),
        }
    }
}

impl From<CommandOptionValueKind> for TransformHintPart {
    fn from(kind: CommandOptionValueKind) -> Self {
        match kind {
            CommandOptionValueKind::Optional(v) => TransformHintPart::from(*v),
            CommandOptionValueKind::String => TransformHintPart {
                multiple: false,
                kind: TransformHintPartKind::String,
            },
            CommandOptionValueKind::Integer => TransformHintPart {
                multiple: false,
                kind: TransformHintPartKind::Integer,
            },
            CommandOptionValueKind::Double => TransformHintPart {
                multiple: false,
                kind: TransformHintPartKind::Float,
            },
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
