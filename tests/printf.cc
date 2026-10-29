#include <cfenv>
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
    int ret = rs_snprintf(buf, nitems(buf), format, ##__VA_ARGS__);            \
    std::string str = buf;                                                     \
    SCOPED_TRACE(str);                                                         \
    ASSERT_EQ(strlen(output), ret);                                            \
    if (ret > 0) {                                                             \
      ASSERT_EQ(0, strcmp(output, buf));                                       \
    }                                                                          \
  } while (0)

TEST(snprintf, binary)
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

TEST(snprintf, null)
{
  REVERT_TO_C_LOCALE();
  ASSERT_EQ(12, rs_snprintf(NULL, 0, "Hello, world"));
}

TEST(snprintf, null_string)
{
  REVERT_TO_C_LOCALE();

  char* ptr = NULL;
  SNPRINTF_TEST("Hello(nuworld", "Hello%.3sworld", ptr);
}

TEST(snprintf, truncation)
{
  REVERT_TO_C_LOCALE();

  char buf[10];
  ASSERT_EQ(12, rs_snprintf(buf, sizeof(buf), "Hello, world"));
  ASSERT_STREQ("Hello, wo", buf);
}

TEST(snprintf, strerror)
{
  SET_GLOBAL_LOCALE("br_FR.UTF-8");

  rs_errno = 0;
  SNPRINTF_TEST("Berzh", "%m");

  rs_errno = -1;
  SNPRINTF_TEST("Fazi dizanv -1", "%m");

  rs_errno = EINVAL;
  SNPRINTF_TEST("Arguzenn fall", "%m");

  rs_errno = 0;
  SNPRINTF_TEST("<0>", "<%#m>");

  rs_errno = -1;
  SNPRINTF_TEST("<-1>", "<%#m>");

  rs_errno = EINVAL;
  SNPRINTF_TEST("<EINVAL>", "<%#m>");
}

TEST(snprintf, smoke)
{
  REVERT_TO_C_LOCALE();

  SNPRINTF_TEST("a", "a");
  SNPRINTF_TEST("%", "%%");
  SNPRINTF_TEST("01234", "01234");
  SNPRINTF_TEST("a01234b", "a%sb", "01234");
  SNPRINTF_TEST("a(null)b", "a%sb", static_cast<char*>(nullptr));
  SNPRINTF_TEST("aabbcc", "aa%scc", "bb");
  SNPRINTF_TEST("abc", "a%cc", 'b');
  SNPRINTF_TEST("a1234b", "a%db", 1234);
  SNPRINTF_TEST("a-8123b", "a%db", -8123);
  SNPRINTF_TEST("a16b", "a%hdb", static_cast<short>(0x7fff0010));
  SNPRINTF_TEST("a16b", "a%hhdb", static_cast<char>(0x7fffff10));
  SNPRINTF_TEST("a68719476736b", "a%lldb", 0x1000000000LL);
  SNPRINTF_TEST("a70000b", "a%ldb", 70000L);
  SNPRINTF_TEST("a0xb0001234b", "a%pb", reinterpret_cast<void*>(0xb0001234));
  SNPRINTF_TEST("a12abz", "a%xz", 0x12ab);
  SNPRINTF_TEST("a12ABz", "a%Xz", 0x12ab);
  SNPRINTF_TEST("a00123456z", "a%08xz", 0x123456);
  SNPRINTF_TEST("a 1234z", "a%5dz", 1234);
  SNPRINTF_TEST("a01234z", "a%05dz", 1234);
  SNPRINTF_TEST("a    1234z", "a%8dz", 1234);
  SNPRINTF_TEST("a1234    z", "a%-8dz", 1234);
  SNPRINTF_TEST("Aabcdef     Z", "A%-11sZ", "abcdef");
  SNPRINTF_TEST("Ahello:1234Z", "A%s:%dZ", "hello", 1234);
  SNPRINTF_TEST("a005:5:05z", "a%03d:%d:%02dz", 5, 5, 5);
  SNPRINTF_TEST("a5,(nil)z", "a%d,%pz", 5, static_cast<void*>(nullptr));
  SNPRINTF_TEST(
    "a68719476736,6,7,8z", "a%lld,%d,%d,%dz", 0x1000000000LL, 6, 7, 8);
  SNPRINTF_TEST("print_me_twice print_me_twice", "%1$s %1$s", "print_me_twice");
}

TEST(snprintf, decimal_fixed_automatic)
{
  REVERT_TO_C_LOCALE();

  SNPRINTF_TEST("|1.000000|", "|%f|", 1.0);
  SNPRINTF_TEST("|1.000000|", "|%#f|", 1.0);
  SNPRINTF_TEST("|1|", "|%g|", 1.0);
  SNPRINTF_TEST("|1.00000|", "|%#g|", 1.0);

  SNPRINTF_TEST("|  1.000000|", "|%10f|", 1.0);
  SNPRINTF_TEST("|  1.000000|", "|%#10f|", 1.0);
  SNPRINTF_TEST("|         1|", "|%10g|", 1.0);
  SNPRINTF_TEST("|   1.00000|", "|%#10g|", 1.0);

  SNPRINTF_TEST("|1.000000  |", "|%-10f|", 1.0);
  SNPRINTF_TEST("|1.000000  |", "|%#-10f|", 1.0);
  SNPRINTF_TEST("|1         |", "|%-10g|", 1.0);
  SNPRINTF_TEST("|1.00000   |", "|%#-10g|", 1.0);

  SNPRINTF_TEST("|001.000000|", "|%010f|", 1.0);
  SNPRINTF_TEST("|001.000000|", "|%#010f|", 1.0);
  SNPRINTF_TEST("|0000000001|", "|%010g|", 1.0);
  SNPRINTF_TEST("|0001.00000|", "|%#010g|", 1.0);

  SNPRINTF_TEST("|         1|", "|%10.0f|", 1.0);
  SNPRINTF_TEST("|        1.|", "|%#10.0f|", 1.0);
  SNPRINTF_TEST("|         1|", "|%10.0g|", 1.0);
  SNPRINTF_TEST("|        1.|", "|%#10.0g|", 1.0);

  SNPRINTF_TEST("|1         |", "|%-10.0f|", 1.0);
  SNPRINTF_TEST("|1.        |", "|%#-10.0f|", 1.0);
  SNPRINTF_TEST("|1         |", "|%-10.0g|", 1.0);
  SNPRINTF_TEST("|1.        |", "|%#-10.0g|", 1.0);

  SNPRINTF_TEST("|0000000001|", "|%010.0f|", 1.0);
  SNPRINTF_TEST("|000000001.|", "|%#010.0f|", 1.0);
  SNPRINTF_TEST("|0000000001|", "|%010.0g|", 1.0);
  SNPRINTF_TEST("|000000001.|", "|%#010.0g|", 1.0);
}

TEST(snprintf, decimal_scientific_zero)
{
  REVERT_TO_C_LOCALE();

  SNPRINTF_TEST("0.000000e+00", "%e", 0.0);
  SNPRINTF_TEST("0e+00", "%.0e", 0.0);
  SNPRINTF_TEST("0.e+00", "%#.0e", 0.0);
  SNPRINTF_TEST(" 0e+00", "% .0e", 0.0);
  SNPRINTF_TEST("+0e+00", "%+.0e", 0.0);
}

TEST(snprintf, decimal_scientific_pi)
{
  REVERT_TO_C_LOCALE();

  SNPRINTF_TEST("3e+00", "%.0e", M_PI);
  SNPRINTF_TEST("3.1e+00", "%.1e", M_PI);
  SNPRINTF_TEST("3.14e+00", "%.2e", M_PI);
  SNPRINTF_TEST("3.142e+00", "%.3e", M_PI);
  SNPRINTF_TEST("3.1416e+00", "%.4e", M_PI);
  SNPRINTF_TEST("3.14159e+00", "%.5e", M_PI);
  SNPRINTF_TEST("3.141593e+00", "%.6e", M_PI);
  SNPRINTF_TEST("3.1415927e+00", "%.7e", M_PI);
  SNPRINTF_TEST("3.14159265e+00", "%.8e", M_PI);

  SNPRINTF_TEST("3.14159265358979312e+00", "%.17e", M_PI);
  SNPRINTF_TEST("3.141592653589793115997963468544e+00", "%.30e", M_PI);

  SNPRINTF_TEST("3."
                "14159265358979311599796346854418516159057617187500000000000000"
                "00000000000000000000000000000000000000e+00",
                "%.100e",
                M_PI);
}

TEST(snprintf, float_limits)
{
  REVERT_TO_C_LOCALE();

  SNPRINTF_TEST(" 1.000000E+00", "%13E", 1.0);
  SNPRINTF_TEST("     1.000000", "%13f", 1.0);
  SNPRINTF_TEST("            1", "%13G", 1.0);
  SNPRINTF_TEST(" 1.000000E+00", "%13LE", 1.0L);
  SNPRINTF_TEST("     1.000000", "%13Lf", 1.0L);
  SNPRINTF_TEST("            1", "%13LG", 1.0L);

  SNPRINTF_TEST("2.718282", "%.*f", -2, 2.7182818);

  SNPRINTF_TEST("1.234568e+06", "%e", 1234567.8);
  SNPRINTF_TEST("1234567.800000", "%f", 1234567.8);
  SNPRINTF_TEST("1.23457E+06", "%G", 1234567.8);
  SNPRINTF_TEST("1.234568e+06", "%Le", 1234567.8L);
  SNPRINTF_TEST("1234567.800000", "%Lf", 1234567.8L);
  SNPRINTF_TEST("1.23457E+06", "%LG", 1234567.8L);
}

TEST(snprintf, float_inf_nan)
{
  REVERT_TO_C_LOCALE();

  SNPRINTF_TEST("nan", "%e", NAN);
  SNPRINTF_TEST("NAN", "%F", NAN);
  SNPRINTF_TEST("nan", "%g", NAN);
  SNPRINTF_TEST("NAN", "%LE", (long double)NAN);
  SNPRINTF_TEST("  nan", "%05e", NAN);

  SNPRINTF_TEST("INF", "%E", HUGE_VAL);
  SNPRINTF_TEST("-inf", "%f", -HUGE_VAL);
  SNPRINTF_TEST("+inf", "%+g", HUGE_VAL);
  SNPRINTF_TEST(" inf", "%4.2Le", HUGE_VALL);
  SNPRINTF_TEST("-inf", "%Lf", -HUGE_VALL);
  SNPRINTF_TEST("  inf", "%05e", HUGE_VAL);
  SNPRINTF_TEST(" -inf", "%05e", -HUGE_VAL);
}

TEST(snprintf, float_padding)
{
  REVERT_TO_C_LOCALE();

  SNPRINTF_TEST("0.000000e+00", "%e", 0.0);
  SNPRINTF_TEST("0.000000", "%F", (double)0.0);
  SNPRINTF_TEST("0", "%G", 0.0);
  SNPRINTF_TEST("  0", "%3.0Lg", 0.0L);
  SNPRINTF_TEST("    0", "%5.0f", 0.001);
}

TEST(snprintf, float_precision)
{
  REVERT_TO_C_LOCALE();

  SNPRINTF_TEST("1.0123e+00", "%.4e", 1.0123456789);
  SNPRINTF_TEST("1.0123", "%.4f", 1.0123456789);
  SNPRINTF_TEST("1.012", "%.4g", 1.0123456789);
  SNPRINTF_TEST("1.2346e-02", "%.4e", 0.0123456789);
  SNPRINTF_TEST("0.0123", "%.4f", 0.0123456789);
  SNPRINTF_TEST("0.01235", "%.4g", 0.0123456789);
}

TEST(snprintf, float_misc)
{
  REVERT_TO_C_LOCALE();

  SNPRINTF_TEST("+2.500000e-01", "%+e", 0.25);
  SNPRINTF_TEST("+0.000000", "%+F", 0.0);
  SNPRINTF_TEST("-1", "%+g", -1.0);

  SNPRINTF_TEST("-1.000000e+00", "% e", -1.0);
  SNPRINTF_TEST("+1.000000", "% +f", 1.0);
  SNPRINTF_TEST(" 1", "% g", 1.0);
  SNPRINTF_TEST(" 0", "% g", 0.0);

  SNPRINTF_TEST("03.2E+00", "%08.1E", 3.25);
  SNPRINTF_TEST("003.25", "%06.2F", 3.25);
  SNPRINTF_TEST("0003.25", "%07.4G", 3.25);

  SNPRINTF_TEST("3.14159e-05", "%g", 3.14159e-5);
  SNPRINTF_TEST("0.000314159", "%g", 3.14159e-4);
  SNPRINTF_TEST("3.14159e+06", "%g", 3.14159e6);
  SNPRINTF_TEST("314159", "%g", 3.14159e5);
  SNPRINTF_TEST("314159.", "%#g", 3.14159e5);

  SNPRINTF_TEST(" 9.000000e+03", "%13e", 9000.0);
  SNPRINTF_TEST(" 9000.000000", "%12f", 9000.0);
  SNPRINTF_TEST(" 9000", "%5g", 9000.0);
  SNPRINTF_TEST(" 900000.", "%#8g", 900000.0);
  SNPRINTF_TEST(" 9e+06", "%6g", 9000000.0);
  SNPRINTF_TEST(" 9.000000e-04", "%13e", 0.0009);
  SNPRINTF_TEST(" 0.000900", "%9f", 0.0009);
  SNPRINTF_TEST(" 0.0009", "%7g", 0.0009);
  SNPRINTF_TEST(" 9e-05", "%6g", 0.00009);
  SNPRINTF_TEST(" 9.00000e-05", "%#12g", 0.00009);
  SNPRINTF_TEST(" 9.e-05", "%#7.1g", 0.00009);

  SNPRINTF_TEST(" 0.0", "%4.1f", 0.0);
  SNPRINTF_TEST("90.0", "%4.1f", 90.0);
  SNPRINTF_TEST(" 100", "%4.0f", 100.0);
  SNPRINTF_TEST("9.0e+01", "%4.1e", 90.0);
  SNPRINTF_TEST("1e+02", "%4.0e", 100.0);
}

TEST(snprintf, decimal_rounding)
{
  REVERT_TO_C_LOCALE();

  fesetround(FE_DOWNWARD);
  SNPRINTF_TEST("4.437", "%.3f", 4.4375);
  SNPRINTF_TEST("-4.438", "%.3f", -4.4375);
  SNPRINTF_TEST("4.437", "%.3Lf", 4.4375L);
  SNPRINTF_TEST("-4.438", "%.3Lf", -4.4375L);

  fesetround(FE_UPWARD);
  SNPRINTF_TEST("4.438", "%.3f", 4.4375);
  SNPRINTF_TEST("-4.437", "%.3f", -4.4375);
  SNPRINTF_TEST("4.438", "%.3Lf", 4.4375L);
  SNPRINTF_TEST("-4.437", "%.3Lf", -4.4375L);

  fesetround(FE_TOWARDZERO);
  SNPRINTF_TEST("4.437", "%.3f", 4.4375);
  SNPRINTF_TEST("-4.437", "%.3f", -4.4375);
  SNPRINTF_TEST("4.437", "%.3Lf", 4.4375L);
  SNPRINTF_TEST("-4.437", "%.3Lf", -4.4375L);

  fesetround(FE_TONEAREST);
  SNPRINTF_TEST("4.438", "%.3f", 4.4375);
  SNPRINTF_TEST("-4.438", "%.3f", -4.4375);
  SNPRINTF_TEST("4.438", "%.3Lf", 4.4375L);
  SNPRINTF_TEST("-4.438", "%.3Lf", -4.4375L);
}

TEST(snprintf, localized_grouping)
{
  REVERT_TO_C_LOCALE();

  SNPRINTF_TEST("12345678.0625", "%'.04f", 12345678.0625);
  SNPRINTF_TEST("0012345678.0625", "%'015.4F", 12345678.0625);

  SET_GLOBAL_LOCALE("ru_RU.UTF-8");
  SNPRINTF_TEST("3,1415", "%g", 3.1415);
  SNPRINTF_TEST("123 456 789,234568", "%'f", 123456789.2345678);

  SET_GLOBAL_LOCALE("el_GR.UTF-8");
  SNPRINTF_TEST("1234,00", "%'.2f", 1234.00);
  SNPRINTF_TEST("123456,789", "%'.3f", 123456.789);

  SET_GLOBAL_LOCALE("fr_CH.UTF-8");
  SNPRINTF_TEST("123456789,234568", "%f", 123456789.2345678);

  REVERT_TO_C_LOCALE();
  SNPRINTF_TEST("12345678.062500", "%'f", 12345678.0625);
  SNPRINTF_TEST("9000.000000", "%'f", 9000.0);
}

TEST(snprintf, hexadecimal_float_nan)
{
  REVERT_TO_C_LOCALE();

  SNPRINTF_TEST("-nan      ", "%-10a", -NAN);
  SNPRINTF_TEST("       NAN", "%10A", NAN);
}

TEST(snprintf, hexadecimal_float_inf)
{
  REVERT_TO_C_LOCALE();

  SNPRINTF_TEST("       inf", "%10a", INFINITY);
  SNPRINTF_TEST("-INF      ", "%-10A", -INFINITY);
}

TEST(snprintf, hexadecimal_float_simple_zero)
{
  REVERT_TO_C_LOCALE();

  SNPRINTF_TEST("0x0p+0", "%a", 0.0);
  SNPRINTF_TEST("0x0.p+0", "%#a", 0.0);
}

TEST(snprintf, hexadecimal_float_simple_one)
{
  REVERT_TO_C_LOCALE();

  SNPRINTF_TEST("0x1p+0", "%a", 1.0);
  SNPRINTF_TEST("0x1.p+0", "%#a", 1.0);
}

TEST(snprintf, hexadecimal_float_simple_number)
{
  REVERT_TO_C_LOCALE();

  SNPRINTF_TEST("0x1.2345678p+24", "%a", 0x1234.5678p12);
}

TEST(snprintf, hexadecimal_float_negative)
{
  REVERT_TO_C_LOCALE();

  SNPRINTF_TEST("-0x1.2345678p+24", "%a", -0x1234.5678p12);
}

TEST(snprintf, hexadecimal_float_round)
{
  REVERT_TO_C_LOCALE();

  ASSERT_EQ(0, fesetround(FE_DOWNWARD));
  SNPRINTF_TEST("0x1.82p+0 -0x1.83p+0", "%.2a %.2a", 1.51, -1.51);
  ASSERT_EQ(0, fesetround(FE_TONEAREST));
  SNPRINTF_TEST("0x1.83p+0 -0x1.83p+0", "%.2a %.2a", 1.51, -1.51);
  ASSERT_EQ(0, fesetround(FE_TOWARDZERO));
  SNPRINTF_TEST("0x1.82p+0 -0x1.82p+0", "%.2a %.2a", 1.51, -1.51);
  ASSERT_EQ(0, fesetround(FE_UPWARD));
  SNPRINTF_TEST("0x1.83p+0 -0x1.82p+0", "%.2a %.2a", 1.51, -1.51);
}

TEST(snprintf, hexadecimal_float_align)
{
  REVERT_TO_C_LOCALE();

  SNPRINTF_TEST("       0x1.2abc00000000000p+16", "%#30.15a", 0x12.abcp12);
  SNPRINTF_TEST("0X1.2ABC00000000000P+16       ", "%#-30.15A", 0x12.abcp12);
  SNPRINTF_TEST("0x00000001.2abc00000000000p+16", "%#030.15a", 0x12.abcp12);
  SNPRINTF_TEST("+0x0000001.2abc00000000000p+16", "%+#030.15a", 0x12.abcp12);
  SNPRINTF_TEST(" 0x0000001.2abc00000000000p+16", "% #030.15a", 0x12.abcp12);
}

#if LDBL_HAS_SUBNORM == 1
struct RoundingGuard
{
  int saved;
  RoundingGuard()
    : saved(fegetround())
  {
  }
  ~RoundingGuard() { fesetround(saved); }
};

TEST(snprintf, hexadecimal_float_subnormal)
{
  REVERT_TO_C_LOCALE();

  fesetround(FE_TONEAREST);

  RoundingGuard guard;

#if LDBL_TYPE == LDBL_IS_F64
  SNPRINTF_TEST("0x0.0000000000001p-1022", "%La", LDBL_TRUE_MIN);
  long double high = nexttowardl(LDBL_MIN, 0.0L);
  SNPRINTF_TEST("0x0.fffffffffffffp-1022", "%La", high);

  ASSERT_EQ(0, fesetround(FE_DOWNWARD));
  SNPRINTF_TEST("0x0.ffp-1022 -0x1.00p-1022", "%.2La %.2La", high, -high);
  ASSERT_EQ(0, fesetround(FE_TONEAREST));
  SNPRINTF_TEST("0x1.00p-1022 -0x1.00p-1022", "%.2La %.2La", high, -high);
  ASSERT_EQ(0, fesetround(FE_TOWARDZERO));
  SNPRINTF_TEST("0x0.ffp-1022 -0x0.ffp-1022", "%.2La %.2La", high, -high);
  ASSERT_EQ(0, fesetround(FE_UPWARD));
  SNPRINTF_TEST("0x1.00p-1022 -0x0.ffp-1022", "%.2La %.2La", high, -high);
#elif LDBL_TYPE == LDBL_IS_F80
  SNPRINTF_TEST("0x0.000000000000001p-16385", "%La", LDBL_TRUE_MIN);
  long double high = nexttowardl(LDBL_MIN, 0.0L);
  SNPRINTF_TEST("0x7.fffffffffffffffp-16385", "%La", high);

  ASSERT_EQ(0, fesetround(FE_DOWNWARD));
  SNPRINTF_TEST("0x7.ffp-16385 -0x8.00p-16385", "%.2La %.2La", high, -high);
  ASSERT_EQ(0, fesetround(FE_TONEAREST));
  SNPRINTF_TEST("0x8.00p-16385 -0x8.00p-16385", "%.2La %.2La", high, -high);
  ASSERT_EQ(0, fesetround(FE_TOWARDZERO));
  SNPRINTF_TEST("0x7.ffp-16385 -0x7.ffp-16385", "%.2La %.2La", high, -high);
  ASSERT_EQ(0, fesetround(FE_UPWARD));
  SNPRINTF_TEST("0x8.00p-16385 -0x7.ffp-16385", "%.2La %.2La", high, -high);
#elif LDBL_TYPE == LDBL_IS_F128
  SNPRINTF_TEST(
    "0x0.0000000000000000000000000001p-16382", "%La", LDBL_TRUE_MIN);
  long double high = nexttowardl(LDBL_MIN, 0.0L);
  SNPRINTF_TEST("0x0.ffffffffffffffffffffffffffffp-16382", "%La", high);

  ASSERT_EQ(0, fesetround(FE_DOWNWARD));
  SNPRINTF_TEST("0x0.ffp-16382 -0x1.00p-16382", "%.2La %.2La", high, -high);
  ASSERT_EQ(0, fesetround(FE_TONEAREST));
  SNPRINTF_TEST("0x1.00p-16382 -0x1.00p-16382", "%.2La %.2La", high, -high);
  ASSERT_EQ(0, fesetround(FE_TOWARDZERO));
  SNPRINTF_TEST("0x0.ffp-16382 -0x0.ffp-16382", "%.2La %.2La", high, -high);
  ASSERT_EQ(0, fesetround(FE_UPWARD));
  SNPRINTF_TEST("0x1.00p-16382 -0x0.ffp-16382", "%.2La %.2La", high, -high);
#else
#error "Unknown floating point type"
#endif
}
#endif

TEST(sprintf, example)
{
  REVERT_TO_C_LOCALE();

  char buf[10];
  ASSERT_EQ(5, rs_sprintf(buf, "Hello"));
  ASSERT_EQ(9, rs_sprintf(buf, "%d + %d = %d", 1, 1, 2));
}
