use caramelo_macros::dry_match;

struct User {
    name: String,
    age: u32,
    city: City,
}

#[derive(Debug, PartialEq)]
struct City {
    name: String,
    state: String,
}

#[allow(dead_code)]
impl User {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn city(&self) -> &City {
        &self.city
    }
}

#[allow(dead_code)]
impl City {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn state(&self) -> &str {
        &self.state
    }
}

fn create_user() -> User {
    User {
        name: "John".to_string(),
        age: 30,
        city: City { name: "New York".to_string(), state: "NY".to_string() },
    }
}

#[test]
fn test_regular() {
    let user = create_user();
    dry_match!(user, User { name: == "John" and ~ "J.*", age: > 29 });
}

#[test]
fn test_range_inclusive() {
    let user = create_user();
    dry_match!(user, User { age: 25..=35 });
}

#[test]
#[should_panic = "Expected 30 to be between 25 and 30 (exclusive)"]
fn test_range_exclusive() {
    let user = create_user();
    dry_match!(user, User { age: 25..30 });
}

#[test]
fn test_range_end_inclusive() {
    let user = create_user();
    dry_match!(user, User { age: ..=30 });
}

#[test]
#[should_panic = "Expected 30 to be less than 30"]
fn test_range_end_exclusive() {
    let user = create_user();
    dry_match!(user, User { age: ..30 });
}

#[test]
#[should_panic = "Expected 30 to be greater than 30"]
fn test_range_start_inclusive() {
    let user = create_user();
    dry_match!(user, User { age: 30.. });
}

#[test]
fn test_regex() {
    let user = create_user();
    dry_match!(user, User { name: ~ ".*hn" });
}

#[test]
#[should_panic = "Expected 30 to be greater than 32"]
fn test_caramelo_failure() {
    let user = create_user();
    dry_match!(user, User { name: == "John", age: > 32 });
}

#[test]
fn test_nested() {
    let user = create_user();
    dry_match!(user, User { name: == "John", city.state(): == "NY" });
}

#[test]
fn test_pipe() {
    let user = create_user();
    dry_match!(user, User { name: "John" | "Jane" });
}

#[test]
fn test_pipe_integer() {
    let user = create_user();
    dry_match!(user, User { age: 25 | 30 });
}

#[test]
fn test_pipe_object() {
    let user = create_user();
    let new_york = City { name: "New York".to_string(), state: "NY".to_string() };
    let los_angeles = City { name: "Los Angeles".to_string(), state: "CA".to_string() };
    dry_match!(user, User { city: new_york | los_angeles });
}

#[test]
#[should_panic = "Expected \"John\" to be in [\"Michael\", \"Jane\"]"]
fn test_pipe_fail() {
    let user = create_user();
    dry_match!(user, User { name: "Michael" | "Jane" });
}
