use {
  super::LocaleObject,
  crate::{c_char, c_int},
  allocation::borrow::Cow,
  core::ffi
};

pub struct MonetaryObject<'a> {
  name: Cow<'a, ffi::CStr>,
  pub mon_decimal_point: Cow<'a, [u8]>,
  pub mon_thousands_sep: Cow<'a, [u8]>,
  pub mon_grouping: Cow<'a, [u8]>,
  pub positive_sign: Cow<'a, [u8]>,
  pub negative_sign: Cow<'a, [u8]>,
  pub currency_symbol: Cow<'a, [u8]>,
  pub frac_digits: c_char,
  pub p_cs_precedes: c_char,
  pub n_cs_precedes: c_char,
  pub p_sep_by_space: c_char,
  pub n_sep_by_space: c_char,
  pub p_sign_posn: c_char,
  pub n_sign_posn: c_char,
  pub int_curr_symbol: Cow<'a, [u8]>,
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
  mon_decimal_point: Cow::Borrowed(&[b'.', b'\0']),
  mon_thousands_sep: Cow::Borrowed(&[b'\0']),
  mon_grouping: Cow::Borrowed(&[b'\0']),
  positive_sign: Cow::Borrowed(&[b'\0']),
  negative_sign: Cow::Borrowed(&[b'\0']),
  currency_symbol: Cow::Borrowed(&[b'\0']),
  frac_digits: c_char::MAX,
  p_cs_precedes: c_char::MAX,
  n_cs_precedes: c_char::MAX,
  p_sep_by_space: c_char::MAX,
  n_sep_by_space: c_char::MAX,
  p_sign_posn: c_char::MAX,
  n_sign_posn: c_char::MAX,
  int_curr_symbol: Cow::Borrowed(&[b'\0']),
  int_frac_digits: c_char::MAX,
  int_p_cs_precedes: c_char::MAX,
  int_n_cs_precedes: c_char::MAX,
  int_p_sep_by_space: c_char::MAX,
  int_n_sep_by_space: c_char::MAX,
  int_p_sign_posn: c_char::MAX,
  int_n_sign_posn: c_char::MAX
};
