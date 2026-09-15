use {
  core::{
    hint::spin_loop,
    sync::atomic::{AtomicBool, Ordering}
  },
  lock_api::{GuardSend, RawMutex}
};

pub struct InnerSpinLock(AtomicBool);

pub type SpinLock<T> = lock_api::Mutex<InnerSpinLock, T>;
pub type SpinLockGuard<'a, T> = lock_api::MutexGuard<'a, InnerSpinLock, T>;

cfg_if! {
    if #[cfg(feature = "use_futex_lock")] {
        compiler_error!("Futex-based locks are not implemented :(");
    } else {
        pub type Lock<T> = SpinLock<T>;
        pub type LockGuard<'a, T> = SpinLockGuard<'a, T>;
    }
}

// https://rigtorp.se/spinlock/
unsafe impl RawMutex for InnerSpinLock {
  const INIT: InnerSpinLock = InnerSpinLock(AtomicBool::new(false));

  type GuardMarker = GuardSend;

  #[inline]
  fn lock(&self) {
    loop {
      if !self.0.swap(true, Ordering::Acquire) {
        return;
      }
      while self.0.load(Ordering::Relaxed) {
        spin_loop();
      }
    }
  }

  #[inline]
  fn try_lock(&self) -> bool {
    !self.0.load(Ordering::Relaxed) && !self.0.swap(true, Ordering::Acquire)
  }

  #[inline]
  unsafe fn unlock(&self) {
    self.0.store(false, Ordering::Release);
  }
}
