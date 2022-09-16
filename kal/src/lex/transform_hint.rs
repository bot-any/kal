use std::collections::HashMap;

use crate::CommandOptionValueKind;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct TransformHintPart {
    pub required: bool,
    pub kind: TransformHintKind,
}

impl TransformHintPart {
    pub fn make_greedy(self) -> Self {
        Self {
            required: self.required,
            kind: self.kind.make_greedy(),
        }
    }
}

impl From<CommandOptionValueKind> for TransformHintPart {
    fn from(kind: CommandOptionValueKind) -> Self {
        match kind {
            CommandOptionValueKind::Optional(v) => TransformHintPart {
                required: false,
                ..TransformHintPart::from(*v)
            },
            CommandOptionValueKind::String => TransformHintPart {
                required: true,
                kind: TransformHintKind::String,
            },
            CommandOptionValueKind::Integer => TransformHintPart {
                required: true,
                kind: TransformHintKind::Integer,
            },
            CommandOptionValueKind::Double => TransformHintPart {
                required: true,
                kind: TransformHintKind::Float,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub enum TransformHint {
    Execute(Vec<TransformHintPart>),
    Select(HashMap<&'static str, TransformHint>),
    SelectOrExecute(HashMap<&'static str, TransformHint>, Vec<TransformHintPart>),
}

pub trait TransformHintProvider {
    fn hint() -> TransformHint;
}
