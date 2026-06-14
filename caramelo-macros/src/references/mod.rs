mod field;
mod method;

pub(crate) use field::*;
pub(crate) use method::*;

use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    Result,
};

#[allow(clippy::enum_variant_names)]
pub(crate) enum Ref {
    RefField(RefField),
    RefMethod(RefMethod),
}

impl Ref {
    pub(crate) fn peek(input: ParseStream) -> bool {
        RefField::peek(input) || RefMethod::peek(input)
    }
}

impl Parse for Ref {
    fn parse(input: ParseStream) -> Result<Self> {
        if RefMethod::peek(input) {
            let method = input.parse::<RefMethod>()?;
            Ok(Ref::RefMethod(method))
        } else if RefField::peek(input) {
            let field = input.parse::<RefField>()?;
            Ok(Ref::RefField(field))
        } else {
            Err(input.error("expected field or method"))
        }
    }
}

impl quote::ToTokens for Ref {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Ref::RefField(field) => field.to_tokens(tokens),
            Ref::RefMethod(method) => method.to_tokens(tokens),
        }
    }
}
