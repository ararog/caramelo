#![allow(unused_variables, unused_assignments, dead_code)]
use crate::acessor::Accessor;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Brace, Dot},
    Ident, Result, Token, Type,
};

/// Parsed dry match expression
///
/// # Arguments
///
/// * `variable` - The variable to match against
/// * `brace_token` - The brace token
/// * `fields` - The fields to match against
pub struct DryMatch {
    pub(crate) variable: Option<Ident>,
    pub(crate) brace_token: Brace,
    pub(crate) fields: Punctuated<Accessor, Token![,]>,
}

impl Parse for DryMatch {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut variable = None;
        match input.parse::<Ident>() {
            Ok(ident) => {
                variable = Some(ident);
            }
            Err(_) => {
                panic!("Expected variable");
            }
        }

        match input.parse::<Token![,]>() {
            Ok(_) => {
                // ok
            }
            Err(_) => {
                panic!("Expected ','");
            }
        }

        match input.parse::<Type>() {
            Ok(_) => {
                // ok
            }
            Err(_) => {
                panic!("Expected type");
            }
        }

        let content;
        let brace_token = syn::braced!(content in input);
        let fields = content.parse_terminated(Accessor::parse, Token![,])?;

        if fields.is_empty() {
            panic!("Expected at least one accessor");
        }

        Ok(DryMatch { variable, brace_token, fields })
    }
}

impl quote::ToTokens for DryMatch {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut expectations = TokenStream::new();
        let variable = &self.variable;
        let fields = &self.fields;
        for field in fields {
            let references = &field.references;
            let mut conditions = TokenStream::new();
            for (index, condition) in field
                .conditions
                .iter()
                .enumerate()
            {
                if let Some(op) = &condition.logic_op {
                    if index == 0 {
                        conditions.extend(quote! { #condition.#op });
                    } else {
                        conditions.extend(quote! { (#condition).#op });
                    }
                } else {
                    if index == 0 {
                        conditions.extend(quote! { #condition });
                    } else {
                        conditions.extend(quote! { (#condition) });
                    }
                }
            }
            expectations.extend(quote! {
                caramelo::expect(#variable #(#references)*)
            });
            expectations.extend(Dot::default().to_token_stream());

            expectations.extend(quote! { to_match(#conditions); });
        }

        tokens.extend(expectations);
    }
}
