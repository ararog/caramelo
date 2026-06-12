use caramelo_macros::dry_match;

struct City {
    name: String,
    state: String,
}

struct User {
    name: String,
    age: u32,
    city: City,
}

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
    dry_match!(user is { name: == "John", age: > 29 });
}

#[test]
fn test_bw() {
    let user = create_user();
    dry_match!(user is { age: 25..=35 });
}

#[test]
fn test_regex() {
    let user = create_user();
    dry_match!(user is { name: ~ ".*hn" });
}

#[test]
#[should_panic = "Expected 30 to be greater than 32"]
fn test_caramelo_failure() {
    let user = create_user();
    dry_match!(user is { name: == "John", age: > 32 });
}

#[test]
fn test_nested() {
    let user = create_user();
    dry_match!(user is { name: == "John", .city.state(): == "NY" });
}
