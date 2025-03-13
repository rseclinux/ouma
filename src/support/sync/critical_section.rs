use {
  super::futex,
  crate::{arch::sys, std::errno},
  core::{
    cell::UnsafeCell,
    ptr,
    sync::atomic::{AtomicU32, Ordering::Relaxed}
  },
  syscalls::raw_syscall
};

pub struct SyncUnsafeCell<T>(UnsafeCell<T>);
unsafe impl<T: Sync> Sync for SyncUnsafeCell<T> {}

static FUTEX: AtomicU32 = AtomicU32::new(0);
static IS_LOCKED: SyncUnsafeCell<bool> = SyncUnsafeCell(UnsafeCell::new(false));

struct FutexCriticalSection;
critical_section::set_impl!(FutexCriticalSection);

unsafe impl critical_section::Impl for FutexCriticalSection {
  unsafe fn acquire() -> bool {
    let state = IS_LOCKED.0.get();
    loop {
      unsafe {
        if FUTEX.load(Relaxed) != u32::from(*state) {
          return false;
        }

        let r: i32 = raw_syscall!(
          sys::SYS_FUTEX,
          &FUTEX as *const AtomicU32,
          futex::FUTEX_WAIT_BITSET | futex::FUTEX_PRIVATE_FLAG,
          u32::from(true),
          0,
          ptr::null::<u32>(),
          !0u32
        ) as i32;
        match r {
          | errno::ETIMEDOUT => return false,
          | errno::EINTR => continue,
          | _ => {
            (*state) = true;
            return true;
          }
        }
      }
    }
  }

  unsafe fn release(nested_cs: bool) {
    if !nested_cs {
      unsafe {
        let r: bool = bool::from(
          raw_syscall!(
            sys::SYS_FUTEX,
            &FUTEX as *const AtomicU32,
            futex::FUTEX_WAKE | futex::FUTEX_PRIVATE_FLAG,
            0
          ) > 0
        );
        let state = IS_LOCKED.0.get();
        (*state) = r;
      }
    }
  }
}
