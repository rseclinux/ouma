use {
  super::static_data,
  crate::{c_char, c_int},
  core::ptr
};

#[unsafe(no_mangle)]
pub extern "C" fn rs_strerrorname_np(num: c_int) -> *const c_char {
  if let Some(e) = static_data::ERRNO_NAMES.get(num as usize) {
    e.as_ptr()
  } else {
    ptr::null()
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strerrordesc_np(num: c_int) -> *const c_char {
  if let Some(e) = static_data::ERRNO_DESCRIPTIONS.get(num as usize) {
    e.as_ptr()
  } else {
    ptr::null()
  }
}
