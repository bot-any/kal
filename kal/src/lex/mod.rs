pub use lexer::{CommandLexError, CommandLexer};
pub use token::{CommandToken, RawStringPattern};
pub use transform_hint::{
    TransformHint, TransformHintKind, TransformHintPart, TransformHintProvider,
};
pub use transformer::{remove_leading, remove_trailing, TokenTransformError, TokenTransformer};

mod lexer;
mod token;
mod transform_hint;
mod transformer;
