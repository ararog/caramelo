use crate::{
    MatchType::{self, ToHave},
    Matcher, TypedMatcher,
};

/// Creates a custom matcher with a function that returns a boolean
pub fn custom<F: Fn(&T) -> bool, T>(f: F, description: &str) -> Custom<F, T> {
    Custom { func: f, description: description.to_string(), _phantom: std::marker::PhantomData }
}

/// A custom matcher that can be used in assertions
pub struct Custom<F: Fn(&T) -> bool, T> {
    func: F,
    description: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, F: Fn(&T) -> bool> Matcher<T> for Custom<F, T> {
    fn matches(&self, value: &T) -> bool {
        (self.func)(value)
    }

    fn description(&self) -> String {
        self.description
            .clone()
    }
}

impl<T, F: Fn(&T) -> bool> TypedMatcher<T> for Custom<F, T> {
    fn matcher_type(&self) -> MatchType {
        ToHave
    }
}
