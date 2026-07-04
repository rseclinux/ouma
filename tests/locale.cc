#include "common.h"
#include "common_locale.h"

#include <gtest/gtest.h>

TEST(localeconv, posix) {
  ASSERT_STREQ("C", rs_setlocale(RS_LC_ALL, "C"));

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

TEST(localeconv, netherlands) {
  ouma_locale_t locale =
      rs_newlocale(RS_LC_NUMERIC_MASK | RS_LC_MONETARY_MASK, "nl_NL.UTF-8", nullptr);
  ASSERT_NE(nullptr, locale);
  ASSERT_NE(ENOENT, rs_errno);

  struct lconv *lconv = rs_localeconv_l(locale);

  ASSERT_STREQ(",", lconv->decimal_point);
  ASSERT_STREQ(".", lconv->thousands_sep);
  ASSERT_STREQ("\x03", lconv->grouping);
  ASSERT_STREQ(",", lconv->mon_decimal_point);
  ASSERT_STREQ(".", lconv->mon_thousands_sep);
  ASSERT_STREQ("\x03", lconv->mon_grouping);
  ASSERT_STREQ("", lconv->positive_sign);
  ASSERT_STREQ("-", lconv->negative_sign);
  ASSERT_STREQ("€", lconv->currency_symbol);
  ASSERT_EQ(2, lconv->frac_digits);
  ASSERT_EQ(1, lconv->p_cs_precedes);
  ASSERT_EQ(1, lconv->p_sep_by_space);
  ASSERT_EQ(3, lconv->p_sign_posn);
  ASSERT_EQ(1, lconv->n_cs_precedes);
  ASSERT_EQ(1, lconv->n_sep_by_space);
  ASSERT_EQ(3, lconv->n_sign_posn);
  ASSERT_STREQ("EUR ", lconv->int_curr_symbol);
  ASSERT_EQ(2, lconv->int_frac_digits);
  ASSERT_EQ(1, lconv->int_p_cs_precedes);
  ASSERT_EQ(1, lconv->int_p_sep_by_space);
  ASSERT_EQ(3, lconv->int_p_sign_posn);
  ASSERT_EQ(1, lconv->int_n_cs_precedes);
  ASSERT_EQ(1, lconv->int_n_sep_by_space);
  ASSERT_EQ(3, lconv->int_n_sign_posn);

  ASSERT_EQ(lconv, rs_localeconv_l(locale));

  rs_freelocale(locale);
}

TEST(localeconv, united_states) {
  ouma_locale_t locale =
      rs_newlocale(RS_LC_NUMERIC_MASK | RS_LC_MONETARY_MASK, "en_US.UTF-8", nullptr);
  ASSERT_NE(nullptr, locale);
  ASSERT_NE(ENOENT, rs_errno);

  struct lconv *lconv = rs_localeconv_l(locale);

  ASSERT_STREQ(".", lconv->decimal_point);
  ASSERT_STREQ(",", lconv->thousands_sep);
  ASSERT_STREQ("\x03", lconv->grouping);
  ASSERT_STREQ(".", lconv->mon_decimal_point);
  ASSERT_STREQ(",", lconv->mon_thousands_sep);
  ASSERT_STREQ("\x03", lconv->mon_grouping);
  ASSERT_STREQ("", lconv->positive_sign);
  ASSERT_STREQ("-", lconv->negative_sign);
  ASSERT_STREQ("$", lconv->currency_symbol);
  ASSERT_EQ(2, lconv->frac_digits);
  ASSERT_EQ(1, lconv->p_cs_precedes);
  ASSERT_EQ(0, lconv->p_sep_by_space);
  ASSERT_EQ(3, lconv->p_sign_posn);
  ASSERT_EQ(1, lconv->n_cs_precedes);
  ASSERT_EQ(0, lconv->n_sep_by_space);
  ASSERT_EQ(3, lconv->n_sign_posn);
  ASSERT_STREQ("USD ", lconv->int_curr_symbol);
  ASSERT_EQ(2, lconv->int_frac_digits);
  ASSERT_EQ(1, lconv->int_p_cs_precedes);
  ASSERT_EQ(0, lconv->int_p_sep_by_space);
  ASSERT_EQ(3, lconv->int_p_sign_posn);
  ASSERT_EQ(1, lconv->int_n_cs_precedes);
  ASSERT_EQ(0, lconv->int_n_sep_by_space);
  ASSERT_EQ(3, lconv->int_n_sign_posn);

  ASSERT_EQ(lconv, rs_localeconv_l(locale));

  rs_freelocale(locale);
}

TEST(localeconv, japan) {
  ouma_locale_t locale =
      rs_newlocale(RS_LC_NUMERIC_MASK | RS_LC_MONETARY_MASK, "ja_JP.UTF-8", nullptr);
  ASSERT_NE(nullptr, locale);
  ASSERT_NE(ENOENT, rs_errno);

  struct lconv *lconv = rs_localeconv_l(locale);

  ASSERT_STREQ(".", lconv->decimal_point);
  ASSERT_STREQ(",", lconv->thousands_sep);
  ASSERT_STREQ("\x03", lconv->grouping);
  ASSERT_STREQ(".", lconv->mon_decimal_point);
  ASSERT_STREQ(",", lconv->mon_thousands_sep);
  ASSERT_STREQ("\x03", lconv->mon_grouping);
  ASSERT_STREQ("", lconv->positive_sign);
  ASSERT_STREQ("-", lconv->negative_sign);
  ASSERT_STREQ("¥", lconv->currency_symbol);
  ASSERT_EQ(0, lconv->frac_digits);
  ASSERT_EQ(1, lconv->p_cs_precedes);
  ASSERT_EQ(0, lconv->p_sep_by_space);
  ASSERT_EQ(3, lconv->p_sign_posn);
  ASSERT_EQ(1, lconv->n_cs_precedes);
  ASSERT_EQ(0, lconv->n_sep_by_space);
  ASSERT_EQ(3, lconv->n_sign_posn);
  ASSERT_STREQ("JPY ", lconv->int_curr_symbol);
  ASSERT_EQ(0, lconv->int_frac_digits);
  ASSERT_EQ(1, lconv->int_p_cs_precedes);
  ASSERT_EQ(0, lconv->int_p_sep_by_space);
  ASSERT_EQ(3, lconv->int_p_sign_posn);
  ASSERT_EQ(1, lconv->int_n_cs_precedes);
  ASSERT_EQ(0, lconv->int_n_sep_by_space);
  ASSERT_EQ(3, lconv->int_n_sign_posn);

  ASSERT_EQ(lconv, rs_localeconv_l(locale));

  rs_freelocale(locale);
}

TEST(localeconv, israel) {
  ouma_locale_t locale =
      rs_newlocale(RS_LC_NUMERIC_MASK | RS_LC_MONETARY_MASK, "he_IL.UTF-8", nullptr);
  ASSERT_NE(nullptr, locale);
  ASSERT_NE(ENOENT, rs_errno);

  struct lconv *lconv = rs_localeconv_l(locale);

  ASSERT_STREQ(".", lconv->decimal_point);
  ASSERT_STREQ(",", lconv->thousands_sep);
  ASSERT_STREQ("\x03", lconv->grouping);
  ASSERT_STREQ(".", lconv->mon_decimal_point);
  ASSERT_STREQ(",", lconv->mon_thousands_sep);
  ASSERT_STREQ("\x03", lconv->mon_grouping);
  ASSERT_STREQ("", lconv->positive_sign);
  ASSERT_STREQ("-", lconv->negative_sign);
  ASSERT_STREQ("₪", lconv->currency_symbol);
  ASSERT_EQ(2, lconv->frac_digits);
  ASSERT_EQ(0, lconv->p_cs_precedes);
  ASSERT_EQ(1, lconv->p_sep_by_space);
  ASSERT_EQ(1, lconv->p_sign_posn);
  ASSERT_EQ(0, lconv->n_cs_precedes);
  ASSERT_EQ(1, lconv->n_sep_by_space);
  ASSERT_EQ(1, lconv->n_sign_posn);
  ASSERT_STREQ("ILS ", lconv->int_curr_symbol);
  ASSERT_EQ(2, lconv->int_frac_digits);
  ASSERT_EQ(0, lconv->int_p_cs_precedes);
  ASSERT_EQ(1, lconv->int_p_sep_by_space);
  ASSERT_EQ(1, lconv->int_p_sign_posn);
  ASSERT_EQ(0, lconv->int_n_cs_precedes);
  ASSERT_EQ(1, lconv->int_n_sep_by_space);
  ASSERT_EQ(1, lconv->int_n_sign_posn);

  ASSERT_EQ(lconv, rs_localeconv_l(locale));

  rs_freelocale(locale);
}

TEST(localeconv, denmark) {
  ouma_locale_t locale =
      rs_newlocale(RS_LC_NUMERIC_MASK | RS_LC_MONETARY_MASK, "da_DK.UTF-8", nullptr);
  ASSERT_NE(nullptr, locale);
  ASSERT_NE(ENOENT, rs_errno);

  struct lconv *lconv = rs_localeconv_l(locale);

  ASSERT_STREQ(",", lconv->decimal_point);
  ASSERT_STREQ(".", lconv->thousands_sep);
  ASSERT_STREQ("\x03", lconv->grouping);
  ASSERT_STREQ(",", lconv->mon_decimal_point);
  ASSERT_STREQ(".", lconv->mon_thousands_sep);
  ASSERT_STREQ("\x03", lconv->mon_grouping);
  ASSERT_STREQ("", lconv->positive_sign);
  ASSERT_STREQ("-", lconv->negative_sign);
  ASSERT_STREQ("kr.", lconv->currency_symbol);
  ASSERT_STREQ("DKK ", lconv->int_curr_symbol);

  ASSERT_EQ(lconv, rs_localeconv_l(locale));

  rs_freelocale(locale);
}

TEST(setlocale, good) {
  const char *locales[] = {"POSIX", "C",           "de_CH.UTF-8", "fr_FR.UTF-8",
                           "en_US", "POSIX.UTF-8", "C.UTF-8",     NULL};
  const char *expectedLocales[] = {"POSIX",       "C",     "de_CH.UTF-8",
                                   "fr_FR.UTF-8", "en_US", "POSIX.UTF-8",
                                   "C.UTF-8",     NULL};

  for (int i = 0; locales[i] != NULL; ++i) {
    char *result = rs_setlocale(RS_LC_ALL, locales[i]);
    ASSERT_STREQ(expectedLocales[i], result);
  }

  for (int i = 1; i <= LC_ALL; ++i)
    ASSERT_NE(rs_setlocale(i, locales[i + 1]), nullptr);

  const char *expectedResult =
      "LC_COLLATE=en_US;LC_CTYPE=C.UTF-8;LC_MESSAGES=C.UTF-8;LC_MONETARY=POSIX."
      "UTF-8;LC_NUMERIC=de_CH.UTF-8;LC_TIME=fr_FR.UTF-8";
  ASSERT_STREQ(expectedResult, rs_setlocale(RS_LC_ALL, nullptr));
}

TEST(setlocale, chinese_languages) {
  ASSERT_STREQ("zh_CN.UTF-8", rs_setlocale(RS_LC_ALL, "zh_CN.UTF-8"));
  ASSERT_STREQ("zh_TW.UTF-8", rs_setlocale(RS_LC_ALL, "zh_TW.UTF-8"));
  ASSERT_STREQ("wuu_CN.UTF-8", rs_setlocale(RS_LC_ALL, "wuu_CN.UTF-8"));
  ASSERT_STREQ("wuu_CN.UTF-8@hans", rs_setlocale(RS_LC_ALL, "wuu_CN.UTF-8@hans"));
  ASSERT_STREQ("wuu_CN.UTF-8@hans", rs_setlocale(RS_LC_ALL, "wuu_CN.UTF-8@hans"));
  ASSERT_STREQ("yue_CN.UTF-8", rs_setlocale(RS_LC_ALL, "yue_CN.UTF-8"));
  ASSERT_STREQ("yue_HK.UTF-8", rs_setlocale(RS_LC_ALL, "yue_HK.UTF-8"));
  ASSERT_STREQ("nan_CN.UTF-8", rs_setlocale(RS_LC_ALL, "nan_CN.UTF-8"));
  ASSERT_STREQ("nan_TW.UTF-8", rs_setlocale(RS_LC_ALL, "nan_TW.UTF-8"));
  ASSERT_STREQ("hak_CN.UTF-8", rs_setlocale(RS_LC_ALL, "hak_CN.UTF-8"));
  ASSERT_STREQ("cmn_CN.UTF-8", rs_setlocale(RS_LC_ALL, "cmn_CN.UTF-8"));
  ASSERT_STREQ("cmn_SG.UTF-8", rs_setlocale(RS_LC_ALL, "cmn_SG.UTF-8"));
  ASSERT_STREQ("cmn_TW.UTF-8", rs_setlocale(RS_LC_ALL, "cmn_TW.UTF-8"));

  ASSERT_EQ(nullptr, rs_setlocale(RS_LC_ALL, "cmn_HK.UTF-8"));
  ASSERT_EQ(nullptr, rs_setlocale(RS_LC_ALL, "nan_HK.UTF-8"));
}

TEST(setlocale, slavic_latin) {
  ASSERT_STREQ("sr_RS.UTF-8@latin", rs_setlocale(RS_LC_ALL, "sr_RS.UTF-8@latin"));
  ASSERT_STREQ("be_BY.UTF-8@latin", rs_setlocale(RS_LC_ALL, "be_BY.UTF-8@latin"));
}

TEST(setlocale, bad) {
  ASSERT_STREQ(NULL, rs_setlocale(1337, "C"));
  ASSERT_STREQ(NULL, rs_setlocale(RS_LC_ALL, "phew"));
  ASSERT_STREQ(NULL, rs_setlocale(RS_LC_CTYPE, "."));
  ASSERT_STREQ(NULL, rs_setlocale(RS_LC_CTYPE, ".no_supported"));
  ASSERT_STREQ(NULL, rs_setlocale(RS_LC_COLLATE, "horrible"));
  ASSERT_STREQ(NULL, rs_setlocale(RS_LC_ALL, "sr-SR@latin"));
}

TEST(newlocale, zero_mask) {
  ouma_locale_t locale = rs_newlocale(0, "Unknown", 0);
  ASSERT_EQ(nullptr, locale);
  ASSERT_EQ(EINVAL, rs_errno);
  rs_freelocale(locale);
}

TEST(newlocale, name_null) {
  ouma_locale_t locale = rs_newlocale(RS_LC_ALL_MASK, nullptr, 0);
  ASSERT_EQ(nullptr, locale);
  ASSERT_EQ(EINVAL, rs_errno);
  rs_freelocale(locale);
}

TEST(newlocale, unknown) {
  ouma_locale_t locale = rs_newlocale(RS_LC_ALL_MASK, "Unknown", 0);
  ASSERT_EQ(nullptr, locale);
  ASSERT_EQ(ENOENT, rs_errno);
}

TEST(newlocale, success_belgium) {
  rs_errno = 0;

  ouma_locale_t locale = rs_newlocale(RS_LC_ALL_MASK, "nl_BE.UTF-8", 0);
  ASSERT_NE(nullptr, locale);
  ASSERT_NE(ENOENT, rs_errno);
  rs_freelocale(locale);
}

TEST(newlocale, success_netherlands) {
  rs_errno = 0;

  ouma_locale_t locale = rs_newlocale(RS_LC_CTYPE_MASK, "nl_NL.UTF-8", 0);
  ASSERT_NE(nullptr, locale);
  ASSERT_NE(ENOENT, rs_errno);
  rs_freelocale(locale);
}

TEST(uselocale, example) {
  rs_uselocale(RS_LC_GLOBAL_LOCALE);

  ouma_locale_t original = rs_uselocale(nullptr);
  ASSERT_NE(original, nullptr);
  ASSERT_EQ(RS_LC_GLOBAL_LOCALE, original);

  ouma_locale_t n = rs_newlocale(RS_LC_ALL_MASK, "C", nullptr);
  ASSERT_NE(n, nullptr);
  ASSERT_NE(n, original);

  ouma_locale_t old = rs_uselocale(n);

  ASSERT_EQ(old, original);
  ASSERT_EQ(n, rs_uselocale(nullptr));
}

TEST(getlocalename_l, good) {
  ouma_locale_t locale = rs_newlocale(RS_LC_ALL_MASK, "en_US.UTF-8", nullptr);

  ASSERT_STREQ("en_US.UTF-8", rs_getlocalename_l(RS_LC_ALL, locale));

  ouma_locale_t new_locale =
      rs_newlocale(RS_LC_MESSAGES_MASK, "de_DE.UTF-8", locale);

  ASSERT_STREQ(
      "LC_COLLATE=en_US.UTF-8;LC_CTYPE=en_US.UTF-8;LC_MESSAGES=de_DE.UTF-8;LC_"
      "MONETARY=en_US.UTF-8;LC_NUMERIC=en_US.UTF-8;LC_TIME=en_US.UTF-8",
      rs_getlocalename_l(RS_LC_ALL, new_locale));

  rs_freelocale(new_locale);
  rs_freelocale(locale);
}

TEST(getlocalename_l, bad) {
  ouma_locale_t locale = rs_newlocale(RS_LC_CTYPE_MASK, "pdc_US", nullptr);

  ASSERT_EQ(nullptr, rs_getlocalename_l(1337, locale));
  ASSERT_EQ(nullptr, rs_getlocalename_l(RS_LC_ALL, nullptr));

  rs_freelocale(locale);
}

TEST(duplocale, example) {
  ouma_locale_t n = rs_newlocale(RS_LC_ALL_MASK, "C", nullptr);
  ASSERT_NE(n, nullptr);

  ouma_locale_t duplicate = rs_duplocale(n);
  ASSERT_NE(duplicate, nullptr);

  ouma_locale_t g = rs_uselocale(duplicate);
  ASSERT_NE(g, nullptr);

  ouma_locale_t f = rs_newlocale(RS_LC_COLLATE_MASK, "en_US", duplicate);
  ASSERT_NE(f, nullptr);

  ouma_locale_t h = rs_uselocale(f);
  ASSERT_NE(h, nullptr);

  ouma_locale_t duplicate2 = rs_duplocale(h);
  ASSERT_NE(duplicate, nullptr);

  ouma_locale_t i = rs_uselocale(duplicate2);
  ASSERT_NE(i, nullptr);
}
