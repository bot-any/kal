//! The lex module provides a lexer for parsing commands.
//! It coudl be enabled with `"lex"` feature flag.
//!
//! ## Lex
//!
//! You can lex the command string with [`CommandLexer`].
//!
//! It produces following tokens:
//! - Basic raw string with pattern analysis
//! - Sequence of whitespace
//! - Quoted string
//! - Named raw string or quoted string
//!
//! See belo example for sure.
//! ```rust
//! # use kal::lex::{CommandLexer, CommandToken, RawStringPattern};
//! use CommandToken as Token;
//! use RawStringPattern as Pat;
//!
//! let mut lexer = CommandLexer::new(r#"hello world 1.0 3 "quote" named=value"#);
//! assert_eq!(lexer.next(), Some(Ok(Token::RawString("hello", Pat::Unrecognized))));
//! assert_eq!(lexer.next(), Some(Ok(Token::Whitespace(" "))));
//! assert_eq!(lexer.next(), Some(Ok(Token::RawString("world", Pat::Unrecognized))));
//! assert_eq!(lexer.next(), Some(Ok(Token::Whitespace(" "))));
//! assert_eq!(lexer.next(), Some(Ok(Token::RawString("1.0", Pat::Float))));
//! assert_eq!(lexer.next(), Some(Ok(Token::Whitespace(" "))));
//! assert_eq!(lexer.next(), Some(Ok(Token::RawString("3", Pat::Integer))));
//! assert_eq!(lexer.next(), Some(Ok(Token::Whitespace(" "))));
//! assert_eq!(lexer.next(), Some(Ok(Token::QuotedString("\"", "quote".to_string(), "\""))));
//! assert_eq!(lexer.next(), Some(Ok(Token::Whitespace(" "))));
//! assert_eq!(
//!     lexer.next(),
//!     Some(Ok(Token::Named(
//!         "named",
//!         Box::new(Token::RawString("value", Pat::Unrecognized)))
//!    ))
//! );
//! assert_eq!(lexer.next(), None);
//! ```
//!
//! ## Transform
//!
//! Tokens contain too much information for command execution.
//! Thus, we will transform [`CommandToken`] to [`CommandFragment`](`crate::CommandFragment`).
//!
//! See xample below:
//!
//! ```rust
//! # use kal::lex::{CommandLexer, TokenTransformer, TransformHint};
//! # use kal::CommandFragment;
//! # use std::collections::HashMap;
//! let hint = TransformHint::Select(
//!     HashMap::from_iter([("hello", TransformHint::Select(
//!         HashMap::from_iter([("world", TransformHint::Execute(vec![]))])
//!     ))])
//! );
//! let transformer = TokenTransformer::command_args(hint);
//! let lexer = CommandLexer::new("hello world");
//! let result: Result<Vec<_>, _> = transformer.transform(lexer).collect();
//! assert_eq!(
//!     result,
//!     Ok(vec![
//!         CommandFragment::Select("hello".to_string()),
//!         CommandFragment::Select("world".to_string()),
//!         CommandFragment::Execute(vec![]),
//!     ])
//! );
//! ```
//!
//! ### Label Strip
//!
//! Sometimes you need to strip the label for match command name correctly.
//! For example, you'll never need the leading `/` from `/hello` command.
//! You can easily strip leading/trailing string constants.
//! It is very useful when you're working with [`command_group!`](`crate::command_group!`).
//!
//! ```rust
//! # use kal::lex::{CommandLexer, TokenTransformer, TransformHint, remove_leading};
//! # use kal::CommandFragment;
//! # use std::collections::HashMap;
//! # let hint = TransformHint::Select(
//! #     HashMap::from_iter([("hello", TransformHint::Select(
//! #         HashMap::from_iter([("world", TransformHint::Execute(vec![]))])
//! #     ))])
//! # );
//! let transformer = TokenTransformer::command_group(|s| remove_leading("/", s), hint);
//! let lexer = CommandLexer::new("/hello world");
//! let result: Result<Vec<_>, _> = transformer.transform(lexer).collect();
//! assert_eq!(
//!     result,
//!     Ok(vec![
//!         CommandFragment::Select("hello".to_string()),
//!         CommandFragment::Select("world".to_string()),
//!         CommandFragment::Execute(vec![]),
//!     ])
//! );
//! ```
//!
//! ### #[derive(TransformHintProvider)]
//!
//! As you can see above, the [`TokenTransformer`] needs [`TransformHint`] to work properly.
//! But writing hints hand-by-hand is a bit tedious job.
//! So, we provide a derive macro to generate it from an item.
//! Enable both `"lex"` and `"derive"` feature flags (later one is enabled by default),
//! and use `#[derive(TransformHintProvider)]` on your item.
//! It will take your `#[argument]` attributes for generating hints.
//!
//! ```rust
//! # use kal::Command;
//! # use kal::lex::{TransformHintProvider, TransformHint, TransformHintPart, TransformHintPartKind};
//! # use std::collections::HashMap;
//! #[derive(Command, TransformHintProvider)]
//! #[command(name = "hello")]
//! pub enum Hello {
//!     #[command(name = "world")]
//!     World {
//!         #[argument(name = "argument")]
//!         argument: String,
//!     },
//!     #[command(name = "sekai")]
//!     Sekai {
//!         #[argument(name = "argument", take_rest)]
//!         argument: String,
//!     },
//! }
//!
//! assert_eq!(
//!     Hello::hint(),
//!     TransformHint::Select(HashMap::from_iter([
//!         (
//!             "world",
//!             TransformHint::Execute(vec![TransformHintPart {
//!                 multiple: false,
//!                 kind: TransformHintPartKind::String,
//!             }]),
//!         ),
//!         (
//!             "sekai",
//!             TransformHint::Execute(vec![TransformHintPart {
//!                 multiple: false,
//!                 kind: TransformHintPartKind::StringGreedy,
//!             }]),
//!         ),
//!    ]))
//! );
//! ```
#![cfg(feature = "lex")]
pub use kal_derive::TransformHintProvider;
pub use label_strip_util::{remove_leading, remove_trailing};
pub use lexer::{CommandLexError, CommandLexer};
pub use token::{CommandToken, RawStringPattern};
pub use transform_hint::{
    TransformHint, TransformHintPart, TransformHintPartKind, TransformHintProvider,
};
pub use transformer::{TokenTransformError, TokenTransformer};

mod label_strip_util;
mod lexer;
mod token;
mod transform_hint;
mod transformer;
