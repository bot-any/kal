use quote::{quote, ToTokens};
use syn::{Ident, Type};

pub struct CommandOption {
    pub ident: Ident,
    pub name: String,
    pub position: usize,
    pub description: String,
    pub ty: Type,
    pub take_rest: bool,
}

impl CommandOption {
    pub fn declaration(&self) -> quote::__private::TokenStream {
        let Self { ident, ty, .. } = self;
        quote! {
            let mut #ident: ::std::option::Option<#ty> = <#ty as ::kal::CommandOptionValueTy>::default();
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

    pub fn match_arms<T: ToTokens>(
        &self,
        value: T,
    ) -> (quote::__private::TokenStream, quote::__private::TokenStream) {
        let Self {
            ident,
            name,
            position,
            ..
        } = self;
        let assignment = quote! { #ident = #value };
        (
            quote! { #name => #assignment },
            quote! { #position => #assignment },
        )
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
    fn build_struct<T: ToTokens>(&self, name: T) -> quote::__private::TokenStream;

    fn make_execute_work<T: ToTokens>(&self, name: T) -> quote::__private::TokenStream;

    fn make_transform_hint_vec(&self) -> quote::__private::TokenStream;
}

impl CommandOptionsExt for Vec<CommandOption> {
    fn build_struct<T: ToTokens>(&self, name: T) -> quote::__private::TokenStream {
        let idents: Vec<_> = self.iter().map(|option| &option.ident).collect();

        quote! {
            match (#(#idents),*) {
                (#(::std::option::Option::Some(#idents)),*) => {
                    ::std::option::Option::Some(#name {
                        #(#idents),*
                    })
                }
                _ => ::std::option::Option::None,
            }
        }
    }

    fn make_execute_work<T: ToTokens>(&self, name: T) -> quote::__private::TokenStream {
        let options_declaration: Vec<_> = self.iter().map(|opt| opt.declaration()).collect();
        let (options_match_arm_named, options_match_arm_positioned): (Vec<_>, Vec<_>) = self
            .iter()
            .map(|opt| {
                opt.match_arms(quote! {
                    ::std::option::Option::Some(value.clone().try_into().ok()?)
                })
            })
            .unzip();
        let options_build_struct = self.build_struct(name);

        quote! {
            {
                #(#options_declaration)*

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
