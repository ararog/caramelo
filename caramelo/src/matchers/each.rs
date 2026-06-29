use crate::{MatchType::ToHave, Matcher, TypedMatcher};
use std::{cell::RefCell, fmt::Debug};

/// Creates a matcher that checks if all items in a collection match the given matcher.
pub fn each<M, T>(matcher: M) -> Each<T>
where
    M: Matcher<T> + 'static,
    T: Debug + Default,
{
    Each::new(matcher)
}

/// A matcher that checks if all items in a collection match the given matcher.
pub struct Each<T> {
    matcher: Box<dyn Matcher<T>>,
    element: RefCell<T>,
}

impl<T> Each<T> {
    /// Creates a new `Each` matcher.
    pub fn new<M>(matcher: M) -> Self
    where
        M: Matcher<T> + 'static,
        T: Debug + Default,
    {
        Each { matcher: Box::new(matcher), element: RefCell::new(T::default()) }
    }
}

impl<T> Matcher<Vec<T>> for Each<T>
where
    T: Clone + Debug,
{
    fn matches(&self, value: &Vec<T>) -> bool {
        for item in value {
            if !self
                .matcher
                .matches(item)
            {
                self.element
                    .borrow_mut()
                    .clone_from(item);
                return false;
            }
        }
        true
    }

    fn description(&self) -> String {
        format!(
            "element {:?} that matches {}",
            self.element
                .borrow(),
            self.matcher
                .description()
        )
    }
}

impl<T> TypedMatcher<Vec<T>> for Each<T>
where
    T: Clone + Debug,
{
    fn matcher_type(&self) -> crate::MatchType {
        ToHave
    }
}
