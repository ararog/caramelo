use crate::acessor::Accessor;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Brace,
    Ident, Result, Token, Type,
};

#[allow(dead_code)]
/// Parsed dry match expression
///
/// # Arguments
///
/// * `variable` - The variable to match against
/// * `brace_token` - The brace token
/// * `fields` - The fields to match against
pub(crate) struct DryMatch {
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
            Err(e) => {
                panic!("Expected variable");
            }
        }

        match input.parse::<Token![,]>() {
            Ok(_) => {
                // ok
            }
            Err(e) => {
                panic!("Expected ','");
            }
        }

        match input.parse::<Type>() {
            Ok(_) => {
                // ok
            }
            Err(e) => {
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
