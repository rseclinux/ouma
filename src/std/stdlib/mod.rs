pub mod constants;

use crate::{size_t, support::locale};

#[unsafe(no_mangle)]
pub extern "C" fn __stroginointernal_get_mb_cur_max() -> size_t {
  let ctype = locale::get_slot(&locale::get_thread_locale().ctype);
  ctype.converter.mb_cur_max
}
