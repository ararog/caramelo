use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, BinOp, Ident, Result, Token,
};

mod kw {
    syn::custom_keyword!(is);
    syn::custom_keyword!(and);
    syn::custom_keyword!(or);
}

pub(crate) struct DryMatchArgs {
    pub(crate) variable: Option<Ident>,
    pub(crate) brace_token: token::Brace,
    pub(crate) fields: Punctuated<FieldArgs, Token![,]>,
}

impl Parse for DryMatchArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut variable = None;
        let mut fields = Punctuated::new();
        match input.parse::<Ident>() {
            Ok(ident) => {
                variable = Some(ident);
            }
            Err(e) => {
                panic!("Expected variable: {}", e);
            }
        }
        match input.parse::<Ident>() {
            Ok(ident) => {
                if ident != "is" {
                    panic!("Expected 'is'");
                }
            }
            Err(e) => {
                panic!("Expected 'is': {}", e);
            }
        }

        let content;
        let brace_token = syn::braced!(content in input);
        fields = content.parse_terminated(FieldArgs::parse, Token![,])?;

        Ok(DryMatchArgs { variable, brace_token, fields })
    }
}

pub(crate) struct FieldArgs {
    pub(crate) field: Ident,
    pub(crate) colon_token: Token![:],
    pub(crate) operator: BinOp,
    pub(crate) value: syn::Expr,
}

impl Parse for FieldArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let field = input.parse::<Ident>()?;
        let colon_token = input.parse::<Token![:]>()?;
        let operator = input.parse::<BinOp>()?;
        let value = input.parse::<syn::Expr>()?;
        Ok(FieldArgs { field, colon_token, operator, value })
    }
}
