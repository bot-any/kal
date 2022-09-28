use darling::FromField;
use syn::Ident;

use crate::common::renamer::rename_to_kebab_case;

#[derive(FromField)]
#[darling(attributes(argument))]
pub struct ArgumentConfig {
    rename: Option<String>,
    #[darling(default)]
    pub take_rest: bool,
}
impl ArgumentConfig {
    pub fn rename_or(&self, ident: &Ident) -> String {
        self.rename
            .clone()
            .unwrap_or(rename_to_kebab_case(ident.to_string()))
    }
}
