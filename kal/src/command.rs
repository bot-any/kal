use crate::{CommandFragment, CommandSpec};

pub trait Command: Sized {
    const NAME: &'static str;

    fn spec() -> CommandSpec;

    fn parse(fragments: &[CommandFragment]) -> Option<Self>;
}
