cfg_if! {
  if #[cfg(target_arch = "arm")] {
    mod arm;
    pub use arm::*;
  } else if #[cfg(target_arch = "aarch64")] {
    mod arm64;
    pub use arm64::*;
  } else if #[cfg(target_arch = "riscv32")] {
    mod riscv32;
    pub use riscv32::*;
  } else if #[cfg(target_arch = "riscv64")] {
    mod riscv64;
    pub use riscv64::*;
  } else if #[cfg(target_arch = "s390x")] {
    mod s390x;
    pub use s390x::*;
  } else if #[cfg(target_arch = "x86")] {
    mod x86;
    pub use x86::*;
  } else if #[cfg(target_arch = "x86_64")] {
    mod x86_64;
    pub use x86_64::*;
  } else {
    compile_error!("Platform is not supported");
  }
}
