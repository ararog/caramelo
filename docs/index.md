---
layout: default
title: Caramelo - Unit testing framework
nav_order: 1
description: "Caramelo is a comprehensive, production-ready unit testing framework for Rust with a focus on simplicity and ease of use."
permalink: /
---
<div align="center">
<h1><b>Caramelo</b></h1>
</div>

[![Crates.io downloads](https://img.shields.io/crates/d/caramelo)](https://crates.io/crates/caramelo) [![crates.io](https://img.shields.io/crates/v/caramelo?style=flat-square)](https://crates.io/crates/caramelo) [![Build Status](https://github.com/ararog/caramelo/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/caramelo/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/caramelo) [![Documentation](https://docs.rs/caramelo/badge.svg)](https://docs.rs/caramelo/latest/caramelo) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/caramelo/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/ararog/caramelo/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/ararog/caramelo)

**Caramelo** (caramel in Portuguese) is a comprehensive, production-ready unit testing framework for Rust with a focus on simplicity and ease of use.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
caramelo = { version = "0.1.0" }
```

Basic usage:

```rust
use caramelo::{expect, matchers::eq};

#[test]
fn test_equal() {
    expect(1).to_be(eq(1));
}

#[test]
#[should_panic(expected = "Expected 1 to be equals to 2")]
fn test_not_equal() {
    expect(1).to_be(eq(2));
}
```

## Crates

| Crate | Description | Documentation |
|-------|-------------|---------------|
| [caramelo](./caramelo) | Core testing framework | [![docs.rs](https://img.shields.io/docsrs/caramelo/latest)](https://docs.rs/caramelo) |
| [caramelo-macros](./caramelo-macros) | Convenience macros | [![docs.rs](https://img.shields.io/docsrs/caramelo-macros/latest)](https://docs.rs/caramelo-macros) |

## Examples

Check out the [examples](./examples.md) for complete examples of how to use Caramelo in your projects.

## Documentation

- [Code of Conduct](./CODE_OF_CONDUCT.md)
- [Contributing Guide](./CONTRIBUTING.md)

## Other Projects

- [deboa](https://crates.io/crates/deboa) - HTTP client
- [easyhttpmock](https://crates.io/crates/easyhttpmock) - HTTP mock server
- [sofie](https://crates.io/crates/sofie) - Fullstack web framework
- [uget](https://crates.io/crates/uget) - CLI HTTP client
- [vetis](https://crates.io/crates/vetis) - Very Tiny Http server

## License

This project is licensed under the [MIT License](./LICENSE.md).

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
