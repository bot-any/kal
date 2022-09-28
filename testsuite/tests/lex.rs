use kal::lex::{CommandLexError, CommandLexer, CommandToken, RawStringPattern};
use pretty_assertions::assert_eq;

#[test]
fn lex() {
    for (src, result) in [
        (
            "/hello world",
            Ok(vec![
                CommandToken::RawString("/hello", RawStringPattern::Unrecognized),
                CommandToken::Whitespace(" "),
                CommandToken::RawString("world", RawStringPattern::Unrecognized),
            ]),
        ),
        (
            "/hello    -1 1   2.3\t3a\r4.5b",
            Ok(vec![
                CommandToken::RawString("/hello", RawStringPattern::Unrecognized),
                CommandToken::Whitespace("    "),
                CommandToken::RawString("-1", RawStringPattern::Integer),
                CommandToken::Whitespace(" "),
                CommandToken::RawString("1", RawStringPattern::Integer),
                CommandToken::Whitespace("   "),
                CommandToken::RawString("2.3", RawStringPattern::Float),
                CommandToken::Whitespace("\t"),
                CommandToken::RawString("3a", RawStringPattern::Unrecognized),
                CommandToken::Whitespace("\r"),
                CommandToken::RawString("4.5b", RawStringPattern::Unrecognized),
            ]),
        ),
    ] {
        let tokens: Result<Vec<_>, _> = CommandLexer::new(src).collect();
        assert_eq!(result, tokens);
    }
}

#[test]
fn lex_quote() {
    for (src, result) in [
        ("\"what", Err(CommandLexError::UnclosedQuote(0, "\"what"))),
        (
            "\"ok\"",
            Ok(vec![CommandToken::QuotedString(
                "\"",
                "ok".to_string(),
                "\"",
            )]),
        ),
        (
            "\"ok\" and \"ok\"",
            Ok(vec![
                CommandToken::QuotedString("\"", "ok".to_string(), "\""),
                CommandToken::Whitespace(" "),
                CommandToken::RawString("and", RawStringPattern::Unrecognized),
                CommandToken::Whitespace(" "),
                CommandToken::QuotedString("\"", "ok".to_string(), "\""),
            ]),
        ),
        (
            r#"
                "escape \\ more \"yes\""
            "#
            .trim(),
            Ok(vec![CommandToken::QuotedString(
                "\"",
                "escape \\ more \"yes\"".to_string(),
                "\"",
            )]),
        ),
    ] {
        let tokens: Result<Vec<_>, _> = CommandLexer::new(src).collect();
        assert_eq!(result, tokens);
    }
}

#[test]
fn lex_named() {
    for (src, result) in [
        (
            "a=",
            Err(CommandLexError::NamedProhibitsWhitespace(0, "a=")),
        ),
        (
            "a=b",
            Ok(vec![CommandToken::Named(
                "a",
                Box::new(CommandToken::RawString("b", RawStringPattern::Unrecognized)),
            )]),
        ),
        (
            "a=b=c",
            Err(CommandLexError::NamedCannotContainNamed(0, "a=b=c")),
        ),
    ] {
        let tokens: Result<Vec<_>, _> = CommandLexer::new(src).collect();
        assert_eq!(result, tokens);
    }
}
