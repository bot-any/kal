use kal::{Command, CommandArgument, CommandArgumentValue, CommandFragment};

#[test]
fn just_execute() {
    #[derive(Command, Debug, PartialEq)]
    #[command(name = "basic", description = "basic struct")]
    struct Basic;

    assert_eq!(
        Some(Basic),
        Basic::parse(&[CommandFragment::Execute(vec![])])
    );
}

#[test]
fn execute_over_subcommand() {
    #[derive(Command, Debug, PartialEq)]
    #[command(name = "a", description = "")]
    struct A;
    #[derive(Command, Debug, PartialEq)]
    #[command(name = "b", description = "")]
    enum B {
        A(A),
    }
    #[derive(Command, Debug, PartialEq)]
    #[command(name = "c", description = "")]
    enum C {
        B(B),
    }
    #[derive(Command, Debug, PartialEq)]
    #[command(name = "d", description = "")]
    enum D {
        C(C),
    }

    assert_eq!(
        Some(D::C(C::B(B::A(A)))),
        D::parse(&[
            CommandFragment::Select("c".to_string()),
            CommandFragment::Select("b".to_string()),
            CommandFragment::Select("a".to_string()),
            CommandFragment::Execute(vec![])
        ])
    );
}

#[test]
fn optionful() {
    #[derive(Command, Debug, PartialEq)]
    #[command(name = "a", description = "")]
    struct A {
        #[argument(name = "s", description = "String")]
        s: String,
        #[argument(name = "i", description = "i64")]
        i: i64,
        #[argument(name = "f", description = "f64")]
        f: f64,
    }

    assert_eq!(
        Some(A {
            s: "string".to_string(),
            i: 64,
            f: 3.141592
        }),
        A::parse(&[CommandFragment::Execute(vec![
            CommandArgument::Named(
                "s".to_string(),
                CommandArgumentValue::String("string".to_string())
            ),
            CommandArgument::Named("f".to_string(), CommandArgumentValue::F64(3.141592)),
            CommandArgument::Named("i".to_string(), CommandArgumentValue::I64(64)),
        ]),])
    );
}
