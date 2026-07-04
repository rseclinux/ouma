use crate::types::c_longlong;

pub type c_char = u8;
pub type c_long = i32;
pub type c_ulong = u32;
pub type wchar_t = u32;

#[repr(C, align(16))]
pub struct max_align_t {
  _ll: c_longlong,
  _ld: [u8; 8]
}
