use {crate::c_char, core::ffi};

pub mod en_us;
pub mod posix;
pub mod ru_ru;

#[derive(Copy, Clone)]
pub struct LCMonetary<'a> {
  pub mon_decimal_point: &'a ffi::CStr,
  pub mon_thousands_sep: &'a ffi::CStr,
  pub mon_grouping: &'a ffi::CStr,
  pub positive_sign: &'a ffi::CStr,
  pub negative_sign: &'a ffi::CStr,
  pub currency_symbol: &'a ffi::CStr,
  pub int_curr_symbol: &'a ffi::CStr,
  pub frac_digits: c_char,
  pub p_cs_precedes: c_char,
  pub n_cs_precedes: c_char,
  pub p_sep_by_space: c_char,
  pub n_sep_by_space: c_char,
  pub p_sign_posn: c_char,
  pub n_sign_posn: c_char,
  pub int_frac_digits: c_char,
  pub int_p_cs_precedes: c_char,
  pub int_n_cs_precedes: c_char,
  pub int_p_sep_by_space: c_char,
  pub int_n_sep_by_space: c_char,
  pub int_p_sign_posn: c_char,
  pub int_n_sign_posn: c_char
}
