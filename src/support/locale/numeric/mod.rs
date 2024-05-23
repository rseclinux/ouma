use core::ffi;

pub mod en_us;
pub mod posix;
pub mod ru_ru;

#[derive(Copy, Clone)]
pub struct LCNumeric<'a> {
  pub decimal_point: &'a ffi::CStr,
  pub thousands_sep: &'a ffi::CStr,
  pub grouping: &'a ffi::CStr
}
