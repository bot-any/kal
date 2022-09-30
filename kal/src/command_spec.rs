/// The specification of coomand
#[derive(Debug, PartialEq)]
pub struct CommandSpec {
    /// The name of command
    pub name: &'static str,

    /// The description of command
    pub description: &'static str,

    /// The options command can take
    pub options: Vec<CommandOption>,

    /// The subcommands command have
    pub subcommands: Vec<CommandSpec>,
}

/// The option command cane take
#[derive(Debug, PartialEq)]
pub struct CommandOption {
    /// The name of option when it is treated as named argument
    pub name: &'static str,

    /// The position of option when it is treated as positional argument
    pub position: usize,

    /// The description of option
    pub description: &'static str,

    /// The kind of value option can take
    pub value: CommandOptionValueKind,
}

/// The kind of value option can take
#[derive(Clone, Debug, PartialEq)]
pub enum CommandOptionValueKind {
    /// A kind of value that can appear or not
    Optional(Box<CommandOptionValueKind>),

    /// A kind of value that can appear or not
    Multiple(Box<CommandOptionValueKind>),

    /// String value
    String,

    /// Integer value
    Integer,

    /// Double precision floating point value
    Double,
}

impl CommandOptionValueKind {
    /// Whether the option value can be optional
    pub fn is_optional(&self) -> bool {
        matches!(self, CommandOptionValueKind::Optional(_))
    }

    /// Make the option value kind as primitive as possible
    pub fn as_primitive(&self) -> CommandOptionValueKind {
        match self {
            CommandOptionValueKind::Optional(t) | CommandOptionValueKind::Multiple(t) => {
                t.as_primitive()
            }
            _ => self.clone(),
        }
    }
}

/// Associate Rust type with [`CommandOptionValueKind`] and provide Rust side default value.
pub trait CommandOptionValueTy: Sized {
    /// Associated value kind
    fn spec_kind() -> CommandOptionValueKind;

    /// The default value for Rust side
    fn default() -> Option<Self> {
        None
    }
}

impl<T: CommandOptionValueTy> CommandOptionValueTy for Option<T> {
    fn spec_kind() -> CommandOptionValueKind {
        CommandOptionValueKind::Optional(Box::new(T::spec_kind()))
    }

    fn default() -> Option<Self> {
        Some(None)
    }
}

impl CommandOptionValueTy for String {
    fn spec_kind() -> CommandOptionValueKind {
        CommandOptionValueKind::String
    }
}

impl CommandOptionValueTy for i64 {
    fn spec_kind() -> CommandOptionValueKind {
        CommandOptionValueKind::Integer
    }
}

impl CommandOptionValueTy for f64 {
    fn spec_kind() -> CommandOptionValueKind {
        CommandOptionValueKind::Double
    }
}


impl<T: CommandOptionValueTy> CommandOptionValueTy for Vec<T> {
    fn spec_kind() -> CommandOptionValueKind {
        CommandOptionValueKind::Multiple(Box::new(T::spec_kind()))
    }
}
