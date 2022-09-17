use super::TokenTransformError;

/// Remove leading string from a string.
/// If it cannot be removed, raise an error.
pub fn remove_leading<'a, 'b: 'a>(
    leading: &'a str,
    s: &'b str,
) -> Result<&'b str, TokenTransformError<'b>> {
    if s.len() > leading.len() && s.starts_with(leading) {
        Ok(&s[leading.len()..])
    } else {
        Err(TokenTransformError::InvalidCommandLabel)
    }
}

/// Remove trailing string from a string.
/// If it cannot be removed, raise an error.
pub fn remove_trailing<'a, 'b: 'a>(
    trailing: &'a str,
    s: &'b str,
) -> Result<&'b str, TokenTransformError<'b>> {
    if s.len() > trailing.len() && s.ends_with(trailing) {
        Ok(&s[..s.len() - trailing.len()])
    } else {
        Err(TokenTransformError::InvalidCommandLabel)
    }
}
