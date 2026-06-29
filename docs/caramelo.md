---
layout: default
title: Caramelo - Unit testing framework
nav_order: 2
---

## Caramelo

**Caramelo** is a comprehensive, production-ready unit testing framework for Rust with a focus on simplicity and ease of use.

## Installation

Add Caramelo to your `Cargo.toml`:

```toml
[dependencies]
caramelo = { version = "0.1.0" }
```

## Usage

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

## API Reference

For detailed API documentation, see the [docs.rs page](https://docs.rs/caramelo).
