use crate::{CommandFragment, CommandSpec};

/// The command
pub trait Command: Sized {
    /// The name of command
    const NAME: &'static str;

    /// The spec of command for command registration.
    fn spec() -> CommandSpec;

    /// Try parse command from [`CommandFramgnet`] sequence.
    fn parse(fragments: &[CommandFragment]) -> Option<Self>;
}
