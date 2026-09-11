use core::arch::asm;

pub const FE_TONEAREST: i32 = 0;
pub const FE_DOWNWARD: i32 = 0x800000;
pub const FE_UPWARD: i32 = 0x400000;
pub const FE_TOWARDZERO: i32 = 0xc00000;

#[inline]
pub fn fegetround() -> i32 {
  let mut control_word = 0u32;
  #[cfg(target_abi = "eabihf")]
  {
    unsafe {
      asm!("vmrs {0}, fpscr", out(reg) control_word, options(nostack));
    };
  }
  #[cfg(all(not(target_abi = "eabihf"), target_feature = "vfp2"))]
  {
    unsafe {
      asm!("mrc p10, 7, {0}, cr1, cr0, 0", out(reg) control_word, options(nostack));
    };
  }
  #[cfg(all(not(target_abi = "eabihf"), not(target_feature = "vfp2")))]
  {
    control_word = FE_TONEAREST as u32;
  }
  (control_word & 0x00c00000u32) as i32
}
