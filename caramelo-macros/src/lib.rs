#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr};

use crate::{DryMatch, RelOp};

pub(crate) use dry_match::*;

mod dry_match;

/// dry_match! is a macro that allows you to write tests in a more readable way.
/// Keep in mind your struct must implement accessor methods that return the same type as the field.
///
/// # Syntax
///
/// ```rust,ignore
/// dry_match!(variable is { field: operator value });
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
///     dry_match!(user is { name: == "John", age: > 25 });
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
        let operator = args.operator;
        let value = args.value;
        let matcher = match operator {
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
                Expr::Range(expr_range) => match expr_range.limits {
                    syn::RangeLimits::Closed(_) => {
                        let start = expr_range
                            .start
                            .as_ref()
                            .unwrap();
                        let end = expr_range
                            .end
                            .as_ref()
                            .unwrap();
                        quote! { caramelo::matchers::in_range_inc(#start, #end) }
                    }
                    syn::RangeLimits::HalfOpen(_) => {
                        let start = expr_range
                            .start
                            .as_ref()
                            .unwrap();
                        let end = expr_range
                            .end
                            .as_ref()
                            .unwrap();
                        quote! { caramelo::matchers::in_range_to(#start, #end) }
                    }
                },
                _ => panic!("value must be a range when no operator is specified"),
            },
        };
        expectations.push(quote! {
          caramelo::expect(#variable #(#references)*).to_be(#matcher);
        });
    }

    let expanded = quote! {
        #(#expectations)*
    };

    TokenStream::from(expanded)
}
