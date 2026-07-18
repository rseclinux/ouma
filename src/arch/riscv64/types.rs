use crate::types::c_longlong;

pub type c_char = u8;
pub type c_long = i64;
pub type c_ulong = u64;
pub type wchar_t = u32;

pub type int_fast16_t = i64;
pub type int_fast32_t = i64;
pub type uint_fast16_t = u64;
pub type uint_fast32_t = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;

#[repr(C, align(16))]
pub struct max_align_t {
  _ll: c_longlong,
  _ld: [u8; 16]
}
