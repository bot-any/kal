pub enum TransformHint {
    Integer,
    Float,
    String,
    StringGreedy,
}

pub trait TransformHintProvider {
    fn hint_sequence() -> &'static [TransformHint];
}
