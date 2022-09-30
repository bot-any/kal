use crate::{
    CommandArgumentValue, CommandArgumentValueType, TryFromArgumentValue, TryFromArgumentValueError,
};

impl TryFromArgumentValue for String {
    fn try_from_argument_value(
        value: CommandArgumentValue,
        strict: bool,
    ) -> Result<Self, TryFromArgumentValueError> {
        match value {
            CommandArgumentValue::String(v) => Ok(v),
            CommandArgumentValue::I64(ref inner) => {
                if strict {
                    Err(TryFromArgumentValueError {
                        expected_type: CommandArgumentValueType::String,
                        actual_value: value,
                    })
                } else {
                    Ok(inner.to_string())
                }
            }
            CommandArgumentValue::F64(ref inner) => {
                if strict {
                    Err(TryFromArgumentValueError {
                        expected_type: CommandArgumentValueType::String,
                        actual_value: value,
                    })
                } else {
                    Ok(inner.to_string())
                }
            }
        }
    }
}
