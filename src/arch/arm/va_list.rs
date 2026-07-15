use {crate::support::ffi::va_list::ExtVaList, core::ffi::c_void};

pub type LDBLBytes = [u8; 8];

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExtVaListInner {
  ptr: *const c_void
}

#[inline]
pub unsafe fn get_long_double_bytes(args: &mut ExtVaList) -> LDBLBytes {
  // On 32-bit ARM EABI long double is just f64
  let v = unsafe { args.next_arg::<f64>() };
  v.to_ne_bytes() // to_ne_bytes() since arm can be big endian
}
