use crate::{condition::Condition, references::Ref};
use syn::{parse::Parse, parse::ParseStream, Result, Token};

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
    pub(crate) conditions: Vec<Condition>,
}

impl Parse for Accessor {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut references = Vec::new();
        while Ref::peek(input) {
            references.push(input.parse::<Ref>()?);
        }
        if references.is_empty() {
            panic!("Expected at least one reference");
        }

        let colon_token = input.parse::<Token![:]>()?;
        let mut conditions = Vec::new();
        while Condition::peek(input) {
            conditions.push(input.parse::<Condition>()?);
        }
        if conditions.is_empty() {
            panic!("Expected at least one condition");
        }

        Ok(Accessor { references, colon_token, conditions })
    }
}
