use proc_macro2::TokenStream;
use syn::ext::IdentExt;
use syn::{
    parse::{Parse, ParseStream},
    Ident, Result, Token,
};

/// Parse field reference
///
/// # Arguments
///
/// * `name` - The name of the field
pub struct RefField {
    pub name: Ident,
}

impl RefField {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(Ident::peek_any)
    }
}

impl Parse for RefField {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;
        Ok(RefField { name })
    }
}

impl quote::ToTokens for RefField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let dot = Token![.](proc_macro2::Span::call_site());
        dot.to_tokens(tokens);
        self.name
            .to_tokens(tokens);
    }
}

/// Parse dot field reference
///
/// # Arguments
///
/// * `dot_token` - The dot token
/// * `name` - The name of the field
pub struct RefDotField {
    pub dot_token: Token![.],
    pub name: Ident,
}

impl RefDotField {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(Token![.]) && input.peek2(Ident)
    }
}

impl Parse for RefDotField {
    fn parse(input: ParseStream) -> Result<Self> {
        let dot_token = input.parse::<Token![.]>()?;
        let name = input.parse::<Ident>()?;
        Ok(RefDotField { dot_token, name })
    }
}

impl quote::ToTokens for RefDotField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.dot_token
            .to_tokens(tokens);
        self.name
            .to_tokens(tokens);
    }
}
