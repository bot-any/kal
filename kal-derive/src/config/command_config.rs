use darling::{FromDeriveInput, FromVariant};
use quote::ToTokens;

use crate::error::Error;

#[derive(FromDeriveInput, FromVariant)]
#[darling(attributes(command))]
pub struct CommandConfig {
    name: Option<String>,
    description: Option<String>,

    #[darling(rename = "self")]
    pub for_self: Option<bool>,
}

impl CommandConfig {
    pub fn prepare<T: ToTokens + Clone>(&self, tokens: T) -> Result<(&String, &String), Error> {
        let name = self.name_or_error_from(tokens.clone())?;
        let description = self.description_or_error_from(tokens)?;

        Ok((name, description))
    }

    pub fn name_or_error_from<T: ToTokens>(&self, tokens: T) -> Result<&String, Error> {
        self.name
            .as_ref()
            .ok_or(syn::Error::new_spanned(tokens, "#[command] attribute requires a name").into())
    }

    pub fn description_or_error_from<T: ToTokens>(&self, tokens: T) -> Result<&String, Error> {
        self.description.as_ref().ok_or(
            syn::Error::new_spanned(tokens, "#[command] attribute requires a description").into(),
        )
    }
}
