use kal::{Command, CommandOption, CommandOptionValueKind, CommandSpec};
use pretty_assertions::assert_eq;

#[test]
fn basic_enum() {
    /// basic struct
    #[derive(Command)]
    enum Basic {
        /// a
        A,

        /// b
        B,
    }

    assert_eq!(
        Basic::spec(),
        CommandSpec {
            name: "basic",
            description: "basic struct",
            options: vec![],
            subcommands: vec![
                CommandSpec {
                    name: "a",
                    description: "a",
                    options: vec![],
                    subcommands: vec![],
                },
                CommandSpec {
                    name: "b",
                    description: "b",
                    options: vec![],
                    subcommands: vec![],
                },
            ],
        }
    );
}
#[test]
fn struct_in_enum() {
    /// a
    #[derive(Command)]
    struct A;

    /// test struct
    #[derive(Command)]
    enum Test {
        A(A),

        /// b
        B,
    }

    assert_eq!(
        Test::spec(),
        CommandSpec {
            name: "test",
            description: "test struct",
            options: vec![],
            subcommands: vec![
                CommandSpec {
                    name: "a",
                    description: "a",
                    options: vec![],
                    subcommands: vec![],
                },
                CommandSpec {
                    name: "b",
                    description: "b",
                    options: vec![],
                    subcommands: vec![],
                },
            ],
        }
    );
}
#[test]
fn struct_style_enum() {
    /// a
    #[derive(Command)]
    struct A {
        /// s
        s: String,
    }

    /// test struct
    #[derive(Command)]
    enum Test {
        A(A),

        /// b
        B {
            /// s
            s: String,
        },
    }

    assert_eq!(
        Test::spec(),
        CommandSpec {
            name: "test",
            description: "test struct",
            options: vec![],
            subcommands: vec![
                CommandSpec {
                    name: "a",
                    description: "a",
                    options: vec![CommandOption {
                        name: "s",
                        position: 0,
                        description: "s",
                        value: CommandOptionValueKind::String,
                    },],
                    subcommands: vec![],
                },
                CommandSpec {
                    name: "b",
                    description: "b",
                    options: vec![CommandOption {
                        name: "s",
                        position: 0,
                        description: "s",
                        value: CommandOptionValueKind::String,
                    },],
                    subcommands: vec![],
                },
            ],
        }
    );
}
