use std::fmt::Display;

/// Trait for greater than or equal assertions
pub trait IsGe: PartialOrd + Display {
    /// Asserts that the value is greater than or equal to the expected value
    fn is_ge(&self, other: &Self) {
        if !self.ge(other) {
            panic!("Expected {} to be greater than or equal to {}", self, other);
        }
    }
}

impl IsGe for &str {}
impl IsGe for String {}
impl IsGe for i8 {}
impl IsGe for i16 {}
impl IsGe for i32 {}
impl IsGe for i64 {}
impl IsGe for u8 {}
impl IsGe for u16 {}
impl IsGe for u32 {}
impl IsGe for u64 {}
impl IsGe for f32 {}
impl IsGe for f64 {}
impl IsGe for bool {}
impl IsGe for char {}
