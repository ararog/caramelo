#[cfg(test)]
mod tests;

#[macro_export]
/// Macro for equality assertions
///
/// # Arguments
///
/// * `value` - The value to assert
/// * `expected` - The expected value
///
/// # Panics
///
/// Panics if the value is not equal to the expected value
///
/// # Examples
///
/// ```
/// use caramelo_macros::is;
///
/// let value = Some(1);
/// is!(&value; Some(1));
/// ```
macro_rules! is {
    ($value:expr; $expected:expr) => {
        caramelo::assertions::Is::is($value, $expected)
    };
}

#[macro_export]
/// Macro for equality assertions
///
/// # Arguments
///
/// * `value` - The value to assert
/// * `expected` - The expected value
///
/// # Panics
///
/// Panics if the value is not equal to the expected value
///
/// # Examples
///
/// ```
/// use caramelo_macros::eq;
///
/// let value = 1;
/// eq!(&value; &1);
/// ```
macro_rules! eq {
    ($value:expr; $expected:expr) => {
        caramelo::assertions::IsEq::is_eq($value, $expected)
    };
}

#[macro_export]
/// Macro for inequality assertions
///
/// # Arguments
///
/// * `value` - The value to assert
/// * `expected` - The expected value
///
/// # Panics
///
/// Panics if the value is equal to the expected value
///
/// # Examples
///
/// ```
/// use caramelo_macros::ne;
///
/// let value = 1;
/// ne!(&value; &2);
/// ```
macro_rules! ne {
    ($value:expr; $expected:expr) => {
        caramelo::assertions::IsNe::is_ne($value, $expected)
    };
}

#[macro_export]
/// Macro for less than assertions
///
/// # Arguments
///
/// * `value` - The value to assert
/// * `expected` - The expected value
///
/// # Panics
///
/// Panics if the value is not less than the expected value
///
/// # Examples
///
/// ```
/// use caramelo_macros::lt;
///
/// let value = 1;
/// lt!(&value; &2);
/// ```
macro_rules! lt {
    ($value:expr; $expected:expr) => {
        caramelo::assertions::IsLt::is_lt($value, $expected)
    };
}

#[macro_export]
/// Macro for less than or equal assertions
///
/// # Arguments
///
/// * `value` - The value to assert
/// * `expected` - The expected value
///
/// # Panics
///
/// Panics if the value is not less than or equal to the expected value
///
/// # Examples
///
/// ```
/// use caramelo_macros::le;
///
/// let value = 1;
/// le!(&value; &1);
/// ```
macro_rules! le {
    ($value:expr; $expected:expr) => {
        caramelo::assertions::IsLe::is_le($value, $expected)
    };
}

#[macro_export]
/// Macro for greater than assertions
///
/// # Arguments
///
/// * `value` - The value to assert
/// * `expected` - The expected value
///
/// # Panics
///
/// Panics if the value is not greater than the expected value
///
/// # Examples
///
/// ```
/// use caramelo_macros::gt;
///
/// let value = 2;
/// gt!(&value; &1);
/// ```
macro_rules! gt {
    ($value:expr; $expected:expr) => {
        caramelo::assertions::IsGt::is_gt($value, $expected)
    };
}

#[macro_export]
/// Macro for greater than or equal assertions
///
/// # Arguments
///
/// * `value` - The value to assert
/// * `expected` - The expected value
///
/// # Panics
///
/// Panics if the value is not greater than or equal to the expected value
///
/// # Examples
///
/// ```
/// use caramelo_macros::ge;
///
/// let value = 2;
/// ge!(&value; &2);
/// ```
macro_rules! ge {
    ($value:expr; $expected:expr) => {
        caramelo::assertions::IsGe::is_ge($value, $expected)
    };
}
