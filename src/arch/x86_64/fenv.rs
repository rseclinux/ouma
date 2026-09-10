use core::arch::asm;

pub const FE_TONEAREST: i32 = 0;
pub const FE_DOWNWARD: i32 = 0x400;
pub const FE_UPWARD: i32 = 0x800;
pub const FE_TOWARDZERO: i32 = 0xc00;

#[inline]
pub fn fegetround() -> i32 {
  let mut control_word = 0u16;
  unsafe {
    asm!("fnstcw ({0})", in(reg) &mut control_word, options(att_syntax, nostack))
  };
  (control_word & 0xc00u16) as i32
}
