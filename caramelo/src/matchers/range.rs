use crate::{
    MatchType::{self, ToBe},
    Matcher, TypedMatcher,
};
use std::{fmt::Debug, ops::RangeBounds};

/// Creates a matcher that matches values between min and max (exclusive)
/// It is the same as 1..10 in Rust
///
/// # Examples
///
/// ```
/// use caramelo::expect;
/// use caramelo::matchers::between;
///
/// expect(5).to_be(between(1..10));
/// ```
pub fn between<R, T>(range: R) -> Between<R, T>
where
    R: RangeBounds<T>,
{
    Between::new(range)
}

/// Matcher that matches values between min and max (inclusive or exclusive)
pub struct Between<R, T> {
    range: R,
    _phantom: std::marker::PhantomData<T>,
}

impl<R, T> Between<R, T> {
    /// Creates a new Between matcher
    pub fn new(range: R) -> Self {
        Self { range, _phantom: std::marker::PhantomData }
    }
}

impl<R, T> Matcher<T> for Between<R, T>
where
    T: PartialOrd + Debug,
    R: RangeBounds<T>,
{
    fn matches(&self, value: &T) -> bool {
        self.range
            .contains(value)
    }

    fn description(&self) -> String {
        match (
            self.range
                .start_bound(),
            self.range
                .end_bound(),
        ) {
            (std::ops::Bound::Included(start), std::ops::Bound::Included(end)) => {
                format!("between {:?} and {:?}", start, end)
            }
            (std::ops::Bound::Included(start), std::ops::Bound::Excluded(end)) => {
                format!("between {:?} and {:?} (exclusive)", start, end)
            }
            (std::ops::Bound::Excluded(start), std::ops::Bound::Included(end)) => {
                format!("between {:?} (exclusive) and {:?}", start, end)
            }
            (std::ops::Bound::Excluded(start), std::ops::Bound::Excluded(end)) => {
                format!("between {:?} (exclusive) and {:?} (exclusive)", start, end)
            }
            (std::ops::Bound::Unbounded, std::ops::Bound::Included(end)) => {
                format!("less than or equal to {:?}", end)
            }
            (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(end)) => {
                format!("less than {:?}", end)
            }
            (std::ops::Bound::Included(start), std::ops::Bound::Unbounded) => {
                format!("greater than or equal to {:?}", start)
            }
            (std::ops::Bound::Excluded(start), std::ops::Bound::Unbounded) => {
                format!("greater than {:?}", start)
            }
            (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded) => "any value".to_string(),
        }
    }
}

impl<R, T> TypedMatcher<T> for Between<R, T>
where
    T: PartialOrd + Debug,
    R: RangeBounds<T>,
{
    fn matcher_type(&self) -> MatchType {
        ToBe
    }
}
