use std::{collections::HashMap, iter::FromIterator};

use kal::{
    lex::{
        remove_leading, remove_trailing, CommandLexer, TokenTransformer, TransformHint,
        TransformHintPart, TransformHintPartKind,
    },
    CommandArgument, CommandArgumentValue, CommandFragment,
};

#[test]
fn transform() {
    let transformer = TokenTransformer::command_group(
        |s| remove_leading("/", s).map(|s| remove_trailing("@my_bot", s).unwrap_or(s)),
        TransformHint::Select({
            HashMap::from_iter([(
                "hello",
                TransformHint::Select(HashMap::from_iter([(
                    "world",
                    TransformHint::Execute(vec![]),
                )])),
            )])
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
            HashMap::from_iter([
                (
                    "int",
                    TransformHint::Execute(vec![TransformHintPart {
                        multiple: false,
                        kind: TransformHintPartKind::Integer,
                    }]),
                ),
                (
                    "float",
                    TransformHint::Execute(vec![TransformHintPart {
                        multiple: false,
                        kind: TransformHintPartKind::Float,
                    }]),
                ),
                (
                    "string",
                    TransformHint::Execute(vec![TransformHintPart {
                        multiple: false,
                        kind: TransformHintPartKind::String,
                    }]),
                ),
                (
                    "greedy",
                    TransformHint::Execute(vec![TransformHintPart {
                        multiple: false,
                        kind: TransformHintPartKind::StringGreedy,
                    }]),
                ),
            ])
        }),
    );

    for (command, result) in [
        (
            "int 1",
            Ok(vec![
                CommandFragment::Select("int".to_string()),
                CommandFragment::Execute(vec![CommandArgument::Positioned(
                    0,
                    CommandArgumentValue::I64(1),
                )]),
            ]),
        ),
        (
            "int i=1",
            Ok(vec![
                CommandFragment::Select("int".to_string()),
                CommandFragment::Execute(vec![CommandArgument::Named(
                    "i".to_string(),
                    CommandArgumentValue::I64(1),
                )]),
            ]),
        ),
        (
            "float 1.3",
            Ok(vec![
                CommandFragment::Select("float".to_string()),
                CommandFragment::Execute(vec![CommandArgument::Positioned(
                    0,
                    CommandArgumentValue::F64(1.3),
                )]),
            ]),
        ),
        (
            "float f=1.3",
            Ok(vec![
                CommandFragment::Select("float".to_string()),
                CommandFragment::Execute(vec![CommandArgument::Named(
                    "f".to_string(),
                    CommandArgumentValue::F64(1.3),
                )]),
            ]),
        ),
        (
            "string aaa",
            Ok(vec![
                CommandFragment::Select("string".to_string()),
                CommandFragment::Execute(vec![CommandArgument::Positioned(
                    0,
                    CommandArgumentValue::String("aaa".to_string()),
                )]),
            ]),
        ),
        (
            "greedy aa 1 a= cq \" wa",
            Ok(vec![
                CommandFragment::Select("greedy".to_string()),
                CommandFragment::Execute(vec![CommandArgument::Positioned(
                    0,
                    CommandArgumentValue::String("aa 1 a= cq \" wa".to_string()),
                )]),
            ]),
        ),
    ] {
        let lexer = CommandLexer::new(command);
        let fragments: Result<Vec<_>, _> = transformer.transform(lexer).collect();
        assert_eq!(result, fragments);
    }
}
