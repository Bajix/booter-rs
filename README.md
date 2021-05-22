# Booter

![License](https://img.shields.io/badge/license-MIT-green.svg)
[![Cargo](https://img.shields.io/crates/v/booter.svg)](https://crates.io/crates/booter)
[![Documentation](https://docs.rs/booter/badge.svg)](https://docs.rs/booter)

This crate allows a simple means to register and call initialization functions

```rust
booter::call_on_boot!({
  println("Hello World!");
});

fn main() {
  booter::boot();
  booter::assert_booted();
}
```
