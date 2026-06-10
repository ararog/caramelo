#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::parsers::DryMatchArgs;

mod parsers;

/// Macro for creating assertions in tests
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
            syn::BinOp::Eq(_) => quote! { caramelo::matchers::eq(#value) },
            syn::BinOp::Gt(_) => quote! { caramelo::matchers::gt(#value) },
            syn::BinOp::Lt(_) => quote! { caramelo::matchers::lt(#value) },
            _ => panic!("Unsupported operator"),
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
