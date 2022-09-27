use kal::{Command, CommandOption, CommandOptionValueKind, CommandSpec};

#[test]
fn basic_enum() {
    /// basic struct
    #[derive(Command)]
    #[command(name = "basic")]
    enum Test {
        /// a
        #[command(name = "a")]
        _A,
        
        /// b
        #[command(name = "b")]
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
    /// a
    #[derive(Command)]
    #[command(name = "a")]
    struct A;

    /// basic struct
    #[derive(Command)]
    #[command(name = "basic")]
    enum Test {
        A(A),

        /// b
        #[command(name = "b")]
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
    /// a
    #[derive(Command)]
    #[command(name = "a")]
    struct A {
        /// s
        #[argument(name = "s")]
        _s: String,
    }
    
    /// basic struct
    #[derive(Command)]
    #[command(name = "basic")]
    enum Test {
        A(A),

        /// b
        #[command(name = "b")]
        B {
            /// s
            #[argument(name = "s")]
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
