use core::arch::asm;

pub const FE_TONEAREST: i32 = 0x000000;
pub const FE_DOWNWARD: i32 = 0x800000;
pub const FE_UPWARD: i32 = 0x400000;
pub const FE_TOWARDZERO: i32 = 0xc00000;

#[inline]
pub fn fegetround() -> i32 {
  let mut result = 0u64;
  unsafe { asm!("mrs {0}, fpcr", out(reg) result, options(nostack)) };
  (result & 0xc00000u64) as i32
}
