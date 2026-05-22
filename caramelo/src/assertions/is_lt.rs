use std::fmt::Display;

/// Trait for equality assertions
pub trait IsLt: PartialOrd + Display {
    /// Asserts that the value is less than the expected value
    ///
    /// # Examples
    ///
    /// ```
    /// use caramelo::assertions::IsLt;
    ///
    /// 3.is_lt(&5);
    /// ```
    fn is_lt(&self, other: &Self) {
        if !self.lt(other) {
            panic!("Expected {} to be less than {}", self, other);
        }
    }
}

impl IsLt for &str {}
impl IsLt for String {}
impl IsLt for i8 {}
impl IsLt for i16 {}
impl IsLt for i32 {}
impl IsLt for i64 {}
impl IsLt for u8 {}
impl IsLt for u16 {}
impl IsLt for u32 {}
impl IsLt for u64 {}
impl IsLt for f32 {}
impl IsLt for f64 {}
impl IsLt for bool {}
impl IsLt for char {}
