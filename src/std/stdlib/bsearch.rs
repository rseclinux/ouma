use {
  super::sort::comparsion_fn_t,
  crate::types::size_t,
  core::{ffi::c_void, ptr}
};

#[unsafe(no_mangle)]
pub extern "C" fn rs_bsearch(
  key: *const c_void,
  base: *const c_void,
  nmemb: size_t,
  size: size_t,
  compar: Option<comparsion_fn_t>
) -> *mut c_void {
  if key.is_null() || base.is_null() || nmemb == 0 || size == 0 {
    return ptr::null_mut();
  }
  let Some(cmp) = compar else {
    return ptr::null_mut();
  };

  let mut base = base as *const u8;
  let mut lim = nmemb;

  while lim != 0 {
    let offset = lim.checked_shr(1).and_then(|c| c.checked_mul(size));
    let Some(offset) = offset else {
      return ptr::null_mut();
    };
    let p = base.wrapping_add(offset);
    let cmp = cmp(key, p.cast());
    if cmp == 0 {
      return p.cast_mut().cast();
    }
    if cmp > 0 {
      base = p.wrapping_add(size);
      lim -= 1;
    }
    lim >>= 1;
  }

  ptr::null_mut()
}
