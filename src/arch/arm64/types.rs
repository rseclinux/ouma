pub type c_char = u8;
pub type c_long = i64;
pub type c_ulong = u64;
pub type wchar_t = u32;

#[repr(C)]
#[repr(align(16))]
pub struct max_align_t {
  _l: [f64; 4]
}
