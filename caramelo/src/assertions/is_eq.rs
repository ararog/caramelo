use std::fmt::Display;

/// Trait for equality assertions
pub trait IsEq: PartialEq + Display {
    /// Asserts that the value is equal to the expected value
    fn is_eq(&self, other: &Self) {
        if !self.eq(other) {
            panic!("Expected {} to be equal to {}", self, other);
        }
    }
}

impl IsEq for &str {}
impl IsEq for String {}
impl IsEq for i8 {}
impl IsEq for i16 {}
impl IsEq for i32 {}
impl IsEq for i64 {}
impl IsEq for u8 {}
impl IsEq for u16 {}
impl IsEq for u32 {}
impl IsEq for u64 {}
impl IsEq for f32 {}
impl IsEq for f64 {}
impl IsEq for bool {}
impl IsEq for char {}
