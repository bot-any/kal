use kal::{
    lex::{remove_leading, remove_trailing, CommandLexer, TokenTransformer, TransformHintProvider},
    Command,
};

#[test]
fn transform() {
    #[derive(Debug, PartialEq, Command, TransformHintProvider)]
    #[command(name = "hello", description = "")]
    enum Hello {
        #[command(name = "world", description = "")]
        World {
            #[argument(name = "test", description = "", take_rest)]
            test: Option<String>,
        },
    }
    let transformer = TokenTransformer {
        label_stripper: |s| {
            remove_leading("/", s).map(|s| remove_trailing("@my_bot", s).unwrap_or(s))
        },
        hint: Hello::hint(),
    };

    let lexer = CommandLexer::new("/hello world");
    let fragments: Result<Vec<_>, _> = transformer.transform(lexer).collect();
    let parsed = fragments.map(|fragments| Hello::parse(&fragments[1..]));
    assert_eq!(Ok(Some(Hello::World { test: None })), parsed);

    let lexer = CommandLexer::new("/hello world it is great");
    let fragments: Result<Vec<_>, _> = transformer.transform(lexer).collect();
    let parsed = fragments.map(|fragments| Hello::parse(&fragments[1..]));
    assert_eq!(
        Ok(Some(Hello::World {
            test: Some("it is great".to_string())
        })),
        parsed
    );
}
