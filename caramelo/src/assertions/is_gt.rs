use std::fmt::Display;

/// Trait for greater than assertions
pub trait IsGt: PartialOrd + Display {
    /// Asserts that the value is greater than the expected value
    ///
    /// # Examples
    ///
    /// ```
    /// use caramelo::assertions::IsGt;
    ///
    /// 5.is_gt(&3);
    /// ```
    fn is_gt(&self, other: &Self) {
        if !self.gt(other) {
            panic!("Expected {} to be greater than {}", self, other);
        }
    }
}

impl IsGt for &str {}
impl IsGt for String {}
impl IsGt for i8 {}
impl IsGt for i16 {}
impl IsGt for i32 {}
impl IsGt for i64 {}
impl IsGt for u8 {}
impl IsGt for u16 {}
impl IsGt for u32 {}
impl IsGt for u64 {}
impl IsGt for f32 {}
impl IsGt for f64 {}
impl IsGt for bool {}
impl IsGt for char {}
