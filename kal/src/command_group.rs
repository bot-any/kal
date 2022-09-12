/// Make an enum implementing [`Command`](`crate::Command`)
/// where its every variant have only one field implements [`Commnad`](`crate::Command`).
/// The enum will provide a parse function matching each variant's name directly.
///
/// Example:
/// ```rust
/// # use kal::{Command, command_group, CommandFragment};
/// # #[derive(Debug, PartialEq)]
/// #[derive(Command)]
/// #[command(name = "a", description = "")]
/// struct A;
///
/// # #[derive(Debug, PartialEq)]
/// #[derive(Command)]
/// #[command(name = "b", description = "")]
/// struct B;
///
///
/// command_group! {
/// #   #[derive(Debug, PartialEq)]
///     enum Root {
///         A(A),
///         B(B)
///     }
/// }
///
/// assert_eq!(Some(Root::A(A)), Root::parse(&[CommandFragment::Select("a".to_string()), CommandFragment::Execute(vec![])]));
/// assert_eq!(Some(Root::B(B)), Root::parse(&[CommandFragment::Select("b".to_string()), CommandFragment::Execute(vec![])]));
/// assert_eq!(None, Root::parse(&[CommandFragment::Select("c".to_string()), CommandFragment::Execute(vec![])]));
#[macro_export]
macro_rules! command_group {
    (
        $(#[$attrs:meta])*
        $vis:vis enum $name:ident {
            $(
                $variant:ident($path:path)
            ),*
        }
    ) => {
        $(#[$attrs])*
        $vis enum $name {
            $($variant($path)),*
        }

        impl $name {
            pub fn contains(name: &str) -> bool {
                ::std::matches!(name, $(<$path as ::kal::Command>::NAME)|*)
            }

            pub fn children_specs() -> ::std::vec::Vec<::kal::CommandSpec> {
                ::std::vec![$(<$path as ::kal::Command>::spec()),*]
            }
        }

        impl ::kal::Command for $name {
            const NAME: &'static str = "<root>";

            fn spec() -> ::kal::CommandSpec {
                ::kal::CommandSpec {
                    name: Self::NAME,
                    description: "",
                    options: ::std::vec::Vec::new(),
                    subcommands: ::std::vec![
                        $(
                            <$path as ::kal::Command>::spec(),
                        )*
                    ],
                }
            }

            fn parse(fragments: &[::kal::CommandFragment]) -> ::std::option::Option<Self> {
                match fragments {
                    [::kal::CommandFragment::Select(name), rest @ ..] => {
                        match name.as_str() {
                            $(
                                <$path as ::kal::Command>::NAME =>
                                    <$path as ::kal::Command>::parse(rest).map($name::$variant),
                            )*
                            _ => ::std::option::Option::None,
                        }
                    },
                    _ => ::std::option::Option::None,
                }
            }
        }
    }
}
