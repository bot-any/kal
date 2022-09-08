use kal::{Command, CommandOption, CommandOptionValueKind, CommandSpec};

#[test]
fn basic_enum() {
    #[derive(Command)]
    #[command(name = "basic", description = "basic struct")]
    enum Test {
        #[command(name = "a", description = "a")]
        _A,
        #[command(name = "b", description = "b")]
        _B,
    }

    assert_eq!(
        Test::spec(),
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
    #[derive(Command)]
    #[command(name = "a", description = "a")]
    struct A;
    #[derive(Command)]
    #[command(name = "basic", description = "basic struct")]
    enum Test {
        A(A),
        #[command(name = "b", description = "b")]
        _B,
    }

    assert_eq!(
        Test::spec(),
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
fn struct_style_enum() {
    #[derive(Command)]
    #[command(name = "a", description = "a")]
    struct A {
        #[argument(name = "s", description = "s")]
        _s: String,
    }
    #[derive(Command)]
    #[command(name = "basic", description = "basic struct")]
    enum Test {
        A(A),
        #[command(name = "b", description = "b")]
        B {
            #[argument(name = "s", description = "s")]
            _s: String,
        },
    }

    assert_eq!(
        Test::spec(),
        CommandSpec {
            name: "basic",
            description: "basic struct",
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
