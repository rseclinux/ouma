
use {crate::support::ffi::va_list::ExtVaList, core::ffi::c_void};

pub type LDBLBytes = [u8; 16];

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExtVaListInner {
  ptr: *const c_void
}

#[inline]
pub unsafe fn get_long_double_bytes(me: &mut ExtVaList) -> LDBLBytes {
  let aligned = (me.inner.ptr as usize + 15) & !15; // align up
  let src = aligned as *const LDBLBytes;

  let result = unsafe { src.read() };

  me.inner.ptr = (aligned + 16) as *const c_void; // advance pointer for futher reading

  result // IEEE quadruple precision floating-point value
}
