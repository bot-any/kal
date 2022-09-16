use derive_command::actual_derive_command;
use derive_transform_hint_provider::actual_derive_transform_hint;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod common;
mod derive_command;
mod derive_transform_hint_provider;

/// Derive Command trait from kal for a struct or an enum.
#[proc_macro_derive(Command, attributes(command, argument))]
pub fn derive_command(item: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(item as DeriveInput);

    match actual_derive_command(derive_input) {
        Ok(stream) => stream,
        Err(error) => TokenStream::from(error),
    }
}

#[cfg(feature = "lex")]
/// Derive TransformHintProvider trait from kal for a struct or an enum.
#[proc_macro_derive(TransformHintProvider, attributes(command, argument))]
pub fn derive_transform_hint(item: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(item as DeriveInput);

    match actual_derive_transform_hint(derive_input) {
        Ok(stream) => stream,
        Err(error) => TokenStream::from(error),
    }
}
