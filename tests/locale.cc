#include "common.h"

extern "C" {
  //strogino_locale_t rs_duplocale(strogino_locale_t);
  //void rs_freelocale(strogino_locale_t);
  struct lconv *rs_localeconv(void);
  //strogino_locale_t rs_newlocale(int, const char *, strogino_locale_t);
  char *rs_setlocale(int, const char *);
  //strogino_locale_t rs_uselocale (strogino_locale_t);
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
