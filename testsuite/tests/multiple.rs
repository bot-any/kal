use kal::{
    CommaSeparated, Command, CommandArgument, CommandArgumentValue, CommandFragment, SpaceSeparated,
};

#[test]
pub fn multiple_f64() {
    #[derive(Command, Debug, PartialEq)]
    #[allow(dead_code)]
    pub struct Command {
        #[argument(take_rest)]
        pub arguments: SpaceSeparated<i64>,
    }

    assert_eq!(
        Ok(Command {
            arguments: SpaceSeparated(vec![0, 1, 2]),
        }),
        Command::parse(&[CommandFragment::Execute(vec![CommandArgument::Positioned(
            0,
            CommandArgumentValue::String("0 1 2".to_string())
        ),])])
    )
}
#[test]
pub fn multiple_word() {
    #[derive(Command, Debug, PartialEq)]
    #[allow(dead_code)]
    pub struct Command {
        #[argument(take_rest)]
        pub arguments: CommaSeparated<String>,
    }

    assert_eq!(
        Ok(Command {
            arguments: CommaSeparated(vec!["0".to_string(), "1".to_string(), "2".to_string()]),
        }),
        Command::parse(&[CommandFragment::Execute(vec![CommandArgument::Positioned(
            0,
            CommandArgumentValue::String("0,1,2".to_string())
        ),])])
    )
}
