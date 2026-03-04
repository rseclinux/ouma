#include "common.h"

extern "C" {
size_t rs_c8rtomb(char *, char8_t, strogino_mbstate_t *);
size_t rs_c16rtomb(char *, char16_t, strogino_mbstate_t *);
size_t rs_c32rtomb(char *, char32_t, strogino_mbstate_t *);
size_t rs_mbrtoc8(char8_t *, const char *, size_t, strogino_mbstate_t *);
size_t rs_mbrtoc16(char16_t *, const char *, size_t, strogino_mbstate_t *);
size_t rs_mbrtoc32(char32_t *, const char *, size_t, strogino_mbstate_t *);
int rs_mbsinit(const strogino_mbstate_t *);
}

TEST(c8rtomb, unicode) {
  rs_setlocale(LC_ALL, "C.UTF-8");

  {
    const char8_t *u8s = (const char8_t *)u8"\x00";
    char buf[MB_LEN_MAX] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_c8rtomb(buf, u8s[0], &s), (size_t)1);
    ASSERT_EQ(buf[0], (char)0x00);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char8_t *u8s = (const char8_t *)u8"\x01";
    char buf[MB_LEN_MAX] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_c8rtomb(buf, u8s[0], &s), (size_t)1);
    ASSERT_EQ(buf[0], (char)0x01);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char8_t *u8s = (const char8_t *)u8"\x7F";
    char buf[MB_LEN_MAX] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_c8rtomb(buf, u8s[0], &s), (size_t)1);
    ASSERT_EQ(buf[0], (char)0x7F);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char8_t *u8s = (const char8_t *)u8"\xC2\x80";
    char buf[MB_LEN_MAX] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_c8rtomb(buf, u8s[0], &s), (size_t)0);
    ASSERT_EQ(rs_c8rtomb(buf, u8s[1], &s), (size_t)2);
    ASSERT_EQ(buf[0], (char)0xC2);
    ASSERT_EQ(buf[1], (char)0x80);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char8_t *u8s = (const char8_t *)u8"\u07FF";
    char buf[MB_LEN_MAX] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_c8rtomb(buf, u8s[0], &s), (size_t)0);
    ASSERT_EQ(rs_c8rtomb(buf, u8s[1], &s), (size_t)2);
    ASSERT_EQ(buf[0], (char)0xDF);
    ASSERT_EQ(buf[1], (char)0xBF);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char8_t *u8s = (const char8_t *)u8"\u0800";
    char buf[MB_LEN_MAX] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_c8rtomb(buf, u8s[0], &s), (size_t)0);
    ASSERT_EQ(rs_c8rtomb(buf, u8s[1], &s), (size_t)0);
    ASSERT_EQ(rs_c8rtomb(buf, u8s[2], &s), (size_t)3);
    ASSERT_EQ(buf[0], (char)0xE0);
    ASSERT_EQ(buf[1], (char)0xA0);
    ASSERT_EQ(buf[2], (char)0x80);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char8_t *u8s = (const char8_t *)u8"\uD7FF";
    char buf[MB_LEN_MAX] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_c8rtomb(buf, u8s[0], &s), (size_t)0);
    ASSERT_EQ(rs_c8rtomb(buf, u8s[1], &s), (size_t)0);
    ASSERT_EQ(rs_c8rtomb(buf, u8s[2], &s), (size_t)3);
    ASSERT_EQ(buf[0], (char)0xED);
    ASSERT_EQ(buf[1], (char)0x9F);
    ASSERT_EQ(buf[2], (char)0xBF);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char8_t *u8s = (const char8_t *)u8"\uE000";
    char buf[MB_LEN_MAX] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_c8rtomb(buf, u8s[0], &s), (size_t)0);
    ASSERT_EQ(rs_c8rtomb(buf, u8s[1], &s), (size_t)0);
    ASSERT_EQ(rs_c8rtomb(buf, u8s[2], &s), (size_t)3);
    ASSERT_EQ(buf[0], (char)0xEE);
    ASSERT_EQ(buf[1], (char)0x80);
    ASSERT_EQ(buf[2], (char)0x80);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char8_t *u8s = (const char8_t *)u8"\uFEFF";
    char buf[MB_LEN_MAX] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_c8rtomb(buf, u8s[0], &s), (size_t)0);
    ASSERT_EQ(rs_c8rtomb(buf, u8s[1], &s), (size_t)0);
    ASSERT_EQ(rs_c8rtomb(buf, u8s[2], &s), (size_t)3);
    ASSERT_EQ(buf[0], (char)0xEF);
    ASSERT_EQ(buf[1], (char)0xBB);
    ASSERT_EQ(buf[2], (char)0xBF);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char8_t *u8s = (const char8_t *)u8"\uFFFD";
    char buf[MB_LEN_MAX] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_c8rtomb(buf, u8s[0], &s), (size_t)0);
    ASSERT_EQ(rs_c8rtomb(buf, u8s[1], &s), (size_t)0);
    ASSERT_EQ(rs_c8rtomb(buf, u8s[2], &s), (size_t)3);
    ASSERT_EQ(buf[0], (char)0xEF);
    ASSERT_EQ(buf[1], (char)0xBF);
    ASSERT_EQ(buf[2], (char)0xBD);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char8_t *u8s = (const char8_t *)u8"\uFFFF";
    char buf[MB_LEN_MAX] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_c8rtomb(buf, u8s[0], &s), (size_t)0);
    ASSERT_EQ(rs_c8rtomb(buf, u8s[1], &s), (size_t)0);
    ASSERT_EQ(rs_c8rtomb(buf, u8s[2], &s), (size_t)3);
    ASSERT_EQ(buf[0], (char)0xEF);
    ASSERT_EQ(buf[1], (char)0xBF);
    ASSERT_EQ(buf[2], (char)0xBF);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char8_t *u8s = (const char8_t *)u8"\U00010000";
    char buf[MB_LEN_MAX] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_c8rtomb(buf, u8s[0], &s), (size_t)0);
    ASSERT_EQ(rs_c8rtomb(buf, u8s[1], &s), (size_t)0);
    ASSERT_EQ(rs_c8rtomb(buf, u8s[2], &s), (size_t)0);
    ASSERT_EQ(rs_c8rtomb(buf, u8s[3], &s), (size_t)4);
    ASSERT_EQ(buf[0], (char)0xF0);
    ASSERT_EQ(buf[1], (char)0x90);
    ASSERT_EQ(buf[2], (char)0x80);
    ASSERT_EQ(buf[3], (char)0x80);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char8_t *u8s = (const char8_t *)u8"\U0010FFFF";
    char buf[MB_LEN_MAX] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_c8rtomb(buf, u8s[0], &s), (size_t)0);
    ASSERT_EQ(rs_c8rtomb(buf, u8s[1], &s), (size_t)0);
    ASSERT_EQ(rs_c8rtomb(buf, u8s[2], &s), (size_t)0);
    ASSERT_EQ(rs_c8rtomb(buf, u8s[3], &s), (size_t)4);
    ASSERT_EQ(buf[0], (char)0xF4);
    ASSERT_EQ(buf[1], (char)0x8F);
    ASSERT_EQ(buf[2], (char)0xBF);
    ASSERT_EQ(buf[3], (char)0xBF);
    ASSERT_NE(0, rs_mbsinit(&s));
  }
}

TEST(c16rtomb, unicode) {
  rs_setlocale(LC_CTYPE, "C.UTF-8");

  char buf[MB_LEN_MAX];
  strogino_mbstate_t mbs{};
  ASSERT_EQ(1, rs_c16rtomb(buf, u'A', &mbs));
  ASSERT_EQ('A', buf[0]);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ(1, rs_c16rtomb(buf, u'\0', &mbs));
  ASSERT_EQ('\0', buf[0]);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ(3, rs_c16rtomb(buf, u'€', &mbs));
  ASSERT_THAT(buf, testing::StartsWith("€"));
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ(0, rs_c16rtomb(buf, 0xd801, &mbs));
  ASSERT_EQ(0, rs_mbsinit(&mbs));
  ASSERT_EQ(4, rs_c16rtomb(buf, 0xdc37, &mbs));
  ASSERT_THAT(buf, testing::StartsWith("𐐷"));
  ASSERT_NE(0, rs_mbsinit(&mbs));
}

TEST(c32rtomb, unicode) {
  rs_setlocale(LC_CTYPE, "C.UTF-8");

  char buf[MB_LEN_MAX];
  ASSERT_EQ(1, rs_c32rtomb(buf, U'A', NULL));
  ASSERT_EQ('A', buf[0]);
  ASSERT_EQ(1, rs_c32rtomb(buf, U'\0', NULL));
  ASSERT_EQ('\0', buf[0]);
  ASSERT_EQ(3, rs_c32rtomb(buf, U'€', NULL));
  ASSERT_THAT(buf, testing::StartsWith("€"));
  ASSERT_EQ((size_t)-1, rs_c32rtomb(buf, 0xd801, NULL));
  ASSERT_EQ(EILSEQ, rs_errno);
}

TEST(mbrtoc8, unicode) {
  rs_setlocale(LC_ALL, "C.UTF-8");

  {
    const char *mbs = "";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 0, &s), (size_t)-2);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\x00";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)0);
    mbs += 1;
    ASSERT_EQ(buf[0], 0x00);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\x01";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)1);
    mbs += 1;
    ASSERT_EQ(buf[0], 0x01);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\x7F";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)1);
    mbs += 1;
    ASSERT_EQ(buf[0], 0x7F);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\xC2\x80";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)2);
    mbs += 2;
    ASSERT_EQ(buf[0], 0xC2);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0x80);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\xC2\x80";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-2);
    mbs += 1;
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)1);
    mbs += 1;
    ASSERT_EQ(buf[0], 0xC2);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0x80);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\xDF\xBF";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)2);
    mbs += 2;
    ASSERT_EQ(buf[0], 0xDF);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0xBF);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\xDF\xBF";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-2);
    mbs += 1;
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)1);
    mbs += 1;
    ASSERT_EQ(buf[0], 0xDF);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0xBF);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\xE0\xA0\x80";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)3);
    mbs += 3;
    ASSERT_EQ(buf[0], 0xE0);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0xA0);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0x80);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\xE0\xA0\x80";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-2);
    mbs += 1;
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-2);
    mbs += 1;
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)1);
    mbs += 1;
    ASSERT_EQ(buf[0], 0xE0);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0xA0);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0x80);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\xED\x9F\xBF";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)3);
    mbs += 3;
    ASSERT_EQ(buf[0], 0xED);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0x9F);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0xBF);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\xED\x9F\xBF";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-2);
    mbs += 1;
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-2);
    mbs += 1;
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)1);
    mbs += 1;
    ASSERT_EQ(buf[0], 0xED);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0x9F);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0xBF);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\xEE\x80\x80";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)3);
    mbs += 3;
    ASSERT_EQ(buf[0], 0xEE);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0x80);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0x80);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\xEE\x80\x80";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-2);
    mbs += 1;
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-2);
    mbs += 1;
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)1);
    mbs += 1;
    ASSERT_EQ(buf[0], 0xEE);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0x80);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0x80);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\xEF\xBB\xBF";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)3);
    mbs += 3;
    ASSERT_EQ(buf[0], 0xEF);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0xBB);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0xBF);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\xEF\xBB\xBF";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-2);
    mbs += 1;
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-2);
    mbs += 1;
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)1);
    mbs += 1;
    ASSERT_EQ(buf[0], 0xEF);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0xBB);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0xBF);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\xEF\xBF\xBD";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)3);
    mbs += 3;
    ASSERT_EQ(buf[0], 0xEF);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0xBF);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0xBD);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\xEF\xBF\xBD";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-2);
    mbs += 1;
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-2);
    mbs += 1;
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)1);
    mbs += 1;
    ASSERT_EQ(buf[0], 0xEF);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0xBF);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0xBD);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\xEF\xBF\xBF";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)3);
    mbs += 3;
    ASSERT_EQ(buf[0], 0xEF);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0xBF);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0xBF);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\xEF\xBF\xBF";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-2);
    mbs += 1;
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-2);
    mbs += 1;
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)1);
    mbs += 1;
    ASSERT_EQ(buf[0], 0xEF);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0xBF);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0xBF);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\xF0\x90\x80\x80";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)4);
    mbs += 4;
    ASSERT_EQ(buf[0], 0xF0);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0x90);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0x80);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0x80);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\xF0\x90\x80\x80";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-2);
    mbs += 1;
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-2);
    mbs += 1;
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-2);
    mbs += 1;
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)1);
    mbs += 1;
    ASSERT_EQ(buf[0], 0xF0);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0x90);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0x80);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0x80);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\xF4\x8F\xBF\xBF";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)4);
    mbs += 4;
    ASSERT_EQ(buf[0], 0xF4);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0x8F);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0xBF);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, strlen(mbs) + 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0xBF);
    ASSERT_NE(0, rs_mbsinit(&s));
  }

  {
    const char *mbs = "\xF4\x8F\xBF\xBF";
    char8_t buf[1] = {0};
    strogino_mbstate_t s = {0};

    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-2);
    mbs += 1;
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-2);
    mbs += 1;
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-2);
    mbs += 1;
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)1);
    mbs += 1;
    ASSERT_EQ(buf[0], 0xF4);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0x8F);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0xBF);
    ASSERT_EQ(rs_mbrtoc8(buf, mbs, 1, &s), (size_t)-3);
    ASSERT_EQ(buf[0], 0xBF);
    ASSERT_NE(0, rs_mbsinit(&s));
  }
}

TEST(mbrtoc16, unicode) {
  rs_setlocale(LC_CTYPE, "C.UTF-8");

  strogino_mbstate_t mbs{};
  char16_t c16;
  ASSERT_EQ(1, rs_mbrtoc16(&c16, "Foo", 3, &mbs));
  ASSERT_EQ(u'F', c16);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ((size_t)-2, rs_mbrtoc16(&c16, "\xf0\x90", 2, &mbs));
  ASSERT_EQ(0, rs_mbsinit(&mbs));
  ASSERT_EQ((size_t)-2, rs_mbrtoc16(&c16, "\x90", 1, &mbs));
  ASSERT_EQ(0, rs_mbsinit(&mbs));
  ASSERT_EQ(1, rs_mbrtoc16(&c16, "\xb7", 1, &mbs));
  ASSERT_EQ(0xd801, c16);
  ASSERT_EQ(0, rs_mbsinit(&mbs));
  ASSERT_EQ((size_t)-3, rs_mbrtoc16(&c16, "AAA", 3, &mbs));
  ASSERT_EQ(0xdc37, c16);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ(0, rs_mbrtoc16(&c16, "", 1, &mbs));
  ASSERT_EQ(u'\0', c16);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ((size_t)-2, rs_mbrtoc16(&c16, "Some text", 0, &mbs));
  ASSERT_NE(0, rs_mbsinit(&mbs));
}

TEST(mbrtoc32, unicode) {
  rs_setlocale(LC_CTYPE, "C.UTF-8");

  strogino_mbstate_t mbs{};
  char32_t c32;
  ASSERT_EQ(1, rs_mbrtoc32(&c32, "Foo", 3, &mbs));
  ASSERT_EQ(U'F', c32);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ((size_t)-2, rs_mbrtoc32(&c32, "\xf0\x90", 2, &mbs));
  ASSERT_EQ(0, rs_mbsinit(&mbs));
  ASSERT_EQ((size_t)-2, rs_mbrtoc32(&c32, "\x90", 1, &mbs));
  ASSERT_EQ(0, rs_mbsinit(&mbs));
  ASSERT_EQ(1, rs_mbrtoc32(&c32, "\xb7", 1, &mbs));
  ASSERT_EQ(U'𐐷', c32);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ(0, rs_mbrtoc32(&c32, "", 1, &mbs));
  ASSERT_EQ(U'\0', c32);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ((size_t)-2, rs_mbrtoc32(&c32, "Some text", 0, &mbs));
  ASSERT_NE(0, rs_mbsinit(&mbs));
}
