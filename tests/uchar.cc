#include "common.h"

extern "C" {
  extern _Thread_local int __stroginointernal_errno;

  size_t rs_c16rtomb(char *, char16_t, strogino_mbstate_t *);
  size_t rs_c32rtomb(char *, char32_t, strogino_mbstate_t *);
  size_t rs_mbrtoc16(char16_t *, const char *, size_t, strogino_mbstate_t *);
  size_t rs_mbrtoc32(char32_t *, const char *, size_t, strogino_mbstate_t *);
  int rs_mbsinit(const strogino_mbstate_t *);

  char *rs_setlocale(int, const char *);
}

TEST(c16rtomb, ascii) {
  rs_setlocale(LC_CTYPE, "C");

  char c;
  strogino_mbstate_t mbs{};
  ASSERT_EQ(1, rs_c16rtomb(&c, u'A', &mbs));
  ASSERT_EQ('A', c);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ(1, rs_c16rtomb(&c, u'\0', &mbs));
  ASSERT_EQ('\0', c);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ((size_t)-1, rs_c16rtomb(&c, u'€', &mbs));
  ASSERT_EQ(EILSEQ, __stroginointernal_errno);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ(0, rs_c16rtomb(&c, 0xd801, &mbs));
  ASSERT_EQ(0, rs_mbsinit(&mbs));
  ASSERT_EQ((size_t)-1, rs_c16rtomb(&c, 0xdc37, &mbs));
  ASSERT_EQ(EILSEQ, __stroginointernal_errno);
  ASSERT_EQ(0, rs_mbsinit(&mbs));
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

TEST(c32rtomb, ascii) {
  rs_setlocale(LC_CTYPE, "C");

  char c;
  ASSERT_EQ(1, rs_c32rtomb(&c, U'A', NULL));
  ASSERT_EQ('A', c);
  ASSERT_EQ(1, rs_c32rtomb(&c, U'\0', NULL));
  ASSERT_EQ('\0', c);
  ASSERT_EQ((size_t)-1, rs_c32rtomb(&c, U'€', NULL));
  ASSERT_EQ(EILSEQ, __stroginointernal_errno);
  ASSERT_EQ((size_t)-1, rs_c32rtomb(&c, 0xd801, NULL));
  ASSERT_EQ(EILSEQ, __stroginointernal_errno);
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
  ASSERT_EQ(EILSEQ, __stroginointernal_errno);
}

TEST(mbrtoc16, ascii) {
  rs_setlocale(LC_CTYPE, "C");

  strogino_mbstate_t mbs{};
  char16_t c16;
  ASSERT_EQ(1, rs_mbrtoc16(&c16, "Foo", 3, &mbs));
  ASSERT_EQ(u'F', c16);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ(0, rs_mbrtoc16(&c16, "", 1, &mbs));
  ASSERT_EQ(u'\0', c16);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ((size_t)-2, rs_mbrtoc16(&c16, "Some text", 0, &mbs));
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ((size_t)-1, rs_mbrtoc16(&c16, "€", 4, &mbs));
  ASSERT_EQ(EILSEQ, __stroginointernal_errno);
  ASSERT_NE(0, rs_mbsinit(&mbs));
}

TEST(mbrtoc16, unicode) {
  rs_setlocale(LC_CTYPE, "C.UTF-8");

  strogino_mbstate_t mbs{};
  char16_t c16;
  ASSERT_EQ(1, rs_mbrtoc16(&c16, "Foo", 3, &mbs));
  ASSERT_EQ(u'F', c16);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ((size_t)-2,
            rs_mbrtoc16(&c16, "\xf0\x90", 2, &mbs));
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
  ASSERT_EQ((size_t)-2,
            rs_mbrtoc16(&c16, "Some text", 0, &mbs));
  ASSERT_NE(0, rs_mbsinit(&mbs));
}

TEST(mbrtoc32, ascii) {
  rs_setlocale(LC_CTYPE, "C");

  strogino_mbstate_t mbs{};
  char32_t c32;
  ASSERT_EQ(1, rs_mbrtoc32(&c32, "Foo", 3, &mbs));
  ASSERT_EQ(U'F', c32);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ(0, rs_mbrtoc32(&c32, "", 1, &mbs));
  ASSERT_EQ(U'\0', c32);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ((size_t)-2, rs_mbrtoc32(&c32, "Some text", 0, &mbs));
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ((size_t)-1, rs_mbrtoc32(&c32, "€", 4, &mbs));
  ASSERT_EQ(EILSEQ, __stroginointernal_errno);
  ASSERT_NE(0, rs_mbsinit(&mbs));
}

TEST(mbrtoc32, unicode) {
  rs_setlocale(LC_CTYPE, "C.UTF-8");

  strogino_mbstate_t mbs{};
  char32_t c32;
  ASSERT_EQ(1, rs_mbrtoc32(&c32, "Foo", 3, &mbs));
  ASSERT_EQ(U'F', c32);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ((size_t)-2,
            rs_mbrtoc32(&c32, "\xf0\x90", 2, &mbs));
  ASSERT_EQ(0, rs_mbsinit(&mbs));
  ASSERT_EQ((size_t)-2, rs_mbrtoc32(&c32, "\x90", 1, &mbs));
  ASSERT_EQ(0, rs_mbsinit(&mbs));
  ASSERT_EQ(1, rs_mbrtoc32(&c32, "\xb7", 1, &mbs));
  ASSERT_EQ(U'𐐷', c32);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ(0, rs_mbrtoc32(&c32, "", 1, &mbs));
  ASSERT_EQ(U'\0', c32);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ((size_t)-2,
            rs_mbrtoc32(&c32, "Some text", 0, &mbs));
  ASSERT_NE(0, rs_mbsinit(&mbs));
}
