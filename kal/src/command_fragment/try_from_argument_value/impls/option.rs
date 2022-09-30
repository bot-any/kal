use crate::{CommandArgumentValue, TryFromArgumentValue, TryFromArgumentValueError};

impl<T: TryFromArgumentValue> TryFromArgumentValue for Option<T> {
    fn try_from_argument_value(
        value: CommandArgumentValue,
        strict: bool,
    ) -> Result<Self, TryFromArgumentValueError> {
        Ok(T::try_from_argument_value(value, strict).ok())
    }
}
