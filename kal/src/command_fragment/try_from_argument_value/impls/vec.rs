use crate::{
    CommaSeparated, CommandArgumentValue, SpaceSeparated, TryFromArgumentValue,
    TryFromArgumentValueError,
};

macro_rules! try_into_vec {
    ($value:expr, $strict:expr, $($split:tt)*) => {
        match $value {
            CommandArgumentValue::String(inner) => inner
                .$($split)*
                .map(|s| T::try_from_argument_value(CommandArgumentValue::String(s.trim().to_string()), false))
                .collect::<Result<_, _>>(),
            value => Ok(vec![T::try_from_argument_value(value, $strict)?]),
        }
    };
}

impl<T: TryFromArgumentValue> TryFromArgumentValue for SpaceSeparated<T> {
    fn try_from_argument_value(
        value: CommandArgumentValue,
        strict: bool,
    ) -> Result<Self, TryFromArgumentValueError> {
        try_into_vec! {
            value,
            strict,
            split_ascii_whitespace()
        }
        .map(SpaceSeparated)
    }
}
impl<T: TryFromArgumentValue> TryFromArgumentValue for CommaSeparated<T> {
    fn try_from_argument_value(
        value: CommandArgumentValue,
        strict: bool,
    ) -> Result<Self, TryFromArgumentValueError> {
        try_into_vec! {
            value,
            strict,
            split(',')
        }
        .map(CommaSeparated)
    }
}
