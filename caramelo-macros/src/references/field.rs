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
    pub dot_token: Option<Token![.]>,
    pub name: Ident,
}

impl RefField {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(Ident::peek_any) || (input.peek(Token![.]) && input.peek2(Ident::peek_any))
    }
}

impl Parse for RefField {
    fn parse(input: ParseStream) -> Result<Self> {
        let dot_token =
            if input.peek(Token![.]) { Some(input.parse::<Token![.]>()?) } else { None };
        let name = input.parse::<Ident>()?;
        Ok(RefField { dot_token, name })
    }
}

impl quote::ToTokens for RefField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let dot_token = Token![.](proc_macro2::Span::call_site());
        dot_token.to_tokens(tokens);
        self.name
            .to_tokens(tokens);
    }
}
