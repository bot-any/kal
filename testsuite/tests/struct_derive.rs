use kal::{Command, CommandOption, CommandOptionValueKind, CommandSpec};
use pretty_assertions::assert_eq;

#[test]
fn basic_struct() {
    /// basic struct
    #[derive(Command)]
    #[command(name = "basic")]
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
    /// basic struct
    #[derive(Command)]
    #[command(name = "basic")]
    struct Test {
        /// String
        #[argument(name = "s")]
        _s: String,

        /// i64
        #[argument(name = "i")]
        _i: i64,

        /// f64
        #[argument(name = "f")]
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
    /// basic struct
    #[derive(Command)]
    #[command(name = "basic")]
    struct Test {
        /// String
        #[argument(name = "s")]
        _s: Option<String>,

        /// i64
        #[argument(name = "i")]
        _i: Option<i64>,

        /// f64
        #[argument(name = "f")]
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
