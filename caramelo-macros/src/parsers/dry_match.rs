use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Ident, Result, Token,
};

mod kw {
    syn::custom_keyword!(is);
    syn::custom_keyword!(and);
    syn::custom_keyword!(or);
}

mod op {
    syn::custom_keyword!(eq);
    syn::custom_keyword!(ne);
    syn::custom_keyword!(lt);
    syn::custom_keyword!(le);
    syn::custom_keyword!(gt);
    syn::custom_keyword!(ge);
    syn::custom_keyword!(bw);
    syn::custom_keyword!(re);
}

#[allow(dead_code)]
pub(crate) enum RelOp {
    Eq(op::eq),
    Ne(op::ne),
    Lt(op::lt),
    Le(op::le),
    Gt(op::gt),
    Ge(op::ge),
    Bw(op::bw),
    Re(op::re),
}

impl Parse for RelOp {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(op::eq) {
            input
                .parse()
                .map(RelOp::Eq)
        } else if input.peek(op::ne) {
            input
                .parse()
                .map(RelOp::Ne)
        } else if input.peek(op::le) {
            input
                .parse()
                .map(RelOp::Le)
        } else if input.peek(op::ge) {
            input
                .parse()
                .map(RelOp::Ge)
        } else if input.peek(op::lt) {
            input
                .parse()
                .map(RelOp::Lt)
        } else if input.peek(op::gt) {
            input
                .parse()
                .map(RelOp::Gt)
        } else if input.peek(op::bw) {
            input
                .parse()
                .map(RelOp::Bw)
        } else if input.peek(op::re) {
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
    pub(crate) operator: RelOp,
    pub(crate) value: syn::Expr,
}

impl Parse for FieldArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let field = input.parse::<Ident>()?;
        let colon_token = input.parse::<Token![:]>()?;
        let operator = input.parse::<RelOp>()?;
        let value = input.parse::<syn::Expr>()?;
        Ok(FieldArgs { field, colon_token, operator, value })
    }
}
