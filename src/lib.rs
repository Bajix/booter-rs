//! This crate allows a simple means to register and call one time initialization functions, the idea being this could be used in conjunction with [static_init](https://crates.io/crates/static_init) or std::mem::MaybeUninit in order to create statics that can be initalized once post-main after Tokio is online and the enviroment configured. 
//! 
//! ```rust
//! booter::call_on_boot!({
//!   println("Hello World!");
//! });
//! 
//! fn main() {
//!   booter::boot();
//!   booter::assert_booted();
//! }
//! ```

#[doc(hidden)]
pub extern crate atomic_take;

#[doc(hidden)]
pub extern crate inventory;

use atomic_take::AtomicTake;
use std::sync::atomic::{AtomicBool, Ordering};

#[doc(hidden)]
pub struct BootBox {
  pub boot_fn: AtomicTake<Box<dyn FnOnce()>>,
}

inventory::collect!(BootBox);

#[cfg(debug_assertions)]
static BOOT_CALLED: AtomicBool = AtomicBool::new(false);

/// Call all functions captured by booter::call_on_boot.
pub fn boot() {
  if cfg!(debug_assertions) {
    assert_eq!(
      BOOT_CALLED.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed),
      Ok(false),
      "booter::boot should be called exactly once after env setup"
    );
  }

  for boot_box in inventory::iter::<BootBox> {
    if let Some(boot_fn) = boot_box.boot_fn.take() {
      boot_fn();
    }
  }
}

/// Development assertion to ensure booter::boot called exactly once. Release builds skip check
pub fn assert_booted() {
  if cfg!(debug_assertions) {
    assert_eq!(
      BOOT_CALLED.load(Ordering::Acquire),
      true,
      "booter::boot should be called exactly once after env setup"
    );
  }
}

/// Register FnOnce to be called on booter::boot
#[macro_export]
macro_rules! call_on_boot {
  ($boot_fn:block) => {
    use $crate::{atomic_take::AtomicTake, inventory, BootBox};

    inventory::submit! {
      BootBox {
        boot_fn: AtomicTake::new(Box::new(|| $boot_fn))
      }
    }
  };
}

#[cfg(test)]
mod tests {
  use super::*;

  use std::sync::atomic::{AtomicBool, Ordering};

  static CALLBACK_CALLED: AtomicBool = AtomicBool::new(false);

  call_on_boot!({
    CALLBACK_CALLED.store(true, Ordering::Release);
  });

  #[test]
  #[should_panic(expected = "booter::boot should be called exactly once after env setup")]
  fn it_asserts_booter_booted() {
    BOOT_CALLED.store(false, Ordering::Release);
    assert_booted();
  }

  #[test]
  fn it_boots() {
    boot();
    assert!(CALLBACK_CALLED.load(Ordering::Acquire));
  }
}
