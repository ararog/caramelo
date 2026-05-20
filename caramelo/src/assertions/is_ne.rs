use std::fmt::Display;

/// Trait for inequality assertions
pub trait IsNe: PartialEq + Display {
    /// Asserts that the value is not equal to the expected value
    fn is_ne(&self, other: &Self) {
        if self.eq(other) {
            panic!("Expected {} to be not equal to {}", self, other);
        }
    }
}

impl IsNe for &str {}
impl IsNe for String {}
impl IsNe for i8 {}
impl IsNe for i16 {}
impl IsNe for i32 {}
impl IsNe for i64 {}
impl IsNe for u8 {}
impl IsNe for u16 {}
impl IsNe for u32 {}
impl IsNe for u64 {}
impl IsNe for f32 {}
impl IsNe for f64 {}
impl IsNe for bool {}
