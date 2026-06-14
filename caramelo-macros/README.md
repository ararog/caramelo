# caramelo-macros

[![Crates.io downloads](https://img.shields.io/crates/d/caramelo-macros)](https://crates.io/crates/caramelo-macros) [![crates.io](https://img.shields.io/crates/v/caramelo-macros?style=flat-square)](https://crates.io/crates/caramelo-macros) [![Build Status](https://github.com/ararog/caramelo-macros/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/caramelo-macros/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/caramelo-macros) [![Documentation](https://docs.rs/caramelo-macros/badge.svg)](https://docs.rs/caramelo-macros/latest/caramelo-macros) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/caramelo-macros/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/ararog/caramelo-macros/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/ararog/caramelo-macros)

Do you find yourself writing a lot of repetitive test code? Do you want to make your tests more readable and easier to understand? If so, caramelo-macros is for you!

## Available macros

- `dry_match!` - A macro that allows you to write tests in a more readable way. Keep in mind your struct must implement accessor methods that return the same type as the field.

Type less, test more!

### Features

- **Match only what matters**: Focus on the fields you care about, ignore the rest
- **Simple syntax**: Use `==`, `>`, `<`, `>=`, `<=`, `!=`, and `~` (regex) operators
- **Range matching**: Use `..=` for inclusive ranges (e.g., `25..=35`)
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