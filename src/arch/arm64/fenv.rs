use core::arch::asm;

pub const FE_TONEAREST: i32 = 0x000000;
pub const FE_DOWNWARD: i32 = 0x800000;
pub const FE_UPWARD: i32 = 0x400000;
pub const FE_TOWARDZERO: i32 = 0xc00000;

#[inline]
pub fn fegetround() -> i32 {
  let mut result64 = 0u64;
  unsafe { asm!("mrs {0}, fpcr", out(reg) result64, options(nostack)) };
  let result = result64 as u32;
  (result & 0xc00000u32) as i32
}
