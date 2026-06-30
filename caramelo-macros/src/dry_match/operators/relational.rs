use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    Result,
};

mod rel_op {
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
    Eq(rel_op::Equals),
    Ne(rel_op::NotEquals),
    Lt(rel_op::LessThan),
    Le(rel_op::LessEquals),
    Gt(rel_op::GreaterThan),
    Ge(rel_op::GreaterEquals),
    Re(rel_op::Regex),
}

impl RelOp {
    pub(crate) fn peek(input: ParseStream) -> bool {
        input.peek(rel_op::Equals)
            || input.peek(rel_op::NotEquals)
            || input.peek(rel_op::LessEquals)
            || input.peek(rel_op::GreaterEquals)
            || input.peek(rel_op::LessThan)
            || input.peek(rel_op::GreaterThan)
            || input.peek(rel_op::Regex)
    }
}

impl Parse for RelOp {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(rel_op::Equals) {
            input
                .parse()
                .map(RelOp::Eq)
        } else if input.peek(rel_op::NotEquals) {
            input
                .parse()
                .map(RelOp::Ne)
        } else if input.peek(rel_op::LessEquals) {
            input
                .parse()
                .map(RelOp::Le)
        } else if input.peek(rel_op::GreaterEquals) {
            input
                .parse()
                .map(RelOp::Ge)
        } else if input.peek(rel_op::LessThan) {
            input
                .parse()
                .map(RelOp::Lt)
        } else if input.peek(rel_op::GreaterThan) {
            input
                .parse()
                .map(RelOp::Gt)
        } else if input.peek(rel_op::Regex) {
            input
                .parse()
                .map(RelOp::Re)
        } else {
            Err(input.error("Expected any of ==, !=, <, <=, >, >=, ~"))
        }
    }
}

impl quote::ToTokens for RelOp {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            RelOp::Eq(op) => op.to_tokens(tokens),
            RelOp::Ne(op) => op.to_tokens(tokens),
            RelOp::Lt(op) => op.to_tokens(tokens),
            RelOp::Le(op) => op.to_tokens(tokens),
            RelOp::Gt(op) => op.to_tokens(tokens),
            RelOp::Ge(op) => op.to_tokens(tokens),
            RelOp::Re(op) => op.to_tokens(tokens),
        }
    }
}
