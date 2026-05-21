use std::fmt::Debug;

/// Trait for equality assertions
pub trait HasItem<T>: AsRef<[T]> + Debug
where
    T: PartialEq + Debug,
{
    /// Asserts that the value is equal to the expected value
    fn has_item(&self, other: &T) {
        if !self
            .as_ref()
            .contains(other)
        {
            panic!("Expected {:?} to contains {:?}", self, other);
        }
    }
}

impl HasItem<f32> for Vec<f32> {}
impl HasItem<f64> for Vec<f64> {}
impl HasItem<i32> for Vec<i32> {}
impl HasItem<i64> for Vec<i64> {}
impl HasItem<u32> for Vec<u32> {}
impl HasItem<u64> for Vec<u64> {}
impl HasItem<char> for Vec<char> {}
impl HasItem<String> for Vec<String> {}
