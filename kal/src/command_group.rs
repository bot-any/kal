#[macro_export]
macro_rules! command_group {
    (
        $vis:vis? enum $name:ident {
            $(
                $variant:ident($path:path)
            ),*
        }
    ) => {
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
