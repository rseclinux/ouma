#include "common.h"

#include <wchar.h>

extern "C" {
  wchar_t *rs_wmemchr(const wchar_t *, wchar_t, size_t);
  int rs_wmemcmp(const wchar_t *, const wchar_t *, size_t);
  wchar_t *rs_wmemcpy(wchar_t *, const wchar_t *, size_t);
  wchar_t *rs_wmemmove(wchar_t *, const wchar_t *, size_t);
  wchar_t *rs_wmemset(wchar_t *, wchar_t, size_t);
  wchar_t *rs_wcpcpy(wchar_t *, const wchar_t *);
  wchar_t *rs_wcpncpy(wchar_t *, const wchar_t *, size_t);
  //int rs_wcscasecmp(const wchar_t *, const wchar_t *);
  //int rs_wcscasecmp_l(const wchar_t *, const wchar_t *, locale_t);
  wchar_t *rs_wcscat(wchar_t *, const wchar_t *);
  wchar_t *rs_wcschr(const wchar_t *, wchar_t);
  int rs_wcscmp(const wchar_t *, const wchar_t *);
  int rs_wcscoll(const wchar_t *, const wchar_t *);
  int rs_wcscoll_l(const wchar_t *, const wchar_t *, locale_t);
  wchar_t *rs_wcscpy(wchar_t *, const wchar_t *);
  size_t rs_wcscspn(const wchar_t *, const wchar_t *);
  wchar_t *rs_wcsdup(const wchar_t *);
  size_t rs_wcslen(const wchar_t *);
  //int rs_wcsncasecmp(const wchar_t *, const wchar_t *, size_t);
  //int rs_wcsncasecmp_l(const wchar_t *, const wchar_t *, size_t, locale_t);
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
  size_t rs_wcsxfrm_l(wchar_t *, const wchar_t *, size_t, locale_t);
}

TEST(wmemchr, null) {
  ASSERT_EQ(NULL, rs_wmemchr((wchar_t *)NULL, L'A', 0));
}

TEST(wmemchr, match) {
  wchar_t buf[] = L"Foo bar baz";
  ASSERT_EQ(buf + 5, rs_wmemchr(buf, L'a', std::size(buf)));
}

TEST(wmemchr, nomatch) {
  wchar_t buf[] = L"Foo bar baz";
  ASSERT_EQ(NULL, rs_wmemchr(buf, L'x', std::size(buf)));
}

TEST(wmemcmp, null) {
  ASSERT_EQ(0, rs_wmemcmp(NULL, NULL, 0));
}

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

TEST(wcslen, example) {
  ASSERT_EQ(22, rs_wcslen(L"ℕ ⊆ ℕ₀ ⊂ ℤ ⊂ ℚ ⊂ ℝ ⊂ ℂ"));
}

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

TEST(wcsncmp, null) {
  ASSERT_EQ(0, rs_wcsncmp(NULL, NULL, 0));
}

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
