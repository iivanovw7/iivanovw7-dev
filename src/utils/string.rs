pub fn capitalize(s: &str) -> String {
    let mut chars = s.trim().chars();

    match chars.next() {
        None => String::new(),
        Some(char) => char.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(capitalize(""), "");
    }

    #[test]
    fn test_single_lowercase_char() {
        assert_eq!(capitalize("a"), "A");
    }

    #[test]
    fn test_single_uppercase_char() {
        assert_eq!(capitalize("B"), "B");
    }

    #[test]
    fn test_lowercase_word() {
        assert_eq!(capitalize("hello"), "Hello");
    }

    #[test]
    fn test_uppercase_word() {
        assert_eq!(capitalize("WORLD"), "WORLD");
    }

    #[test]
    fn test_mixed_case_word() {
        assert_eq!(capitalize("rustLang"), "RustLang");
    }

    #[test]
    fn test_input_with_spaces() {
        assert_eq!(capitalize("rust Lang"), "Rust Lang");
    }

    #[test]
    fn test_string_with_spaces() {
        assert_eq!(capitalize("hello world"), "Hello world");
    }

    #[test]
    fn test_string_with_leading_and_trailing_spaces() {
        assert_eq!(capitalize("  test  "), "Test");
    }

    #[test]
    fn test_string_with_numbers() {
        assert_eq!(capitalize("123test"), "123test");
    }

    #[test]
    fn test_string_with_special_characters() {
        assert_eq!(capitalize("!@#test"), "!@#test");
    }
}
