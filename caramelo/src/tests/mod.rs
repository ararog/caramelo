use crate::{expect, matchers::eq};

#[test]
fn test_equal() {
    expect(1).to_be(eq(1));
}

#[test]
#[should_panic(expected = "Expected 1 to be equals to 2")]
fn test_not_equal() {
    expect(1).to_be(eq(2));
}
