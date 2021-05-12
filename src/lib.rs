pub extern crate atomic_take;
pub extern crate inventory;

use atomic_take::AtomicTake;

pub struct BootBox {
  pub boot_fn: AtomicTake<Box<dyn FnOnce()>>,
}

inventory::collect!(BootBox);

// Iterate over all boot functions as captured by booter::call_on_boot
pub fn boot() {
  for boot_box in inventory::iter::<BootBox> {
    if let Some(boot_fn) = boot_box.boot_fn.take() {
      boot_fn();
    }
  }
}

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

  static BOOT_CALLED: AtomicBool = AtomicBool::new(false);

  call_on_boot!({
    BOOT_CALLED.store(true, Ordering::SeqCst);
  });

  #[test]
  fn it_boots() {
    boot();
    assert!(BOOT_CALLED.load(Ordering::SeqCst));
  }
}
