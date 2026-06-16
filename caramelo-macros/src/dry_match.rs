#![allow(unused_variables, unused_assignments, dead_code)]
use crate::{acessor::Accessor, operators::LogicOp};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Brace,
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
        let mut expectations = Vec::new();
        let variable = &self.variable;
        let fields = &self.fields;
        for field in fields {
            let references = &field.references;
            let conditions = &field.conditions;
            let mut and_conditions = Vec::new();
            let mut or_conditions = Vec::new();
            let logic_operator = conditions
                .first()
                .and_then(|c| c.logic_op.as_ref());
            for condition in conditions.iter() {
                if let Some(operator) = logic_operator {
                    match operator {
                        LogicOp::LogicOpAnd(_) => {
                            and_conditions.push(condition);
                        }
                        LogicOp::LogicOpOr(_) => {
                            or_conditions.push(condition);
                        }
                    }
                }
            }
            if !and_conditions.is_empty() {
                expectations.push(quote! {
                    caramelo::expect(#variable #(#references)*).to_self(caramelo::and!(#(#and_conditions),*));
                });
            } else if !or_conditions.is_empty() {
                expectations.push(quote! {
                    caramelo::expect(#variable #(#references)*).to_self(caramelo::or!(#(#or_conditions),*));
                });
            } else {
                expectations.push(quote! {
                    caramelo::expect(#variable #(#references)*).to_self(#(#conditions),*);
                });
            }
        }

        let expanded = quote! {
            #(#expectations)*
        };

        tokens.extend(expanded);
    }
}
