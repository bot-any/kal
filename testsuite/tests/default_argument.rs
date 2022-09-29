use kal::{Command, CommandFragment};

#[test]
pub fn basic_default_argument() {
    fn function_call() -> String {
        "hello".to_string()
    }

    #[derive(Command, Debug, PartialEq)]
    pub struct Basic {
        #[argument(default = "0")]
        a: i64,

        #[argument(default = "function_call()")]
        b: String,

        #[argument(default = "3.141592 * 2.0")]
        c: f64,
    }

    assert_eq!(
        Basic::parse(&[CommandFragment::Execute(vec![])]),
        Ok(Basic {
            a: 0,
            b: "hello".to_string(),
            c: 6.283184,
        })
    );
}
