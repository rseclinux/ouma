use {crate::support::ffi::va_list::ExtVaList, core::ffi::c_void};

pub type LDBLBytes = [u8; 16];

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExtVaListInner {
  gp_offset: i32,
  fp_offset: i32,
  overflow_arg_area: *const c_void,
  reg_save_area: *const c_void
}

#[inline]
pub unsafe fn get_long_double_bytes(me: &mut ExtVaList) -> LDBLBytes {
  let aligned = (me.inner.overflow_arg_area as usize + 15) & !15; // align up
  let src = aligned as *const LDBLBytes;

  let result = unsafe { src.read() };

  me.inner.overflow_arg_area = (aligned + 16) as *const c_void; // advance for futher argument reading

  result // 80-bit Intel value with 6 bytes padding for alignment
}
