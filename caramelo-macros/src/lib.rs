#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use crate::dry_match::DryMatch;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

pub(crate) mod dry_match;

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
    let dry_match = parse_macro_input!(item as DryMatch);

    let expanded = quote! {
        use caramelo::MatcherExt;
        #dry_match
    };

    TokenStream::from(expanded)
}
