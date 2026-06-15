use caramelo_macros::dry_match;

struct User {
    name: String,
}

fn main() {
    let user = User { name: "John".to_string() };
    dry_match!(user, User { name: });
}