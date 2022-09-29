//! # kal-derive
//!
//! Provide conveinence `#[derive(Command)]` macros for [kal](https://crates.io/crates/kal).

#![deny(missing_docs)]

use crate::common::{
    codegen::command_option_codegen::{CommandOption, CommandOptionsExt},
    config::{argument_config::ArgumentConfig, command_config::CommandConfig},
    doc_string::join_doc_string,
    error::{self, Error},
};
use darling::{FromDeriveInput, FromField, FromVariant};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields};

pub fn actual_derive_command(derive_input: DeriveInput) -> error::Result<TokenStream> {
    let root_command_config = CommandConfig::from_derive_input(&derive_input)?;
    let root_command_name = root_command_config.rename_or(&derive_input.ident);
    let root_command_description = join_doc_string(&derive_input.attrs);

    let name = derive_input.ident;

    let mut options = Vec::new();

    let mut subcommands = Vec::new();
    let mut subcommands_named_fields_match_arms = Vec::new();
    let mut subcommand_match_arms = Vec::new();

    let mut self_discovered = Vec::new();

    match derive_input.data {
        syn::Data::Struct(data) => {
            self_discovered.push(quote! {
                #name
            });
            for field in data.fields {
                let argument_config = ArgumentConfig::from_field(&field)?;
                let field_ident = field
                    .ident
                    .clone()
                    .ok_or_else(|| Error::new(&field, "enum variant field must have a name"))?;
                let argument_name = argument_config.rename_or(&field_ident);
                let argument_description = join_doc_string(&field.attrs);

                options.push(CommandOption {
                    ident: field_ident,
                    name: argument_name,
                    position: options.len(),
                    description: argument_description,
                    ty: field.ty,
                    take_rest: argument_config.take_rest,
                    default: argument_config.default,
                });
            }
        }
        syn::Data::Enum(data) => {
            for variant in data.variants {
                let command_config = CommandConfig::from_variant(&variant);

                match variant.fields {
                    Fields::Named(fields) => {
                        let mut inner_options = Vec::new();
                        for field in fields.named {
                            let argument_config = ArgumentConfig::from_field(&field)?;
                            let ident = field.ident.clone().ok_or_else(|| {
                                Error::new(&field, "enum variant field must have a name")
                            })?;

                            let argument_name = argument_config.rename_or(&ident);
                            let argument_description = join_doc_string(&field.attrs);
                            inner_options.push(CommandOption {
                                ident,
                                name: argument_name,
                                position: inner_options.len(),
                                description: argument_description,
                                ty: field.ty,
                                take_rest: argument_config.take_rest,
                                default: argument_config.default,
                            });
                        }

                        let command_config = command_config?;
                        let variant_ident = variant.ident;
                        let variant_full_name = quote! { #name::#variant_ident };

                        if command_config.for_self.unwrap_or(false) {
                            self_discovered.push(variant_full_name);
                            options = inner_options;
                        } else {
                            let command_name = command_config.rename_or(&variant_ident);
                            let command_description = join_doc_string(&variant.attrs);

                            let inner_options_kal: Vec<_> =
                                inner_options.iter().map(|opt| opt.kal_option()).collect();
                            subcommands.push(quote! {
                                ::kal::CommandSpec {
                                    name: #command_name,
                                    description: #command_description,
                                    options: ::std::vec![#(#inner_options_kal),*],
                                    subcommands: ::std::vec::Vec::new(),
                                }
                            });

                            let inner_options_execute_work =
                                inner_options.make_execute_work(variant_full_name);
                            subcommands_named_fields_match_arms.push(quote! {
                                #command_name => #inner_options_execute_work
                            });
                        }
                    }
                    Fields::Unnamed(fields) => {
                        if fields.unnamed.len() != 1 {
                            return Err(Error::new(
                                fields,
                                "Unnamed enum variant must have one field",
                            ));
                        }
                        let variant_name = variant.ident;
                        let ty = &fields.unnamed[0].ty;
                        subcommand_match_arms.push(quote! {
                            <#ty as ::kal::Command>::NAME => {
                                <#ty as ::kal::Command>::parse(rest).map(#name::#variant_name)
                            }
                        });
                        subcommands.push(quote! {
                            <#ty as ::kal::Command>::spec()
                        });
                    }
                    Fields::Unit => {
                        let command_config = command_config?;

                        let command_name = command_config.rename_or(&variant.ident);
                        let command_description = join_doc_string(&variant.attrs);
                        subcommands.push(quote! {
                            ::kal::CommandSpec {
                                name: #command_name,
                                description: #command_description,
                                options: ::std::vec::Vec::new(),
                                subcommands: ::std::vec::Vec::new(),
                            }
                        })
                    }
                };
            }
        }
        syn::Data::Union(data) => {
            return Err(Error::new(
                data.union_token,
                "Cannot derive Command for union",
            ))
        }
    };

    if self_discovered.len() > 1 {
        return Err(Error::new(
            &self_discovered[0],
            "Cannot set #[command(self)] more than once",
        ));
    }

    let options_kal: Vec<_> = options.iter().map(|opt| opt.kal_option()).collect();

    let self_arm = if let Some(self_token) = self_discovered.first() {
        let work = options.make_execute_work(self_token);
        quote! {
            [::kal::CommandFragment::Execute(arguments), ..] => #work
        }
    } else {
        quote! {
            [::kal::CommandFragment::Execute(arguments), ..] =>
                ::std::result::Result::Err(::kal::CommandParseError::ExecuteTooEarly)
        }
    };

    Ok(quote! {
        impl ::kal::Command for #name {
            const NAME: &'static str = #root_command_name;

            fn spec() -> ::kal::CommandSpec {
                ::kal::CommandSpec {
                    name: #root_command_name,
                    description: #root_command_description,
                    options: ::std::vec![#(#options_kal),*],
                    subcommands: ::std::vec![#(#subcommands),*],
                }
            }
            fn parse(fragments: &[::kal::CommandFragment]) -> ::std::result::Result<Self, ::kal::CommandParseError> {
                match fragments {
                    [
                        ::kal::CommandFragment::Select(name),
                        execute @ ::kal::CommandFragment::Execute(arguments),
                        ..
                    ] => {
                        let rest = ::std::slice::from_ref(execute);
                        match name.as_str() {
                            #(#subcommands_named_fields_match_arms),*
                            #(#subcommand_match_arms),*
                            _ => ::std::result::Result::Err(::kal::CommandParseError::UnknownCommand(name)),
                        }
                    }
                    [::kal::CommandFragment::Select(name), rest @ ..] => {
                        match name.as_str() {
                            #(#subcommand_match_arms),*
                            _ => ::std::result::Result::Err(::kal::CommandParseError::UnknownCommand(name)),
                        }
                    }
                    #self_arm,
                    [] => ::std::result::Result::Err(::kal::CommandParseError::IncompleteCommand),
                }
            }
        }
    }
    .into())
}
