use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    Result,
};

mod logic_op {
    syn::custom_keyword!(and);
    syn::custom_keyword!(or);
}

#[allow(dead_code)]
/// Logical operator
///
/// # Variants
///
/// * `And` - Logical AND operator
/// * `Or` - Logical OR operator
pub(crate) enum LogicOp {
    LogicOpAnd(logic_op::and),
    LogicOpOr(logic_op::or),
}

impl LogicOp {
    pub(crate) fn peek(input: ParseStream) -> bool {
        input.peek(logic_op::and) || input.peek(logic_op::or)
    }
}

impl Parse for LogicOp {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(logic_op::and) {
            input
                .parse()
                .map(LogicOp::LogicOpAnd)
        } else if input.peek(logic_op::or) {
            input
                .parse()
                .map(LogicOp::LogicOpOr)
        } else {
            Err(input.error("expected 'and' or 'or'"))
        }
    }
}

impl quote::ToTokens for LogicOp {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            LogicOp::LogicOpAnd(_) => {
                tokens.extend(quote::quote! { and });
            }
            LogicOp::LogicOpOr(_) => {
                tokens.extend(quote::quote! { or });
            }
        }
    }
}
