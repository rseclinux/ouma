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
