#include "common.h"

#define STROGINO_LC_GLOBAL_LOCALE ((strogino_locale_t)-1)

extern "C" {
  extern _Thread_local int __stroginointernal_errno;

  strogino_locale_t rs_duplocale(strogino_locale_t);
  void rs_freelocale(strogino_locale_t);
  //struct lconv *rs_localeconv(void);
  strogino_locale_t rs_newlocale(int, const char *, strogino_locale_t);
  char *rs_setlocale(int, const char *);
  strogino_locale_t rs_uselocale (strogino_locale_t);
}

TEST(setlocale, good) {
  ASSERT_STREQ("C", rs_setlocale(LC_MESSAGES, "C"));
  ASSERT_STREQ("POSIX", rs_setlocale(LC_TIME, "POSIX"));
  ASSERT_STREQ("ru_RU.UTF-8", rs_setlocale(LC_ALL, "ru_RU.UTF-8"));
  ASSERT_STREQ("en_US.UTF-8", rs_setlocale(LC_MONETARY, "en_US.UTF-8"));
}

TEST(setlocale, bad) {
  ASSERT_EQ(NULL, rs_setlocale(LC_ALL, "De"));
  ASSERT_EQ(NULL, rs_setlocale(1337, "ru_RU.UTF-8"));
  ASSERT_EQ(NULL, rs_setlocale(7, "ru_RU"));
}

TEST(duplocale, global) {
  strogino_locale_t cloned_global = rs_duplocale(STROGINO_LC_GLOBAL_LOCALE);
  ASSERT_TRUE(cloned_global != nullptr);
  rs_freelocale(cloned_global);
}

TEST(duplocale, nouveau) {
  strogino_locale_t nouveau = rs_newlocale(LC_MESSAGES_MASK, "ru_RU.UTF-8", 0);
  strogino_locale_t cloned = rs_duplocale(nouveau);
  ASSERT_TRUE(cloned != nullptr);
  rs_freelocale(cloned);
}

TEST(newlocale, zero) {
  strogino_locale_t locale = rs_newlocale(0, "Unknown", 0);
  ASSERT_EQ(STROGINO_LC_GLOBAL_LOCALE, locale);
  rs_freelocale(locale);
}

TEST(newlocale, empty_name) {
  strogino_locale_t locale = rs_newlocale(LC_ALL_MASK, "", 0);
  ASSERT_EQ(STROGINO_LC_GLOBAL_LOCALE, locale);
  rs_freelocale(locale);
}

TEST(newlocale, unknown) {
  strogino_locale_t locale = rs_newlocale(LC_ALL_MASK, "Unknown", 0);
  ASSERT_EQ(0, locale);
  ASSERT_EQ(ENOENT, __stroginointernal_errno);
}

TEST(newlocale, posix) {
  strogino_locale_t locale = rs_newlocale(LC_ALL_MASK, "POSIX", 0);
  ASSERT_NE(STROGINO_LC_GLOBAL_LOCALE, locale);
  rs_freelocale(locale);
}

TEST(newlocale, c) {
  strogino_locale_t locale = rs_newlocale(LC_ALL_MASK, "C", 0);
  ASSERT_NE(STROGINO_LC_GLOBAL_LOCALE, locale);
  rs_freelocale(locale);
}

TEST(newlocale, c_unicode) {
  strogino_locale_t locale = rs_newlocale(LC_ALL_MASK, "C.UTF-8", 0);
  ASSERT_NE(STROGINO_LC_GLOBAL_LOCALE, locale);
  rs_freelocale(locale);
}

TEST(newlocale, charset_only) {
  strogino_locale_t locale = rs_newlocale(LC_CTYPE_MASK, ".UTF-8", 0);
  ASSERT_NE(STROGINO_LC_GLOBAL_LOCALE, locale);
  rs_freelocale(locale);
}

TEST(uselocale, example) {
  strogino_locale_t original = rs_uselocale(nullptr);
  EXPECT_FALSE(original == nullptr);
  EXPECT_EQ(STROGINO_LC_GLOBAL_LOCALE, original);
  strogino_locale_t n = rs_newlocale(LC_CTYPE_MASK, "C", nullptr);
  EXPECT_FALSE(n == original);
  strogino_locale_t old = rs_uselocale(n);
  EXPECT_TRUE(old == original);
  EXPECT_EQ(STROGINO_LC_GLOBAL_LOCALE, rs_uselocale(nullptr));
}
