use proc_macro2::TokenStream;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, AngleBracketedGenericArguments, Expr, Ident, Result, Token,
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
/// Relational operator
///
/// # Variants
///
/// * `Eq` - Equality operator
/// * `Ne` - Not equality operator
/// * `Lt` - Less than operator
/// * `Le` - Less than or equal operator
/// * `Gt` - Greater than operator
/// * `Ge` - Greater than or equal operator
/// * `Re` - Regex operator
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
/// Parsed dry match expression
///
/// # Arguments
///
/// * `variable` - The variable to match against
/// * `brace_token` - The brace token
/// * `fields` - The fields to match against
pub(crate) struct DryMatch {
    pub(crate) variable: Option<Ident>,
    pub(crate) brace_token: token::Brace,
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
        let fields = content.parse_terminated(Accessor::parse, Token![,])?;

        Ok(DryMatch { variable, brace_token, fields })
    }
}

#[allow(dead_code)]
/// Parsed accessor expression
///
/// # Arguments
///
/// * `references` - The references to match against
/// * `colon_token` - The colon token
/// * `operator` - The operator to use for comparison
/// * `value` - The value to match against
pub(crate) struct Accessor {
    pub(crate) references: Vec<Ref>,
    pub(crate) colon_token: Token![:],
    pub(crate) operator: Option<RelOp>,
    pub(crate) value: Expr,
}

impl Parse for Accessor {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut references = Vec::new();
        while Ref::peek(input) {
            references.push(input.parse::<Ref>()?);
        }
        let colon_token = input.parse::<Token![:]>()?;
        let operator = if Expr::peek(input) { None } else { Some(input.parse::<RelOp>()?) };
        let value = input.parse::<Expr>()?;
        Ok(Accessor { references, colon_token, operator, value })
    }
}

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

/// Parse dot method reference
///
/// # Arguments
///
/// * `dot_token` - The dot token
/// * `name` - The name of the method
/// * `turbofish` - The turbofish token
/// * `paren_token` - The paren token
/// * `args` - The arguments to the method
pub struct RefDotMethod {
    pub dot_token: Token![.],
    pub name: Ident,
    pub turbofish: Option<AngleBracketedGenericArguments>,
    pub paren_token: token::Paren,
    pub args: Punctuated<Expr, Token![,]>,
}

impl RefDotMethod {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(Token![.])
            && input.peek2(Ident)
            && (input.peek3(token::Paren) || input.peek3(Token![::]))
    }
}

impl Parse for RefDotMethod {
    fn parse(input: ParseStream) -> Result<Self> {
        let dot_token = input.parse::<Token![.]>()?;
        let name = input.parse::<Ident>()?;
        let turbofish = if input.peek(Token![::]) {
            Some(input.parse::<AngleBracketedGenericArguments>()?)
        } else {
            None
        };
        let content;
        let paren_token = syn::parenthesized!(content in input);
        let args = content.parse_terminated(Expr::parse, Token![,])?;
        Ok(RefDotMethod { dot_token, name, turbofish, paren_token, args })
    }
}

impl quote::ToTokens for RefDotMethod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.dot_token
            .to_tokens(tokens);
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
