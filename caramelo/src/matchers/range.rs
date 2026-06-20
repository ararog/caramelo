use crate::{
    MatchType::{self, ToBe},
    Matcher, TypedMatcher,
};
use std::{fmt::Debug, ops::RangeBounds};

/// Creates a matcher that matches values in a range (inclusive or exclusive)
/// It is the same as 1..10 in Rust
///
/// # Examples
///
/// ```
/// use caramelo::expect;
/// use caramelo::matchers::range;
///
/// expect(5).to_be(range(1..10));
/// ```
pub fn range<R, T>(range: R) -> Range<R, T>
where
    R: RangeBounds<T>,
{
    Range::new(range)
}

/// Matcher that matches values in a range (inclusive or exclusive)
pub struct Range<R, T> {
    range: R,
    _phantom: std::marker::PhantomData<T>,
}

impl<R, T> Range<R, T> {
    /// Creates a new Range matcher
    pub fn new(range: R) -> Self {
        Self { range, _phantom: std::marker::PhantomData }
    }
}

impl<R, T> Matcher<T> for Range<R, T>
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

impl<R, T> TypedMatcher<T> for Range<R, T>
where
    T: PartialOrd + Debug,
    R: RangeBounds<T>,
{
    fn matcher_type(&self) -> MatchType {
        ToBe
    }
}
