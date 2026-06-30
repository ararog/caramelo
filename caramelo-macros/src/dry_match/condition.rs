use crate::dry_match::operators::{LogicOp, RelOp};
use proc_macro2::{Punct, Spacing, TokenStream, TokenTree};
use quote::{quote, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    Expr, Ident, Lit, Result, Token,
};

/// Parsed condition expression
///
/// # Arguments
///
/// * `rel_op` - The relational operator to use for comparison
/// * `value` - The value to match against
/// * `logic_op` - The logic operator to use for comparison
pub(crate) struct Condition {
    pub(crate) rel_op: Option<RelOp>,
    pub(crate) value: Value,
    pub(crate) logic_op: Option<LogicOp>,
}

impl Condition {
    pub(crate) fn peek(input: ParseStream) -> bool {
        RelOp::peek(input) || Expr::peek(input)
    }
}

impl Parse for Condition {
    fn parse(input: ParseStream) -> Result<Self> {
        let rel_op = if RelOp::peek(input) { Some(input.parse::<RelOp>()?) } else { None };
        let value = input.parse::<Value>()?;
        let logic_op = if LogicOp::peek(input) { Some(input.parse::<LogicOp>()?) } else { None };
        Ok(Condition { rel_op, value, logic_op })
    }
}

impl quote::ToTokens for Condition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value = &self.value;
        let matcher = match &self.rel_op {
            Some(op) => match op {
                RelOp::Eq(_) => quote! { caramelo::matchers::eq(#value) },
                RelOp::Gt(_) => quote! { caramelo::matchers::gt(#value) },
                RelOp::Lt(_) => quote! { caramelo::matchers::lt(#value) },
                RelOp::Ge(_) => quote! { caramelo::matchers::ge(#value) },
                RelOp::Le(_) => quote! { caramelo::matchers::le(#value) },
                RelOp::Ne(_) => quote! { caramelo::matchers::ne(#value) },
                RelOp::Re(_) => quote! { caramelo::matchers::contains(#value) },
            },
            None => match &self.value {
                Value::ValueExpr(expr) => match expr {
                    Expr::Range(expr_range) => {
                        quote! { caramelo::matchers::range(#expr_range) }
                    }
                    Expr::Closure(closure) => {
                        quote! { caramelo::matchers::custom(#closure, "custom matcher") }
                    }
                    _ => panic!("Expected range expression"),
                },
                Value::ValuePipedOr(piped_or) => {
                    let items = piped_or
                        .items
                        .iter()
                        .map(|item| {
                            quote! { #item }
                        });
                    quote! { caramelo::matchers::_in_(vec![#(#items),*]) }
                }
            },
        };

        tokens.extend(matcher);
    }
}

/// Parsed value expression
///
/// # Arguments
///
/// * `ValueExpr` - The value expression to match against
/// * `ValuePipedOr` - The piped or expression to match against
pub(crate) enum Value {
    ValueExpr(Expr),
    ValuePipedOr(PipedOr),
}

impl Parse for Value {
    fn parse(input: ParseStream) -> Result<Self> {
        if PipedOr::peek(input) {
            Ok(Value::ValuePipedOr(input.parse()?))
        } else {
            Ok(Value::ValueExpr(input.parse()?))
        }
    }
}

impl quote::ToTokens for Value {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Value::ValueExpr(expr) => expr.to_tokens(tokens),
            Value::ValuePipedOr(piped_or) => piped_or.to_tokens(tokens),
        }
    }
}

/// Parsed piped or expression
///
/// # Arguments
///
/// * `items` - The items to match against
pub(crate) struct PipedOr {
    pub(crate) items: Vec<PipeItem>,
}

impl PipedOr {
    pub(crate) fn peek(input: ParseStream) -> bool {
        PipeItem::peek(input) && input.peek2(Token![|])
    }
}

impl Parse for PipedOr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut items = Vec::new();
        while PipeItem::peek(input) {
            items.push(input.parse()?);

            if input.peek(Token![|]) {
                input.parse::<Token![|]>()?;
            } else {
                break;
            }
        }
        Ok(PipedOr { items })
    }
}

impl quote::ToTokens for PipedOr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for (i, item) in self
            .items
            .iter()
            .enumerate()
        {
            if i > 0 {
                tokens.append(TokenTree::Punct(Punct::new('|', Spacing::Alone)));
            }
            item.to_tokens(tokens);
        }
    }
}

/// Parsed pipe item
///
/// # Arguments
///
/// * `PipeItemLit` - The literal value
/// * `PipeItemIdent` - The identifier
pub(crate) enum PipeItem {
    PipeItemLit(Lit),
    PipeItemIdent(Ident),
}

impl PipeItem {
    pub(crate) fn peek(input: ParseStream) -> bool {
        input.peek(Lit) || input.peek(Ident)
    }
}

impl Parse for PipeItem {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Lit) {
            Ok(PipeItem::PipeItemLit(input.parse()?))
        } else {
            Ok(PipeItem::PipeItemIdent(input.parse()?))
        }
    }
}

impl quote::ToTokens for PipeItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            PipeItem::PipeItemLit(lit) => lit.to_tokens(tokens),
            PipeItem::PipeItemIdent(ident) => ident.to_tokens(tokens),
        }
    }
}
