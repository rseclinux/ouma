use core::arch::asm;

pub const FE_TONEAREST: i32 = 0x0;
pub const FE_DOWNWARD: i32 = 0x2;
pub const FE_UPWARD: i32 = 0x3;
pub const FE_TOWARDZERO: i32 = 0x1;

#[inline]
pub fn fegetround() -> i32 {
  let mut result = 0i32;
  unsafe { asm!("frrm {0}", out(reg) result, options(nostack)) };
  result
}
