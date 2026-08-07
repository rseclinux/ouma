use {
  super::EncodedLDBLReturn,
  crate::{c_char, c_int, size_t}
};

#[unsafe(no_mangle)]
pub extern "C" fn __oumainternal_strfromenc(
  _s: *mut c_char,
  _n: size_t,
  _format: *const c_char,
  _val: EncodedLDBLReturn
) -> c_int {
  todo!("implement strfroml :(")
}
