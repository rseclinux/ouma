#include "common.h"

extern "C" {
  struct lconv *rs_localeconv(void);
}

TEST(localeconv, posix) {
  ASSERT_STREQ("C", rs_setlocale(LC_ALL, "POSIX"));

  struct lconv *lconv = rs_localeconv();

  ASSERT_STREQ(".", lconv->decimal_point);
  ASSERT_STREQ("", lconv->thousands_sep);
  ASSERT_STREQ("", lconv->grouping);
  ASSERT_STREQ("", lconv->mon_decimal_point);
  ASSERT_STREQ("", lconv->mon_thousands_sep);
  ASSERT_STREQ("", lconv->mon_grouping);
  ASSERT_STREQ("", lconv->positive_sign);
  ASSERT_STREQ("", lconv->negative_sign);
  ASSERT_STREQ("", lconv->currency_symbol);
  ASSERT_EQ(CHAR_MAX, lconv->frac_digits);
  ASSERT_EQ(CHAR_MAX, lconv->p_cs_precedes);
  ASSERT_EQ(CHAR_MAX, lconv->p_sep_by_space);
  ASSERT_EQ(CHAR_MAX, lconv->p_sign_posn);
  ASSERT_EQ(CHAR_MAX, lconv->n_cs_precedes);
  ASSERT_EQ(CHAR_MAX, lconv->n_sep_by_space);
  ASSERT_EQ(CHAR_MAX, lconv->n_sign_posn);
  ASSERT_STREQ("", lconv->int_curr_symbol);
  ASSERT_EQ(CHAR_MAX, lconv->int_frac_digits);
  ASSERT_EQ(CHAR_MAX, lconv->int_p_cs_precedes);
  ASSERT_EQ(CHAR_MAX, lconv->int_p_sep_by_space);
  ASSERT_EQ(CHAR_MAX, lconv->int_p_sign_posn);
  ASSERT_EQ(CHAR_MAX, lconv->int_n_cs_precedes);
  ASSERT_EQ(CHAR_MAX, lconv->int_n_sep_by_space);
  ASSERT_EQ(CHAR_MAX, lconv->int_n_sign_posn);

  ASSERT_EQ(lconv, rs_localeconv());
}

TEST(localeconv, ukraine) {
  ASSERT_STREQ("uk_UA.UTF-8", rs_setlocale(LC_ALL, "uk_UA.UTF-8"));

  struct lconv *lconv = rs_localeconv();

  ASSERT_STREQ(",", lconv->decimal_point);
  ASSERT_STREQ("", lconv->thousands_sep);
  ASSERT_STREQ("\x03\x03", lconv->grouping);
  ASSERT_STREQ(",", lconv->mon_decimal_point);
  ASSERT_STREQ("", lconv->mon_thousands_sep);
  ASSERT_STREQ("\x03\x03", lconv->mon_grouping);
  ASSERT_STREQ("", lconv->positive_sign);
  ASSERT_STREQ("-", lconv->negative_sign);
  ASSERT_STREQ("₴", lconv->currency_symbol);
  ASSERT_EQ(2, lconv->frac_digits);
  ASSERT_EQ(0, lconv->p_cs_precedes);
  ASSERT_EQ(1, lconv->p_sep_by_space);
  ASSERT_EQ(1, lconv->p_sign_posn);
  ASSERT_EQ(0, lconv->n_cs_precedes);
  ASSERT_EQ(1, lconv->n_sep_by_space);
  ASSERT_EQ(1, lconv->n_sign_posn);
  ASSERT_STREQ("UAH ", lconv->int_curr_symbol);
  ASSERT_EQ(2, lconv->int_frac_digits);
  ASSERT_EQ(0, lconv->int_p_cs_precedes);
  ASSERT_EQ(1, lconv->int_p_sep_by_space);
  ASSERT_EQ(1, lconv->int_p_sign_posn);
  ASSERT_EQ(0, lconv->int_n_cs_precedes);
  ASSERT_EQ(1, lconv->int_n_sep_by_space);
  ASSERT_EQ(1, lconv->int_n_sign_posn);

  ASSERT_EQ(lconv, rs_localeconv());
}

TEST(localeconv, denmark) {
  ASSERT_STREQ("da_DK.UTF-8", rs_setlocale(LC_ALL, "da_DK.UTF-8"));

  struct lconv *lconv = rs_localeconv();

  ASSERT_STREQ("kr.", lconv->currency_symbol);

  ASSERT_EQ(lconv, rs_localeconv());
}

TEST(localeconv, syria) {
  ASSERT_STREQ("ar_SY.UTF-8", rs_setlocale(LC_ALL, "ar_SY.UTF-8"));

  struct lconv *lconv = rs_localeconv();

  ASSERT_STREQ("ل.س.", lconv->currency_symbol);
  ASSERT_EQ(1, lconv->p_cs_precedes);
  ASSERT_EQ(1, lconv->p_sep_by_space);
  ASSERT_EQ(1, lconv->n_cs_precedes);
  ASSERT_EQ(2, lconv->n_sep_by_space);

  ASSERT_EQ(lconv, rs_localeconv());
}

TEST(setlocale, good) {
  ASSERT_STREQ("C", rs_setlocale(LC_ALL, "C"));
  ASSERT_STREQ("en_US.UTF-8", rs_setlocale(LC_ALL, "en_US.UTF-8"));
  ASSERT_STREQ("sv_SE", rs_setlocale(LC_COLLATE, "sv_SE"));
  ASSERT_STREQ("sr-SR", rs_setlocale(LC_MONETARY, "sr-SR"));
  ASSERT_STREQ(".UTF-8", rs_setlocale(LC_CTYPE, ".UTF-8"));
  ASSERT_STREQ("pdc", rs_setlocale(LC_MESSAGES, "pdc"));
  // TODO: fix todo*
  ASSERT_STREQ("LC_COLLATE=sv_SE;LC_CTYPE=.UTF-8;LC_MESSAGES=pdc;LC_MONETARY=sr-SR;LC_NUMERIC=en_US.UTF-8;LC_TIME=todo time",
    rs_setlocale(LC_ALL, NULL));
  ASSERT_STREQ("sr-SR@latin", rs_setlocale(LC_MONETARY, "sr-SR@latin"));
  ASSERT_STREQ("sr-SR.UTF-8@latin", rs_setlocale(LC_ALL, "sr-SR.UTF-8@latin"));
}

TEST(setlocale, bad) {
  ASSERT_STREQ(NULL, rs_setlocale(1337, "C"));
  ASSERT_STREQ(NULL, rs_setlocale(LC_ALL, "phew"));
  ASSERT_STREQ(NULL, rs_setlocale(LC_CTYPE, "."));
  ASSERT_STREQ(NULL, rs_setlocale(LC_CTYPE, ".no_supported"));
  ASSERT_STREQ(NULL, rs_setlocale(LC_COLLATE, "horrible"));
  ASSERT_STREQ(NULL, rs_setlocale(LC_ALL, "sr-SR@latin"));
}
