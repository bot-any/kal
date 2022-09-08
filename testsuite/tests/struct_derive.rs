use kal::{Command, CommandOption, CommandOptionValueKind, CommandSpec};

#[test]
fn basic_struct() {
    #[derive(Command)]
    #[command(name = "basic", description = "basic struct")]
    struct Test;

    assert_eq!(
        Test::spec(),
        CommandSpec {
            name: "basic",
            description: "basic struct",
            options: vec![],
            subcommands: vec![],
        }
    );
}

#[test]
fn required_arguments() {
    #[derive(Command)]
    #[command(name = "basic", description = "basic struct")]
    struct Test {
        #[argument(name = "s", description = "String")]
        _s: String,
        #[argument(name = "i", description = "i64")]
        _i: i64,
        #[argument(name = "f", description = "f64")]
        _f: f64,
    }

    assert_eq!(
        Test::spec(),
        CommandSpec {
            name: "basic",
            description: "basic struct",
            options: vec![
                CommandOption {
                    name: "s",
                    position: 0,
                    description: "String",
                    value: CommandOptionValueKind::String,
                },
                CommandOption {
                    name: "i",
                    position: 1,
                    description: "i64",
                    value: CommandOptionValueKind::Integer,
                },
                CommandOption {
                    name: "f",
                    position: 2,
                    description: "f64",
                    value: CommandOptionValueKind::Double,
                }
            ],
            subcommands: vec![],
        }
    );
}

#[test]
fn optional_arguments() {
    #[derive(Command)]
    #[command(name = "basic", description = "basic struct")]
    struct Test {
        #[argument(name = "s", description = "String")]
        _s: Option<String>,
        #[argument(name = "i", description = "i64")]
        _i: Option<i64>,
        #[argument(name = "f", description = "f64")]
        _f: Option<f64>,
    }

    assert_eq!(
        Test::spec(),
        CommandSpec {
            name: "basic",
            description: "basic struct",
            options: vec![
                CommandOption {
                    name: "s",
                    position: 0,
                    description: "String",
                    value: CommandOptionValueKind::Optional(Box::new(
                        CommandOptionValueKind::String
                    )),
                },
                CommandOption {
                    name: "i",
                    position: 1,
                    description: "i64",
                    value: CommandOptionValueKind::Optional(Box::new(
                        CommandOptionValueKind::Integer
                    )),
                },
                CommandOption {
                    name: "f",
                    position: 2,
                    description: "f64",
                    value: CommandOptionValueKind::Optional(Box::new(
                        CommandOptionValueKind::Double
                    )),
                }
            ],
            subcommands: vec![],
        }
    );
}
