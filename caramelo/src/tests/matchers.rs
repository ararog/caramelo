use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

use crate::{
    expect,
    matchers::{any, between, contains, empty, eq, ge, gt, item, le, len, length, lt, ne},
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
fn test_between() {
    expect(5).to_be(between(1, 10));
}

#[test]
#[should_panic(expected = "Expected 1 to be between 2 and 10")]
fn test_not_between() {
    expect(1).to_be(between(2, 10));
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
