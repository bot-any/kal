use kal::{Command, CommandOption, CommandOptionValueKind, CommandSpec};
use pretty_assertions::assert_eq;

#[test]
fn basic_struct() {
    /// basic struct
    #[derive(Command)]
    struct Basic;

    assert_eq!(
        Basic::spec(),
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
    /// test struct
    #[derive(Command)]
    #[allow(dead_code)]
    struct Test {
        /// String
        s: String,

        /// i64
        i: i64,

        /// f64
        f: f64,
    }

    assert_eq!(
        Test::spec(),
        CommandSpec {
            name: "test",
            description: "test struct",
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
    /// test struct
    #[derive(Command)]
    #[allow(dead_code)]
    struct Test {
        /// String
        s: Option<String>,

        /// i64
        i: Option<i64>,

        /// f64
        f: Option<f64>,
    }

    assert_eq!(
        Test::spec(),
        CommandSpec {
            name: "test",
            description: "test struct",
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
