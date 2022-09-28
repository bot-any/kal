pub fn rename_to_kebab_case(s: String) -> String {
    let mut result = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if i != 0 {
                result.push('-');
            }
            result.push(ch.to_ascii_lowercase());
        } else if ch == '_' && i != 0 {
            result.push('-');
        } else {
            result.push(ch);
        }
    }
    result
}
