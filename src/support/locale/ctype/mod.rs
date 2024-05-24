use {
  crate::{c_char, c_int, char32_t, mbstate_t, size_t, ssize_t},
  core::ffi
};

pub mod ascii;
pub mod utf8;

#[derive(Copy, Clone)]
pub struct LCCtype<'a> {
  pub codeset: &'a ffi::CStr,
  pub mbtoc32:
    fn(*mut char32_t, *const c_char, size_t, *mut mbstate_t) -> ssize_t,
  pub c32tomb: fn(*mut c_char, char32_t, *mut mbstate_t) -> ssize_t,
  pub mb_cur_max: c_int
}
