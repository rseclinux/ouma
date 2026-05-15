pub type c_char = u8;
pub type c_long = i32;
pub type c_ulong = u32;
pub type wchar_t = u32;

#[repr(C, align(8))]
pub struct max_align_t {
  priv_: [f64; 1]
}
