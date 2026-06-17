use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

use crate::{
    expect,
    matchers::{
        _in_, and, any, contains, empty, eq, ge, gt, in_exc, in_inc, item, le, len, length, lt, ne,
        or,
    },
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
fn test_not_equal() {
    expect(1).to(ne(2));
}

#[test]
#[should_panic(expected = "Expected 1 to not equal to 1")]
fn test_not_equal_failure() {
    expect(1).to(ne(1));
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
    expect(5).to_be(in_inc(1, 10));
}

#[test]
#[should_panic(expected = "Expected 1 to be between 2 and 10")]
fn test_not_in_range_inc() {
    expect(1).to_be(in_inc(2, 10));
}

#[test]
fn test_in_range_to() {
    expect(5).to_be(in_exc(1, 10));
}

#[test]
#[should_panic(expected = "Expected 10 to be between 2 and 10 (exclusive)")]
fn test_not_in_range_to() {
    expect(10).to_be(in_exc(2, 10));
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
    expect(5).to_self(and(vec![Box::new(gt(1)), Box::new(lt(6))]));
}

#[test]
#[should_panic(expected = "Expected 5 to be greater than 1 and less than 4")]
fn test_and_failure() {
    expect(5).to_self(and(vec![Box::new(gt(1)), Box::new(lt(4))]));
}

#[test]
fn test_or() {
    expect(5).to_self(or(vec![Box::new(gt(10)), Box::new(lt(6))]));
}

#[test]
#[should_panic(expected = "Expected 5 to be greater than 10 or less than 4")]
fn test_or_failure() {
    expect(5).to_self(or(vec![Box::new(gt(10)), Box::new(lt(4))]));
}

#[test]
fn test_in() {
    expect(5).to_be(_in_(vec![1, 2, 3, 4, 5]));
}

#[test]
#[should_panic(expected = "Expected 6 to be in [1, 2, 3, 4, 5]")]
fn test_in_failure() {
    expect(6).to_be(_in_(vec![1, 2, 3, 4, 5]));
}
