use crate::types::c_longlong;

pub type c_char = i8;
pub type c_long = i32;
pub type c_ulong = u32;
pub type wchar_t = i32;

pub type int_fast16_t = i32;
pub type int_fast32_t = i32;
pub type uint_fast16_t = u32;
pub type uint_fast32_t = u32;
pub type intmax_t = i32;
pub type uintmax_t = u32;

#[repr(C, align(16))]
pub struct max_align_t {
  _ll: c_longlong,
  _ld: [u8; 12]
}
