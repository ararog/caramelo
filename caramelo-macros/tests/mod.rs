use caramelo_macros::dry_match;

struct User {
    name: String,
    age: u32,
}

impl User {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn age(&self) -> u32 {
        self.age
    }
}

#[test]
fn test_caramelo() {
    let user = User { name: "John".to_string(), age: 30 };

    dry_match!(user is { name: eq "John", age: gt 29 });
}

#[test]
fn test_caramelo_bw() {
    let user = User { name: "John".to_string(), age: 30 };

    dry_match!(user is { age: bw (25, 35) });
}

#[test]
fn test_caramelo_re() {
    let user = User { name: "John".to_string(), age: 30 };

    dry_match!(user is { name: re ".*hn" });
}

#[test]
#[should_panic = "Expected 30 to be greater than 32"]
fn test_caramelo_failure() {
    let user = User { name: "John".to_string(), age: 30 };

    dry_match!(user is { name: eq "John", age: gt 32 });
}
