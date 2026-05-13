pub trait Matcher {
    fn matches(&self, value: &str) -> bool;
}
