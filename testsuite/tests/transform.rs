use std::collections::HashMap;

use kal::{
    lex::{
        remove_leading, remove_trailing, CommandLexer, TokenTransformer, TransformHint,
        TransformHintKind, TransformHintPart,
    },
    CommandArgument, CommandArgumentValue, CommandFragment,
};

#[test]
fn transform() {
    let transformer = TokenTransformer::command_group(
        |s| remove_leading("/", s).map(|s| remove_trailing("@my_bot", s).unwrap_or(s)),
        TransformHint::Select({
            let mut map = HashMap::new();

            map.insert(
                "hello",
                TransformHint::Select({
                    let mut map = HashMap::new();

                    map.insert(
                        "world",
                        TransformHint::Execute(vec![TransformHintPart {
                            required: false,
                            kind: TransformHintKind::StringGreedy,
                        }]),
                    );

                    map
                }),
            );

            map
        }),
    );

    let lexer = CommandLexer::new("/hello world");

    let fragments: Result<Vec<_>, _> = transformer.transform(lexer).collect();

    assert_eq!(
        Ok(vec![
            CommandFragment::Select("hello".to_string()),
            CommandFragment::Select("world".to_string()),
            CommandFragment::Execute(vec![]),
        ]),
        fragments
    );
}

#[test]
fn transform_argument() {
    let transformer = TokenTransformer::command_group(
        |s| Ok(s),
        TransformHint::Select({
            let mut map = HashMap::new();

            map.insert(
                "int",
                TransformHint::Execute(vec![TransformHintPart {
                    required: true,
                    kind: TransformHintKind::Integer,
                }]),
            );

            map.insert(
                "float",
                TransformHint::Execute(vec![TransformHintPart {
                    required: true,
                    kind: TransformHintKind::Float,
                }]),
            );

            map.insert(
                "string",
                TransformHint::Execute(vec![TransformHintPart {
                    required: true,
                    kind: TransformHintKind::String,
                }]),
            );

            map.insert(
                "greedy",
                TransformHint::Execute(vec![TransformHintPart {
                    required: true,
                    kind: TransformHintKind::StringGreedy,
                }]),
            );

            map
        }),
    );

    let lexer = CommandLexer::new("int 1");
    let fragments: Result<Vec<_>, _> = transformer.transform(lexer).collect();
    assert_eq!(
        Ok(vec![
            CommandFragment::Select("int".to_string()),
            CommandFragment::Execute(vec![CommandArgument::Positioned(
                0,
                CommandArgumentValue::I64(1)
            )]),
        ]),
        fragments
    );

    let lexer = CommandLexer::new("float 1.3");
    let fragments: Result<Vec<_>, _> = transformer.transform(lexer).collect();
    assert_eq!(
        Ok(vec![
            CommandFragment::Select("float".to_string()),
            CommandFragment::Execute(vec![CommandArgument::Positioned(
                0,
                CommandArgumentValue::F64(1.3)
            )]),
        ]),
        fragments
    );

    let lexer = CommandLexer::new("string aaa");
    let fragments: Result<Vec<_>, _> = transformer.transform(lexer).collect();
    assert_eq!(
        Ok(vec![
            CommandFragment::Select("string".to_string()),
            CommandFragment::Execute(vec![CommandArgument::Positioned(
                0,
                CommandArgumentValue::String("aaa".to_string())
            )]),
        ]),
        fragments
    );

    let lexer = CommandLexer::new("greedy aa 1 a= cq \" wa");
    let fragments: Result<Vec<_>, _> = transformer.transform(lexer).collect();
    assert_eq!(
        Ok(vec![
            CommandFragment::Select("greedy".to_string()),
            CommandFragment::Execute(vec![CommandArgument::Positioned(
                0,
                CommandArgumentValue::String("aa 1 a= cq \" wa".to_string())
            )]),
        ]),
        fragments
    );
}
