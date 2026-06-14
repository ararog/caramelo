use crate::operators::{LogicOp, RelOp};
use proc_macro2::{Punct, Spacing, TokenStream, TokenTree};
use quote::TokenStreamExt;
use syn::{
    parse::{Parse, ParseStream},
    Expr, Lit, Result, Token,
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
        if let Some(rel_op) = &self.rel_op {
            rel_op.to_tokens(tokens);
        }
        self.value
            .to_tokens(tokens);
        if let Some(logic_op) = &self.logic_op {
            logic_op.to_tokens(tokens);
        }
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
    pub(crate) items: Vec<Lit>,
}

impl PipedOr {
    pub(crate) fn peek(input: ParseStream) -> bool {
        input.peek(Lit) && input.peek2(Token![|])
    }
}

impl Parse for PipedOr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut items = Vec::new();
        while input.peek(Lit) {
            items.push(input.parse::<Lit>()?);

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
