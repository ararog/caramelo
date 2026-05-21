use crate::{
    expect,
    matchers::{any, contains, empty, eq, ge, gt, item, le, len, length, lt, ne},
};

#[test]
fn test_equal() {
    expect(1).to_be(eq(1));
}

#[test]
#[should_panic(expected = "Expected 1 to be equals to 2")]
fn test_equal_failure() {
    expect(1).to_be(eq(2));
}

#[test]
#[should_panic(expected = "Expected 1 to be not equal to 1")]
fn test_not_equal() {
    expect(1).to_be(ne(1));
}

#[test]
fn test_greater_than() {
    expect(1).to_be(gt(0));
}

#[test]
fn test_greater_than_or_equal() {
    expect(1).to_be(ge(1));
}

#[test]
fn test_less_than() {
    expect(1).to_be(lt(2));
}

#[test]
fn test_less_than_or_equal() {
    expect(2).to_be(le(2));
}

#[test]
fn test_empty() {
    expect("").to_be(empty());
}

#[test]
fn test_empty_vec() {
    expect(Vec::<i32>::new()).to_be(empty());
}

#[test]
#[should_panic(expected = "Expected [1, 2, 3] to be empty")]
fn test_not_empty_vec() {
    expect(vec![1, 2, 3]).to_be(empty());
}

#[test]
fn test_item() {
    expect(vec![1, 2, 3]).to_have(item(2));
}

#[test]
fn test_item_letter() {
    expect(vec!['a', 'b', 'c']).to_have(item('b'));
}

#[test]
fn test_length() {
    expect(vec![1, 2, 3]).to_have(length(3));
}

#[test]
#[should_panic(expected = "Expected [1, 2, 3] to have length equals to 4")]
fn test_length_failure() {
    expect(vec![1, 2, 3]).to_have(len(4));
}

#[test]
fn test_any_int() {
    expect(1).to_be(any::<i32>());
}

#[test]
fn test_any_float() {
    expect(1.0).to_be(any::<f64>());
}

#[test]
fn test_any_char() {
    expect('a').to_be(any::<char>());
}

#[test]
fn test_any_string() {
    expect("hello").to_be(any::<&str>());
}

#[test]
fn test_contains() {
    expect("hello").to(contains("ell"));
}

#[test]
#[should_panic(expected = "Expected \"hello\" to contains Regex(\"xyz\")")]
fn test_contains_failure() {
    expect("hello").to(contains("xyz"));
}
