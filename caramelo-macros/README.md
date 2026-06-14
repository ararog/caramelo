# caramelo-macros

[![Crates.io downloads](https://img.shields.io/crates/d/caramelo-macros)](https://crates.io/crates/caramelo-macros) [![crates.io](https://img.shields.io/crates/v/caramelo-macros?style=flat-square)](https://crates.io/crates/caramelo-macros) [![Build Status](https://github.com/ararog/caramelo-macros/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/caramelo-macros/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/caramelo-macros) [![Documentation](https://docs.rs/caramelo-macros/badge.svg)](https://docs.rs/caramelo-macros/latest/caramelo-macros) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/caramelo-macros/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/ararog/caramelo-macros/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/ararog/caramelo-macros)

Do you find yourself writing a lot of repetitive test code? Do you want to make your tests more readable and easier to understand? If so, caramelo-macros is for you!

## Available macros

- `dry_match!` - A macro that allows you to write tests in a more readable way. Keep in mind your struct must implement accessor methods that return the same type as the field.

Type less, test more!

### Rationale

`dry_match!` primary goal is to reduce boilerplate code in tests by allowing you to match only the fields you care about, ignoring the rest.

So instead of:

```rust, compile_fail
assert_eq!(user.name, "John");
assert_eq!(user.age, 30);
```

You can write:

```rust, compile_fail
dry_match!(user, User { name: == "John", age: == 30 });
```

Well, this is a simple example, but you can do much more with `dry_match!`. For example, you can use ranges, pipes, and regex patterns.

Where you usualy write:

```rust, compile_fail
assert!(user.age == 18 || user.age == 65);
```

You can write:

```rust, compile_fail
dry_match!(user, User { age: 18 | 65 });
```

You can also match nested structures:

```rust, compile_fail
use caramelo_macros::drymatch;

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
- **Clear error messages**: Panics with descriptive messages when expectations aren't met

## Quick Start

Add caramelo-macros to your `Cargo.toml`:

```toml
caramelo-macros = { version = "0.1.0" }
```

## Usage Example

Here's how simple it is to create unit tests with caramelo-macros:

```rust,ignore
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
    let user = User { name: "John".to_string(), age: 30 };

    dry_match!(user, User { name: == "John", age: > 32 });
}
```

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  (LICENSE-MIT or https://opensource.org/licenses/MIT)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>