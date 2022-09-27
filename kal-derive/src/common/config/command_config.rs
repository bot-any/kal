use darling::{FromDeriveInput, FromVariant};
use quote::ToTokens;

use crate::common::error::Error;

#[derive(FromDeriveInput, FromVariant)]
#[darling(attributes(command))]
pub struct CommandConfig {
    name: Option<String>,

    #[darling(rename = "self")]
    pub for_self: Option<bool>,
}

impl CommandConfig {
    pub fn name_or_error_from<T: ToTokens>(&self, tokens: T) -> Result<&String, Error> {
        self.name.as_ref().ok_or_else(|| {
            syn::Error::new_spanned(tokens, "#[command] attribute requires a name").into()
        })
    }
}
