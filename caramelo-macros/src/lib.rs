#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use crate::{condition::Value, dry_match::DryMatch, operators::RelOp};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr};

pub(crate) mod acessor;
pub(crate) mod condition;
pub(crate) mod dry_match;
pub(crate) mod operators;
pub(crate) mod references;

/// dry_match! is a macro that allows you to write tests in a more readable way.
/// Keep in mind your struct must implement accessor methods that return the same type as the field.
///
/// # Syntax
///
/// ```rust,ignore
/// dry_match!(variable, Type { field: operator value });
/// ```
///
/// # Example
///
/// ```rust,ignore
/// use caramelo_macros::dry_match;
///
/// #[test]
/// fn test_dry_match() {
///     let user = User { name: "John".to_string(), age: 30 };
///     dry_match!(user, User { name: == "John", age: > 25 });
/// }
/// ```
#[proc_macro]
pub fn dry_match(item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(item as DryMatch);
    let variable = match args.variable {
        Some(ident) => ident,
        None => panic!("variable is required"),
    };

    let fields = args.fields;
    let mut expectations = Vec::new();
    for args in fields {
        let references = args.references;
        let conditions = args.conditions;
        let mut matches = Vec::new();
        for (index, condition) in conditions
            .iter()
            .enumerate()
        {
            let rel_op = &condition.rel_op;
            let value = &condition.value;
            let log_op = &condition.logic_op;
            let matcher = match rel_op {
                Some(op) => match op {
                    RelOp::Eq(_) => quote! { caramelo::matchers::eq(#value) },
                    RelOp::Gt(_) => quote! { caramelo::matchers::gt(#value) },
                    RelOp::Lt(_) => quote! { caramelo::matchers::lt(#value) },
                    RelOp::Ge(_) => quote! { caramelo::matchers::ge(#value) },
                    RelOp::Le(_) => quote! { caramelo::matchers::le(#value) },
                    RelOp::Ne(_) => quote! { caramelo::matchers::ne(#value) },
                    RelOp::Re(_) => quote! { caramelo::matchers::contains(#value) },
                },
                None => match value {
                    Value::ValueExpr(expr) => match expr {
                        Expr::Range(expr_range) => match expr_range.limits {
                            syn::RangeLimits::Closed(_) => {
                                let start = expr_range
                                    .start
                                    .as_deref();
                                let end = expr_range
                                    .end
                                    .as_deref();
                                match (start, end) {
                                    (Some(start), Some(end)) => {
                                        quote! { caramelo::matchers::in_range_inc(#start, #end) }
                                    }
                                    (None, Some(end)) => {
                                        quote! { caramelo::matchers::le(#end) }
                                    }
                                    _ => panic!("start and end must be specified for closed range"),
                                }
                            }
                            syn::RangeLimits::HalfOpen(_) => {
                                let start = expr_range
                                    .start
                                    .as_deref();
                                let end = expr_range
                                    .end
                                    .as_deref();
                                match (start, end) {
                                    (Some(start), Some(end)) => {
                                        quote! { caramelo::matchers::in_range_to(#start, #end) }
                                    }
                                    (Some(start), None) => {
                                        quote! { caramelo::matchers::gt(#start) }
                                    }
                                    (None, Some(end)) => {
                                        quote! { caramelo::matchers::lt(#end) }
                                    }
                                    _ => panic!(
                                        "start and end must be specified for half-open range"
                                    ),
                                }
                            }
                        },
                        _ => panic!("unsupported expression type"),
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
            if index == 0 {
                matches.push(quote! {
                    .to_be(#matcher)
                });
            } else {
                if let Some(log_op) = log_op {
                    matches.push(quote! {
                        .#log_op(#matcher)
                    });
                } else {
                    matches.push(quote! {
                        .and(#matcher)
                    });
                }
            }
        }
        expectations.push(quote! {
            caramelo::expect(#variable #(#references)*)#(#matches)*;
        });
    }

    let expanded = quote! {
        #(#expectations)*
    };

    TokenStream::from(expanded)
}
