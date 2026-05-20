use std::fmt::Display;

/// Trait for less than or equal assertions
pub trait IsLe: PartialOrd + Display {
    /// Asserts that the value is less than or equal to the expected value
    fn is_le(&self, other: &Self) {
        if !self.le(other) {
            panic!("Expected {} to be less than or equal to {}", self, other);
        }
    }
}

impl IsLe for &str {}
impl IsLe for String {}
impl IsLe for i8 {}
impl IsLe for i16 {}
impl IsLe for i32 {}
impl IsLe for i64 {}
impl IsLe for u8 {}
impl IsLe for u16 {}
impl IsLe for u32 {}
impl IsLe for u64 {}
impl IsLe for f32 {}
impl IsLe for f64 {}
impl IsLe for bool {}
