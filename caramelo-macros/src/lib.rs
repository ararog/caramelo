#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr};

use crate::parsers::{DryMatchArgs, RelOp};

mod parsers;

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
///     dry_match!(user is { name: eq "John", age: gt 25 });
/// }
/// ```
#[proc_macro]
pub fn dry_match(item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(item as DryMatchArgs);
    let variable = match args.variable {
        Some(ident) => ident,
        None => panic!("variable is required"),
    };

    let fields = args.fields;
    let mut expectations = Vec::new();
    for args in fields {
        let field_name = args.field;
        let operator = args.operator;
        let value = args.value;
        let matcher = match operator {
            RelOp::Eq(_) => quote! { caramelo::matchers::eq(#value) },
            RelOp::Gt(_) => quote! { caramelo::matchers::gt(#value) },
            RelOp::Lt(_) => quote! { caramelo::matchers::lt(#value) },
            RelOp::Ge(_) => quote! { caramelo::matchers::ge(#value) },
            RelOp::Le(_) => quote! { caramelo::matchers::le(#value) },
            RelOp::Ne(_) => quote! { caramelo::matchers::ne(#value) },
            RelOp::Re(_) => quote! { caramelo::matchers::contains(#value) },
            RelOp::Bw(_) => {
                if let Expr::Tuple(tuple) = &value {
                    let first = &tuple.elems[0];
                    let second = &tuple.elems[1];
                    quote! { caramelo::matchers::between(#first, #second) }
                } else {
                    panic!("between operator requires a tuple");
                }
            }
        };
        expectations.push(quote! {
          caramelo::expect(#variable.#field_name()).to_be(#matcher);
        });
    }

    let expanded = quote! {
        #(#expectations)*
    };

    TokenStream::from(expanded)
}
