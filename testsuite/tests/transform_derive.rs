use kal::{
    lex::{CommandLexer, TokenTransformer, TransformHintProvider},
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
    let transformer = TokenTransformer::command_args(Hello::hint());

    let lexer = CommandLexer::new("world");
    let fragments: Result<Vec<_>, _> = transformer.transform(lexer).collect();
    dbg!(&fragments);
    let parsed = fragments.map(|fragments| Hello::parse(&fragments));
    assert_eq!(Ok(Some(Hello::World { test: None })), parsed);

    let lexer = CommandLexer::new("world it is great");
    let fragments: Result<Vec<_>, _> = transformer.transform(lexer).collect();
    let parsed = fragments.map(|fragments| Hello::parse(&fragments));
    assert_eq!(
        Ok(Some(Hello::World {
            test: Some("it is great".to_string())
        })),
        parsed
    );
}
