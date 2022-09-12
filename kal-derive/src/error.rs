use std::{fmt::Display, result::Result as StdResult};

use proc_macro::TokenStream;
use quote::ToTokens;

pub enum Error {
    Syn(syn::Error),
    Darling(darling::Error),
}

impl Error {
    pub fn new<T: ToTokens, D: Display>(tokens: T, message: D) -> Self {
        Error::Syn(syn::Error::new_spanned(tokens, message))
    }
}

impl From<Error> for TokenStream {
    fn from(error: Error) -> Self {
        match error {
            Error::Syn(e) => TokenStream::from(e.into_compile_error()),
            Error::Darling(e) => TokenStream::from(e.write_errors()),
        }
    }
}

impl From<syn::Error> for Error {
    fn from(e: syn::Error) -> Self {
        Error::Syn(e)
    }
}

impl From<darling::Error> for Error {
    fn from(e: darling::Error) -> Self {
        Error::Darling(e)
    }
}

pub type Result<T> = StdResult<T, Error>;
