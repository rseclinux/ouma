use super::LCMonetary;

const MONETARY_EN_US: LCMonetary = LCMonetary {
  mon_decimal_point: c".",
  mon_thousands_sep: c",",
  mon_grouping: c"3;3",
  positive_sign: c"",
  negative_sign: c"-",
  currency_symbol: c"$",
  int_curr_symbol: c"USD",
  frac_digits: 2,
  p_cs_precedes: 1,
  n_cs_precedes: 1,
  p_sep_by_space: 0,
  n_sep_by_space: 0,
  p_sign_posn: 1,
  n_sign_posn: 1,
  int_frac_digits: 2,
  int_p_cs_precedes: 1,
  int_n_cs_precedes: 1,
  int_p_sep_by_space: 1,
  int_n_sep_by_space: 1,
  int_p_sign_posn: 1,
  int_n_sign_posn: 1
};
