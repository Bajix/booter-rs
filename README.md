# Booter

![License](https://img.shields.io/badge/license-MIT-green.svg)
[![Cargo](https://img.shields.io/crates/v/booter.svg)](https://crates.io/crates/booter)
[![Documentation](https://docs.rs/booter/badge.svg)](https://docs.rs/booter)

This crate allows a simple means to register and call one time initialization functions, the idea being this could be used in conjunction with [static_init](https://crates.io/crates/static_init) in order to create statics that can created pre-main and initalized post-main once Tokio is online and the enviroment configured.

```rust
booter::call_on_boot!({
  println("Hello World!");
});

fn main() {
  booter::boot();
}
```
