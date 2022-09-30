use darling::{FromDeriveInput, FromVariant};
use syn::Ident;

use crate::common::renamer::rename_to_kebab_case;

#[derive(FromDeriveInput, FromVariant)]
#[darling(attributes(command))]
pub struct CommandConfig {
    rename: Option<String>,

    #[darling(rename = "self")]
    pub for_self: Option<bool>,
}

impl CommandConfig {
    pub fn rename_or(&self, ident: &Ident) -> String {
        self.rename
            .clone()
            .unwrap_or_else(|| rename_to_kebab_case(ident.to_string()))
    }
}
