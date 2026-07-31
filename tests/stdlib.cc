#include "common.h"
#include "common_float.h"
#include "common_locale.h"

extern "C" {
double rs_atof(const char *);
int rs_atoi(const char *);
long int rs_atol(const char *);
long long int rs_atoll(const char *);
double rs_strtod(const char *__restrict__, char **__restrict__);
float rs_strtof(const char *__restrict__, char **__restrict__);
long double rs_strtold(const char *__restrict__, char **__restrict__);
long int rs_strtol(const char *__restrict__, char **__restrict__, int);
long long int rs_strtoll(const char *__restrict__, char **__restrict__, int);
unsigned long int rs_strtoul(const char *__restrict__, char **__restrict__,
                             int);
unsigned long long int rs_strtoull(const char *__restrict__,
                                   char **__restrict__, int);
int rs_mblen(const char *, size_t);
int rs_mbtowc(wchar_t *__restrict__, const char *__restrict__, size_t);
int rs_wctomb(char *, wchar_t wc);
size_t rs_mbstowcs(wchar_t *__restrict__s, const char *__restrict__, size_t);
size_t rs_wcstombs(char *__restrict__, const wchar_t *__restrict__s, size_t);
}

TEST(atoi, examples) {
  ASSERT_EQ(12, rs_atoi("  12"));
  ASSERT_EQ(-3, rs_atoi("-03"));
  ASSERT_EQ(0, rs_atoi("0x5"));
}

TEST(atol, examples) {
  ASSERT_EQ(123456, rs_atol("  123456"));
  ASSERT_EQ(-8860, rs_atol("-08860"));
}

TEST(atoll, examples) { ASSERT_EQ(5050505, rs_atoll(" 5050505 ")); }

TEST(strtol, positive) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str;
  char *endptr;

  str = "0";
  ASSERT_EQ(0, rs_strtol(str, NULL, 0));
  ASSERT_EQ(0, rs_errno);

  str = "1";
  ASSERT_EQ(1, rs_strtol(str, NULL, 0));
  ASSERT_EQ(0, rs_errno);

  str = "0x7ffffffffffffffe";
  errno = 0;
  ASSERT_EQ(LONG_MAX - 1, rs_strtol(str, &endptr, 0));
  ASSERT_EQ(str + 18, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "0x7fffffffffffffff";
  ASSERT_EQ(LONG_MAX, rs_strtol(str, &endptr, 0));
  ASSERT_EQ(str + 18, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "0x8000000000000000";
  ASSERT_EQ(LONG_MAX, rs_strtol(str, &endptr, 0));
  ASSERT_EQ(str + 18, endptr);
  ASSERT_EQ(ERANGE, rs_errno);
}

TEST(strtol, negative) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str;
  char *endptr;

  str = "-0";
  ASSERT_EQ(0, rs_strtol(str, NULL, 0));
  ASSERT_EQ(0, rs_errno);

  str = "-1";
  ASSERT_EQ(-1, rs_strtol(str, NULL, 0));
  ASSERT_EQ(0, rs_errno);

  str = "-0x7fffffffffffffff";
  errno = 0;
  ASSERT_EQ(LONG_MIN + 1, rs_strtol(str, &endptr, 0));
  ASSERT_EQ(str + 19, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "-0x8000000000000000";
  ASSERT_EQ(LONG_MIN, rs_strtol(str, &endptr, 0));
  ASSERT_EQ(str + 19, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "-0x8000000000000001";
  ASSERT_EQ(LONG_MIN, rs_strtol(str, &endptr, 0));
  ASSERT_EQ(str + 19, endptr);
  ASSERT_EQ(ERANGE, rs_errno);
}

TEST(strtoll, positive) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str;
  char *endptr;

  str = "0";
  ASSERT_EQ(0, rs_strtoll(str, NULL, 0));
  ASSERT_EQ(0, rs_errno);

  str = "1";
  ASSERT_EQ(1, rs_strtoll(str, NULL, 0));
  ASSERT_EQ(0, rs_errno);

  str = "0x7ffffffffffffffe";
  ASSERT_EQ(LLONG_MAX - 1, rs_strtoll(str, &endptr, 0));
  ASSERT_EQ(str + 18, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "0x7fffffffffffffff";
  ASSERT_EQ(LLONG_MAX, rs_strtoll(str, &endptr, 0));
  ASSERT_EQ(str + 18, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "0x8000000000000000";
  ASSERT_EQ(LLONG_MAX, rs_strtoll(str, &endptr, 0));
  ASSERT_EQ(str + 18, endptr);
  ASSERT_EQ(ERANGE, rs_errno);
}

TEST(strtoll, negative) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str;
  char *endptr;

  str = "-0";
  ASSERT_EQ(0, rs_strtoll(str, NULL, 0));
  ASSERT_EQ(0, rs_errno);

  str = "-1";
  ASSERT_EQ(-1, rs_strtoll(str, NULL, 0));
  ASSERT_EQ(0, rs_errno);

  str = "-0x7fffffffffffffff";
  ASSERT_EQ(LLONG_MIN + 1, rs_strtoll(str, &endptr, 0));
  ASSERT_EQ(str + 19, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "-0x8000000000000000";
  ASSERT_EQ(LLONG_MIN, rs_strtoll(str, &endptr, 0));
  ASSERT_EQ(str + 19, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "-0x8000000000000001";
  ASSERT_EQ(LLONG_MIN, rs_strtoll(str, &endptr, 0));
  ASSERT_EQ(str + 19, endptr);
  ASSERT_EQ(ERANGE, rs_errno);
}

TEST(strtoul, examples) {
  rs_setlocale(RS_LC_ALL, "C");

  const char *str = "  57";
  char *endptr;
  rs_errno = 0;
  ASSERT_EQ(57, rs_strtoul(str, NULL, 10));
  ASSERT_EQ(0, rs_errno);

  str = "          ";
  ASSERT_EQ(0, rs_strtoul(str, &endptr, 10));
  ASSERT_EQ(str, endptr);
  ASSERT_EQ(EINVAL, rs_errno);

  str = "  01234hello";
  rs_errno = 0;
  ASSERT_EQ(1234, rs_strtoul(str, &endptr, 10));
  ASSERT_EQ(str + 7, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "  01234hello";
  ASSERT_EQ(194, rs_strtoul(str, &endptr, 5));
  ASSERT_EQ(str + 7, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "  01234hello";
  ASSERT_EQ(01234, rs_strtoul(str, &endptr, 0));
  ASSERT_EQ(str + 7, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "Hello!";
  ASSERT_EQ(29234652, rs_strtoul(str, &endptr, 36));
  ASSERT_EQ(str + 5, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "\n-42boom";
  ASSERT_EQ((unsigned long)-26, rs_strtoul(str, &endptr, 6));
  ASSERT_EQ(str + 4, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "\t-000000";
  rs_errno = 0;
  ASSERT_EQ(0, rs_strtoul(str, &endptr, 6));
  ASSERT_EQ(str + 8, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "0x123";
  ASSERT_EQ(0x123, rs_strtoul(str, &endptr, 0));
  ASSERT_EQ(str + 5, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "456";
  ASSERT_EQ(0x456, rs_strtoul(str, &endptr, 16));
  ASSERT_EQ(str + 3, endptr);
  ASSERT_EQ(0, rs_errno);
}

TEST(strtoull, positive) {
  rs_setlocale(RS_LC_ALL, "C");

  const char *str;
  char *endptr;

  rs_errno = 0;
  str = "0xfffffffffffffffe";
  ASSERT_EQ(ULLONG_MAX - 1, rs_strtoull(str, &endptr, 0));
  ASSERT_EQ(str + 18, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "0xffffffffffffffff";
  ASSERT_EQ(ULLONG_MAX, rs_strtoull(str, &endptr, 0));
  ASSERT_EQ(str + 18, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "0x10000000000000000";
  ASSERT_EQ(ULLONG_MAX, rs_strtoull(str, &endptr, 0));
  ASSERT_EQ(str + 19, endptr);
  ASSERT_EQ(ERANGE, rs_errno);

  str = "0xfffffffffffffffff";
  rs_errno = 0;
  ASSERT_EQ(ULLONG_MAX, rs_strtoull(str, &endptr, 0));
  ASSERT_EQ(str + 19, endptr);
  ASSERT_EQ(ERANGE, rs_errno);
}

TEST(strtoull, negative) {
  rs_setlocale(RS_LC_ALL, "C");

  const char *str;
  char *endptr;

  rs_errno = 0;
  str = "0";
  ASSERT_EQ(0, rs_strtoull(str, &endptr, 0));
  ASSERT_EQ(str + 1, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "-0";
  ASSERT_EQ(0, rs_strtoull(str, &endptr, 0));
  ASSERT_EQ(str + 2, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "-1";
  ASSERT_EQ(ULLONG_MAX, rs_strtoull(str, &endptr, 0));
  ASSERT_EQ(str + 2, endptr);
  ASSERT_EQ(0, rs_errno);
}

TEST(mblen, bad) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  ASSERT_EQ(-1, rs_mblen("", 0));
  ASSERT_EQ(EILSEQ, rs_errno);
  ASSERT_EQ(-1, rs_mblen("Hello", 0));
  ASSERT_EQ(EILSEQ, rs_errno);
}

TEST(mblen, ascii) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  ASSERT_EQ(0, rs_mblen(NULL, 12345));

  char c = 0;
  ASSERT_EQ(0, rs_mblen(&c, 12345));
  for (int i = 1; i < 128; ++i) {
    SCOPED_TRACE(i);
    c = i;
    ASSERT_EQ(1, rs_mblen(&c, 12345));
  }
  for (int i = 128; i < 256; ++i) {
    SCOPED_TRACE(i);
    c = i;
    ASSERT_EQ(-1, rs_mblen(&c, 12345));
    ASSERT_EQ(EILSEQ, rs_errno);
  }
}

TEST(mblen, unicode) {
  rs_setlocale(RS_LC_ALL, "C.UTF-8");
  rs_errno = 0;

  char euro[] = "€";
  for (size_t i = 0; i < sizeof(euro) - 1; ++i) {
    ASSERT_EQ(-1, rs_mblen(euro, i));
    ASSERT_EQ(EILSEQ, rs_errno);
  }

  ASSERT_EQ(sizeof(euro) - 1, rs_mblen(euro, sizeof(euro) - 1));
  ASSERT_EQ(sizeof(euro) - 1, rs_mblen(euro, sizeof(euro)));
}

TEST(mbtowc, bad) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  ASSERT_EQ(-1, rs_mbtowc(NULL, "", 0));
  ASSERT_EQ(EILSEQ, rs_errno);
  ASSERT_EQ(-1, rs_mbtowc(NULL, "Hello", 0));
  ASSERT_EQ(EILSEQ, rs_errno);
}

TEST(mbtowc, ascii) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  ASSERT_EQ(0, rs_mbtowc(NULL, NULL, 12345));

  wchar_t wc;
  char c = 0;
  ASSERT_EQ(0, rs_mbtowc(&wc, &c, 12345));
  ASSERT_EQ(0, wc);
  for (int i = 1; i < 128; ++i) {
    SCOPED_TRACE(i);
    c = i;
    ASSERT_EQ(1, rs_mbtowc(&wc, &c, 12345));
    ASSERT_EQ(i, wc);
  }
  for (int i = 128; i < 256; ++i) {
    SCOPED_TRACE(i);
    c = i;
    ASSERT_EQ(-1, rs_mbtowc(NULL, &c, 12345));
    ASSERT_EQ(EILSEQ, rs_errno);
  }
}

TEST(mbtowc, unicode) {
  rs_setlocale(RS_LC_ALL, "C.UTF-8");
  rs_errno = 0;

  char euro[] = "€";
  for (size_t i = 0; i < sizeof(euro) - 1; ++i) {
    ASSERT_EQ(-1, rs_mbtowc(NULL, euro, i));
    ASSERT_EQ(EILSEQ, rs_errno);
  }

  wchar_t wc;
  ASSERT_EQ(sizeof(euro) - 1, rs_mbtowc(&wc, euro, sizeof(euro) - 1));
  ASSERT_EQ(L'€', wc);
  ASSERT_EQ(sizeof(euro) - 1, rs_mbtowc(&wc, euro, sizeof(euro)));
  ASSERT_EQ(L'€', wc);
}

TEST(mbstowcs, bad) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  ASSERT_EQ(-1, rs_mbstowcs(NULL, "München", 42));
  ASSERT_EQ(EILSEQ, rs_errno);
}

TEST(mbstowcs, zero) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  ASSERT_EQ(0, rs_mbstowcs((wchar_t *)0x42, "Hello", 0));
}

TEST(mbstowcs, length) {
  rs_setlocale(RS_LC_ALL, "nl_NL.UTF-8");
  rs_errno = 0;

  ASSERT_EQ(10, rs_mbstowcs(NULL, "Düsseldorf", 0));
  ASSERT_EQ(10, rs_mbstowcs(NULL, "Düsseldorf", 5));
  ASSERT_EQ(10, rs_mbstowcs(NULL, "Düsseldorf", 40));
  ASSERT_EQ(10, rs_mbstowcs(NULL, "Düsseldorf", SIZE_MAX));
}

TEST(mbstowcs, convert) {
  rs_setlocale(RS_LC_ALL, "nl_NL.UTF-8");
  rs_errno = 0;

  {
    wchar_t buf[] = L"AAAAAAAAAAAA";
    ASSERT_EQ(0, rs_mbstowcs(buf, "Düsseldorf", 0));
    ASSERT_THAT(buf, testing::ElementsAreArray(L"AAAAAAAAAAAA"));
  }
  {
    wchar_t buf[] = L"AAAAAAAAAAAA";
    ASSERT_EQ(4, rs_mbstowcs(buf, "Düsseldorf", 4));
    ASSERT_THAT(buf, testing::ElementsAreArray(L"DüssAAAAAAAA"));
  }
  {
    wchar_t buf[] = L"AAAAAAAAAAAA";
    ASSERT_EQ(9, rs_mbstowcs(buf, "Düsseldorf", 9));
    ASSERT_THAT(buf, testing::ElementsAreArray(L"DüsseldorAAA"));
  }
  {
    wchar_t buf[] = L"AAAAAAAAAAAA";
    ASSERT_EQ(10, rs_mbstowcs(buf, "Düsseldorf", 10));
    ASSERT_THAT(buf, testing::ElementsAreArray(L"DüsseldorfAA"));
  }
  {
    wchar_t buf[] = L"AAAAAAAAAAAA";
    ASSERT_EQ(10, rs_mbstowcs(buf, "Düsseldorf", 11));
    ASSERT_THAT(buf, testing::ElementsAreArray(L"Düsseldorf\0A"));
  }
  {
    wchar_t buf[] = L"AAAAAAAAAAAA";
    ASSERT_EQ(10, rs_mbstowcs(buf, "Düsseldorf", 12));
    ASSERT_THAT(buf, testing::ElementsAreArray(L"Düsseldorf\0A"));
  }
}
TEST(wctomb, ascii) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  ASSERT_EQ(0, rs_wctomb(NULL, L'€'));

  for (int i = 0; i < 128; ++i) {
    SCOPED_TRACE(i);
    char c;
    ASSERT_EQ(1, rs_wctomb(&c, i));
    ASSERT_EQ(i, c);
  }
  for (int i = 128; i < 256; ++i) {
    SCOPED_TRACE(i);
    char c;
    ASSERT_EQ(-1, rs_wctomb(&c, i));
    ASSERT_EQ(EILSEQ, rs_errno);
  }
}

TEST(wctomb, unicode) {
  rs_setlocale(RS_LC_ALL, "C.UTF-8");
  rs_errno = 0;

  char buf[MB_LEN_MAX];
  ASSERT_EQ(sizeof("€") - 1, rs_wctomb(buf, L'€'));
  ASSERT_THAT(buf, testing::StartsWith("€"));
}

TEST(wcstombs, bad) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  ASSERT_EQ(-1, rs_wcstombs(NULL, L"München", 42));
  ASSERT_EQ(EILSEQ, rs_errno);
}

TEST(wcstombs, zero) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  ASSERT_EQ(0, rs_wcstombs((char *)0x42, L"Hello", 0));
}

TEST(wcstombs, length) {
  rs_setlocale(RS_LC_ALL, "C.UTF-8");
  rs_errno = 0;

  ASSERT_EQ(11, rs_wcstombs(NULL, L"Düsseldorf", 0));
  ASSERT_EQ(11, rs_wcstombs(NULL, L"Düsseldorf", 5));
  ASSERT_EQ(11, rs_wcstombs(NULL, L"Düsseldorf", 40));
  ASSERT_EQ(11, rs_wcstombs(NULL, L"Düsseldorf", SIZE_MAX));
}

TEST(wcstombs, convert) {
  rs_setlocale(RS_LC_ALL, "C.UTF-8");
  rs_errno = 0;

  {
    char buf[] = "AAAAAAAAAAAAA";
    ASSERT_EQ(0, rs_wcstombs(buf, L"Düsseldorf", 0));
    ASSERT_THAT(buf, testing::ElementsAreArray("AAAAAAAAAAAAA"));
  }
  {
    char buf[] = "AAAAAAAAAAAAA";
    ASSERT_EQ(1, rs_wcstombs(buf, L"Düsseldorf", 1));
    ASSERT_THAT(buf, testing::ElementsAreArray("DAAAAAAAAAAAA"));
  }
  {
    char buf[] = "AAAAAAAAAAAAA";
    ASSERT_EQ(1, rs_wcstombs(buf, L"Düsseldorf", 2));
    ASSERT_THAT(buf, testing::ElementsAreArray("DAAAAAAAAAAAA"));
  }
  {
    char buf[] = "AAAAAAAAAAAAA";
    ASSERT_EQ(3, rs_wcstombs(buf, L"Düsseldorf", 3));
    ASSERT_THAT(buf, testing::ElementsAreArray("DüAAAAAAAAAA"));
  }
  {
    char buf[] = "AAAAAAAAAAAAA";
    ASSERT_EQ(10, rs_wcstombs(buf, L"Düsseldorf", 10));
    ASSERT_THAT(buf, testing::ElementsAreArray("DüsseldorAAA"));
  }
  {
    char buf[] = "AAAAAAAAAAAAA";
    ASSERT_EQ(11, rs_wcstombs(buf, L"Düsseldorf", 11));
    ASSERT_THAT(buf, testing::ElementsAreArray("DüsseldorfAA"));
  }
  {
    char buf[] = "AAAAAAAAAAAAA";
    ASSERT_EQ(11, rs_wcstombs(buf, L"Düsseldorf", 12));
    ASSERT_THAT(buf, testing::ElementsAreArray("Düsseldorf\0A"));
  }
  {
    char buf[] = "AAAAAAAAAAAAA";
    ASSERT_EQ(11, rs_wcstombs(buf, L"Düsseldorf", 13));
    ASSERT_THAT(buf, testing::ElementsAreArray("Düsseldorf\0A"));
  }
}
