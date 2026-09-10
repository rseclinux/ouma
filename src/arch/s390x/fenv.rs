use core::arch::asm;

pub const FE_TONEAREST: i32 = 0;
pub const FE_DOWNWARD: i32 = 0x3;
pub const FE_UPWARD: i32 = 0x2;
pub const FE_TOWARDZERO: i32 = 0x1;

#[inline]
pub fn fegetround() -> i32 {
  let mut control_word = 0u32;
  unsafe { asm!("efpc {0}", out(reg) result, options(nostack)) };
  (control_word & 0x00000003u32) as i32
}
