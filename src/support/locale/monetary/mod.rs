use {
  super::LocaleObject,
  crate::{c_char, c_int},
  allocation::borrow::Cow,
  core::ffi
};

pub struct MonetaryObject<'a> {
  name: Cow<'a, ffi::CStr>,
  pub mon_decimal_point: &'a ffi::CStr,
  pub mon_thousands_sep: &'a ffi::CStr,
  pub mon_grouping: &'a ffi::CStr,
  pub positive_sign: &'a ffi::CStr,
  pub negative_sign: &'a ffi::CStr,
  pub currency_symbol: &'a ffi::CStr,
  pub frac_digits: c_char,
  pub p_cs_precedes: c_char,
  pub n_cs_precedes: c_char,
  pub p_sep_by_space: c_char,
  pub n_sep_by_space: c_char,
  pub p_sign_posn: c_char,
  pub n_sign_posn: c_char,
  pub int_curr_symbol: &'a ffi::CStr,
  pub int_frac_digits: c_char,
  pub int_p_cs_precedes: c_char,
  pub int_n_cs_precedes: c_char,
  pub int_p_sep_by_space: c_char,
  pub int_n_sep_by_space: c_char,
  pub int_p_sign_posn: c_char,
  pub int_n_sign_posn: c_char
}

impl<'a> LocaleObject for MonetaryObject<'a> {
  fn setlocale(
    &mut self,
    _: &ffi::CStr
  ) -> Result<&ffi::CStr, c_int> {
    Ok(self.set_to_posix())
  }

  fn set_to_posix(&mut self) -> &ffi::CStr {
    *self = DEFAULT_MONETARY;

    self.name.as_ref()
  }

  fn get_name(&self) -> &ffi::CStr {
    self.name.as_ref()
  }
}

impl<'a> Default for MonetaryObject<'a> {
  fn default() -> Self {
    DEFAULT_MONETARY
  }
}

pub const DEFAULT_MONETARY: MonetaryObject = MonetaryObject {
  name: Cow::Borrowed(c"C"),
  mon_decimal_point: c"",
  mon_thousands_sep: c"",
  mon_grouping: c"",
  positive_sign: c"",
  negative_sign: c"",
  currency_symbol: c"",
  frac_digits: c_char::MAX,
  p_cs_precedes: c_char::MAX,
  n_cs_precedes: c_char::MAX,
  p_sep_by_space: c_char::MAX,
  n_sep_by_space: c_char::MAX,
  p_sign_posn: c_char::MAX,
  n_sign_posn: c_char::MAX,
  int_curr_symbol: c"",
  int_frac_digits: c_char::MAX,
  int_p_cs_precedes: c_char::MAX,
  int_n_cs_precedes: c_char::MAX,
  int_p_sep_by_space: c_char::MAX,
  int_n_sep_by_space: c_char::MAX,
  int_p_sign_posn: c_char::MAX,
  int_n_sign_posn: c_char::MAX
};
