#include <gtest/gtest.h>

#include "common.h"
#include "common_float.h"
#include "common_locale.h"

extern "C"
{
  int rs_snprintf(char* s, size_t n, const char* format, ...);
  int rs_sprintf(char* s, const char* format, ...);
  int rs_swprintf(wchar_t* s, size_t n, const wchar_t* format, ...);
  int rs_asprintf(char** strp, const char* format, ...);
  int rs_aswprintf(wchar_t** strp, const wchar_t* format, ...);
}

/*-
 * Copyright (c) 2023 Dag-Erling Smørgrav
 *
 * SPDX-License-Identifier: BSD-2-Clause
 */

#define nitems(a) (sizeof(a) / sizeof(a[0]))

#define SNPRINTF_TEST(output, format, ...)                                     \
  do {                                                                         \
    char buf[256];                                                             \
    assert(strlen(output) < nitems(buf));                                      \
    int ret = rs_snprintf(buf, nitems(buf), format, __VA_ARGS__);              \
    ASSERT_EQ(strlen(output), ret);                                            \
    if (ret > 0) {                                                             \
      ASSERT_EQ(0, strcmp(output, buf));                                       \
    }                                                                          \
  } while (0)

TEST(snprintf, integer_binary)
{
  SET_GLOBAL_LOCALE("en_US.UTF-8");

  SNPRINTF_TEST("0", "%b", 0);
  SNPRINTF_TEST("           0", "%12b", 0);
  SNPRINTF_TEST("000000000000", "%012b", 0);
  SNPRINTF_TEST("1", "%b", 1);
  SNPRINTF_TEST("           1", "%12b", 1);
  SNPRINTF_TEST("000000000001", "%012b", 1);
  SNPRINTF_TEST("1111111111111111111111111111111", "%b", INT_MAX);
  SNPRINTF_TEST("0", "%#b", 0);
  SNPRINTF_TEST("           0", "%#12b", 0);
  SNPRINTF_TEST("000000000000", "%#012b", 0);
  SNPRINTF_TEST("0b1", "%#b", 1);
  SNPRINTF_TEST("         0b1", "%#12b", 1);
  SNPRINTF_TEST("0b0000000001", "%#012b", 1);
  SNPRINTF_TEST("0b1111111111111111111111111111111", "%#b", INT_MAX);

  SNPRINTF_TEST("0", "%B", 0);
  SNPRINTF_TEST("           0", "%12B", 0);
  SNPRINTF_TEST("000000000000", "%012B", 0);
  SNPRINTF_TEST("1", "%B", 1);
  SNPRINTF_TEST("           1", "%12B", 1);
  SNPRINTF_TEST("000000000001", "%012B", 1);
  SNPRINTF_TEST("1111111111111111111111111111111", "%B", INT_MAX);
  SNPRINTF_TEST("0", "%#B", 0);
  SNPRINTF_TEST("           0", "%#12B", 0);
  SNPRINTF_TEST("000000000000", "%#012B", 0);
  SNPRINTF_TEST("0B1", "%#B", 1);
  SNPRINTF_TEST("         0B1", "%#12B", 1);
  SNPRINTF_TEST("0B0000000001", "%#012B", 1);
  SNPRINTF_TEST("0B1111111111111111111111111111111", "%#B", INT_MAX);
}

TEST(snprintf, decimal)
{
  SET_GLOBAL_LOCALE("en_US.UTF-8");

  SNPRINTF_TEST("0", "%d", 0);
  SNPRINTF_TEST("           0", "%12d", 0);
  SNPRINTF_TEST("000000000000", "%012d", 0);
  SNPRINTF_TEST("1", "%d", 1);
  SNPRINTF_TEST("           1", "%12d", 1);
  SNPRINTF_TEST("000000000001", "%012d", 1);
  SNPRINTF_TEST("2147483647", "%d", INT_MAX);
  SNPRINTF_TEST("  2147483647", "%12d", INT_MAX);
  SNPRINTF_TEST("002147483647", "%012d", INT_MAX);
  SNPRINTF_TEST("2,147,483,647", "%'d", INT_MAX);
}

TEST(snprintf, integer)
{
  SET_GLOBAL_LOCALE("en_US.UTF-8");

  SNPRINTF_TEST("0", "%i", 0);
  SNPRINTF_TEST("           0", "%12i", 0);
  SNPRINTF_TEST("000000000000", "%012i", 0);
  SNPRINTF_TEST("1", "%i", 1);
  SNPRINTF_TEST("           1", "%12i", 1);
  SNPRINTF_TEST("000000000001", "%012i", 1);
  SNPRINTF_TEST("-1", "%i", -1);
  SNPRINTF_TEST("          -1", "%12i", -1);
  SNPRINTF_TEST("-00000000001", "%012i", -1);
  SNPRINTF_TEST("2147483647", "%i", INT_MAX);
  SNPRINTF_TEST("  2147483647", "%12i", INT_MAX);
  SNPRINTF_TEST("002147483647", "%012i", INT_MAX);
  SNPRINTF_TEST("2,147,483,647", "%'i", INT_MAX);
  SNPRINTF_TEST("-2147483648", "%i", INT_MIN);
  SNPRINTF_TEST(" -2147483648", "%12i", INT_MIN);
  SNPRINTF_TEST("-02147483648", "%012i", INT_MIN);
}

TEST(snprintf, unsigned)
{
  SET_GLOBAL_LOCALE("en_US.UTF-8");

  SNPRINTF_TEST("0", "%u", 0);
  SNPRINTF_TEST("           0", "%12u", 0);
  SNPRINTF_TEST("000000000000", "%012u", 0);
  SNPRINTF_TEST("1", "%u", 1);
  SNPRINTF_TEST("           1", "%12u", 1);
  SNPRINTF_TEST("000000000001", "%012u", 1);
  SNPRINTF_TEST("2147483647", "%u", INT_MAX);
  SNPRINTF_TEST("  2147483647", "%12u", INT_MAX);
  SNPRINTF_TEST("002147483647", "%012u", INT_MAX);
  SNPRINTF_TEST("4294967295", "%u", UINT_MAX);
  SNPRINTF_TEST("  4294967295", "%12u", UINT_MAX);
  SNPRINTF_TEST("004294967295", "%012u", UINT_MAX);
  SNPRINTF_TEST("4,294,967,295", "%'u", UINT_MAX);
  SNPRINTF_TEST("4294967295", "%u", -1);
  SNPRINTF_TEST("  4294967295", "%12u", -1);
  SNPRINTF_TEST("004294967295", "%012u", -1);
  SNPRINTF_TEST("4294967294", "%u", -2);
  SNPRINTF_TEST("2147483648", "%u", INT_MIN);
  SNPRINTF_TEST("  2147483648", "%12u", INT_MIN);
  SNPRINTF_TEST("002147483648", "%012u", INT_MIN);
  SNPRINTF_TEST("2,147,483,648", "%'u", INT_MIN);
}

TEST(snprintf, octal)
{
  SET_GLOBAL_LOCALE("en_US.UTF-8");

  SNPRINTF_TEST("0", "%o", 0);
  SNPRINTF_TEST("           0", "%12o", 0);
  SNPRINTF_TEST("000000000000", "%012o", 0);
  SNPRINTF_TEST("1", "%o", 1);
  SNPRINTF_TEST("           1", "%12o", 1);
  SNPRINTF_TEST("000000000001", "%012o", 1);
  SNPRINTF_TEST("10", "%o", 8);
  SNPRINTF_TEST("17777777777", "%o", INT_MAX);
  SNPRINTF_TEST(" 17777777777", "%12o", INT_MAX);
  SNPRINTF_TEST("017777777777", "%012o", INT_MAX);
  SNPRINTF_TEST("37777777777", "%o", UINT_MAX);
  SNPRINTF_TEST(" 37777777777", "%12o", UINT_MAX);
  SNPRINTF_TEST("037777777777", "%012o", UINT_MAX);
  SNPRINTF_TEST("37777777777", "%o", -1);
  SNPRINTF_TEST(" 37777777777", "%12o", -1);
  SNPRINTF_TEST("037777777777", "%012o", -1);
  SNPRINTF_TEST("37777777776", "%o", -2);
  SNPRINTF_TEST("20000000000", "%o", INT_MIN);
  SNPRINTF_TEST(" 20000000000", "%12o", INT_MIN);
  SNPRINTF_TEST("020000000000", "%012o", INT_MIN);
  SNPRINTF_TEST("0", "%#o", 0);
  SNPRINTF_TEST("           0", "%#12o", 0);
  SNPRINTF_TEST("01", "%#o", 1);
  SNPRINTF_TEST("010", "%#o", 8);
  SNPRINTF_TEST("017777777777", "%#o", INT_MAX);
  SNPRINTF_TEST("037777777777", "%#o", UINT_MAX);
}

TEST(snprintf, hexadecimal)
{
  SET_GLOBAL_LOCALE("en_US.UTF-8");

  SNPRINTF_TEST("0", "%x", 0);
  SNPRINTF_TEST("           0", "%12x", 0);
  SNPRINTF_TEST("000000000000", "%012x", 0);
  SNPRINTF_TEST("1", "%x", 1);
  SNPRINTF_TEST("           1", "%12x", 1);
  SNPRINTF_TEST("000000000001", "%012x", 1);
  SNPRINTF_TEST("7fffffff", "%x", INT_MAX);
  SNPRINTF_TEST("    7fffffff", "%12x", INT_MAX);
  SNPRINTF_TEST("00007fffffff", "%012x", INT_MAX);
  SNPRINTF_TEST("0", "%#x", 0);
  SNPRINTF_TEST("           0", "%#12x", 0);
  SNPRINTF_TEST("000000000000", "%#012x", 0);
  SNPRINTF_TEST("0x1", "%#x", 1);
  SNPRINTF_TEST("         0x1", "%#12x", 1);
  SNPRINTF_TEST("0x0000000001", "%#012x", 1);
  SNPRINTF_TEST("0x7fffffff", "%#x", INT_MAX);
  SNPRINTF_TEST("  0x7fffffff", "%#12x", INT_MAX);
  SNPRINTF_TEST("0x007fffffff", "%#012x", INT_MAX);

  SNPRINTF_TEST("0", "%X", 0);
  SNPRINTF_TEST("           0", "%12X", 0);
  SNPRINTF_TEST("000000000000", "%012X", 0);
  SNPRINTF_TEST("1", "%X", 1);
  SNPRINTF_TEST("           1", "%12X", 1);
  SNPRINTF_TEST("000000000001", "%012X", 1);
  SNPRINTF_TEST("7FFFFFFF", "%X", INT_MAX);
  SNPRINTF_TEST("    7FFFFFFF", "%12X", INT_MAX);
  SNPRINTF_TEST("00007FFFFFFF", "%012X", INT_MAX);
  SNPRINTF_TEST("0", "%#X", 0);
  SNPRINTF_TEST("           0", "%#12X", 0);
  SNPRINTF_TEST("000000000000", "%#012X", 0);
  SNPRINTF_TEST("0X1", "%#X", 1);
  SNPRINTF_TEST("         0X1", "%#12X", 1);
  SNPRINTF_TEST("0X0000000001", "%#012X", 1);
  SNPRINTF_TEST("0X7FFFFFFF", "%#X", INT_MAX);
  SNPRINTF_TEST("  0X7FFFFFFF", "%#12X", INT_MAX);
  SNPRINTF_TEST("0X007FFFFFFF", "%#012X", INT_MAX);
}

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wformat"

TEST(snprintf, width)
{
  SET_GLOBAL_LOCALE("en_US.UTF-8");

  SNPRINTF_TEST("0", "%w8d", (int8_t)0);
  SNPRINTF_TEST("-128", "%w8d", (int8_t)SCHAR_MIN);
  SNPRINTF_TEST("127", "%w8d", (int8_t)SCHAR_MAX);
  SNPRINTF_TEST("0", "%w8u", (uint8_t)0);
  SNPRINTF_TEST("255", "%w8u", (uint8_t)UCHAR_MAX);

  SNPRINTF_TEST("0", "%w16d", (int16_t)0);
  SNPRINTF_TEST("-32768", "%w16d", (int16_t)SHRT_MIN);
  SNPRINTF_TEST("32767", "%w16d", (int16_t)SHRT_MAX);
  SNPRINTF_TEST("0", "%w16u", (uint16_t)0);
  SNPRINTF_TEST("65535", "%w16u", (uint16_t)USHRT_MAX);

  SNPRINTF_TEST("0", "%w32d", (int32_t)0);
  SNPRINTF_TEST("-2147483648", "%w32d", (int32_t)INT_MIN);
  SNPRINTF_TEST("2147483647", "%w32d", (int32_t)INT_MAX);
  SNPRINTF_TEST("0", "%w32u", (uint32_t)0);
  SNPRINTF_TEST("4294967295", "%w32u", (uint32_t)UINT_MAX);

  SNPRINTF_TEST("0", "%w64d", (int64_t)0);
  SNPRINTF_TEST("-9223372036854775808", "%w64d", (int64_t)LLONG_MIN);
  SNPRINTF_TEST("9223372036854775807", "%w64d", (int64_t)LLONG_MAX);
  SNPRINTF_TEST("0", "%w64u", (uint64_t)0);
  SNPRINTF_TEST("18446744073709551615", "%w64u", (uint64_t)ULLONG_MAX);

  SNPRINTF_TEST("wd", "%wd", 0);
  SNPRINTF_TEST("w1d", "%w1d", 0);
  SNPRINTF_TEST("w128d", "%w128d", 0);
}

TEST(snprintf, width_fast)
{
  SET_GLOBAL_LOCALE("en_US.UTF-8");

  SNPRINTF_TEST("0", "%wf8d", (int_fast8_t)0);
  SNPRINTF_TEST("-128", "%wf8d", (int_fast8_t)INT_FAST8_MIN);
  SNPRINTF_TEST("127", "%wf8d", (int_fast8_t)INT_FAST8_MAX);
  SNPRINTF_TEST("0", "%wf8u", (uint_fast8_t)0);
  SNPRINTF_TEST("255", "%wf8u", (uint_fast8_t)UINT_FAST8_MAX);

  SNPRINTF_TEST("0", "%wf16d", (int_fast16_t)0);
  SNPRINTF_TEST("-9223372036854775808", "%wf16d", (int_fast16_t)INT_FAST16_MIN);
  SNPRINTF_TEST("9223372036854775807", "%wf16d", (int_fast16_t)INT_FAST16_MAX);
  SNPRINTF_TEST("0", "%wf16u", (uint_fast16_t)0);
  SNPRINTF_TEST(
    "18446744073709551615", "%wf16u", (uint_fast16_t)UINT_FAST16_MAX);

  SNPRINTF_TEST("0", "%wf32d", (int_fast32_t)0);
  SNPRINTF_TEST("-9223372036854775808", "%wf32d", (int_fast32_t)INT_FAST32_MIN);
  SNPRINTF_TEST("9223372036854775807", "%wf32d", (int_fast32_t)INT_FAST32_MAX);
  SNPRINTF_TEST("0", "%wf32u", (uint_fast32_t)0);
  SNPRINTF_TEST(
    "18446744073709551615", "%wf32u", (uint_fast32_t)UINT_FAST32_MAX);

  SNPRINTF_TEST("0", "%wf64d", (int_fast64_t)0);
  SNPRINTF_TEST("-9223372036854775808", "%wf64d", (int_fast64_t)INT_FAST64_MIN);
  SNPRINTF_TEST("9223372036854775807", "%wf64d", (int_fast64_t)INT_FAST64_MAX);
  SNPRINTF_TEST("0", "%wf64u", (uint_fast64_t)0);
  SNPRINTF_TEST(
    "18446744073709551615", "%wf64u", (uint_fast64_t)UINT_FAST64_MAX);

  SNPRINTF_TEST("wfd", "%wfd", 0);
  SNPRINTF_TEST("wf1d", "%wf1d", 0);
  SNPRINTF_TEST("wf128d", "%wf128d", 0);
}

#pragma GCC diagnostic pop

#define S_UINT64MAX "18446744073709551615"
#define S_UINT32MAX "4294967295"
#define S_INT32MIN "-2147483648"
#define S_INT64MIN "-9223372036854775808"
#define S_ULLONGMAX S_UINT64MAX
#if ULONG_MAX == UINT64_MAX
#define S_ULONGMAX S_UINT64MAX
#else
#define S_ULONGMAX S_UINT32MAX
#endif
#if SIZE_MAX == UINT64_MAX
#define S_SIZEMAX S_UINT64MAX
#else
#define S_SIZEMAX S_UINT32MAX
#endif

TEST(snprintf, int_within_limits)
{
  REVERT_TO_C_LOCALE();

  static_assert(UINTMAX_MAX == UINT64_MAX, "");
  static_assert(UINT_MAX == UINT32_MAX, "");
  static_assert(USHRT_MAX == 0xffff, "");
  static_assert(UCHAR_MAX == 0xff, "");

  SNPRINTF_TEST("-1", "%jd", (intmax_t)-1);
  SNPRINTF_TEST(S_UINT64MAX, "%ju", UINT64_MAX);

  SNPRINTF_TEST("-1", "%td", (ptrdiff_t)-1);
  SNPRINTF_TEST(S_SIZEMAX, "%tu", (size_t)-1);

  SNPRINTF_TEST("-1", "%zd", (ssize_t)-1);
  SNPRINTF_TEST(S_SIZEMAX, "%zu", (ssize_t)-1);

  SNPRINTF_TEST("-1", "%ld", (long)-1);
  SNPRINTF_TEST(S_ULONGMAX, "%lu", ULONG_MAX);

  SNPRINTF_TEST("-1", "%lld", (long long)-1);
  SNPRINTF_TEST(S_ULLONGMAX, "%llu", ULLONG_MAX);

  SNPRINTF_TEST("-1", "%d", -1);
  SNPRINTF_TEST(S_UINT32MAX, "%u", UINT32_MAX);

  SNPRINTF_TEST("-1", "%hd", -1);
  SNPRINTF_TEST("65535", "%hu", USHRT_MAX);

  SNPRINTF_TEST("-1", "%hhd", -1);
  SNPRINTF_TEST("255", "%hhu", UCHAR_MAX);
}

TEST(snprintf, int_limits)
{
  REVERT_TO_C_LOCALE();

  SNPRINTF_TEST(S_INT32MIN, "%d", INT_MIN);
  SNPRINTF_TEST(S_INT64MIN, "%jd", INTMAX_MIN);
}

TEST(snprintf, number_width_precision_left)
{
  REVERT_TO_C_LOCALE();

  SNPRINTF_TEST("          0000000005", "%20.10d", 5);
}

TEST(snprintf, number_width_precision_right)
{
  REVERT_TO_C_LOCALE();

  SNPRINTF_TEST("0000000005          ", "%-20.10d", 5);
}

TEST(snprintf, numbered_arguments)
{
  REVERT_TO_C_LOCALE();

  SNPRINTF_TEST("7 5", "%2$d %1$d", 5, 7);
}
