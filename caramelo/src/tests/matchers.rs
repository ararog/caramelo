use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

use crate::{
    expect,
    matchers::{
        _in_, any, contains, custom, each, empty, eq, err, ge, gt, item, le, len, length, lt, ne,
        none, ok, range, some,
    },
    MatchType::{To, ToBe},
    MatcherExt, TypedMatcher,
};

#[test]
fn test_init_eq() {
    let be_eq = eq(2);
    expect(be_eq.matcher_type()).to_be(eq(ToBe));
}

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
fn test_init_ne() {
    let be_ne = ne(2);
    expect(be_ne.matcher_type()).to_be(eq(To));
}

#[test]
fn test_not_equal() {
    expect(1).to(ne(2));
}

#[test]
#[should_panic(expected = "Expected 1 to not equal to 1")]
fn test_not_equal_failure() {
    expect(1).to(ne(1));
}

#[test]
fn test_init_gt() {
    let be_gt = gt(2);
    expect(be_gt.matcher_type()).to_be(eq(ToBe));
}

#[test]
fn test_greater_than() {
    expect(1).to_be(gt(0));
}

#[test]
#[should_panic(expected = "Expected 0 to be greater than 1")]
fn test_not_greater_than() {
    expect(0).to_be(gt(1));
}

#[test]
fn test_init_ge() {
    let be_ge = ge(2);
    expect(be_ge.matcher_type()).to_be(eq(ToBe));
}

#[test]
fn test_greater_than_or_equal() {
    expect(1).to_be(ge(1));
}

#[test]
#[should_panic(expected = "Expected 0 to be greater than or equal to 1")]
fn test_not_greater_than_or_equal() {
    expect(0).to_be(ge(1));
}

#[test]
fn test_less_than() {
    expect(1).to_be(lt(2));
}

#[test]
#[should_panic(expected = "Expected 2 to be less than 1")]
fn test_not_less_than() {
    expect(2).to_be(lt(1));
}

#[test]
fn test_init_le() {
    let be_le = le(2);
    expect(be_le.matcher_type()).to_be(eq(ToBe));
}

#[test]
fn test_less_than_or_equal() {
    expect(2).to_be(le(2));
}

#[test]
#[should_panic(expected = "Expected 3 to be less than or equal to 2")]
fn test_not_less_than_or_equal() {
    expect(3).to_be(le(2));
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
#[should_panic(expected = "Expected [\"a\", \"b\", \"c\"] to have item d")]
fn test_item_string_not_in_list() {
    expect(vec!["a", "b", "c"]).to_have(item("d"));
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
#[should_panic(expected = "Invalid regex pattern")]
fn test_contains_invalid_regex() {
    expect("hello").to(contains(r"\[[/w+"));
}

#[test]
#[should_panic(expected = "Expected \"hello\" to contains Regex(\"xyz\")")]
fn test_contains_failure() {
    expect("hello").to(contains("xyz"));
}

#[test]
fn test_any() {
    expect(1).to_be(any::<i32>());
}

#[test]
fn test_not_any_char() {
    // This one wont compile because 'a' is a char, not an i32
    // expect('a').to_be(any::<char>());
}

#[test]
fn test_in_range_inc() {
    expect(5).to_be(range(1..=10));
}

#[test]
#[should_panic(expected = "Expected 1 to be between 2 and 10")]
fn test_not_in_range_inc() {
    expect(1).to_be(range(2..=10));
}

#[test]
fn test_in_range_to() {
    expect(5).to_be(range(1..10));
}

#[test]
#[should_panic(expected = "Expected 10 to be between 2 and 10 (exclusive)")]
fn test_not_in_range_to() {
    expect(10).to_be(range(2..10));
}

#[test]
fn test_hashmap_length() {
    let mut map = HashMap::new();
    map.insert("key", "value");
    expect(map).to_have(length(1));
}

#[test]
#[should_panic(expected = "Expected {\"key\": \"value\"} to have length equals to 2")]
fn test_hashmap_length_failure() {
    let mut map = HashMap::new();
    map.insert("key", "value");
    expect(map).to_have(length(2));
}

#[test]
fn test_btreemap_length() {
    let mut map = BTreeMap::new();
    map.insert("key", "value");
    expect(map).to_have(length(1));
}

#[test]
#[should_panic(expected = "Expected {\"key\": \"value\"} to have length equals to 2")]
fn test_btreemap_length_failure() {
    let mut map = BTreeMap::new();
    map.insert("key", "value");
    expect(map).to_have(length(2));
}

#[test]
fn test_hashset_length() {
    let mut set = HashSet::new();
    set.insert("key");
    expect(set).to_have(length(1));
}

#[test]
#[should_panic(expected = "Expected {\"key\"} to have length equals to 2")]
fn test_hashset_length_failure() {
    let mut set = HashSet::new();
    set.insert("key");
    expect(set).to_have(length(2));
}

#[test]
fn test_btreehashset_length() {
    let mut set = BTreeSet::new();
    set.insert("key");
    expect(set).to_have(length(1));
}

#[test]
#[should_panic(expected = "Expected {\"key\"} to have length equals to 2")]
fn test_btreehashset_length_failure() {
    let mut set = BTreeSet::new();
    set.insert("key");
    expect(set).to_have(length(2));
}

#[test]
fn test_vecdeque_length() {
    let mut set = VecDeque::new();
    set.push_back("key");
    expect(set).to_have(length(1));
}

#[test]
#[should_panic(expected = "Expected [\"key\"] to have length equals to 2")]
fn test_vecdeque_length_failure() {
    let mut set = VecDeque::new();
    set.push_back("key");
    expect(set).to_have(length(2));
}

#[test]
fn test_linkedlist_length() {
    let mut set = LinkedList::new();
    set.push_back("key");
    expect(set).to_have(length(1));
}

#[test]
#[should_panic(expected = "Expected [\"key\"] to have length equals to 2")]
fn test_linkedlist_length_failure() {
    let mut set = LinkedList::new();
    set.push_back("key");
    expect(set).to_have(length(2));
}

#[test]
fn test_and() {
    expect(5).to_be(gt(1).and(lt(6)));
}

#[test]
#[should_panic(expected = "Matcher must be a 'to' matcher")]
fn test_to_failure() {
    expect(5).to(gt(1));
}

#[test]
#[should_panic(expected = "Matcher must be a 'to be' matcher")]
fn test_to_be_failure() {
    expect("John").to_be(contains("John"));
}

#[test]
#[should_panic(expected = "Matcher must be a 'to have' matcher")]
fn test_to_have_failure() {
    expect(5).to_have(gt(1));
}

#[test]
#[should_panic(expected = "Expected 5 to be greater than 1 and less than 4")]
fn test_and_failure() {
    expect(5).to_be(gt(1).and(lt(4)));
}

#[test]
fn test_or() {
    expect(5).to_be(gt(10).or(lt(6)));
}

#[test]
#[should_panic(expected = "Expected 5 to be greater than 10 or less than 4")]
fn test_or_failure() {
    expect(5).to_be(gt(10).or(lt(4)));
}

#[test]
fn test_in_vec() {
    expect(5).to_be(_in_(vec![1, 2, 3, 4, 5]));
}

#[test]
#[should_panic(expected = "Expected 6 to be in [1, 2, 3, 4, 5]")]
fn test_in_vec_failure() {
    expect(6).to_be(_in_(vec![1, 2, 3, 4, 5]));
}

#[test]
fn test_in_i32_hashset() {
    let set = HashSet::from([1, 2, 3, 4, 5]);
    expect(5).to_be(_in_(set));
}

#[test]
fn test_in_str_hashset() {
    let set = HashSet::from(["1", "2", "3", "4", "5"]);
    expect("1").to_be(_in_(set));
}

#[test]
fn test_init_some() {
    let be_some = some::<i32>();
    expect(be_some.matcher_type()).to_be(eq(ToBe));
}

#[test]
fn test_some() {
    expect(Some(1)).to_be(some());
}

#[test]
#[should_panic(expected = "Expected Some(1) to be none")]
fn test_some_failure() {
    expect(Some(1)).to_be(none());
}

#[test]
fn test_init_none() {
    let be_none = none::<i32>();
    expect(be_none.matcher_type()).to_be(eq(ToBe));
}

#[test]
fn test_none() {
    expect(None).to_be(none::<i32>());
}

#[test]
#[should_panic(expected = "Expected None to be some")]
fn test_none_failure() {
    expect(None).to_be(some::<i32>());
}

#[test]
fn test_init_ok() {
    let be_ok = ok::<i32, &str>();
    expect(be_ok.matcher_type()).to_be(eq(ToBe));
}

#[test]
fn test_ok() {
    expect(Ok::<i32, &str>(5)).to_be(ok());
}

#[test]
#[should_panic(expected = "Expected Err(\"error\") to be ok")]
fn test_ok_failure() {
    expect(Err::<i32, &str>("error")).to_be(ok());
}

#[test]
fn test_init_err() {
    let be_err = err::<i32, &str>();
    expect(be_err.matcher_type()).to_be(eq(ToBe));
}

#[test]
fn test_err() {
    expect(Err::<i32, &str>("error")).to_be(err());
}

#[test]
#[should_panic(expected = "Expected Ok(5) to be err")]
fn test_err_failure() {
    expect(Ok::<i32, &str>(5)).to_be(err());
}

#[test]
fn test_custom() {
    let is_even = custom(|x: &i32| x % 2 == 0, "even number");
    expect(4).to_have(is_even);
}

#[test]
#[should_panic(expected = "Expected 3 to have even number")]
fn test_custom_failure() {
    let is_even = custom(|x: &i32| x % 2 == 0, "even number");
    expect(3).to_have(is_even);
}

#[test]
fn test_equals_str_ref() {
    let data = Some(&"hello".to_string());
    #[allow(clippy::unnecessary_literal_unwrap)]
    expect(data.unwrap()).to_be(eq("hello"));
}

#[test]
fn test_each() {
    expect(vec![1, 2, 3, 4, 5]).to_have(each(gt(0).and(le(6))));
}

#[test]
#[should_panic(
    expected = "Expected [1, 2, 3, 4, 5] to have element 4 that matches greater than 0 and less than 4"
)]
fn test_each_failure() {
    expect(vec![1, 2, 3, 4, 5]).to_have(each(gt(0).and(lt(4))));
}
