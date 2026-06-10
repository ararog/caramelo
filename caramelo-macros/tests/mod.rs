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
#[should_panic = "Expected 30 to be greater than 32"]
fn test_caramelo() {
    let user = User { name: "John".to_string(), age: 30 };

    dry_match!(user is { name: == "John", age: > 32 });
}
