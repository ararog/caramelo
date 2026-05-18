use std::fmt::Debug;

pub mod matchers;

#[cfg(test)]
mod tests;

pub struct Expect<T, M> {
    value: T,
    _matcher: std::marker::PhantomData<M>,
}

impl<T, M> Expect<T, M>
where
    T: Debug,
    M: Matcher<T>,
{
    pub fn new(value: T) -> Self {
        Expect { value, _matcher: std::marker::PhantomData }
    }

    pub fn to_be(&self, matcher: M) {
        if !matcher.matches(&self.value) {
            panic!("Expected {:?} to be {}", self.value, matcher.description());
        }
    }

    pub fn to_have(&self, matcher: M) {
        self.to_be(matcher);
    }
}

pub fn expect<T: Debug, M: Matcher<T>>(value: T) -> Expect<T, M> {
    Expect::new(value)
}

pub trait Matcher<T> {
    fn matches(&self, value: &T) -> bool;
    fn description(&self) -> String;
}
