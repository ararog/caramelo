use std::fmt::Display;

use crate::assertions::IsEq;

/// Trait for equality assertions
pub trait IsLt: PartialEq + Display {
    /// Asserts that the value is equal to the expected value
    fn is_eq(&self, other: &Self) {
        if !self.eq(other) {
            panic!("Expected {} to be equal to {}", self, other);
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
