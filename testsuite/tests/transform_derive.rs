use kal::{
    lex::{CommandLexer, TokenTransformer, TransformHintProvider},
    Command, CommandParseError,
};
use pretty_assertions::assert_eq;

#[test]
fn transform() {
    #[derive(Debug, PartialEq, Command, TransformHintProvider)]
    enum Hello {
        World {
            #[argument(take_rest)]
            test: Option<String>,
        },
    }
    let transformer = TokenTransformer::command_args(Hello::hint());

    let lexer = CommandLexer::new("world");
    let fragments: Result<Vec<_>, _> = transformer.transform(lexer).collect();
    let parsed = fragments
        .as_ref()
        .map_err(CommandParseError::from)
        .and_then(|fragments| Hello::parse(fragments));
    assert_eq!(Ok(Hello::World { test: None }), parsed);

    let lexer = CommandLexer::new("world it is great");
    let fragments: Result<Vec<_>, _> = transformer.transform(lexer).collect();
    let parsed = fragments
        .as_ref()
        .map_err(CommandParseError::from)
        .and_then(|fragments| Hello::parse(fragments));
    assert_eq!(
        Ok(Hello::World {
            test: Some("it is great".to_string())
        }),
        parsed
    );
}
