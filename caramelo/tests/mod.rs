use caramelo::{expect, pat};

#[test]
#[should_panic(expected = "Expected 25 to have captured by pattern 5 | 10")]
fn test_match_with_in_failure() {
    let data = 25;
    expect(data).to_have(pat!(5 | 10));
}
