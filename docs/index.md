---
layout: default
title: Caramelo - The elegant full-stack web framework
nav_order: 1
description: "🌟 The elegant full-stack web framework that makes Rust web development effortless"
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
caramelo = { version = "0.1.0-beta.1" }
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

## Examples

Check out the [examples](./examples.md) for complete examples of how to use Caramelo in your projects.

## Documentation

- [API Reference](https://docs.rs/caramelo)
- [Contributing Guide](./CONTRIBUTING.md)

## License

This project is licensed under the [MIT License](./LICENSE.md).

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
