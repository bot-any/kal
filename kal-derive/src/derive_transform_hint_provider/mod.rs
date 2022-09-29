use darling::{FromField, FromVariant};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields};

use crate::common::{
    codegen::command_option_codegen::{CommandOption, CommandOptionsExt},
    config::{argument_config::ArgumentConfig, command_config::CommandConfig},
    doc_string::join_doc_string,
    error::{self, Error},
};

pub fn actual_derive_transform_hint(derive_input: DeriveInput) -> error::Result<TokenStream> {
    let name = derive_input.ident;

    let mut options = Vec::new();
    let mut subcommands = Vec::new();

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
                            let transform_hint_vec = inner_options.make_transform_hint_vec();

                            subcommands.push((
                                quote! { #command_name },
                                quote! { ::kal::lex::TransformHint::Execute(#transform_hint_vec) },
                            ));
                        }
                    }
                    Fields::Unnamed(fields) => {
                        if fields.unnamed.len() != 1 {
                            return Err(Error::new(
                                fields,
                                "Unnamed enum variant must have one field",
                            ));
                        }
                        let ty = &fields.unnamed[0].ty;
                        subcommands.push((
                            quote! { <#ty as ::kal::Command>::NAME },
                            quote! { <#ty as ::kal::lex::TransformHintProvider>::hint() },
                        ));
                    }
                    Fields::Unit => {
                        let command_config = command_config?;

                        let command_name = command_config.rename_or(&variant.ident);
                        subcommands
                            .push((quote! { #command_name }, quote! { ::std::vec::Vec::new() }));
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

    let transform_hint_vec = if options.is_empty() {
        None
    } else {
        Some(options.make_transform_hint_vec())
    };
    let subcommand_select = if subcommands.is_empty() {
        None
    } else {
        let (name, value): (Vec<_>, Vec<_>) = subcommands.into_iter().unzip();
        Some(quote! {
            {
                ::std::collections::HashMap::from_iter(
                    [#((#name, #value)),*]
                )
            }
        })
    };

    let hint_variant = match (transform_hint_vec, subcommand_select) {
        (None, None) => quote! { ::kal::lex::TransformHint::Execute(::std::vec::Vec::new()) },
        (None, Some(subcommand_select)) => {
            quote! { ::kal::lex::TransformHint::Select(#subcommand_select) }
        }
        (Some(transform_hint_vec), None) => {
            quote! { ::kal::lex::TransformHint::Execute(#transform_hint_vec) }
        }
        (Some(transform_hint_vec), Some(subcommand_select)) => {
            quote! { ::kal::lex::TransformHint::SelectOrExecute(#subcommand_select, #transform_hint_vec) }
        }
    };

    Ok(quote! {
        impl ::kal::lex::TransformHintProvider for #name {
            fn hint() -> ::kal::lex::TransformHint {
                #hint_variant
            }
        }
    }
    .into())
}
