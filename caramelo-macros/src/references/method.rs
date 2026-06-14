use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
    AngleBracketedGenericArguments, Expr, Ident, Result, Token,
};

/// Parse dot method reference
///
/// # Arguments
///
/// * `name` - The name of the method
/// * `turbofish` - The turbofish token
/// * `paren_token` - The paren token
/// * `args` - The arguments to the method
pub struct RefMethod {
    pub dot_token: Option<Token![.]>,
    pub name: Ident,
    pub turbofish: Option<AngleBracketedGenericArguments>,
    pub paren_token: Paren,
    pub args: Punctuated<Expr, Token![,]>,
}

impl RefMethod {
    pub fn peek(input: ParseStream) -> bool {
        (input.peek(Ident) && (input.peek2(Paren) || input.peek2(Token![::])))
            || (input.peek(Token![.])
                && input.peek2(Ident)
                && (input.peek3(Paren) || input.peek3(Token![::])))
    }
}

impl Parse for RefMethod {
    fn parse(input: ParseStream) -> Result<Self> {
        let dot_token =
            if input.peek(Token![.]) { Some(input.parse::<Token![.]>()?) } else { None };
        let name = input.parse::<Ident>()?;
        let turbofish = if input.peek(Token![::]) {
            Some(input.parse::<AngleBracketedGenericArguments>()?)
        } else {
            None
        };
        let content;
        let paren_token = syn::parenthesized!(content in input);
        let args = content.parse_terminated(Expr::parse, Token![,])?;
        Ok(RefMethod { dot_token, name, turbofish, paren_token, args })
    }
}

impl quote::ToTokens for RefMethod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let dot_token = Token![.](proc_macro2::Span::call_site());
        dot_token.to_tokens(tokens);
        self.name
            .to_tokens(tokens);
        if let Some(turbofish) = &self.turbofish {
            turbofish.to_tokens(tokens);
        }
        self.paren_token
            .surround(tokens, |tokens| {
                self.args
                    .to_tokens(tokens);
            });
    }
}
