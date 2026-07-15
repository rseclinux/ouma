use {crate::support::ffi::va_list::ExtVaList, core::ffi::c_void};

pub type LDBLBytes = [u8; 16];

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExtVaListInner {
  stack: *const c_void,
  gr_top: *const c_void,
  vr_top: *const c_void,
  gr_offs: i32,
  vr_offs: i32
}

#[inline]
pub unsafe fn get_long_double_bytes(me: &mut ExtVaList) -> LDBLBytes {
  let dst: [u8; 16];

  if !me.inner.vr_top.is_null() {
    // In case if long double is on floating point/simd registers
    let ptr = unsafe {
      (me.inner.vr_top as *const u8).offset(me.inner.vr_offs as isize)
    };
    let aligned = (ptr as usize + 15) & !15; // Align up

    let src = aligned as *const LDBLBytes;
    dst = unsafe { src.read() };

    me.inner.vr_offs += 16;
  } else {
    // otherwise it is on stack
    let ptr = me.inner.stack as usize;
    let aligned = (ptr + 15) & !15; // Align up

    let src = aligned as *const LDBLBytes;
    dst = unsafe { src.read() };

    me.inner.stack = (aligned + 16) as *const c_void; // advance pointer for futher reading
  }

  dst
}
