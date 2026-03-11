#include "common.h"

#include <wchar.h>

extern "C"
{
  wchar_t* rs_wmemchr(const wchar_t*, wchar_t, size_t);
  int rs_wmemcmp(const wchar_t*, const wchar_t*, size_t);
  wchar_t* rs_wmemcpy(wchar_t*, const wchar_t*, size_t);
  wchar_t* rs_wmemmove(wchar_t*, const wchar_t*, size_t);
  wchar_t* rs_wmemset(wchar_t*, wchar_t, size_t);
  wchar_t* rs_wcpcpy(wchar_t*, const wchar_t*);
  wchar_t* rs_wcpncpy(wchar_t*, const wchar_t*, size_t);
  wchar_t* rs_wcscat(wchar_t*, const wchar_t*);
  wchar_t* rs_wcschr(const wchar_t*, wchar_t);
  int rs_wcscmp(const wchar_t*, const wchar_t*);
  int rs_wcscoll(const wchar_t*, const wchar_t*);
  wchar_t* rs_wcscpy(wchar_t*, const wchar_t*);
  size_t rs_wcscspn(const wchar_t*, const wchar_t*);
  wchar_t* rs_wcsdup(const wchar_t*);
  size_t rs_wcslen(const wchar_t*);
  wchar_t* rs_wcsncat(wchar_t*, const wchar_t*, size_t);
  int rs_wcsncmp(const wchar_t*, const wchar_t*, size_t);
  wchar_t* rs_wcsncpy(wchar_t*, const wchar_t*, size_t);
  size_t rs_wcsnlen(const wchar_t*, size_t);
  wchar_t* rs_wcspbrk(const wchar_t*, const wchar_t*);
  wchar_t* rs_wcsrchr(const wchar_t*, wchar_t);
  size_t rs_wcsspn(const wchar_t*, const wchar_t*);
  wchar_t* rs_wcsstr(const wchar_t*, const wchar_t*);
  wchar_t* rs_wcstok(wchar_t*, const wchar_t*, wchar_t**);
  size_t rs_wcsxfrm(wchar_t*, const wchar_t*, size_t);
  wint_t rs_btowc(int);
  size_t rs_mbrlen(const char*, size_t, strogino_mbstate_t*);
  size_t rs_mbrtowc(wchar_t*, const char*, size_t, strogino_mbstate_t*);
  int rs_mbsinit(const strogino_mbstate_t*);
  size_t rs_mbsnrtowcs(wchar_t*,
                       const char**,
                       size_t,
                       size_t,
                       strogino_mbstate_t*);
  size_t rs_mbsrtowcs(wchar_t*, const char**, size_t, strogino_mbstate_t*);
  size_t rs_wcrtomb(char*, wchar_t, strogino_mbstate_t*);
  size_t rs_wcsnrtombs(char*,
                       const wchar_t**,
                       size_t,
                       size_t,
                       strogino_mbstate_t*);
  size_t rs_wcsrtombs(char*, const wchar_t**, size_t, strogino_mbstate_t*);
  int rs_wctob(wint_t);
  int rs_wcscasecmp(const wchar_t* ws1, const wchar_t* ws2);
  int rs_wcscasecmp_l(const wchar_t* ws1,
                      const wchar_t* ws2,
                      strogino_locale_t locale);
  int rs_wcsncasecmp(const wchar_t* ws1, const wchar_t* ws2, size_t n);
  int rs_wcsncasecmp_l(const wchar_t* ws1,
                       const wchar_t* ws2,
                       size_t n,
                       strogino_locale_t locale);
  int rs_wcwidth(wchar_t);
  int rs_wcswidth(const wchar_t*, size_t);
}

TEST(wmemchr, null)
{
  ASSERT_EQ(NULL, rs_wmemchr((wchar_t*)NULL, L'A', 0));
}

TEST(wmemchr, match)
{
  wchar_t buf[] = L"Foo bar baz";
  ASSERT_EQ(buf + 5, rs_wmemchr(buf, L'a', std::size(buf)));
}

TEST(wmemchr, nomatch)
{
  wchar_t buf[] = L"Foo bar baz";
  ASSERT_EQ(NULL, rs_wmemchr(buf, L'x', std::size(buf)));
}

TEST(wmemcmp, null)
{
  ASSERT_EQ(0, rs_wmemcmp(NULL, NULL, 0));
}

TEST(wmemcmp, example)
{
  const wchar_t buf1[] = L"Hello";
  const wchar_t buf2[] = L"Helxo";
  ASSERT_EQ(0, rs_wmemcmp(buf1, buf1, std::size(buf1)));
  ASSERT_GT(0, rs_wmemcmp(buf1, buf2, std::size(buf1)));
  ASSERT_LT(0, rs_wmemcmp(buf2, buf1, std::size(buf1)));
}

TEST(wmemcpy, null)
{
  ASSERT_EQ((wchar_t*)42, rs_wmemcpy((wchar_t*)42, (wchar_t*)123, 0));
}

TEST(wmemcpy, example)
{
  const wchar_t buf1[8] = L"Foo\0Bar";
  wchar_t buf2[8];
  ASSERT_EQ(buf2, rs_wmemcpy(buf2, buf1, std::size(buf1)));
  ASSERT_THAT(buf2, testing::ElementsAreArray(buf1));
}

TEST(wmemmove, null)
{
  ASSERT_EQ((wchar_t*)42, rs_wmemmove((wchar_t*)42, (wchar_t*)34, 0));
}

TEST(wmemmove, example1)
{
  wchar_t buf[] = L"abcdefghijkl";
  ASSERT_EQ(buf, rs_wmemmove(buf, buf + 4, 8));
  ASSERT_STREQ(L"efghijklijkl", buf);
}

TEST(wmemmove, example2)
{
  wchar_t buf[] = L"abcdefghijkl";
  ASSERT_EQ(buf + 4, rs_wmemmove(buf + 4, buf, 8));
  ASSERT_STREQ(L"abcdabcdefgh", buf);
}

TEST(wmemset, null)
{
  ASSERT_EQ((wchar_t*)5, rs_wmemset((wchar_t*)5, L'A', 0));
}

TEST(wmemset, example)
{
  wchar_t buf[11];
  ASSERT_EQ(buf, rs_wmemset(buf, L'!', 10));
  buf[10] = L'\0';
  ASSERT_STREQ(L"!!!!!!!!!!", buf);
}

TEST(wcpcpy, example)
{
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

TEST(wcpncpy, null)
{
  ASSERT_EQ((wchar_t*)12, rs_wcpncpy((wchar_t*)12, (wchar_t*)500, 0));
}

TEST(wcpncpy, example1)
{
  wchar_t buf[] = L"AAAAAAAAAAAA";
  ASSERT_EQ(buf + 5, rs_wcpncpy(buf, L"Hello", 12));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"Hello\0\0\0\0\0\0\0"));
}

TEST(wcpncpy, example2)
{
  wchar_t buf[13];
  ASSERT_EQ(buf + 12, rs_wcpncpy(buf, L"This is a very long string", 12));
  buf[12] = '\0';
  ASSERT_THAT(buf, testing::ElementsAreArray(L"This is a ve"));
}

TEST(wcscat, example)
{
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

TEST(wcscpy, example)
{
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

TEST(wcscspn, example)
{
  const wchar_t* wcs = L"Hello, world";
  ASSERT_EQ(0, rs_wcscspn(wcs, L"H"));
  ASSERT_EQ(7, rs_wcscspn(wcs, L"rdw"));
  ASSERT_EQ(12, rs_wcscspn(wcs, L"XYZ"));
}

TEST(wcslen, example)
{
  ASSERT_EQ(22, rs_wcslen(L"ℕ ⊆ ℕ₀ ⊂ ℤ ⊂ ℚ ⊂ ℝ ⊂ ℂ"));
}

TEST(wcsncat, example)
{
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

TEST(wcsncmp, null)
{
  ASSERT_EQ(0, rs_wcsncmp(NULL, NULL, 0));
}

TEST(wcsncmp, examples)
{
  ASSERT_EQ(0, rs_wcsncmp(L"", L"", 100));
  ASSERT_EQ(0, rs_wcsncmp(L"Hello", L"Hello", 100));

  ASSERT_EQ(0, rs_wcsncmp(L"Hello", L"Hello, world", 5));
  ASSERT_GT(0, rs_wcsncmp(L"Hello", L"Hello, world", 6));
  ASSERT_LT(0, rs_wcsncmp(L"Hello, world", L"Hello", 100));

  ASSERT_EQ(0, rs_wcsncmp(L"Hello!", L"Hello.", 5));
  ASSERT_GT(0, rs_wcsncmp(L"Hello!", L"Hello.", 6));
  ASSERT_LT(0, rs_wcsncmp(L"Hello.", L"Hello!", 100));
}

TEST(wcsncpy, null)
{
  ASSERT_EQ((wchar_t*)12, rs_wcsncpy((wchar_t*)12, (wchar_t*)500, 0));
}

TEST(wcsncpy, example1)
{
  wchar_t buf[] = L"AAAAAAAAAAAA";
  ASSERT_EQ(buf, rs_wcsncpy(buf, L"Hello", 12));
  ASSERT_THAT(buf, testing::ElementsAreArray(L"Hello\0\0\0\0\0\0\0"));
}

TEST(wcsncpy, example2)
{
  wchar_t buf[13];
  ASSERT_EQ(buf, rs_wcsncpy(buf, L"This is a very long string", 12));
  buf[12] = L'\0';
  ASSERT_THAT(buf, testing::ElementsAreArray(L"This is a ve"));
}

TEST(wcsnlen, null)
{
  ASSERT_EQ(0, rs_wcsnlen(NULL, 0));
  ASSERT_EQ(0, rs_wcsnlen(L"", 100));
  ASSERT_EQ(7, rs_wcsnlen(L"Σὲ γνωρίζω ἀπὸ τὴν κόψη", 7));
}

TEST(wcspbrk, example)
{
  const wchar_t* wcs = L"Hello, world";
  ASSERT_EQ(wcs, rs_wcspbrk(wcs, L"H"));
  ASSERT_EQ(wcs + 7, rs_wcspbrk(wcs, L"rdw"));
  ASSERT_EQ(NULL, rs_wcspbrk(wcs, L"XYZ"));
}

TEST(wcsspn, example)
{
  const wchar_t* wcs = L"Hello, world";
  ASSERT_EQ(0, rs_wcsspn(wcs, L""));
  ASSERT_EQ(0, rs_wcsspn(wcs, L"Foo"));
  ASSERT_EQ(5, rs_wcsspn(wcs, L"olHe"));
  ASSERT_EQ(12, rs_wcsspn(wcs, L"Helo, wrld"));
}

TEST(wcsstr, examples)
{
  const wchar_t* str = (const wchar_t*)0x42;
  ASSERT_EQ(str, rs_wcsstr(str, L""));

  str = L"Hello world";
  ASSERT_EQ(str + 2, rs_wcsstr(str, L"ll"));
  ASSERT_EQ(str + 4, rs_wcsstr(str, L"o worl"));
  ASSERT_EQ(str + 6, rs_wcsstr(str, L"world"));
  ASSERT_EQ(str + 10, rs_wcsstr(str, L"d"));
  ASSERT_EQ(NULL, rs_wcsstr(str, L"word"));
  ASSERT_EQ(NULL, rs_wcsstr(str, L"world!"));
}

TEST(wcstok, example)
{
  wchar_t line[] = L"LINE  TO BE\t\tSEPARATED\n";
  const wchar_t* split = L" \t\n";
  wchar_t* lastws;
  ASSERT_STREQ(L"LINE", rs_wcstok(line, split, &lastws));
  ASSERT_STREQ(L"TO", rs_wcstok(NULL, split, &lastws));
  ASSERT_STREQ(L"BE", rs_wcstok(NULL, split, &lastws));
  ASSERT_STREQ(L"SEPARATED", rs_wcstok(NULL, split, &lastws));
  ASSERT_EQ(NULL, rs_wcstok(NULL, split, &lastws));
}

struct coll_data
{
  const wchar_t* a;
  const wchar_t* b;
  int result;
};

static int
sign(int a)
{
  if (a < 0)
    return -1;
  if (a > 0)
    return 1;
  return 0;
}

static void
test_wcscoll(const coll_data* coll)
{
  for (unsigned int i = 0; coll[i].a != NULL; ++i) {
    int result = sign(rs_wcscoll(coll[i].a, coll[i].b));
    ASSERT_EQ(result, coll[i].result);
  }
}

static void
test_wcsxfrm(const coll_data* coll)
{
  for (unsigned int i = 0; coll[i].a != NULL; ++i) {
    int result = 0;
    wchar_t sortKeyA[100], sortKeyB[100];
    rs_wcsxfrm(sortKeyA, coll[i].a, 100);
    rs_wcsxfrm(sortKeyB, coll[i].b, 100);
    result = sign(rs_wcscmp(sortKeyA, sortKeyB));
    ASSERT_EQ(result, coll[i].result);
  }
}

TEST(wcscoll, posix)
{
  rs_setlocale(LC_COLLATE, "C");

  const coll_data coll[] = {
    { L"", L"", 0 },
    { L"test", L"test", 0 },
    { L"tester", L"tester", 0 },
    { L"côté", L"côté", 0 },
    { NULL, NULL, 0 },
  };

  test_wcscoll(coll);
  test_wcsxfrm(coll);
}

TEST(wcscoll, uca)
{
  rs_setlocale(LC_COLLATE, "en_US");

  const coll_data coll[] = {
    { L"", L"", 0 },
    { L"test", L"test", 0 },
    { L"tester", L"test", 1 },
    { L"tEst", L"test", 1 },
    { L"test", L"tester", -1 },
    { L"täst", L"täst", 0 },
    { L"tast", L"täst", -1 },
    { L"tbst", L"täst", 1 },
    { L"tbst", L"tæst", 1 },
    { L"täst", L"tÄst", -1 },
    { L"tBst", L"tÄst", 1 },
    { L"tBst", L"täst", 1 },
    { L"taest", L"tæst", -1 },
    { L"tafst", L"tæst", 1 },
    { L"taa", L"täa", -1 },
    { L"tab", L"täb", -1 },
    { L"tad", L"täd", -1 },
    { L"tae", L"täe", -1 },
    { L"taf", L"täf", -1 },
    { L"cote", L"coté", -1 },
    { L"coté", L"côte", -1 },
    { L"côte", L"côté", -1 },
    { NULL, NULL, 0 },
  };

  test_wcscoll(coll);
  test_wcsxfrm(coll);
}

#if 0
TEST(btowc, simple) {
  rs_setlocale(LC_CTYPE, "C");

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
  rs_setlocale(LC_CTYPE, "C.UTF-8");

  char euro[] = "€";
  strogino_mbstate_t mbs{};
  ASSERT_EQ((size_t)-2, rs_mbrlen(&euro[0], 1, &mbs));
  ASSERT_EQ((size_t)-2, rs_mbrlen(&euro[1], 1, &mbs));
  ASSERT_EQ(1, rs_mbrlen(&euro[2], 1, &mbs));
  ASSERT_EQ(0, rs_mbrlen(&euro[3], 1, &mbs));
}

TEST(mbrtowc, ascii) {
  rs_setlocale(LC_CTYPE, "C");

  strogino_mbstate_t mbs{};
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
  rs_setlocale(LC_ALL, "C.UTF-8");

  strogino_mbstate_t mbs{};
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
  strogino_mbstate_t initial_mbstate{};
  ASSERT_NE(0, rs_mbsinit(&initial_mbstate));
}

#define num_bytes 128
#define NUM_WCHARS(num_bytes) ((num_bytes) / sizeof(wchar_t))

static void test_mbsrtowcs(strogino_mbstate_t *ps) {
  constexpr const char *VALID = "A"
                                "\xc2\xa2"
                                "\xe2\x82\xac"
                                "\xf0\xa4\xad\xa2"
                                "ef";
  constexpr const char *INVALID = "A"
                                  "\xc2\x20"
                                  "ef";
  constexpr const char *INCOMPLETE = "A"
                                     "\xc2";
  wchar_t out[4];
  const char *valid = VALID;
  ASSERT_EQ(4U, rs_mbsrtowcs(out, &valid, 4, ps));
  ASSERT_EQ(L'A', out[0]);
  ASSERT_EQ(static_cast<wchar_t>(0x00a2), out[1]);
  ASSERT_EQ(static_cast<wchar_t>(0x20ac), out[2]);
  ASSERT_EQ(static_cast<wchar_t>(0x24b62), out[3]);
  ASSERT_EQ('e', *valid);
  wmemset(out, L'x', NUM_WCHARS(sizeof(out)));
  ASSERT_EQ(2U, rs_mbsrtowcs(out, &valid, 4, ps));
  ASSERT_EQ(L'e', out[0]);
  ASSERT_EQ(L'f', out[1]);
  ASSERT_EQ(L'\0', out[2]);
  ASSERT_EQ(L'x', out[3]);
  ASSERT_EQ(nullptr, valid);
  const char *invalid = INVALID;
  ASSERT_EQ(static_cast<size_t>(-1), rs_mbsrtowcs(out, &invalid, 4, ps));
  EXPECT_EQ(EILSEQ, rs_errno);
  ASSERT_EQ('\xc2', *invalid);
  const char *incomplete = INCOMPLETE;
  ASSERT_EQ(static_cast<size_t>(-1), rs_mbsrtowcs(out, &incomplete, 2, ps));
  EXPECT_EQ(EILSEQ, rs_errno);
  ASSERT_EQ('\xc2', *incomplete);
  const char *mbs = VALID;
  EXPECT_EQ(6U, rs_mbsrtowcs(nullptr, &mbs, 0, ps));
  EXPECT_EQ(VALID, mbs);
  mbs = INVALID;
  EXPECT_EQ(static_cast<size_t>(-1), rs_mbsrtowcs(nullptr, &mbs, 0, ps));
  EXPECT_EQ(INVALID, mbs);
  mbs = INCOMPLETE;
  EXPECT_EQ(static_cast<size_t>(-1), rs_mbsrtowcs(nullptr, &mbs, 0, ps));
  EXPECT_EQ(INCOMPLETE, mbs);
}

TEST(mbsrtowcs, example) {
  rs_setlocale(LC_CTYPE, "C.UTF-8");

  strogino_mbstate_t ps;
  memset(&ps, 0, sizeof(ps));
  test_mbsrtowcs(&ps);
  test_mbsrtowcs(nullptr);
  const char *invalid = "\x20";
  wchar_t out;
  ASSERT_EQ(static_cast<size_t>(-2), rs_mbrtowc(&out, "\xc2", 1, &ps));
  ASSERT_EQ(static_cast<size_t>(-1), rs_mbsrtowcs(&out, &invalid, 1, &ps));
  EXPECT_EQ(EILSEQ, rs_errno);
  ASSERT_EQ('\x20', *invalid);
}

TEST(wcrtomb, ascii) {
  rs_setlocale(LC_CTYPE, "C");

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
  rs_setlocale(LC_CTYPE, "C.UTF-8");

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

TEST(wcsrtombs, ascii) {
  rs_setlocale(LC_CTYPE, "C");

  const wchar_t *src = L"Hello, world";
  char dst[13];
  strogino_mbstate_t mbs{};
  ASSERT_EQ(sizeof(dst) - 1, rs_wcsrtombs(dst, &src, sizeof(dst), &mbs));
  ASSERT_EQ(NULL, src);
  ASSERT_STREQ("Hello, world", dst);
}

TEST(wcsrtombs, unicode) {
  rs_setlocale(LC_ALL, "C.UTF-8");

  const wchar_t chars[] = {L'h', L'e', L'l', L'l', L'o', 0};
  const wchar_t *src = L"ℕ ⊆ ℕ₀ ⊂ ℤ ⊂ ℚ ⊂ ℝ ⊂ ℂ";
  char dst[47];
  strogino_mbstate_t mbs{};
  ASSERT_EQ(sizeof(dst) - 1, rs_wcsrtombs(dst, &src, sizeof(dst), &mbs));
  ASSERT_EQ(NULL, src);
  ASSERT_STREQ("ℕ ⊆ ℕ₀ ⊂ ℤ ⊂ ℚ ⊂ ℝ ⊂ ℂ", dst);

  src = chars;
  ASSERT_EQ(5U, rs_wcsrtombs(nullptr, &src, 0, nullptr));
  ASSERT_EQ(&chars[0], src);
  src = chars;
  ASSERT_EQ(5U, rs_wcsrtombs(nullptr, &src, 4, nullptr));
  ASSERT_EQ(&chars[0], src);
  src = chars;
  ASSERT_EQ(5U, rs_wcsrtombs(nullptr, &src, 256, nullptr));
  ASSERT_EQ(&chars[0], src);
}

TEST(wctob, simple) {
  rs_setlocale(LC_CTYPE, "C");

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
#endif

TEST(wcscasecmp, example)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C"), nullptr);

  ASSERT_EQ(0, rs_wcscasecmp(L"hello", L"hello"));
  ASSERT_EQ(0, rs_wcscasecmp(L"hElLo", L"hello"));

  ASSERT_GT(0, rs_wcscasecmp(L"doge", L"dogS"));
  ASSERT_LT(0, rs_wcscasecmp(L"dogs", L"dogE"));
}

TEST(wcscasecmp, unicode)
{
  strogino_locale_t loc = rs_newlocale(LC_CTYPE_MASK, "nl_BE.UTF-8", 0);
  ASSERT_NE(nullptr, loc);
  ASSERT_NE(ENOENT, rs_errno);
  ASSERT_STREQ("nl_BE.UTF-8", rs_getlocalename_l(LC_CTYPE, loc));

  ASSERT_LT(0, rs_wcscasecmp_l(L"München?", L"MÜNCHEN!", loc));
  ASSERT_EQ(0, rs_wcscasecmp_l(L"München", L"MÜNCHEN", loc));

  rs_freelocale(loc);
}

TEST(wcsncasecmp, null)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C"), nullptr);

  ASSERT_EQ(0, rs_wcsncasecmp(NULL, NULL, 0));
}

TEST(wcsncasecmp, example)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C"), nullptr);

  ASSERT_EQ(0, rs_wcsncasecmp(L"hello", L"hello", 100));
  ASSERT_EQ(0, rs_wcsncasecmp(L"hElLo", L"hello", 100));

  ASSERT_EQ(0, rs_wcsncasecmp(L"doge", L"dogS", 3));
  ASSERT_GT(0, rs_wcsncasecmp(L"doge", L"dogS", 4));
  ASSERT_EQ(0, rs_wcsncasecmp(L"dogs", L"dogE", 3));
  ASSERT_LT(0, rs_wcsncasecmp(L"dogs", L"dogE", 4));
}

TEST(wcsncasecmp, unicode)
{
  strogino_locale_t loc = rs_newlocale(LC_CTYPE_MASK, "nl_BE.UTF-8", 0);
  ASSERT_NE(nullptr, loc);
  ASSERT_NE(ENOENT, rs_errno);
  ASSERT_STREQ("nl_BE.UTF-8", rs_getlocalename_l(LC_CTYPE, loc));

  ASSERT_EQ(0, rs_wcsncasecmp_l(L"München?", L"MÜNCHEN!", 7, loc));
  ASSERT_LT(0, rs_wcsncasecmp_l(L"München?", L"MÜNCHEN!", 8, loc));

  rs_freelocale(loc);
}

TEST(wcwidth, korean)
{
  ASSERT_NE(rs_setlocale(LC_CTYPE, "ko_KR.UTF-8"), nullptr);

  EXPECT_EQ(2, rs_wcwidth(L'ㅜ'));
  EXPECT_EQ(2, rs_wcwidth(L'ㅋ'));
}

TEST(wcwidth, korean_jeongeul_syllables)
{
  ASSERT_NE(rs_setlocale(LC_CTYPE, "ko_KR.UTF-8"), nullptr);

  EXPECT_EQ(2, rs_wcwidth(0xac00));
  EXPECT_EQ(2, rs_wcwidth(0xd7a3));
}

TEST(wcswidth, simple)
{
  ASSERT_NE(rs_setlocale(LC_CTYPE, "en_US.UTF-8"), nullptr);

  const wchar_t str[] = L"Iñtërnâtiônàlizætiøn";
  ASSERT_EQ(19, rs_wcswidth(str, std::size(str) - 2));
  ASSERT_EQ(20, rs_wcswidth(str, std::size(str) - 1));
  ASSERT_EQ(20, rs_wcswidth(str, std::size(str)));
  ASSERT_EQ(20, rs_wcswidth(str, std::size(str) + 1));
}

TEST(wcswidth, japanese)
{
  ASSERT_NE(rs_setlocale(LC_CTYPE, "en_US.UTF-8"), nullptr);

  const wchar_t str[] = L"コンニチハ";
  ASSERT_EQ(8, rs_wcswidth(str, std::size(str) - 2));
  ASSERT_EQ(10, rs_wcswidth(str, std::size(str) - 1));
  ASSERT_EQ(10, rs_wcswidth(str, std::size(str)));
  ASSERT_EQ(10, rs_wcswidth(str, std::size(str) + 1));
}

TEST(wcswidth, thai)
{
  ASSERT_NE(rs_setlocale(LC_CTYPE, "en_US.UTF-8"), nullptr);

  const wchar_t str[] = L"๏ แผ่นดินฮั่นเสื่อมโทรมแสนสังเวช";
  ASSERT_EQ(24, rs_wcswidth(str, std::size(str) - 2));
  ASSERT_EQ(25, rs_wcswidth(str, std::size(str) - 1));
  ASSERT_EQ(25, rs_wcswidth(str, std::size(str)));
  ASSERT_EQ(25, rs_wcswidth(str, std::size(str) + 1));
}

TEST(wcswidth, zalgo)
{
  ASSERT_NE(rs_setlocale(LC_CTYPE, "en_US.UTF-8"), nullptr);

  const wchar_t str[] = L"T̫̺̳o̬̜ ì̬͎̲̟nv̖̗̻̣̹̕o͖̗̠̜̤k͍͚̹͖̼e̦̗̪͍̪͍ ̬ͅt̕h̠͙̮͕͓e̱̜̗͙̭ ̥͔̫͙̪͍̣͝ḥi̼̦͈̼v҉̩̟͚̞͎e͈̟̻͙̦̤-m̷̘̝̱í͚̞̦̳n̝̲̯̙̮͞d̴̺̦͕̫ ̗̭̘͎͖r̞͎̜̜͖͎̫͢ep͇r̝̯̝͖͉͎̺e̴s̥e̵̖̳͉͍̩̗n̢͓̪͕̜̰̠̦t̺̞̰i͟n҉̮̦̖̟g̮͍̱̻͍̜̳ ̳c̖̮̙̣̰̠̩h̷̗͍̖͙̭͇͈a̧͎̯̹̲̺̫ó̭̞̜̣̯͕s̶̤̮̩̘.̨̻̪̖͔";
  ASSERT_EQ(43, rs_wcswidth(str, std::size(str)));
}
