/// Trait for equality assertions
pub trait Is {
    /// Asserts that the value is equal to the expected value
    fn is(&self, other: Self);
}

impl<T> Is for Option<T>
where
    T: std::fmt::Debug,
{
    fn is(&self, other: Self) {
        match (self, other) {
            (None, Some(value)) => panic!("Expected None to be Some({:?})", value),
            (None, None) => {}
            (Some(value), None) => panic!("Expected Some({:?}) to be None", value),
            (Some(_), Some(_)) => {}
        }
    }
}

impl<T, E> Is for Result<T, E>
where
    T: std::fmt::Debug,
    E: std::fmt::Debug,
{
    fn is(&self, other: Self) {
        match (self, other) {
            (Err(err), Ok(value)) => panic!("Expected Err({:?}) to be Ok({:?})", err, value),
            (Ok(value), Err(err)) => panic!("Expected Ok({:?}) to be Err({:?})", value, err),
            _ => {}
        }
    }
}
