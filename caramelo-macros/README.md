# caramelo-macros

[![Crates.io downloads](https://img.shields.io/crates/d/caramelo-macros)](https://crates.io/crates/caramelo-macros) [![crates.io](https://img.shields.io/crates/v/caramelo-macros?style=flat-square)](https://crates.io/crates/caramelo-macros) [![Build Status](https://github.com/ararog/caramelo-macros/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/caramelo-macros/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/caramelo-macros) [![Documentation](https://docs.rs/caramelo-macros/badge.svg)](https://docs.rs/caramelo-macros/latest/caramelo-macros) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/caramelo-macros/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/ararog/caramelo-macros/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/ararog/caramelo-macros)

Do you find yourself writing a lot of repetitive test code? Do you want to make your tests more readable and easier to understand? If so, caramelo-macros is for you!

Type less, test more!

## Quick Start

Add caramelo-macros to your `Cargo.toml`:

```toml
caramelo-macros = { version = "0.1.0" }
```

## Usage Example

Here's how simple it is to create unit tests with caramelo-macros:

```rust
use caramelo_macros::{eq, is, lt, ne};

#[test]
fn test_expect_is() {
    let value = Some(1);
    is!(&value; Some(1));
}

#[test]
fn test_expect_eq() {
    let value = 1;
    eq!(&value; &1);
}

#[test]
fn test_expect_ne() {
    let value = 1;
    ne!(&value; &2);
}

#[test]
fn test_expect_lt() {
    let value = 1;
    lt!(&value; &2);
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
