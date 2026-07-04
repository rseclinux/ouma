#include "common.h"
#include "common_mbstate.h"
#include "common_locale.h"

#include <gtest/gtest.h>
#include <wchar.h>

extern "C" {
wchar_t *rs_wmemchr(const wchar_t *, wchar_t, size_t);
int rs_wmemcmp(const wchar_t *, const wchar_t *, size_t);
wchar_t *rs_wmemcpy(wchar_t *, const wchar_t *, size_t);
wchar_t *rs_wmemmove(wchar_t *, const wchar_t *, size_t);
wchar_t *rs_wmemset(wchar_t *, wchar_t, size_t);
wchar_t *rs_wcpcpy(wchar_t *, const wchar_t *);
wchar_t *rs_wcpncpy(wchar_t *, const wchar_t *, size_t);
wchar_t *rs_wcscat(wchar_t *, const wchar_t *);
wchar_t *rs_wcschr(const wchar_t *, wchar_t);
int rs_wcscmp(const wchar_t *, const wchar_t *);
int rs_wcscoll(const wchar_t *, const wchar_t *);
wchar_t *rs_wcscpy(wchar_t *, const wchar_t *);
size_t rs_wcscspn(const wchar_t *, const wchar_t *);
wchar_t *rs_wcsdup(const wchar_t *);
size_t rs_wcslen(const wchar_t *);
wchar_t *rs_wcsncat(wchar_t *, const wchar_t *, size_t);
int rs_wcsncmp(const wchar_t *, const wchar_t *, size_t);
wchar_t *rs_wcsncpy(wchar_t *, const wchar_t *, size_t);
size_t rs_wcsnlen(const wchar_t *, size_t);
wchar_t *rs_wcspbrk(const wchar_t *, const wchar_t *);
wchar_t *rs_wcsrchr(const wchar_t *, wchar_t);
size_t rs_wcsspn(const wchar_t *, const wchar_t *);
wchar_t *rs_wcsstr(const wchar_t *, const wchar_t *);
wchar_t *rs_wcstok(wchar_t *, const wchar_t *, wchar_t **);
size_t rs_wcsxfrm(wchar_t *, const wchar_t *, size_t);
size_t rs_wcslcat(wchar_t *, const wchar_t *, size_t);
size_t rs_wcslcpy(wchar_t *, const wchar_t *, size_t);
wint_t rs_btowc(int);
size_t rs_mbrlen(const char *, size_t, ouma_mbstate_t *);
size_t rs_mbrtowc(wchar_t *, const char *, size_t, ouma_mbstate_t *);
size_t rs_mbsnrtowcs(wchar_t *, const char **, size_t, size_t,
                     ouma_mbstate_t *);
size_t rs_mbsrtowcs(wchar_t *, const char **, size_t, ouma_mbstate_t *);
size_t rs_wcrtomb(char *, wchar_t, ouma_mbstate_t *);
size_t rs_wcsnrtombs(char *, const wchar_t **, size_t, size_t,
                     ouma_mbstate_t *);
size_t rs_wcsrtombs(char *, const wchar_t **, size_t, ouma_mbstate_t *);
int rs_wctob(wint_t);
int rs_wcscasecmp(const wchar_t *ws1, const wchar_t *ws2);
int rs_wcscasecmp_l(const wchar_t *ws1, const wchar_t *ws2,
                    ouma_locale_t locale);
int rs_wcsncasecmp(const wchar_t *ws1, const wchar_t *ws2, size_t n);
int rs_wcsncasecmp_l(const wchar_t *ws1, const wchar_t *ws2, size_t n,
                     ouma_locale_t locale);
int rs_wcwidth(wchar_t);
int rs_wcswidth(const wchar_t *, size_t);
}

TEST(wmemchr, null) { ASSERT_EQ(NULL, rs_wmemchr((wchar_t *)NULL, L'A', 0)); }

TEST(wmemchr, match) {
  wchar_t buf[] = L"Foo bar baz";
  ASSERT_EQ(buf + 5, rs_wmemchr(buf, L'a', std::size(buf)));
}

TEST(wmemchr, nomatch) {
  wchar_t buf[] = L"Foo bar baz";
  ASSERT_EQ(NULL, rs_wmemchr(buf, L'x', std::size(buf)));
}

TEST(wmemcmp, null) { ASSERT_EQ(0, rs_wmemcmp(NULL, NULL, 0)); }

TEST(wmemcmp, example) {
  const wchar_t buf1[] = L"Hello";
  const wchar_t buf2[] = L"Helxo";
  ASSERT_EQ(0, rs_wmemcmp(buf1, buf1, std::size(buf1)));
  ASSERT_GT(0, rs_wmemcmp(buf1, buf2, std::size(buf1)));
  ASSERT_LT(0, rs_wmemcmp(buf2, buf1, std::size(buf1)));
}

TEST(wmemcpy, null) {
  ASSERT_EQ((wchar_t *)42, rs_wmemcpy((wchar_t *)42, (wchar_t *)123, 0));
}

TEST(wmemcpy, example) {
  const wchar_t buf1[8] = L"Foo\0Bar";
  wchar_t buf2[8];
  ASSERT_EQ(buf2, rs_wmemcpy(buf2, buf1, std::size(buf1)));
  ASSERT_THAT(buf2, testing::ElementsAreArray(buf1));
}

TEST(wmemmove, null) {
  ASSERT_EQ((wchar_t *)42, rs_wmemmove((wchar_t *)42, (wchar_t *)34, 0));
}

TEST(wmemmove, example1) {
  wchar_t buf[] = L"abcdefghijkl";
  ASSERT_EQ(buf, rs_wmemmove(buf, buf + 4, 8));
  ASSERT_STREQ(L"efghijklijkl", buf);
}

TEST(wmemmove, example2) {
  wchar_t buf[] = L"abcdefghijkl";
  ASSERT_EQ(buf + 4, rs_wmemmove(buf + 4, buf, 8));
  ASSERT_STREQ(L"abcdabcdefgh", buf);
}

TEST(wmemset, null) {
  ASSERT_EQ((wchar_t *)5, rs_wmemset((wchar_t *)5, L'A', 0));
}

TEST(wmemset, example) {
  wchar_t buf[11];
  ASSERT_EQ(buf, rs_wmemset(buf, L'!', 10));
  buf[10] = L'\0';
  ASSERT_STREQ(L"!!!!!!!!!!", buf);
}

TEST(wcpcpy, example) {
  wchar_t buf[] = L"AAAAAAAAAA";
  ASSERT_EQ(buf, rs_wcpcpy(buf, L""));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"\0AAAAAAAAA"));
  ASSERT_EQ(buf + 5, rs_wcpcpy(buf, L"Hello"));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"Hello\0AAAA"));
  ASSERT_EQ(buf, rs_wcpcpy(buf, L""));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"\0ello\0AAAA"));
  ASSERT_EQ(buf + 9, rs_wcpcpy(buf, L"Example!!"));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"Example!!\0"));
}

TEST(wcpncpy, null) {
  ASSERT_EQ((wchar_t *)12, rs_wcpncpy((wchar_t *)12, (wchar_t *)500, 0));
}

TEST(wcpncpy, example1) {
  wchar_t buf[] = L"AAAAAAAAAAAA";
  ASSERT_EQ(buf + 5, rs_wcpncpy(buf, L"Hello", 12));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"Hello\0\0\0\0\0\0\0"));
}

TEST(wcpncpy, example2) {
  wchar_t buf[13];
  ASSERT_EQ(buf + 12, rs_wcpncpy(buf, L"This is a very long string", 12));
  buf[12] = '\0';
  ASSERT_THAT(buf, testing::ElementsAreArray(L"This is a ve"));
}

TEST(wcscat, example) {
  wchar_t buf[] = L"\0AAAAAAAAA";
  ASSERT_EQ(buf, rs_wcscat(buf, L""));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"\0AAAAAAAAA"));
  ASSERT_EQ(buf, rs_wcscat(buf, L"Hello"));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"Hello\0AAAA"));
  ASSERT_EQ(buf, rs_wcscat(buf, L""));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"Hello\0AAAA"));
  ASSERT_EQ(buf, rs_wcscat(buf, L"!!!!"));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"Hello!!!!\0"));
}

TEST(wcscpy, example) {
  wchar_t buf[] = L"AAAAAAAAAA";
  ASSERT_EQ(buf, rs_wcscpy(buf, L""));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"\0AAAAAAAAA"));
  ASSERT_EQ(buf, rs_wcscpy(buf, L"Hello"));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"Hello\0AAAA"));
  ASSERT_EQ(buf, rs_wcscpy(buf, L""));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"\0ello\0AAAA"));
  ASSERT_EQ(buf, rs_wcscpy(buf, L"Example!!"));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"Example!!\0"));
}

TEST(wcscspn, example) {
  const wchar_t *wcs = L"Hello, world";
  ASSERT_EQ(0, rs_wcscspn(wcs, L"H"));
  ASSERT_EQ(7, rs_wcscspn(wcs, L"rdw"));
  ASSERT_EQ(12, rs_wcscspn(wcs, L"XYZ"));
}

TEST(wcslen, example) { ASSERT_EQ(22, rs_wcslen(L"ℕ ⊆ ℕ₀ ⊂ ℤ ⊂ ℚ ⊂ ℝ ⊂ ℂ")); }

TEST(wcsncat, example) {
  wchar_t buf[11] = L"\0AAAAAAAAA";
  ASSERT_EQ(buf, rs_wcsncat(buf, L"", 0));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"\0AAAAAAAAA"));
  ASSERT_EQ(buf, rs_wcsncat(buf, L"Hello", 99999));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"Hello\0AAAA"));
  ASSERT_EQ(buf, rs_wcsncat(buf, L"", 1));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"Hello\0AAAA"));
  ASSERT_EQ(buf, rs_wcsncat(buf, L"!!!!!!!!!!!!", 3));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"Hello!!!\0A"));
}

TEST(wcsncmp, null) { ASSERT_EQ(0, rs_wcsncmp(NULL, NULL, 0)); }

TEST(wcsncmp, examples) {
  ASSERT_EQ(0, rs_wcsncmp(L"", L"", 100));
  ASSERT_EQ(0, rs_wcsncmp(L"Hello", L"Hello", 100));

  ASSERT_EQ(0, rs_wcsncmp(L"Hello", L"Hello, world", 5));
  ASSERT_GT(0, rs_wcsncmp(L"Hello", L"Hello, world", 6));
  ASSERT_LT(0, rs_wcsncmp(L"Hello, world", L"Hello", 100));

  ASSERT_EQ(0, rs_wcsncmp(L"Hello!", L"Hello.", 5));
  ASSERT_GT(0, rs_wcsncmp(L"Hello!", L"Hello.", 6));
  ASSERT_LT(0, rs_wcsncmp(L"Hello.", L"Hello!", 100));
}

TEST(wcsncpy, null) {
  ASSERT_EQ((wchar_t *)12, rs_wcsncpy((wchar_t *)12, (wchar_t *)500, 0));
}

TEST(wcsncpy, example1) {
  wchar_t buf[] = L"AAAAAAAAAAAA";
  ASSERT_EQ(buf, rs_wcsncpy(buf, L"Hello", 12));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"Hello\0\0\0\0\0\0\0"));
}

TEST(wcsncpy, example2) {
  wchar_t buf[13];
  ASSERT_EQ(buf, rs_wcsncpy(buf, L"This is a very long string", 12));
  buf[12] = L'\0';
  ASSERT_THAT(buf, testing::ElementsAreArray(L"This is a ve"));
}

TEST(wcsnlen, null) {
  ASSERT_EQ(0, rs_wcsnlen(NULL, 0));
  ASSERT_EQ(0, rs_wcsnlen(L"", 100));
  ASSERT_EQ(7, rs_wcsnlen(L"Σὲ γνωρίζω ἀπὸ τὴν κόψη", 7));
}

TEST(wcspbrk, example) {
  const wchar_t *wcs = L"Hello, world";
  ASSERT_EQ(wcs, rs_wcspbrk(wcs, L"H"));
  ASSERT_EQ(wcs + 7, rs_wcspbrk(wcs, L"rdw"));
  ASSERT_EQ(NULL, rs_wcspbrk(wcs, L"XYZ"));
}

TEST(wcsspn, example) {
  const wchar_t *wcs = L"Hello, world";
  ASSERT_EQ(0, rs_wcsspn(wcs, L""));
  ASSERT_EQ(0, rs_wcsspn(wcs, L"Foo"));
  ASSERT_EQ(5, rs_wcsspn(wcs, L"olHe"));
  ASSERT_EQ(12, rs_wcsspn(wcs, L"Helo, wrld"));
}

TEST(wcsstr, examples) {
  const wchar_t *str = (const wchar_t *)0x42;
  ASSERT_EQ(str, rs_wcsstr(str, L""));

  str = L"Hello world";
  ASSERT_EQ(str + 2, rs_wcsstr(str, L"ll"));
  ASSERT_EQ(str + 4, rs_wcsstr(str, L"o worl"));
  ASSERT_EQ(str + 6, rs_wcsstr(str, L"world"));
  ASSERT_EQ(str + 10, rs_wcsstr(str, L"d"));
  ASSERT_EQ(NULL, rs_wcsstr(str, L"word"));
  ASSERT_EQ(NULL, rs_wcsstr(str, L"world!"));
}

TEST(wcstok, example) {
  wchar_t line[] = L"LINE  TO BE\t\tSEPARATED\n";
  const wchar_t *split = L" \t\n";
  wchar_t *lastws;
  ASSERT_STREQ(L"LINE", rs_wcstok(line, split, &lastws));
  ASSERT_STREQ(L"TO", rs_wcstok(NULL, split, &lastws));
  ASSERT_STREQ(L"BE", rs_wcstok(NULL, split, &lastws));
  ASSERT_STREQ(L"SEPARATED", rs_wcstok(NULL, split, &lastws));
  ASSERT_EQ(NULL, rs_wcstok(NULL, split, &lastws));
}

TEST(wcschr, examples) {
  const wchar_t *str = L"Hello, world";
  ASSERT_EQ(NULL, rs_wcschr(str, L'A'));
  ASSERT_EQ(str + 4, rs_wcschr(str, L'o'));
  ASSERT_EQ(str + 12, rs_wcschr(str, L'\0'));
}

TEST(wcsrchr, examples) {
  const wchar_t *str = L"Hello, world";
  ASSERT_EQ(NULL, rs_wcsrchr(str, L'A'));
  ASSERT_EQ(str + 8, rs_wcsrchr(str, L'o'));
  ASSERT_EQ(str + 12, rs_wcsrchr(str, L'\0'));
}

struct coll_data {
  const wchar_t *a;
  const wchar_t *b;
  int result;
};

static int sign(int a) {
  if (a < 0)
    return -1;
  if (a > 0)
    return 1;
  return 0;
}

static void test_wcscoll(const coll_data *coll) {
  for (unsigned int i = 0; coll[i].a != NULL; ++i) {
    int result = sign(rs_wcscoll(coll[i].a, coll[i].b));
    ASSERT_EQ(result, coll[i].result);
  }
}

static void test_wcsxfrm(const coll_data *coll) {
  for (unsigned int i = 0; coll[i].a != NULL; ++i) {
    int result = 0;
    wchar_t sortKeyA[100], sortKeyB[100];
    rs_wcsxfrm(sortKeyA, coll[i].a, 100);
    rs_wcsxfrm(sortKeyB, coll[i].b, 100);
    result = sign(rs_wcscmp(sortKeyA, sortKeyB));
    ASSERT_EQ(result, coll[i].result);
  }
}

TEST(wcscoll, posix) {
  ASSERT_STREQ("C", rs_setlocale(RS_LC_COLLATE, "C"));

  const coll_data coll[] = {
      {L"", L"", 0},         {L"test", L"test", 0}, {L"tester", L"tester", 0},
      {L"côté", L"côté", 0}, {NULL, NULL, 0},
  };

  test_wcscoll(coll);
  test_wcsxfrm(coll);
}

TEST(wcscoll, uca) {
  ASSERT_STREQ("en_US", rs_setlocale(RS_LC_COLLATE, "en_US"));

  const coll_data coll[] = {
      {L"", L"", 0},
      {L"test", L"test", 0},
      {L"tester", L"test", 1},
      {L"tEst", L"test", 1},
      {L"test", L"tester", -1},
      {L"täst", L"täst", 0},
      {L"tast", L"täst", -1},
      {L"tbst", L"täst", 1},
      {L"tbst", L"tæst", 1},
      {L"täst", L"tÄst", -1},
      {L"tBst", L"tÄst", 1},
      {L"tBst", L"täst", 1},
      {L"taest", L"tæst", -1},
      {L"tafst", L"tæst", 1},
      {L"taa", L"täa", -1},
      {L"tab", L"täb", -1},
      {L"tad", L"täd", -1},
      {L"tae", L"täe", -1},
      {L"taf", L"täf", -1},
      {L"cote", L"coté", -1},
      {L"coté", L"côte", -1},
      {L"côte", L"côté", -1},
      {NULL, NULL, 0},
  };

  test_wcscoll(coll);
  test_wcsxfrm(coll);
}

TEST(wcslcat, null) { ASSERT_EQ(5, rs_wcslcat(nullptr, L"Hello", 0)); }

TEST(wcslcat, one) {
  wchar_t buf = L'\0';
  ASSERT_EQ(6, rs_wcslcat(&buf, L"Banana", 1));
  ASSERT_EQ(L'\0', buf);

  buf = L'A';
  ASSERT_EQ(7, rs_wcslcat(&buf, L"Banana", 1));
  ASSERT_EQ(L'A', buf);
}

TEST(wcslcat, longer) {
  wchar_t buf[] = L"AAAAAAAAAAAA";
  ASSERT_EQ(15, rs_wcslcat(buf, L"Foo", std::size(buf) - 1));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"AAAAAAAAAAAA"));

  buf[4] = L'\0';
  ASSERT_EQ(7, rs_wcslcat(buf, L"Bar", std::size(buf) - 1));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"AAAABar\0AAAA"));

  ASSERT_EQ(16, rs_wcslcat(buf, L"Very long", std::size(buf) - 1));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"AAAABarVery\0"));
}

TEST(wcslcpy, null) { ASSERT_EQ(5, rs_wcslcpy(NULL, L"Hello", 0)); }

TEST(wcslcpy, one) {
  wchar_t buf;
  ASSERT_EQ(6, rs_wcslcpy(&buf, L"Banana", 1));
  ASSERT_EQ(L'\0', buf);
}

TEST(wcslcpy, longer) {
  wchar_t buf[] = L"AAAAAAAAAA";
  ASSERT_EQ(3, rs_wcslcpy(buf, L"Dog", std::size(buf)));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"Dog\0AAAAAA"));
}

TEST(wcslcpy, longest) {
  wchar_t buf[12];
  ASSERT_EQ(23, rs_wcslcpy(buf, L"This is a long sentence", std::size(buf)));
  ASSERT_STREQ(L"This is a l", buf);
}

TEST(btowc, simple) {
  ASSERT_STREQ("C", rs_setlocale(RS_LC_CTYPE, "C"));

  ASSERT_EQ(WEOF, rs_btowc(EOF));
  for (int i = 0; i < 128; ++i) {
    SCOPED_TRACE(i);
    ASSERT_EQ(i, rs_btowc(i));
  }
  for (int i = 128; i < 256; ++i) {
    SCOPED_TRACE(i);
    ASSERT_EQ(WEOF, rs_btowc(i));
  }
}

TEST(mbrlen, euro) {
  ASSERT_STREQ("C.UTF-8", rs_setlocale(RS_LC_CTYPE, "C.UTF-8"));

  char euro[] = "€";
  ouma_mbstate_t mbs{};
  ASSERT_EQ((size_t)-2, rs_mbrlen(&euro[0], 1, &mbs));
  ASSERT_EQ((size_t)-2, rs_mbrlen(&euro[1], 1, &mbs));
  ASSERT_EQ(1, rs_mbrlen(&euro[2], 1, &mbs));
  ASSERT_EQ(0, rs_mbrlen(&euro[3], 1, &mbs));
}

TEST(mbrtowc, ascii) {
  ASSERT_STREQ("C", rs_setlocale(RS_LC_CTYPE, "C"));

  ouma_mbstate_t mbs{};
  wchar_t wc;
  ASSERT_EQ(1, rs_mbrtowc(&wc, "Foo", 3, &mbs));
  ASSERT_EQ(U'F', wc);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ(0, rs_mbrtowc(&wc, "", 1, &mbs));
  ASSERT_EQ(U'\0', wc);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ((size_t)-2, rs_mbrtowc(&wc, "Some text", 0, &mbs));
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ((size_t)-1, rs_mbrtowc(&wc, "€", 4, &mbs));
  ASSERT_NE(0, rs_mbsinit(&mbs));
}

TEST(mbrtowc, unicode) {
  ASSERT_STREQ("C.UTF-8", rs_setlocale(RS_LC_CTYPE, "C.UTF-8"));

  ouma_mbstate_t mbs{};
  wchar_t wc;
  ASSERT_EQ(1, rs_mbrtowc(&wc, "Foo", 3, &mbs));
  ASSERT_EQ(U'F', wc);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ((size_t)-2, rs_mbrtowc(&wc, "\xf0\x90", 2, &mbs));
  ASSERT_EQ(0, rs_mbsinit(&mbs));
  ASSERT_EQ((size_t)-2, rs_mbrtowc(&wc, "\x90", 1, &mbs));
  ASSERT_EQ(0, rs_mbsinit(&mbs));
  ASSERT_EQ(1, rs_mbrtowc(&wc, "\xb7", 1, &mbs));
  ASSERT_EQ(U'𐐷', wc);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ(0, rs_mbrtowc(&wc, "", 1, &mbs));
  ASSERT_EQ(U'\0', wc);
  ASSERT_NE(0, rs_mbsinit(&mbs));
  ASSERT_EQ((size_t)-2, rs_mbrtowc(&wc, "Some text", 0, &mbs));
  ASSERT_NE(0, rs_mbsinit(&mbs));
}

TEST(mbsinit, init) {
  ASSERT_NE(0, rs_mbsinit(NULL));
  ouma_mbstate_t initial_mbstate{};
  ASSERT_NE(0, rs_mbsinit(&initial_mbstate));
}

TEST(mbsrtowcs, posix) {
  ASSERT_STREQ("C", rs_setlocale(RS_LC_CTYPE, "C"));

  char srcbuf[128];
  wchar_t dstbuf[128];
  char *src;
  ouma_mbstate_t s;

  memset(srcbuf, 0xcc, sizeof(srcbuf));
  strcpy(srcbuf, "hello");
  wmemset(dstbuf, 0xcccc, sizeof(dstbuf) / sizeof(*dstbuf));
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  ASSERT_EQ(rs_mbsrtowcs(dstbuf, (const char **)&src,
                         sizeof(dstbuf) / sizeof(*dstbuf), &s),
            5);
  ASSERT_EQ(wcscmp(dstbuf, L"hello"), 0);
  ASSERT_EQ(dstbuf[6], 0xcccc);
  ASSERT_EQ(src, nullptr);

  memset(srcbuf, 0xcc, sizeof(srcbuf));
  strcpy(srcbuf, "hello");
  wmemset(dstbuf, 0xcccc, sizeof(dstbuf) / sizeof(*dstbuf));
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  ASSERT_EQ(rs_mbsrtowcs(dstbuf, (const char **)&src, 4, &s), 4);
  ASSERT_EQ(wmemcmp(dstbuf, L"hell", 4), 0);
  ASSERT_EQ(dstbuf[5], 0xcccc);
  ASSERT_EQ(src, srcbuf + 4);

  memset(srcbuf, 0xcc, sizeof(srcbuf));
  strcpy(srcbuf, "hello");
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  ASSERT_EQ(rs_mbsrtowcs(nullptr, (const char **)&src, 0, &s), 5);

  memset(srcbuf, 0xcc, sizeof(srcbuf));
  strcpy(srcbuf, "hello");
  wmemset(dstbuf, 0xcccc, sizeof(dstbuf) / sizeof(*dstbuf));
  src = srcbuf;
  ASSERT_EQ(rs_mbsrtowcs(dstbuf, (const char **)&src,
                         sizeof(dstbuf) / sizeof(*dstbuf), nullptr),
            5);
  ASSERT_EQ(wcscmp(dstbuf, L"hello"), 0);
  ASSERT_EQ(dstbuf[6], 0xcccc);
  ASSERT_EQ(src, nullptr);

  memset(srcbuf, 0xcc, sizeof(srcbuf));
  strcpy(srcbuf, "hello");
  src = srcbuf;
  ASSERT_EQ(rs_mbsrtowcs(nullptr, (const char **)&src, 0, nullptr), 5);

  memset(srcbuf, 0xcc, sizeof(srcbuf));
  srcbuf[0] = '\0';
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  wmemset(dstbuf, 0xcccc, sizeof(dstbuf) / sizeof(*dstbuf));
  ASSERT_EQ(rs_mbsrtowcs(dstbuf, (const char **)&src, 1, &s), 0);
  ASSERT_EQ(dstbuf[0], 0);
  ASSERT_EQ(dstbuf[1], 0xcccc);
  ASSERT_EQ(src, nullptr);

  memset(srcbuf, 0xcc, sizeof(srcbuf));
  strcpy(srcbuf, "hello");
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  wmemset(dstbuf, 0xcccc, sizeof(dstbuf) / sizeof(*dstbuf));
  ASSERT_EQ(rs_mbsrtowcs(dstbuf, (const char **)&src, 0, &s), 0);
  ASSERT_EQ(dstbuf[0], 0xcccc);
  ASSERT_EQ(src, srcbuf);
}

TEST(mbsnrtowcs, posix) {
  ASSERT_STREQ("C", rs_setlocale(RS_LC_CTYPE, "C"));

  char srcbuf[128];
  wchar_t dstbuf[128];
  char *src;
  ouma_mbstate_t s;

  memset(srcbuf, 0xcc, sizeof(srcbuf));
  strcpy(srcbuf, "hello");
  wmemset(dstbuf, 0xcccc, sizeof(dstbuf) / sizeof(*dstbuf));
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  ASSERT_EQ(rs_mbsnrtowcs(dstbuf, (const char **)&src, 6,
                          sizeof(dstbuf) / sizeof(*dstbuf), &s),
            5);
  ASSERT_EQ(wcscmp(dstbuf, L"hello"), 0);
  ASSERT_EQ(dstbuf[6], 0xcccc);
  ASSERT_EQ(src, nullptr);

  memset(srcbuf, 0xcc, sizeof(srcbuf));
  strcpy(srcbuf, "hello");
  wmemset(dstbuf, 0xcccc, sizeof(dstbuf) / sizeof(*dstbuf));
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  ASSERT_EQ(rs_mbsnrtowcs(dstbuf, (const char **)&src, 4,
                          sizeof(dstbuf) / sizeof(*dstbuf), &s),
            4);
  ASSERT_EQ(wmemcmp(dstbuf, L"hell", 4), 0);
  ASSERT_EQ(dstbuf[5], 0xcccc);
  ASSERT_EQ(src, srcbuf + 4);

  memset(srcbuf, 0xcc, sizeof(srcbuf));
  strcpy(srcbuf, "hello");
  wmemset(dstbuf, 0xcccc, sizeof(dstbuf) / sizeof(*dstbuf));
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  ASSERT_EQ(rs_mbsnrtowcs(dstbuf, (const char **)&src, 6, 4, &s), 4);
  ASSERT_EQ(wmemcmp(dstbuf, L"hell", 4), 0);
  ASSERT_EQ(dstbuf[5], 0xcccc);
  ASSERT_EQ(src, srcbuf + 4);

  memset(srcbuf, 0xcc, sizeof(srcbuf));
  strcpy(srcbuf, "hello");
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  ASSERT_EQ(rs_mbsnrtowcs(nullptr, (const char **)&src, 6, 0, &s), 5);

  memset(srcbuf, 0xcc, sizeof(srcbuf));
  strcpy(srcbuf, "hello");
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  ASSERT_EQ(rs_mbsnrtowcs(nullptr, (const char **)&src, 4, 0, &s), 4);

  memset(srcbuf, 0xcc, sizeof(srcbuf));
  strcpy(srcbuf, "hello");
  wmemset(dstbuf, 0xcccc, sizeof(dstbuf) / sizeof(*dstbuf));
  src = srcbuf;
  ASSERT_EQ(rs_mbsnrtowcs(dstbuf, (const char **)&src, 6,
                          sizeof(dstbuf) / sizeof(*dstbuf), nullptr),
            5);
  ASSERT_EQ(wcscmp(dstbuf, L"hello"), 0);
  ASSERT_EQ(dstbuf[6], 0xcccc);
  ASSERT_EQ(src, nullptr);

  memset(srcbuf, 0xcc, sizeof(srcbuf));
  strcpy(srcbuf, "hello");
  src = srcbuf;
  ASSERT_EQ(rs_mbsnrtowcs(nullptr, (const char **)&src, 6, 0, nullptr), 5);

  memset(srcbuf, 0xcc, sizeof(srcbuf));
  srcbuf[0] = '\0';
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  wmemset(dstbuf, 0xcccc, sizeof(dstbuf) / sizeof(*dstbuf));
  ASSERT_EQ(rs_mbsnrtowcs(dstbuf, (const char **)&src, 1, 1, &s), 0);
  ASSERT_EQ(dstbuf[0], 0);
  ASSERT_EQ(dstbuf[1], 0xcccc);
  ASSERT_EQ(src, nullptr);

  memset(srcbuf, 0xcc, sizeof(srcbuf));
  strcpy(srcbuf, "hello");
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  wmemset(dstbuf, 0xcccc, sizeof(dstbuf) / sizeof(*dstbuf));
  ASSERT_EQ(rs_mbsnrtowcs(dstbuf, (const char **)&src, 1, 0, &s), 0);
  ASSERT_EQ(dstbuf[0], 0xcccc);
  ASSERT_EQ(src, srcbuf);

  memset(srcbuf, 0xcc, sizeof(srcbuf));
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  wmemset(dstbuf, 0xcccc, sizeof(dstbuf) / sizeof(*dstbuf));
  ASSERT_EQ(rs_mbsnrtowcs(dstbuf, (const char **)&src, 0, 1, &s), 0);
  ASSERT_EQ(dstbuf[0], 0xcccc);
  ASSERT_EQ(src, srcbuf);
}

TEST(wcrtomb, ascii) {
  ASSERT_STREQ("C", rs_setlocale(RS_LC_CTYPE, "C"));

  char c;
  ASSERT_EQ(1, rs_wcrtomb(&c, U'A', NULL));
  ASSERT_EQ('A', c);
  ASSERT_EQ(1, rs_wcrtomb(&c, U'\0', NULL));
  ASSERT_EQ('\0', c);
  ASSERT_EQ((size_t)-1, rs_wcrtomb(&c, U'€', NULL));
  ASSERT_EQ(EILSEQ, rs_errno);
  ASSERT_EQ((size_t)-1, rs_wcrtomb(&c, 0xd801, NULL));
  ASSERT_EQ(EILSEQ, rs_errno);
}

TEST(wcrtomb, unicode) {
  ASSERT_STREQ("C.UTF-8", rs_setlocale(RS_LC_CTYPE, "C.UTF-8"));

  char buf[MB_LEN_MAX];
  ASSERT_EQ(1, rs_wcrtomb(buf, U'A', NULL));
  ASSERT_EQ('A', buf[0]);
  ASSERT_EQ(1, rs_wcrtomb(buf, U'\0', NULL));
  ASSERT_EQ('\0', buf[0]);
  ASSERT_EQ(3, rs_wcrtomb(buf, U'€', NULL));
  ASSERT_THAT(buf, testing::StartsWith("€"));
  ASSERT_EQ((size_t)-1, rs_wcrtomb(buf, 0xd801, NULL));
  ASSERT_EQ(EILSEQ, rs_errno);
}

TEST(wcsrtombs, posix) {
  ASSERT_STREQ("C", rs_setlocale(RS_LC_CTYPE, "C"));

  wchar_t srcbuf[128];
  char dstbuf[128];
  wchar_t *src;
  ouma_mbstate_t s;

  wmemset(srcbuf, 0xcc, sizeof(srcbuf) / sizeof(*srcbuf));
  wcscpy(srcbuf, L"hello");
  memset(dstbuf, 0xcc, sizeof(dstbuf));
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  ASSERT_EQ(rs_wcsrtombs(dstbuf, (const wchar_t **)&src, sizeof(dstbuf), &s),
            5);
  ASSERT_EQ(strcmp(dstbuf, "hello"), 0);
  ASSERT_EQ((unsigned char)dstbuf[6], 0xcc);
  ASSERT_EQ(src, nullptr);

  wmemset(srcbuf, 0xcc, sizeof(srcbuf) / sizeof(*srcbuf));
  wcscpy(srcbuf, L"hello");
  memset(dstbuf, 0xcc, sizeof(dstbuf));
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  ASSERT_EQ(rs_wcsrtombs(dstbuf, (const wchar_t **)&src, 4, &s), 4);
  ASSERT_EQ(memcmp(dstbuf, "hell", 4), 0);
  ASSERT_EQ((unsigned char)dstbuf[5], 0xcc);
  ASSERT_EQ(src, srcbuf + 4);

  wmemset(srcbuf, 0xcc, sizeof(srcbuf) / sizeof(*srcbuf));
  wcscpy(srcbuf, L"hello");
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  ASSERT_EQ(rs_wcsrtombs(nullptr, (const wchar_t **)&src, sizeof(dstbuf), &s),
            5);

  wmemset(srcbuf, 0xcc, sizeof(srcbuf) / sizeof(*srcbuf));
  wcscpy(srcbuf, L"hello");
  memset(dstbuf, 0xcc, sizeof(dstbuf));
  src = srcbuf;
  ASSERT_EQ(
      rs_wcsrtombs(dstbuf, (const wchar_t **)&src, sizeof(dstbuf), nullptr), 5);
  ASSERT_EQ(strcmp(dstbuf, "hello"), 0);
  ASSERT_EQ((unsigned char)dstbuf[6], 0xcc);
  ASSERT_EQ(src, nullptr);

  wmemset(srcbuf, 0xcc, sizeof(srcbuf) / sizeof(*srcbuf));
  wcscpy(srcbuf, L"hello");
  src = srcbuf;
  ASSERT_EQ(rs_wcsrtombs(nullptr, (const wchar_t **)&src, 0, nullptr), 5);

  wmemset(srcbuf, 0xcc, sizeof(srcbuf) / sizeof(*srcbuf));
  srcbuf[0] = L'\0';
  memset(dstbuf, 0xcc, sizeof(dstbuf));
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  ASSERT_EQ(rs_wcsrtombs(dstbuf, (const wchar_t **)&src, sizeof(dstbuf), &s),
            0);
  ASSERT_EQ(dstbuf[0], L'\0');

  wmemset(srcbuf, 0xcc, sizeof(srcbuf) / sizeof(*srcbuf));
  wcscpy(srcbuf, L"hello");
  memset(dstbuf, 0xcc, sizeof(dstbuf));
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  ASSERT_EQ(rs_wcsrtombs(dstbuf, (const wchar_t **)&src, 0, &s), 0);
  ASSERT_EQ((unsigned char)dstbuf[0], 0xcc);
}

TEST(wcsnrtombs, posix) {
  ASSERT_STREQ("C", rs_setlocale(RS_LC_CTYPE, "C"));

  wchar_t srcbuf[128];
  char dstbuf[128];
  wchar_t *src;
  ouma_mbstate_t s;

  wmemset(srcbuf, 0xcc, sizeof(srcbuf) / sizeof(*srcbuf));
  wcscpy(srcbuf, L"hello");
  memset(dstbuf, 0xcc, sizeof(dstbuf));
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  ASSERT_EQ(
      rs_wcsnrtombs(dstbuf, (const wchar_t **)&src, 6, sizeof(dstbuf), &s), 5);
  ASSERT_EQ(strcmp(dstbuf, "hello"), 0);
  ASSERT_EQ((unsigned char)dstbuf[6], 0xcc);
  ASSERT_EQ(src, nullptr);

  /* Simple nullptr terminated string, stopping early. */
  wmemset(srcbuf, 0xcc, sizeof(srcbuf) / sizeof(*srcbuf));
  wcscpy(srcbuf, L"hello");
  memset(dstbuf, 0xcc, sizeof(dstbuf));
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  ASSERT_EQ(
      rs_wcsnrtombs(dstbuf, (const wchar_t **)&src, 4, sizeof(dstbuf), &s), 4);
  ASSERT_EQ(memcmp(dstbuf, "hell", 4), 0);
  ASSERT_EQ((unsigned char)dstbuf[5], 0xcc);
  ASSERT_EQ(src, srcbuf + 4);

  wmemset(srcbuf, 0xcc, sizeof(srcbuf) / sizeof(*srcbuf));
  wcscpy(srcbuf, L"hello");
  memset(dstbuf, 0xcc, sizeof(dstbuf));
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  ASSERT_EQ(rs_wcsnrtombs(dstbuf, (const wchar_t **)&src, 6, 4, &s), 4);
  ASSERT_EQ(memcmp(dstbuf, "hell", 4), 0);
  ASSERT_EQ((unsigned char)dstbuf[5], 0xcc);
  ASSERT_EQ(src, srcbuf + 4);

  wmemset(srcbuf, 0xcc, sizeof(srcbuf) / sizeof(*srcbuf));
  wcscpy(srcbuf, L"hello");
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  ASSERT_EQ(
      rs_wcsnrtombs(nullptr, (const wchar_t **)&src, 6, sizeof(dstbuf), &s), 5);

  wmemset(srcbuf, 0xcc, sizeof(srcbuf) / sizeof(*srcbuf));
  wcscpy(srcbuf, L"hello");
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  ASSERT_EQ(
      rs_wcsnrtombs(nullptr, (const wchar_t **)&src, 4, sizeof(dstbuf), &s), 4);

  wmemset(srcbuf, 0xcc, sizeof(srcbuf) / sizeof(*srcbuf));
  wcscpy(srcbuf, L"hello");
  memset(dstbuf, 0xcc, sizeof(dstbuf));
  src = srcbuf;
  ASSERT_EQ(
      rs_wcsnrtombs(dstbuf, (const wchar_t **)&src, 6, sizeof(dstbuf), nullptr),
      5);
  ASSERT_EQ(strcmp(dstbuf, "hello"), 0);
  ASSERT_EQ((unsigned char)dstbuf[6], 0xcc);
  ASSERT_EQ(src, nullptr);

  wmemset(srcbuf, 0xcc, sizeof(srcbuf) / sizeof(*srcbuf));
  wcscpy(srcbuf, L"hello");
  src = srcbuf;
  ASSERT_EQ(rs_wcsnrtombs(nullptr, (const wchar_t **)&src, 6, 0, nullptr), 5);

  wmemset(srcbuf, 0xcc, sizeof(srcbuf) / sizeof(*srcbuf));
  srcbuf[0] = L'\0';
  memset(dstbuf, 0xcc, sizeof(dstbuf));
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  ASSERT_EQ(
      rs_wcsnrtombs(dstbuf, (const wchar_t **)&src, 1, sizeof(dstbuf), &s), 0);
  ASSERT_EQ(dstbuf[0], L'\0');

  wmemset(srcbuf, 0xcc, sizeof(srcbuf) / sizeof(*srcbuf));
  wcscpy(srcbuf, L"hello");
  memset(dstbuf, 0xcc, sizeof(dstbuf));
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  ASSERT_EQ(rs_wcsnrtombs(dstbuf, (const wchar_t **)&src, 6, 0, &s), 0);
  ASSERT_EQ((unsigned char)dstbuf[0], 0xcc);

  wmemset(srcbuf, 0xcc, sizeof(srcbuf) / sizeof(*srcbuf));
  memset(dstbuf, 0xcc, sizeof(dstbuf));
  src = srcbuf;
  memset(&s, 0, sizeof(s));
  ASSERT_EQ(
      rs_wcsnrtombs(dstbuf, (const wchar_t **)&src, 0, sizeof(dstbuf), &s), 0);
  ASSERT_EQ((unsigned char)dstbuf[0], 0xcc);
  ASSERT_EQ(src, srcbuf);
}

TEST(wctob, simple) {
  ASSERT_STREQ("C", rs_setlocale(RS_LC_CTYPE, "C"));

  ASSERT_EQ(EOF, rs_wctob(WEOF));
  for (wint_t i = 0; i < 128; ++i) {
    SCOPED_TRACE(i);
    ASSERT_EQ(i, rs_wctob(i));
  }
  for (wint_t i = 128; i < 1000; ++i) {
    SCOPED_TRACE(i);
    ASSERT_EQ(EOF, rs_wctob(i));
  }
}

struct btowc_wctob_test {
  const char *locale;
  const char *illegal;
  const char *legal;
  const wchar_t wlegal[8];
  const wchar_t willegal[8];
} btowc_wctob_tests[] = {
    {"en_US.UTF-8",
     "\200",
     "ABC123@\t",
     {'A', 'B', 'C', '1', '2', '3', '@', '\t'},
     {0xfdd0, 0x10fffe, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0}},
    {NULL,
     NULL,
     NULL,
     {0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0},
     {0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0}},
};

static void h_btowc(struct btowc_wctob_test *t) {
  const char *cp;
  unsigned char c;
  char *str;
  const wchar_t *wcp;

  ASSERT_STREQ("en_US.UTF-8", rs_setlocale(RS_LC_CTYPE, t->locale));

  ASSERT_EQ(btowc(EOF), WEOF);
  ASSERT_EQ(wctob(WEOF), EOF);

  for (cp = t->illegal; *cp != '\0'; ++cp) {
    ASSERT_EQ(btowc(*cp), WEOF);
  }

  for (cp = t->legal; *cp != '\0'; ++cp) {
    c = (unsigned char)*cp;

    ASSERT_NE(btowc(c), WEOF);
    ASSERT_EQ(wctob(btowc(c)), c);
  }
}

static void h_iso10646(struct btowc_wctob_test *t) {
  const char *cp;
  int c, wc;
  char *str;
  const wchar_t *wcp;

  ASSERT_STREQ("en_US.UTF-8", rs_setlocale(RS_LC_CTYPE, t->locale));

  for (cp = t->legal, wcp = t->wlegal; *cp != '\0'; ++cp, ++wcp) {
    c = (int)(unsigned char)*cp;
    wc = btowc(c);

    ASSERT_NE(rs_errno, 0);
    ASSERT_EQ(btowc(c), *wcp);
  }

  for (wcp = t->willegal; *wcp != '\0'; ++wcp) {
    ASSERT_EQ(wctob(*wcp), EOF);
  }
}

TEST(btowc_wctob, unicode) {
  struct btowc_wctob_test *t;

  for (t = btowc_wctob_tests; t->locale != NULL; ++t) {
    h_btowc(t);
    h_iso10646(t);
  }
}

TEST(wcscasecmp, example) {
  ASSERT_STREQ("C", rs_setlocale(RS_LC_ALL, "C"));

  ASSERT_EQ(rs_wcscasecmp(nullptr, nullptr), 0);
  ASSERT_EQ(rs_wcscasecmp(L"", L""), 0);
  ASSERT_EQ(rs_wcscasecmp(L"abc", L"abc"), 0);
  ASSERT_EQ(rs_wcscasecmp(L"ABC", L"ABC"), 0);
  ASSERT_EQ(rs_wcscasecmp(L"abc", L"ABC"), 0);
  ASSERT_EQ(rs_wcscasecmp(L"ABC", L"abc"), 0);
  ASSERT_NE(rs_wcscasecmp(L"abc", L"xyz"), 0);
  ASSERT_NE(rs_wcscasecmp(L"ABC", L"xyz"), 0);
  ASSERT_NE(rs_wcscasecmp(L"abc", L"XYZ"), 0);
  ASSERT_NE(rs_wcscasecmp(L"ABC", L"XYZ"), 0);
  ASSERT_NE(rs_wcscasecmp(L"xyz", L"abc"), 0);
  ASSERT_NE(rs_wcscasecmp(L"XYZ", L"abc"), 0);
  ASSERT_NE(rs_wcscasecmp(L"xyz", L"ABC"), 0);
  ASSERT_NE(rs_wcscasecmp(L"XYZ", L"ABC"), 0);
  ASSERT_NE(rs_wcscasecmp(L"abc", L"ABCD"), 0);
  ASSERT_NE(rs_wcscasecmp(L"ABC", L"abcd"), 0);
  ASSERT_NE(rs_wcscasecmp(L"abcd", L"ABC"), 0);
  ASSERT_NE(rs_wcscasecmp(L"ABCD", L"abc"), 0);
}

TEST(wcscasecmp, unicode) {
    rs_errno = 0;

  ouma_locale_t loc = rs_newlocale(RS_LC_CTYPE_MASK, "en_US.UTF-8", 0);
  ASSERT_NE(nullptr, loc);
  ASSERT_NE(ENOENT, rs_errno);
  ASSERT_STREQ("en_US.UTF-8", rs_getlocalename_l(RS_LC_CTYPE, loc));

  ASSERT_EQ(rs_wcscasecmp_l(L"λ", L"Λ", loc), 0);
  ASSERT_NE(rs_wcscasecmp_l(L"λ", L"Ω", loc), 0);
  ASSERT_NE(rs_wcscasecmp_l(L"Ω", L"λ", loc), 0);

  rs_freelocale(loc);
}

TEST(wcsncasecmp, example) {
  ASSERT_STREQ("C", rs_setlocale(RS_LC_ALL, "C"));

  ASSERT_EQ(rs_wcsncasecmp(nullptr, nullptr, 0), 0);
  ASSERT_EQ(rs_wcsncasecmp(L"", L"", 50), 0);
  ASSERT_NE(rs_wcsncasecmp(L"abc", L"ABCD", 4), 0);
  ASSERT_NE(rs_wcsncasecmp(L"ABC", L"abcd", 4), 0);
  ASSERT_NE(rs_wcsncasecmp(L"abcd", L"ABC", 4), 0);
  ASSERT_NE(rs_wcsncasecmp(L"ABCD", L"abc", 4), 0);
  ASSERT_EQ(rs_wcsncasecmp(L"abc", L"ABCD", 3), 0);
  ASSERT_EQ(rs_wcsncasecmp(L"ABC", L"abcd", 3), 0);
}

TEST(wcsncasecmp, unicode) {
    rs_errno = 0;

  ouma_locale_t loc = rs_newlocale(RS_LC_CTYPE_MASK, "en_US.UTF-8", 0);
  ASSERT_NE(nullptr, loc);
  ASSERT_NE(ENOENT, rs_errno);
  ASSERT_STREQ("en_US.UTF-8", rs_getlocalename_l(RS_LC_CTYPE, loc));

  ASSERT_EQ(rs_wcsncasecmp_l(L"λ", L"Λ", 1, loc), 0);
  ASSERT_NE(rs_wcsncasecmp_l(L"λ", L"Ω", 1, loc), 0);
  ASSERT_NE(rs_wcsncasecmp_l(L"Ω", L"λ", 1, loc), 0);

  rs_freelocale(loc);
}

TEST(wcwidth, korean) {
  ASSERT_STREQ("ko_KR.UTF-8", rs_setlocale(RS_LC_CTYPE, "ko_KR.UTF-8"));

  EXPECT_EQ(2, rs_wcwidth(L'ㅜ'));
  EXPECT_EQ(2, rs_wcwidth(L'ㅋ'));
}

TEST(wcwidth, korean_jeongeul_syllables) {
  ASSERT_STREQ("ko_KR.UTF-8", rs_setlocale(RS_LC_CTYPE, "ko_KR.UTF-8"));

  EXPECT_EQ(2, rs_wcwidth(0xac00));
}

TEST(wcwidth, korean_jamo_jieut) {
  ASSERT_STREQ("ko_KR.UTF-8", rs_setlocale(RS_LC_CTYPE, "ko_KR.UTF-8"));

  EXPECT_EQ(2, rs_wcwidth(0x11bd));
}

TEST(wcwidth, emoji) {
  ASSERT_STREQ("ko_KR.UTF-8", rs_setlocale(RS_LC_CTYPE, "ko_KR.UTF-8"));

  EXPECT_EQ(2, rs_wcwidth(0x0001f60e));
  EXPECT_EQ(4, rs_wcswidth(L"👩🏿", 4));
}

TEST(wcswidth, simple) {
  ASSERT_STREQ("en_US.UTF-8", rs_setlocale(RS_LC_CTYPE, "en_US.UTF-8"));

  const wchar_t str[] = L"Iñtërnâtiônàlizætiøn";
  ASSERT_EQ(19, rs_wcswidth(str, std::size(str) - 2));
  ASSERT_EQ(20, rs_wcswidth(str, std::size(str) - 1));
  ASSERT_EQ(20, rs_wcswidth(str, std::size(str)));
  ASSERT_EQ(20, rs_wcswidth(str, std::size(str) + 1));
}

TEST(wcswidth, japanese) {
  ASSERT_STREQ("en_US.UTF-8", rs_setlocale(RS_LC_CTYPE, "en_US.UTF-8"));

  const wchar_t str[] = L"コンニチハ";
  ASSERT_EQ(8, rs_wcswidth(str, std::size(str) - 2));
  ASSERT_EQ(10, rs_wcswidth(str, std::size(str) - 1));
  ASSERT_EQ(10, rs_wcswidth(str, std::size(str)));
  ASSERT_EQ(10, rs_wcswidth(str, std::size(str) + 1));
}

TEST(wcswidth, thai) {
  ASSERT_STREQ("en_US.UTF-8", rs_setlocale(RS_LC_CTYPE, "en_US.UTF-8"));

  const wchar_t str[] = L"๏ แผ่นดินฮั่นเสื่อมโทรมแสนสังเวช";
  ASSERT_EQ(31, rs_wcswidth(str, std::size(str) - 2));
  ASSERT_EQ(32, rs_wcswidth(str, std::size(str) - 1));
  ASSERT_EQ(32, rs_wcswidth(str, std::size(str)));
  ASSERT_EQ(32, rs_wcswidth(str, std::size(str) + 1));
}

TEST(wcswidth, zalgo) {
  ASSERT_STREQ("en_US.UTF-8", rs_setlocale(RS_LC_CTYPE, "en_US.UTF-8"));

  const wchar_t str[] = L"T̫̺̳o̬̜ ì̬͎̲̟nv̖̗̻̣̹̕o͖̗̠̜̤k͍͚̹͖̼e̦̗̪͍̪͍ ̬ͅt̕h̠͙̮͕͓e̱̜̗͙̭ ̥͔̫͙̪͍̣͝ḥi̼̦͈̼v҉̩̟͚̞͎e͈̟̻͙̦̤-m̷̘̝̱í͚̞̦̳n̝̲̯̙̮͞d̴̺̦͕̫ ̗̭̘͎͖r̞͎̜̜͖͎̫͢ep͇r̝̯̝͖͉͎̺e̴s̥e̵̖̳͉͍̩̗n̢͓̪͕̜̰̠̦t̺̞̰i͟n҉̮̦̖̟g̮͍̱̻͍̜̳ ̳c̖̮̙̣̰̠̩h̷̗͍̖͙̭͇͈a̧͎̯̹̲̺̫ó̭̞̜̣̯͕s̶̤̮̩̘.̨̻̪̖͔";
  ASSERT_EQ(223, rs_wcswidth(str, std::size(str)));
}
