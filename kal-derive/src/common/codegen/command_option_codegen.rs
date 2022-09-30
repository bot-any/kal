use quote::{format_ident, quote, ToTokens};
use syn::{Expr, Ident, Type};

pub struct CommandOption {
    pub ident: Ident,
    pub name: String,
    pub position: usize,
    pub description: String,
    pub ty: Type,
    pub take_rest: bool,
    pub default: Option<Expr>,
}

impl CommandOption {
    pub fn declaration(&self) -> quote::__private::TokenStream {
        let Self {
            ident, ty, default, ..
        } = self;
        let ident = format_ident!("{}_field", ident);
        let default = default
            .as_ref()
            .map(|expr| quote! { Some(#expr) })
            .unwrap_or(quote! { <#ty as ::kal::CommandOptionValueTy>::default() });
        quote! {
            let mut #ident: ::std::option::Option<#ty> = #default;
        }
    }

    pub fn kal_option(&self) -> quote::__private::TokenStream {
        let Self {
            name,
            position,
            description,
            ty,
            ..
        } = self;
        quote! {
            ::kal::CommandOption {
                name: #name,
                position: #position,
                description: #description,
                value: <#ty as ::kal::CommandOptionValueTy>::spec_kind(),
            }
        }
    }

    pub fn match_arms(&self) -> (quote::__private::TokenStream, quote::__private::TokenStream) {
        let Self {
            ident,
            name,
            position,
            ty,
            ..
        } = self;
        let ident = format_ident!("{}_field", ident);
        let value = quote! {
            <#ty as kal::TryFromArgumentValue>::try_from_argument_value(value.clone(), true).ok()
        };
        let assignment = quote! { #ident = #value };
        (
            quote! { #name => #assignment },
            quote! { #position => #assignment },
        )
    }

    pub fn check_missed<T: ToTokens>(&self, collection_name: T) -> quote::__private::TokenStream {
        let Self { ident, name, .. } = self;
        let ident = format_ident!("{}_field", ident);
        quote! {
            if #ident.is_none() {
                #collection_name.push(#name);
            }
        }
    }

    pub fn transform_hint_part(&self) -> quote::__private::TokenStream {
        let Self { ty, take_rest, .. } = self;
        let make_greedy = if *take_rest {
            quote! { .make_greedy() }
        } else {
            quote! {}
        };
        quote! { ::kal::lex::TransformHintPart::from(<#ty as ::kal::CommandOptionValueTy>::spec_kind()) #make_greedy }
    }
}

pub trait CommandOptionsExt {
    fn build_struct<T: ToTokens, U: ToTokens>(
        &self,
        name: T,
        missing_arguments: U,
    ) -> quote::__private::TokenStream;

    fn make_execute_work<T: ToTokens>(&self, name: T) -> quote::__private::TokenStream;

    fn make_transform_hint_vec(&self) -> quote::__private::TokenStream;
}

impl CommandOptionsExt for Vec<CommandOption> {
    fn build_struct<T: ToTokens, U: ToTokens>(
        &self,
        name: T,
        missing_arguments: U,
    ) -> quote::__private::TokenStream {
        let idents: Vec<_> = self.iter().map(|option| &option.ident).collect();
        let idents_field: Vec<_> = self
            .iter()
            .map(|option| format_ident!("{}_field", option.ident))
            .collect();

        quote! {
            match (#(#idents_field),*) {
                (#(::std::option::Option::Some(#idents)),*) => {
                    ::std::result::Result::Ok(#name {
                        #(#idents),*
                    })
                }
                _ => ::std::result::Result::Err(::kal::CommandParseError::MissingArguments(#missing_arguments)),
            }
        }
    }

    fn make_execute_work<T: ToTokens>(&self, name: T) -> quote::__private::TokenStream {
        let options_declaration: Vec<_> = self.iter().map(|opt| opt.declaration()).collect();
        let (options_match_arm_named, options_match_arm_positioned): (Vec<_>, Vec<_>) =
            self.iter().map(|opt| opt.match_arms()).unzip();
        let options_check_missed: Vec<_> = self
            .iter()
            .map(|opt| {
                opt.check_missed(quote! {
                    missing_arguments
                })
            })
            .collect();
        let options_build_struct = self.build_struct(
            name,
            quote! {
                missing_arguments
            },
        );

        quote! {
            {
                #(#options_declaration)*

                let mut missing_arguments = Vec::new();

                for argument in arguments {
                    match argument {
                        ::kal::CommandArgument::Named(name, value) => {
                            match name.as_str() {
                                #(#options_match_arm_named,)*
                                _ => continue
                            }
                        }
                        ::kal::CommandArgument::Positioned(position, value) => {
                            match position {
                                #(#options_match_arm_positioned,)*
                                _ => continue
                            }
                        }
                    }
                }

                #(#options_check_missed)*

                #options_build_struct
            }
        }
    }

    fn make_transform_hint_vec(&self) -> quote::__private::TokenStream {
        let parts: Vec<_> = self.iter().map(|opt| opt.transform_hint_part()).collect();

        quote! {
            vec![#(#parts),*]
        }
    }
}
