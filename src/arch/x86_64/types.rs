use crate::types::c_longlong;

pub type c_char = i8;
pub type c_long = i64;
pub type c_ulong = u64;
pub type wchar_t = i32;

#[repr(C, align(16))]
pub struct max_align_t {
  _ll: c_longlong,
  _ld: [u8; 16]
}
