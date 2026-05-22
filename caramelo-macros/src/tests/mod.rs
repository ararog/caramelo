use crate::{eq, ge, gt, is, le, lt, ne};

#[test]
fn test_expect_some() {
    let value = Some(1);
    is!(&value; Some(1));
}

#[test]
fn test_expect_none() {
    let value: Option<i32> = None;
    is!(&value; None);
}

#[test]
fn test_expect_ok() {
    let value: Result<i32, i32> = Ok(1);
    is!(&value; Ok(1));
}

#[test]
fn test_expect_error() {
    let value: Result<i32, i32> = Err(1);
    is!(&value; Err(1));
}

#[test]
fn test_expect_eq() {
    let value = 1;
    eq!(&value; &1);
}

#[test]
#[should_panic(expected = "Expected 1 to be equal to 2")]
fn test_expect_eq_panic() {
    let value = 1;
    eq!(&value; &2);
}

#[test]
fn test_expect_ne() {
    let value = 1;
    ne!(&value; &2);
}

#[test]
#[should_panic(expected = "Expected 1 to be not equal to 1")]
fn test_expect_ne_panic() {
    let value = 1;
    ne!(&value; &1);
}

#[test]
fn test_expect_lt() {
    let value = 1;
    lt!(&value; &2);
}

#[test]
#[should_panic(expected = "Expected 1 to be less than 1")]
fn test_expect_lt_panic() {
    let value = 1;
    lt!(&value; &1);
}

#[test]
fn test_expect_le() {
    let value = 1;
    le!(&value; &1);
}

#[test]
#[should_panic(expected = "Expected 2 to be less than or equal to 1")]
fn test_expect_le_panic() {
    let value = 2;
    le!(&value; &1);
}

#[test]
fn test_expect_gt() {
    let value = 2;
    gt!(&value; &1);
}

#[test]
#[should_panic(expected = "Expected 1 to be greater than 2")]
fn test_expect_gt_panic() {
    let value = 1;
    gt!(&value; &2);
}

#[test]
fn test_expect_ge() {
    let value = 2;
    ge!(&value; &2);
}

#[test]
#[should_panic(expected = "Expected 1 to be greater than or equal to 2")]
fn test_expect_ge_panic() {
    let value = 1;
    ge!(&value; &2);
}
