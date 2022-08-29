#[macro_export]
macro_rules! command_group {
    ($name:ident { $($children:ident,)* }) => {
        pub enum $name {
            $($children($children)),*
        }

        impl $name {
            pub fn contains(name: &str) -> bool {
                ::std::matches!(name, $(<$children as ::kal::Command>::NAME)|*)
            }

            pub fn children_specs() -> ::std::vec::Vec<::kal::CommandSpec> {
                ::std::vec![$(<$children as ::kal::Command>::spec()),*]
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
                            <$children as ::kal::Command>::spec(),
                        )*
                    ],
                }
            }

            fn parse(fragments: &[::kal::CommandFragment]) -> ::std::option::Option<Self> {
                match fragments {
                    [::kal::CommandFragment::Select(name), rest @ ..] => {
                        match name.as_str() {
                            $(
                                <$children as ::kal::Command>::NAME =>
                                    <$children as ::kal::Command>::parse(rest).map($name::$children),
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
