#include "common.h"

#include <wchar.h>

extern "C"
{
  int rs_ffs(int);
  int rs_ffsl(long);
  int rs_ffsll(long long);
  int rs_strcasecmp(const char*, const char*);
  int rs_strcasecmp_l(const char*, const char*, strogino_locale_t);
  int rs_strncasecmp(const char*, const char*, size_t);
  int rs_strncasecmp_l(const char*, const char*, size_t, strogino_locale_t);
}

TEST(strcasecmp, example)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C"), nullptr);

  ASSERT_EQ(0, rs_strcasecmp("hello", "hello"));
  ASSERT_EQ(0, rs_strcasecmp("hElLo", "hello"));

  ASSERT_GT(0, rs_strcasecmp("doge", "dogS"));
  ASSERT_LT(0, rs_strcasecmp("dogs", "dogE"));
}

TEST(strcasecmp, unicode)
{
  strogino_locale_t loc = rs_newlocale(LC_CTYPE_MASK, "nl_BE.UTF-8", 0);
  ASSERT_NE(nullptr, loc);
  ASSERT_NE(ENOENT, rs_errno);
  ASSERT_STREQ("nl_BE.UTF-8", rs_getlocalename_l(LC_CTYPE, loc));

  ASSERT_LT(0, rs_strcasecmp_l("München?", "MÜNCHEN!", loc));
  ASSERT_EQ(0, rs_strcasecmp_l("München", "MÜNCHEN", loc));

  rs_freelocale(loc);
}

TEST(strncasecmp, null)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C"), nullptr);

  ASSERT_EQ(0, rs_strncasecmp(NULL, NULL, 0));
}

TEST(strncasecmp, example)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C"), nullptr);

  ASSERT_EQ(0, rs_strncasecmp("hello", "hello", 100));
  ASSERT_EQ(0, rs_strncasecmp("hElLo", "hello", 100));

  ASSERT_EQ(0, rs_strncasecmp("doge", "dogS", 3));
  ASSERT_GT(0, rs_strncasecmp("doge", "dogS", 4));
  ASSERT_EQ(0, rs_strncasecmp("dogs", "dogE", 3));
  ASSERT_LT(0, rs_strncasecmp("dogs", "dogE", 4));
}

TEST(strncasecmp, unicode)
{
  strogino_locale_t loc = rs_newlocale(LC_CTYPE_MASK, "nl_BE.UTF-8", 0);
  ASSERT_NE(nullptr, loc);
  ASSERT_NE(ENOENT, rs_errno);
  ASSERT_STREQ("nl_BE.UTF-8", rs_getlocalename_l(LC_CTYPE, loc));

  ASSERT_EQ(0, rs_strncasecmp_l("München?", "MÜNCHEN!", 8, loc));
  ASSERT_LT(0, rs_strncasecmp_l("München?", "MÜNCHEN!", 9, loc));

  rs_freelocale(loc);
}
