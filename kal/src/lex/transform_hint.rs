use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum TransformHintKind {
    Integer,
    Float,
    String,
    StringGreedy,
}

#[derive(Clone)]
pub struct TransformHintPart {
    pub required: bool,
    pub kind: TransformHintKind,
}

#[derive(Clone)]
pub enum TransformHint {
    Execute(Vec<TransformHintPart>),
    Select(HashMap<&'static str, TransformHint>),
    SelectOrExecute(HashMap<&'static str, TransformHint>, Vec<TransformHintPart>),
}

pub trait TransformHintProvider {
    fn hint() -> TransformHint;
}
