// TODO: Replace it with out own mutex when pthreads will be there...

use {
  core::{
    mem::MaybeUninit,
    sync::atomic::{AtomicBool, Ordering}
  },
  spin::{Mutex, MutexGuard}
};

static GLOBAL_MUTEX: Mutex<()> = Mutex::new(());

static mut GLOBAL_GUARD: MaybeUninit<MutexGuard<'static, ()>> =
  MaybeUninit::uninit();

static IS_LOCKED: AtomicBool = AtomicBool::new(false);

struct CriticalSection;
critical_section::set_impl!(CriticalSection);

unsafe impl critical_section::Impl for CriticalSection {
  unsafe fn acquire() -> bool {
    if IS_LOCKED.load(Ordering::Relaxed) {
      return true;
    }
    let g = GLOBAL_MUTEX.lock();
    IS_LOCKED.store(true, Ordering::Relaxed);
    unsafe { core::ptr::addr_of_mut!(GLOBAL_GUARD).write(MaybeUninit::new(g)) };
    false
  }

  unsafe fn release(nested_cs: bool) {
    if !nested_cs {
      IS_LOCKED.store(false, Ordering::Relaxed);
      unsafe { core::ptr::addr_of_mut!(GLOBAL_GUARD).drop_in_place() };
    }
  }
}
