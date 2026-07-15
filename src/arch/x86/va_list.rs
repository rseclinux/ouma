
use {crate::support::ffi::va_list::ExtVaList, core::ffi::c_void};

pub type LDBLBytes = [u8; 12];

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExtVaListInner {
  ptr: *const c_void
}

#[inline]
pub unsafe fn get_long_double_bytes(me: &mut ExtVaList) -> LDBLBytes {
  let aligned = (me.inner.ptr as usize + 3) & !3; // align up
  let src = aligned as *const [u8; 12];

  let result = unsafe { src.read() };

  // 12 = 10 bytes (the Intel floating-point number itself) + 2 bytes is alignment
  me.inner.ptr = (aligned + 12) as *const c_void; // advance pointer for futher reading

  result // 80-bit Intel floating-point value
}
