use kal::{Command, CommandArgument, CommandArgumentValue, CommandFragment};

#[test]
pub fn multiple() {
    #[derive(Command, Debug, PartialEq)]
    #[allow(dead_code)]
    pub struct Command {
        #[argument(take_rest)]
        pub arguments: Vec<i64>,
    }

    assert_eq!(
        Ok(Command {
            arguments: vec![0, 1, 2],
        }),
        Command::parse(&[CommandFragment::Execute(vec![CommandArgument::Positioned(
            0,
            CommandArgumentValue::String("0 1 2".to_string())
        ),])])
    )
}
