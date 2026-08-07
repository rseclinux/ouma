pub mod constants;
pub mod multibyte;
pub mod num;
pub mod strfromfloat;

use crate::{
  c_longdouble,
  size_t,
  support::{locale, traits::float::Float}
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EncodedLDBLReturn {
  pub bytes: [u8; c_longdouble::SIZE_IN_BYTES]
}

#[unsafe(no_mangle)]
pub(crate) extern "C" fn __oumainternal_get_mb_cur_max() -> size_t {
  let ctype =
    locale::get_slot(&locale::get_thread_locale().ctype).unwrap_or_default();
  ctype.converter.mb_cur_max
}
