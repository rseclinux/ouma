#include "common.h"
#include <clocale>
#include <gtest/gtest.h>
#include <locale>

extern "C"
{
  struct lconv* rs_localeconv(void);
  struct lconv* rs_localeconv_l(strogino_locale_t);

  strogino_locale_t rs_duplocale(strogino_locale_t);
  void rs_freelocale(strogino_locale_t);
  const char* rs_getlocalename_l(int, strogino_locale_t);
  strogino_locale_t rs_newlocale(int, const char*, strogino_locale_t);
  strogino_locale_t rs_uselocale(strogino_locale_t);
}

TEST(localeconv, posix)
{
  ASSERT_STREQ("C", rs_setlocale(LC_ALL, "POSIX"));

  struct lconv* lconv = rs_localeconv();

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

TEST(localeconv, ukraine)
{
  strogino_locale_t locale =
    rs_newlocale(LC_NUMERIC_MASK | LC_MONETARY_MASK, "uk_UA.UTF-8", nullptr);
  ASSERT_NE(nullptr, locale);
  ASSERT_NE(ENOENT, rs_errno);

  struct lconv* lconv = rs_localeconv_l(locale);

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

  ASSERT_EQ(lconv, rs_localeconv_l(locale));

  rs_freelocale(locale);
}

TEST(setlocale, good)
{
  ASSERT_STREQ("C", rs_setlocale(LC_ALL, "C"));
  ASSERT_STREQ("en_US.UTF-8", rs_setlocale(LC_ALL, "en_US.UTF-8"));
  ASSERT_STREQ("sv_SE.utf8", rs_setlocale(LC_COLLATE, "sv_SE.utf8"));
  ASSERT_STREQ("sr_RS.utf8", rs_setlocale(LC_MONETARY, "sr_RS.utf8"));
  ASSERT_STREQ("en_US", rs_setlocale(LC_CTYPE, "en_US"));
  ASSERT_STREQ("POSIX.utf8", rs_setlocale(LC_CTYPE, "POSIX.utf8"));
  ASSERT_STREQ("pdc_US.utf8", rs_setlocale(LC_MESSAGES, "pdc_US.utf8"));
  // TODO: fix todo*
  ASSERT_STREQ(
    "LC_COLLATE=sv_SE.utf8;LC_CTYPE=POSIX.utf8;LC_MESSAGES=pdc_US.utf8;LC_"
    "MONETARY=sr_RS.utf8;LC_NUMERIC=en_US.UTF-8;LC_TIME=todo time",
    rs_setlocale(LC_ALL, NULL));
}

TEST(setlocale, bad)
{
  ASSERT_STREQ(NULL, rs_setlocale(1337, "C"));
  ASSERT_STREQ(NULL, rs_setlocale(LC_ALL, "phew"));
  ASSERT_STREQ(NULL, rs_setlocale(LC_CTYPE, "."));
  ASSERT_STREQ(NULL, rs_setlocale(LC_CTYPE, ".no_supported"));
  ASSERT_STREQ(NULL, rs_setlocale(LC_COLLATE, "horrible"));
  ASSERT_STREQ(NULL, rs_setlocale(LC_ALL, "sr-SR@latin"));
}

TEST(newlocale, zero_mask)
{
  strogino_locale_t locale = rs_newlocale(0, "Unknown", 0);
  ASSERT_EQ(nullptr, locale);
  ASSERT_EQ(EINVAL, rs_errno);
  rs_freelocale(locale);
}

TEST(newlocale, name_null)
{
  strogino_locale_t locale = rs_newlocale(LC_ALL_MASK, nullptr, 0);
  ASSERT_EQ(nullptr, locale);
  ASSERT_EQ(EINVAL, rs_errno);
  rs_freelocale(locale);
}

TEST(newlocale, unknown)
{
  strogino_locale_t locale = rs_newlocale(LC_ALL_MASK, "Unknown", 0);
  ASSERT_EQ(nullptr, locale);
  ASSERT_EQ(ENOENT, rs_errno);
}

TEST(newlocale, success_belgium)
{
  rs_errno = 0;

  strogino_locale_t locale = rs_newlocale(LC_ALL_MASK, "nl_BE.UTF-8", 0);
  ASSERT_NE(nullptr, locale);
  ASSERT_NE(ENOENT, rs_errno);
  rs_freelocale(locale);
}

TEST(newlocale, success_netherlands)
{
  rs_errno = 0;

  strogino_locale_t locale = rs_newlocale(LC_CTYPE_MASK, "nl_NL.UTF-8", 0);
  ASSERT_NE(nullptr, locale);
  ASSERT_NE(ENOENT, rs_errno);
  rs_freelocale(locale);
}

TEST(uselocale, example)
{
  rs_uselocale(STROGINO_LC_GLOBAL_LOCALE);

  strogino_locale_t original = rs_uselocale(nullptr);
  ASSERT_NE(original, nullptr);
  ASSERT_EQ(STROGINO_LC_GLOBAL_LOCALE, original);

  strogino_locale_t n = rs_newlocale(LC_ALL_MASK, "C", nullptr);
  ASSERT_NE(n, nullptr);
  ASSERT_NE(n, original);

  strogino_locale_t old = rs_uselocale(n);

  ASSERT_EQ(old, original);
  ASSERT_EQ(n, rs_uselocale(nullptr));
}

TEST(getlocalename_l, good)
{
  strogino_locale_t locale = rs_newlocale(LC_ALL_MASK, "en_US.UTF-8", nullptr);

  ASSERT_STREQ("en_US.UTF-8", rs_getlocalename_l(LC_ALL, locale));

  strogino_locale_t new_locale =
    rs_newlocale(LC_MESSAGES_MASK, "ru_RU.UTF-8", locale);

  ASSERT_STREQ(
    "LC_COLLATE=en_US.UTF-8;LC_CTYPE=en_US.UTF-8;LC_MESSAGES=ru_RU.UTF-8;LC_"
    "MONETARY=en_US.UTF-8;LC_NUMERIC=en_US.UTF-8;LC_TIME=todo time",
    rs_getlocalename_l(LC_ALL, new_locale));

  rs_freelocale(new_locale);
  rs_freelocale(locale);
}

TEST(getlocalename_l, bad)
{
  strogino_locale_t locale = rs_newlocale(LC_CTYPE_MASK, "pdc_US", nullptr);

  ASSERT_EQ(nullptr, rs_getlocalename_l(1337, locale));
  ASSERT_EQ(nullptr, rs_getlocalename_l(LC_ALL, nullptr));

  rs_freelocale(locale);
}
