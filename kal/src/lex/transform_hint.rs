use std::collections::HashMap;

use crate::CommandOptionValueKind;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransformHintKind {
    Integer,
    Float,
    String,
    StringGreedy,
}

impl TransformHintKind {
    pub fn make_greedy(self) -> Self {
        match self {
            TransformHintKind::String => TransformHintKind::StringGreedy,
            itself => itself,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransformHintPart {
    pub multiple: bool,
    pub kind: TransformHintKind,
}

impl TransformHintPart {
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
                kind: TransformHintKind::String,
            },
            CommandOptionValueKind::Integer => TransformHintPart {
                multiple: false,
                kind: TransformHintKind::Integer,
            },
            CommandOptionValueKind::Double => TransformHintPart {
                multiple: false,
                kind: TransformHintKind::Float,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransformHint {
    Execute(Vec<TransformHintPart>),
    Select(HashMap<&'static str, TransformHint>),
    SelectOrExecute(HashMap<&'static str, TransformHint>, Vec<TransformHintPart>),
}

pub trait TransformHintProvider {
    fn hint() -> TransformHint;
}
