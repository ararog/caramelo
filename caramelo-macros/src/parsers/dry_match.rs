use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Expr, Ident, Result, Token,
};

mod kw {
    syn::custom_keyword!(is);
    syn::custom_keyword!(and);
    syn::custom_keyword!(or);
}

mod op {
    syn::custom_punctuation!(Equals, ==);
    syn::custom_punctuation!(NotEquals, !=);
    syn::custom_punctuation!(LessThan, <);
    syn::custom_punctuation!(GreaterThan, >);
    syn::custom_punctuation!(LessEquals, <=);
    syn::custom_punctuation!(GreaterEquals, >=);
    syn::custom_punctuation!(Regex, ~);
}

#[allow(dead_code)]
pub(crate) enum RelOp {
    Eq(op::Equals),
    Ne(op::NotEquals),
    Lt(op::LessThan),
    Le(op::LessEquals),
    Gt(op::GreaterThan),
    Ge(op::GreaterEquals),
    Re(op::Regex),
}

impl Parse for RelOp {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(op::Equals) {
            input
                .parse()
                .map(RelOp::Eq)
        } else if input.peek(op::NotEquals) {
            input
                .parse()
                .map(RelOp::Ne)
        } else if input.peek(op::LessEquals) {
            input
                .parse()
                .map(RelOp::Le)
        } else if input.peek(op::GreaterEquals) {
            input
                .parse()
                .map(RelOp::Ge)
        } else if input.peek(op::LessThan) {
            input
                .parse()
                .map(RelOp::Lt)
        } else if input.peek(op::GreaterThan) {
            input
                .parse()
                .map(RelOp::Gt)
        } else if input.peek(op::Regex) {
            input
                .parse()
                .map(RelOp::Re)
        } else {
            Err(input.error("Expected binary operator"))
        }
    }
}

#[allow(dead_code)]
pub(crate) struct DryMatchArgs {
    pub(crate) variable: Option<Ident>,
    pub(crate) brace_token: token::Brace,
    pub(crate) fields: Punctuated<FieldArgs, Token![,]>,
}

impl Parse for DryMatchArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut variable = None;
        match input.parse::<Ident>() {
            Ok(ident) => {
                variable = Some(ident);
            }
            Err(e) => {
                panic!("Expected variable: {}", e);
            }
        }
        match input.parse::<kw::is>() {
            Ok(_) => {
                // ok
            }
            Err(e) => {
                panic!("Expected 'is': {}", e);
            }
        }

        let content;
        let brace_token = syn::braced!(content in input);
        let fields = content.parse_terminated(FieldArgs::parse, Token![,])?;

        Ok(DryMatchArgs { variable, brace_token, fields })
    }
}

#[allow(dead_code)]
pub(crate) struct FieldArgs {
    pub(crate) field: Ident,
    pub(crate) colon_token: Token![:],
    pub(crate) operator: Option<RelOp>,
    pub(crate) value: syn::Expr,
}

impl Parse for FieldArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let field = input.parse::<Ident>()?;
        let colon_token = input.parse::<Token![:]>()?;
        let operator = if Expr::peek(&input) { None } else { Some(input.parse::<RelOp>()?) };
        let value = input.parse::<syn::Expr>()?;
        Ok(FieldArgs { field, colon_token, operator, value })
    }
}
