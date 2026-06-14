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
    RefDotField(RefDotField),
    RefDotMethod(RefDotMethod),
}

impl Ref {
    pub(crate) fn peek(input: ParseStream) -> bool {
        RefField::peek(input) || RefDotField::peek(input) || RefDotMethod::peek(input)
    }
}

impl Parse for Ref {
    fn parse(input: ParseStream) -> Result<Self> {
        if RefDotMethod::peek(input) {
            let method = input.parse::<RefDotMethod>()?;
            Ok(Ref::RefDotMethod(method))
        } else if RefDotField::peek(input) {
            let field = input.parse::<RefDotField>()?;
            Ok(Ref::RefDotField(field))
        } else if RefField::peek(input) {
            let field = input.parse::<RefField>()?;
            Ok(Ref::RefField(field))
        } else {
            Err(input.error("expected field, dot field, or dot method"))
        }
    }
}

impl quote::ToTokens for Ref {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Ref::RefField(field) => field.to_tokens(tokens),
            Ref::RefDotField(field) => field.to_tokens(tokens),
            Ref::RefDotMethod(method) => method.to_tokens(tokens),
        }
    }
}
