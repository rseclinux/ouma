use {super::LCMonetary, crate::c_char};

pub const MONETARY_POSIX: LCMonetary = LCMonetary {
  mon_decimal_point: c"",
  mon_thousands_sep: c"",
  mon_grouping: c"",
  positive_sign: c"",
  negative_sign: c"",
  currency_symbol: c"",
  int_curr_symbol: c"",
  frac_digits: c_char::MAX,
  p_cs_precedes: c_char::MAX,
  n_cs_precedes: c_char::MAX,
  p_sep_by_space: c_char::MAX,
  n_sep_by_space: c_char::MAX,
  p_sign_posn: c_char::MAX,
  n_sign_posn: c_char::MAX,
  int_frac_digits: c_char::MAX,
  int_p_cs_precedes: c_char::MAX,
  int_n_cs_precedes: c_char::MAX,
  int_p_sep_by_space: c_char::MAX,
  int_n_sep_by_space: c_char::MAX,
  int_p_sign_posn: c_char::MAX,
  int_n_sign_posn: c_char::MAX
};
