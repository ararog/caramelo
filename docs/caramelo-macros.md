---
layout: default
title: Caramelo Macros - Unit testing macros
nav_order: 3
---

## Caramelo Macros

**Caramelo Macros** is a collection of macros for unit testing in Rust.

## Available macros

- `dry_match!` - A macro that allows you to write tests in a more readable way. Keep in mind your struct must implement accessor methods that return the same type as the field.

### Rationale

`dry_match!` primary goal is to reduce boilerplate code in tests by allowing you to match only the fields you care about, ignoring the rest.

So instead of:

```rust
assert_eq!(user.name, "John");
assert_eq!(user.age, 30);
```

You can write:

```rust
dry_match!(user, User { name: == "John", age: == 30 });
```

Well, this is a simple example, but you can do much more with `dry_match!`. For example, you can use ranges, pipes, and regex patterns.

Where you usualy write:

```rust
assert!(user.age == 18 || user.age == 65);
```

You can write:

```rust
dry_match!(user, User { age: 18 | 65 });
```

You can also match nested structures:

```rust
struct Address {
    street: String,
    city: String,
}

impl Address {
    pub fn city(&self) -> &str {
        &self.city
    }
}

struct User {
    name: String,
    age: u32,
    address: Address,
}

impl User {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn address(&self) -> &Address {
        &self.address
    }
}

#[test]
fn test_nested() {
    let user = User {
        name: "John".to_string(),
        age: 30,
        address: Address {
            street: "123 Main St".to_string(),
            city: "Anytown".to_string(),
        },
    };

    dry_match!(user, User {
        name: == "John",
        age: == 30,
        address.city: == "Anytown"
    });

    // but you can also have method calls

    dry_match!(user, User {
        name: == "John",
        age: == 30,
        address.city(): == "Anytown"
    });

    // which is usually written like this:

    assert_eq!(user.name(), "John");
    assert_eq!(user.age(), 30);
    assert_eq!(user.address().city(), "Anytown");
}
```

### Features

- **Match only what matters**: Focus on the fields you care about, ignore the rest
- **Simple syntax**: Use `==`, `>`, `<`, `>=`, `<=`, `!=`, and `~` (regex) operators
- **Range matching**: Use `1..=2, 1..2, 1.., ..2 and ..=2` for inclusive and exclusive ranges
- **Piped matching**: Use `|` to match any of the values (e.g., `age: 25 | 30`)
- **Regex matching**: Use `~` for pattern matching (e.g., `~ ".*hn"`)
- **Combine operators**: You can combine operators with and/or operators (e.g., `age: > 30 and < 50`)
- **Custom matching**: Use closures for custom matching logic (e.g., `age: |&value| { value >= 30 }`)
- **Clear error messages**: Panics with descriptive messages when expectations aren't met

## Installation

Add Caramelo Macros to your `Cargo.toml`:

```toml
[dependencies]
caramelo = { version = "0.1.0" }
caramelo-macros = { version = "0.1.0" }
```

## Usage

```rust
use caramelo_macros::dry_match;

struct User {
    name: String,
    age: u32,
}

// Keep in mind accessor methods must
// return the same type as the field
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
    let user = User { name: "John", age: 30 };

    dry_match!(user, User { name: == "John", age: > 32 });
}
```

## API Reference

For detailed API documentation, see the [docs.rs page](https://docs.rs/caramelo-macros).
