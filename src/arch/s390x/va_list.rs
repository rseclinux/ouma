use {crate::support::ffi::va_list::ExtVaList, core::ffi::c_void};

pub type LDBLBytes = [u8; 16];

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExtVaListInner {
  gpr: i64,
  fpr: i64,
  overflow_arg_area: *const c_void,
  reg_save_area: *const c_void
}

#[inline]
pub unsafe fn get_long_double_bytes(args: &mut ExtVaList) -> LDBLBytes {
  // SINCE ON ELF S390X LONG DOUBLES ARE PASSED AS REFERENCES
  // ON GPRS YOU CAN LITERALLY DO THE FOLLOWING
  // if GPRs get exhausted then off the stack
  let ptr = unsafe { args.next_arg::<*const [u8; 16]>() };
  unsafe { ptr.read() }
}
