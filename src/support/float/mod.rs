#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod intel_extended;

pub mod f128;
pub mod math;
pub mod rounding_mode;

// Taken from LLVM libc's FPBits
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Sign {
  Negative,
  Positive
}
