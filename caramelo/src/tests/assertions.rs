use crate::assertions::{Has, HasItem, HasPat, Is, IsEq, IsGe, IsGt, IsLe, IsLt, IsNe};

#[test]
fn test_has() {
    "Hello".has(&"llo");
}

#[test]
#[should_panic(expected = "Expected Hello to contain xyz")]
fn test_has_panic() {
    "Hello".has(&"xyz");
}

#[test]
fn test_has_item() {
    let vec = vec![1, 2, 3];
    vec.has_item(&2);
}

#[test]
#[should_panic(expected = "Expected [1, 2, 3] to contains 4")]
fn test_has_no_item() {
    let vec = vec![1, 2, 3];
    vec.has_item(&4);
}

#[test]
fn test_is_none() {
    let none: Option<i32> = None;
    none.is(None);
}

#[test]
#[should_panic(expected = "Expected None to be Some(1)")]
fn test_is_none_panic() {
    let none: Option<i32> = None;
    none.is(Some(1));
}

#[test]
fn test_is_some() {
    let some: Option<i32> = Some(1);
    some.is(Some(1));
}

#[test]
#[should_panic(expected = "Expected Some(1) to be None")]
fn test_is_some_panic() {
    let some: Option<i32> = Some(1);
    some.is(None);
}

#[test]
fn test_is_ok() {
    let ok: Result<i32, i32> = Ok(1);
    ok.is(Ok(1));
}

#[test]
#[should_panic(expected = "Expected Ok(1) to be Err(1)")]
fn test_is_ok_panic() {
    let ok: Result<i32, i32> = Ok(1);
    ok.is(Err(1));
}

#[test]
fn test_is_err() {
    let err: Result<i32, i32> = Err(1);
    err.is(Err(1));
}

#[test]
#[should_panic(expected = "Expected Err(1) to be Ok(1)")]
fn test_is_err_panic() {
    let err: Result<i32, i32> = Err(1);
    err.is(Ok(1));
}

#[test]
fn test_eq() {
    "a".is_eq(&"a");
}

#[test]
#[should_panic(expected = "Expected a to be equal to b")]
fn test_eq_panic() {
    "a".is_eq(&"b");
}

#[test]
fn test_ne() {
    "a".is_ne(&"b");
}

#[test]
#[should_panic(expected = "Expected a to be not equal to a")]
fn test_ne_panic() {
    "a".is_ne(&"a");
}

#[test]
fn test_is_lt() {
    'a'.is_lt(&'b');
}

#[test]
#[should_panic(expected = "Expected b to be less than a")]
fn test_is_lt_panic() {
    'b'.is_lt(&'a');
}

#[test]
fn test_is_le_eq() {
    'a'.is_le(&'a');
}

#[test]
#[should_panic(expected = "Expected b to be less than or equal to a")]
fn test_is_le_panic() {
    'b'.is_le(&'a');
}

#[test]
fn test_is_ge() {
    'b'.is_ge(&'b');
}

#[test]
#[should_panic(expected = "Expected a to be greater than or equal to b")]
fn test_is_ge_panic() {
    'a'.is_ge(&'b');
}

#[test]
fn test_is_gt() {
    'b'.is_gt(&'a');
}

#[test]
#[should_panic(expected = "Expected a to be greater than b")]
fn test_is_gt_panic() {
    'a'.is_gt(&'b');
}

#[test]
fn test_has_pattern() {
    "hello".has_pat("ell");
}

#[test]
#[should_panic(expected = "Expected \"hello\" to match \"xyz\"")]
fn test_has_pattern_panic() {
    "hello".has_pat("xyz");
}
