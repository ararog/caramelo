# caramelo

[![Crates.io downloads](https://img.shields.io/crates/d/caramelo)](https://crates.io/crates/caramelo) [![crates.io](https://img.shields.io/crates/v/caramelo?style=flat-square)](https://crates.io/crates/caramelo) [![Build Status](https://github.com/ararog/caramelo/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/caramelo/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/caramelo) [![Documentation](https://docs.rs/caramelo/badge.svg)](https://docs.rs/caramelo/latest/caramelo) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/caramelo/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/ararog/caramelo/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/ararog/caramelo)

**Caramelo** (caramel in Portuguese) is a comprehensive, production-ready unit testing framework for Rust with a focus on simplicity and ease of use.

## Quick Start

Add caramelo to your `Cargo.toml`:

```toml
caramelo = { version = "0.1.0", features = ["caramelo"] }
```

## Usage Example

Here's how simple it is to create unit tests with caramelo:

```rust
use caramelo::{Equal, expect};

#[test]
fn test_equal() {
    expect(1).to_be(Equal(1));
}

#[test]
#[should_panic(expected = "Expected 1 to be equals to 2")]
fn test_not_equal() {
    expect(1).to_be(Equal(2));
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
