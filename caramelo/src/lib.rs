use std::fmt::Debug;

#[cfg(test)]
mod tests;

pub struct Expect<T> {
    value: T,
}

impl<T> Expect<T>
where
    T: Debug,
{
    pub fn new(value: T) -> Self {
        Expect { value }
    }

    pub fn to_be<M: Matcher<T>>(&self, matcher: M) {
        match matcher.matches(&self.value) {
            true => (),
            false => panic!("Expected {:?} to be {}", self.value, matcher.description()),
        }
    }
}

pub fn expect<T: Debug>(value: T) -> Expect<T> {
    Expect::new(value)
}

pub trait Matcher<T> {
    fn matches(&self, value: &T) -> bool;
    fn description(&self) -> String;
}

pub struct Equal<T>(T);

impl<T> Matcher<T> for Equal<T>
where
    T: PartialEq + Debug,
{
    fn matches(&self, value: &T) -> bool {
        self.0 == *value
    }

    fn description(&self) -> String {
        format!("equals to {:?}", self.0)
    }
}
