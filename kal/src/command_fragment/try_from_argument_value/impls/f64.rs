use crate::{
    CommandArgumentValue, CommandArgumentValueType, TryFromArgumentValue, TryFromArgumentValueError,
};

impl TryFromArgumentValue for f64 {
    fn try_from_argument_value(
        value: CommandArgumentValue,
        strict: bool,
    ) -> Result<Self, TryFromArgumentValueError> {
        match value {
            CommandArgumentValue::String(ref inner) => {
                let parsed = if strict { None } else { inner.parse().ok() };
                if let Some(parsed) = parsed {
                    Ok(parsed)
                } else {
                    Err(TryFromArgumentValueError {
                        expected_type: CommandArgumentValueType::I64,
                        actual_value: value,
                    })
                }
            }
            CommandArgumentValue::F64(v) => Ok(v),
            value => Err(TryFromArgumentValueError {
                expected_type: CommandArgumentValueType::F64,
                actual_value: value,
            }),
        }
    }
}
