use core::panic::PanicInfo;

#[inline(always)]
fn ensure_unique_backtrace() {
  static ENSURE_UNIQUE_BACKTRACE_INSTANCE: u8 = 0;
  unsafe {
    core::arch::asm!(
        "/* {0} */",
        in(reg) &ENSURE_UNIQUE_BACKTRACE_INSTANCE,
        options(nomem, nostack, preserves_flags),
    );
  }
}

#[inline(always)]
fn crash_with_unique_backtrace() -> ! {
  ensure_unique_backtrace();
  core::intrinsics::abort()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  crash_with_unique_backtrace();
}
