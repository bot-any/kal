use proc_macro::TokenStream;
use syn::DeriveInput;

use crate::error;

pub fn actual_derive_transform_hint(derive_input: DeriveInput) -> error::Result<TokenStream> {
    todo!()
}
