use crate::{expect, Equal};

#[test]
fn test_equal() {
    expect(1).to_be(Equal(1));
}

#[test]
#[should_panic(expected = "Expected 1 to be equals to 2")]
fn test_not_equal() {
    expect(1).to_be(Equal(2));
}
