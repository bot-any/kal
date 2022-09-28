use kal::{Command, CommandArgument, CommandArgumentValue, CommandFragment};
use pretty_assertions::assert_eq;

#[test]
fn just_execute() {
    #[derive(Command, Debug, PartialEq)]
    struct Basic;

    assert_eq!(Ok(Basic), Basic::parse(&[CommandFragment::Execute(vec![])]));
}

#[test]
fn execute_over_subcommand() {
    #[derive(Command, Debug, PartialEq)]
    struct A;

    #[derive(Command, Debug, PartialEq)]
    enum B {
        A(A),
    }

    #[derive(Command, Debug, PartialEq)]
    enum C {
        B(B),
    }

    #[derive(Command, Debug, PartialEq)]
    enum D {
        C(C),
    }

    assert_eq!(
        Ok(D::C(C::B(B::A(A)))),
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
    struct A {
        string_value: String,
        integer_value: i64,
        float_value: f64,
    }

    assert_eq!(
        Ok(A {
            string_value: "string".to_string(),
            integer_value: 64,
            float_value: 3.141592
        }),
        A::parse(&[CommandFragment::Execute(vec![
            CommandArgument::Named(
                "string-value".to_string(),
                CommandArgumentValue::String("string".to_string())
            ),
            CommandArgument::Named(
                "float-value".to_string(),
                CommandArgumentValue::F64(3.141592)
            ),
            CommandArgument::Named("integer-value".to_string(), CommandArgumentValue::I64(64)),
        ]),])
    );
}
