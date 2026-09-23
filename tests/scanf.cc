#include "common.h"
#include "common_float.h"
#include "common_locale.h"
#include <cfenv>
#include <cmath>
#include <gmock/gmock.h>
#include <gtest/gtest.h>
#include <string_view>

extern "C"
{
  int rs_sscanf(const char* buffer, const char* format, ...);
  int rs_swscanf(const wchar_t* buffer, const wchar_t* format, ...);
}

/*-
 * Copyright (c) 2023 Dag-Erling Smørgrav
 *
 * SPDX-License-Identifier: BSD-2-Clause
 */

// Wide character stuff taken from there:
// https://github.com/SibiSiddharthan/windows-libc/blob/main/tests/stdio/test-scanf.c

template<typename Char, typename = void>
class sscanf_test;

template<typename Char>
class sscanf_test<Char>
{
public:
  std::basic_string_view<Char> input;
  struct
  {
    int ret, val, len;
  } b, o, d, x, i, u;
};

// clang-format off

sscanf_test<char> narrow_sscanf_tests[] = {
  // input        binary          octal           decimal         hexadecimal     automatic       unsigned
  { "0",    { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "1",    { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 }, },
  { "2",    { 0,   0, 0 },  { 1,   2, 1 },  { 1,   2, 1 },  { 1,   2, 1 },  { 1,   2, 1 },  { 1,   2, 1 }, },
  { "3",    { 0,   0, 0 },  { 1,   3, 1 },  { 1,   3, 1 },  { 1,   3, 1 },  { 1,   3, 1 },  { 1,   3, 1 }, },
  { "4",    { 0,   0, 0 },  { 1,   4, 1 },  { 1,   4, 1 },  { 1,   4, 1 },  { 1,   4, 1 },  { 1,   4, 1 }, },
  { "5",    { 0,   0, 0 },  { 1,   5, 1 },  { 1,   5, 1 },  { 1,   5, 1 },  { 1,   5, 1 },  { 1,   5, 1 }, },
  { "6",    { 0,   0, 0 },  { 1,   6, 1 },  { 1,   6, 1 },  { 1,   6, 1 },  { 1,   6, 1 },  { 1,   6, 1 }, },
  { "7",    { 0,   0, 0 },  { 1,   7, 1 },  { 1,   7, 1 },  { 1,   7, 1 },  { 1,   7, 1 },  { 1,   7, 1 }, },
  { "8",    { 0,   0, 0 },  { 0,   0, 0 },  { 1,   8, 1 },  { 1,   8, 1 },  { 1,   8, 1 },  { 1,   8, 1 }, },
  { "9",    { 0,   0, 0 },  { 0,   0, 0 },  { 1,   9, 1 },  { 1,   9, 1 },  { 1,   9, 1 },  { 1,   9, 1 }, },
  { "A",    { 0,   0, 0 },  { 0,   0, 0 },  { 0,   0, 0 },  { 1,  10, 1 },  { 0,   0, 0 },  { 0,   0, 0 }, },
  { "B",    { 0,   0, 0 },  { 0,   0, 0 },  { 0,   0, 0 },  { 1,  11, 1 },  { 0,   0, 0 },  { 0,   0, 0 }, },
  { "C",    { 0,   0, 0 },  { 0,   0, 0 },  { 0,   0, 0 },  { 1,  12, 1 },  { 0,   0, 0 },  { 0,   0, 0 }, },
  { "D",    { 0,   0, 0 },  { 0,   0, 0 },  { 0,   0, 0 },  { 1,  13, 1 },  { 0,   0, 0 },  { 0,   0, 0 }, },
  { "E",    { 0,   0, 0 },  { 0,   0, 0 },  { 0,   0, 0 },  { 1,  14, 1 },  { 0,   0, 0 },  { 0,   0, 0 }, },
  { "F",    { 0,   0, 0 },  { 0,   0, 0 },  { 0,   0, 0 },  { 1,  15, 1 },  { 0,   0, 0 },  { 0,   0, 0 }, },
  { "X",    { 0,   0, 0 },  { 0,   0, 0 },  { 0,   0, 0 },  { 0,   0, 0 },  { 0,   0, 0 },  { 0,   0, 0 }, },
  { "a",    { 0,   0, 0 },  { 0,   0, 0 },  { 0,   0, 0 },  { 1,  10, 1 },  { 0,   0, 0 },  { 0,   0, 0 }, },
  { "b",    { 0,   0, 0 },  { 0,   0, 0 },  { 0,   0, 0 },  { 1,  11, 1 },  { 0,   0, 0 },  { 0,   0, 0 }, },
  { "c",    { 0,   0, 0 },  { 0,   0, 0 },  { 0,   0, 0 },  { 1,  12, 1 },  { 0,   0, 0 },  { 0,   0, 0 }, },
  { "d",    { 0,   0, 0 },  { 0,   0, 0 },  { 0,   0, 0 },  { 1,  13, 1 },  { 0,   0, 0 },  { 0,   0, 0 }, },
  { "e",    { 0,   0, 0 },  { 0,   0, 0 },  { 0,   0, 0 },  { 1,  14, 1 },  { 0,   0, 0 },  { 0,   0, 0 }, },
  { "f",    { 0,   0, 0 },  { 0,   0, 0 },  { 0,   0, 0 },  { 1,  15, 1 },  { 0,   0, 0 },  { 0,   0, 0 }, },
  { "x",    { 0,   0, 0 },  { 0,   0, 0 },  { 0,   0, 0 },  { 0,   0, 0 },  { 0,   0, 0 },  { 0,   0, 0 }, },
  { "00",   { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 }, },
  { "01",   { 1,   1, 2 },  { 1,   1, 2 },  { 1,   1, 2 },  { 1,   1, 2 },  { 1,   1, 2 },  { 1,   1, 2 }, },
  { "02",   { 1,   0, 1 },  { 1,   2, 2 },  { 1,   2, 2 },  { 1,   2, 2 },  { 1,   2, 2 },  { 1,   2, 2 }, },
  { "03",   { 1,   0, 1 },  { 1,   3, 2 },  { 1,   3, 2 },  { 1,   3, 2 },  { 1,   3, 2 },  { 1,   3, 2 }, },
  { "04",   { 1,   0, 1 },  { 1,   4, 2 },  { 1,   4, 2 },  { 1,   4, 2 },  { 1,   4, 2 },  { 1,   4, 2 }, },
  { "05",   { 1,   0, 1 },  { 1,   5, 2 },  { 1,   5, 2 },  { 1,   5, 2 },  { 1,   5, 2 },  { 1,   5, 2 }, },
  { "06",   { 1,   0, 1 },  { 1,   6, 2 },  { 1,   6, 2 },  { 1,   6, 2 },  { 1,   6, 2 },  { 1,   6, 2 }, },
  { "07",   { 1,   0, 1 },  { 1,   7, 2 },  { 1,   7, 2 },  { 1,   7, 2 },  { 1,   7, 2 },  { 1,   7, 2 }, },
  { "08",   { 1,   0, 1 },  { 1,   0, 1 },  { 1,   8, 2 },  { 1,   8, 2 },  { 1,   0, 1 },  { 1,   8, 2 }, },
  { "09",   { 1,   0, 1 },  { 1,   0, 1 },  { 1,   9, 2 },  { 1,   9, 2 },  { 1,   0, 1 },  { 1,   9, 2 }, },
  { "0A",   { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  10, 2 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0B",   { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  11, 2 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0C",   { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  12, 2 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0D",   { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  13, 2 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0E",   { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  14, 2 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0F",   { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  15, 2 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0X",   { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0a",   { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  10, 2 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0b",   { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  11, 2 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0c",   { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  12, 2 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0d",   { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  13, 2 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0e",   { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  14, 2 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0f",   { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  15, 2 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0x",   { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "000",  { 1,   0, 3 },  { 1,   0, 3 },  { 1,   0, 3 },  { 1,   0, 3 },  { 1,   0, 3 },  { 1,   0, 3 }, },
  { "001",  { 1,   1, 3 },  { 1,   1, 3 },  { 1,   1, 3 },  { 1,   1, 3 },  { 1,   1, 3 },  { 1,   1, 3 }, },
  { "002",  { 1,   0, 2 },  { 1,   2, 3 },  { 1,   2, 3 },  { 1,   2, 3 },  { 1,   2, 3 },  { 1,   2, 3 }, },
  { "003",  { 1,   0, 2 },  { 1,   3, 3 },  { 1,   3, 3 },  { 1,   3, 3 },  { 1,   3, 3 },  { 1,   3, 3 }, },
  { "004",  { 1,   0, 2 },  { 1,   4, 3 },  { 1,   4, 3 },  { 1,   4, 3 },  { 1,   4, 3 },  { 1,   4, 3 }, },
  { "005",  { 1,   0, 2 },  { 1,   5, 3 },  { 1,   5, 3 },  { 1,   5, 3 },  { 1,   5, 3 },  { 1,   5, 3 }, },
  { "006",  { 1,   0, 2 },  { 1,   6, 3 },  { 1,   6, 3 },  { 1,   6, 3 },  { 1,   6, 3 },  { 1,   6, 3 }, },
  { "007",  { 1,   0, 2 },  { 1,   7, 3 },  { 1,   7, 3 },  { 1,   7, 3 },  { 1,   7, 3 },  { 1,   7, 3 }, },
  { "008",  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   8, 3 },  { 1,   8, 3 },  { 1,   0, 2 },  { 1,   8, 3 }, },
  { "009",  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   9, 3 },  { 1,   9, 3 },  { 1,   0, 2 },  { 1,   9, 3 }, },
  { "00A",  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 },  { 1,  10, 3 },  { 1,   0, 2 },  { 1,   0, 2 }, },
  { "00B",  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 },  { 1,  11, 3 },  { 1,   0, 2 },  { 1,   0, 2 }, },
  { "00C",  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 },  { 1,  12, 3 },  { 1,   0, 2 },  { 1,   0, 2 }, },
  { "00D",  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 },  { 1,  13, 3 },  { 1,   0, 2 },  { 1,   0, 2 }, },
  { "00E",  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 },  { 1,  14, 3 },  { 1,   0, 2 },  { 1,   0, 2 }, },
  { "00F",  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 },  { 1,  15, 3 },  { 1,   0, 2 },  { 1,   0, 2 }, },
  { "00X",  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 }, },
  { "00a",  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 },  { 1,  10, 3 },  { 1,   0, 2 },  { 1,   0, 2 }, },
  { "00b",  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 },  { 1,  11, 3 },  { 1,   0, 2 },  { 1,   0, 2 }, },
  { "00c",  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 },  { 1,  12, 3 },  { 1,   0, 2 },  { 1,   0, 2 }, },
  { "00d",  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 },  { 1,  13, 3 },  { 1,   0, 2 },  { 1,   0, 2 }, },
  { "00e",  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 },  { 1,  14, 3 },  { 1,   0, 2 },  { 1,   0, 2 }, },
  { "00f",  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 },  { 1,  15, 3 },  { 1,   0, 2 },  { 1,   0, 2 }, },
  { "00x",  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 },  { 1,   0, 2 }, },
  { "10",   { 1,   2, 2 },  { 1,   8, 2 },  { 1,  10, 2 },  { 1,  16, 2 },  { 1,  10, 2 },  { 1,  10, 2 }, },
  { "11",   { 1,   3, 2 },  { 1,   9, 2 },  { 1,  11, 2 },  { 1,  17, 2 },  { 1,  11, 2 },  { 1,  11, 2 }, },
  { "12",   { 1,   1, 1 },  { 1,  10, 2 },  { 1,  12, 2 },  { 1,  18, 2 },  { 1,  12, 2 },  { 1,  12, 2 }, },
  { "13",   { 1,   1, 1 },  { 1,  11, 2 },  { 1,  13, 2 },  { 1,  19, 2 },  { 1,  13, 2 },  { 1,  13, 2 }, },
  { "14",   { 1,   1, 1 },  { 1,  12, 2 },  { 1,  14, 2 },  { 1,  20, 2 },  { 1,  14, 2 },  { 1,  14, 2 }, },
  { "15",   { 1,   1, 1 },  { 1,  13, 2 },  { 1,  15, 2 },  { 1,  21, 2 },  { 1,  15, 2 },  { 1,  15, 2 }, },
  { "16",   { 1,   1, 1 },  { 1,  14, 2 },  { 1,  16, 2 },  { 1,  22, 2 },  { 1,  16, 2 },  { 1,  16, 2 }, },
  { "17",   { 1,   1, 1 },  { 1,  15, 2 },  { 1,  17, 2 },  { 1,  23, 2 },  { 1,  17, 2 },  { 1,  17, 2 }, },
  { "18",   { 1,   1, 1 },  { 1,   1, 1 },  { 1,  18, 2 },  { 1,  24, 2 },  { 1,  18, 2 },  { 1,  18, 2 }, },
  { "19",   { 1,   1, 1 },  { 1,   1, 1 },  { 1,  19, 2 },  { 1,  25, 2 },  { 1,  19, 2 },  { 1,  19, 2 }, },
  { "1A",   { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 },  { 1,  26, 2 },  { 1,   1, 1 },  { 1,   1, 1 }, },
  { "1B",   { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 },  { 1,  27, 2 },  { 1,   1, 1 },  { 1,   1, 1 }, },
  { "1C",   { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 },  { 1,  28, 2 },  { 1,   1, 1 },  { 1,   1, 1 }, },
  { "1D",   { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 },  { 1,  29, 2 },  { 1,   1, 1 },  { 1,   1, 1 }, },
  { "1E",   { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 },  { 1,  30, 2 },  { 1,   1, 1 },  { 1,   1, 1 }, },
  { "1F",   { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 },  { 1,  31, 2 },  { 1,   1, 1 },  { 1,   1, 1 }, },
  { "1X",   { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 }, },
  { "1a",   { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 },  { 1,  26, 2 },  { 1,   1, 1 },  { 1,   1, 1 }, },
  { "1b",   { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 },  { 1,  27, 2 },  { 1,   1, 1 },  { 1,   1, 1 }, },
  { "1c",   { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 },  { 1,  28, 2 },  { 1,   1, 1 },  { 1,   1, 1 }, },
  { "1d",   { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 },  { 1,  29, 2 },  { 1,   1, 1 },  { 1,   1, 1 }, },
  { "1e",   { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 },  { 1,  30, 2 },  { 1,   1, 1 },  { 1,   1, 1 }, },
  { "1f",   { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 },  { 1,  31, 2 },  { 1,   1, 1 },  { 1,   1, 1 }, },
  { "1x",   { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 },  { 1,   1, 1 }, },
  { "0b0",  { 1,   0, 3 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1, 176, 3 },  { 1,   0, 3 },  { 1,   0, 1 }, },
  { "0b1",  { 1,   1, 3 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1, 177, 3 },  { 1,   1, 3 },  { 1,   0, 1 }, },
  { "0b2",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1, 178, 3 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0b3",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1, 179, 3 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0b4",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1, 180, 3 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0b5",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1, 181, 3 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0b6",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1, 182, 3 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0b7",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1, 183, 3 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0b8",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1, 184, 3 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0b9",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1, 185, 3 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0bA",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1, 186, 3 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0bB",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1, 187, 3 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0bC",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1, 188, 3 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0bD",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1, 189, 3 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0bE",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1, 190, 3 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0bF",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1, 191, 3 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0bX",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  11, 2 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0ba",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1, 186, 3 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0bb",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1, 187, 3 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0bc",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1, 188, 3 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0bd",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1, 189, 3 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0be",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1, 190, 3 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0bf",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1, 191, 3 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0bx",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  11, 2 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0x0",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 3 },  { 1,   0, 3 },  { 1,   0, 1 }, },
  { "0x1",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   1, 3 },  { 1,   1, 3 },  { 1,   0, 1 }, },
  { "0x2",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   2, 3 },  { 1,   2, 3 },  { 1,   0, 1 }, },
  { "0x3",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   3, 3 },  { 1,   3, 3 },  { 1,   0, 1 }, },
  { "0x4",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   4, 3 },  { 1,   4, 3 },  { 1,   0, 1 }, },
  { "0x5",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   5, 3 },  { 1,   5, 3 },  { 1,   0, 1 }, },
  { "0x6",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   6, 3 },  { 1,   6, 3 },  { 1,   0, 1 }, },
  { "0x7",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   7, 3 },  { 1,   7, 3 },  { 1,   0, 1 }, },
  { "0x8",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   8, 3 },  { 1,   8, 3 },  { 1,   0, 1 }, },
  { "0x9",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   9, 3 },  { 1,   9, 3 },  { 1,   0, 1 }, },
  { "0xA",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  10, 3 },  { 1,  10, 3 },  { 1,   0, 1 }, },
  { "0xB",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  11, 3 },  { 1,  11, 3 },  { 1,   0, 1 }, },
  { "0xC",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  12, 3 },  { 1,  12, 3 },  { 1,   0, 1 }, },
  { "0xD",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  13, 3 },  { 1,  13, 3 },  { 1,   0, 1 }, },
  { "0xE",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  14, 3 },  { 1,  14, 3 },  { 1,   0, 1 }, },
  { "0xF",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  15, 3 },  { 1,  15, 3 },  { 1,   0, 1 }, },
  { "0xX",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 }, },
  { "0xa",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  10, 3 },  { 1,  10, 3 },  { 1,   0, 1 }, },
  { "0xb",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  11, 3 },  { 1,  11, 3 },  { 1,   0, 1 }, },
  { "0xc",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  12, 3 },  { 1,  12, 3 },  { 1,   0, 1 }, },
  { "0xd",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  13, 3 },  { 1,  13, 3 },  { 1,   0, 1 }, },
  { "0xe",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  14, 3 },  { 1,  14, 3 },  { 1,   0, 1 }, },
  { "0xf",  { 1,   0, 1 },  { 1,   0, 1 },  { 1,   0, 1 },  { 1,  15, 3 },  { 1,  15, 3 },  { 1,   0, 1 }, },
  // terminator
  { "" }
};

// clang-format on

#define SSCANF_TEST(string, format, expret, expval, explen)                    \
  do {                                                                         \
    int ret = 0, val = 0, len = 0;                                             \
    ret = rs_sscanf(string, format "%n", &val, &len);                          \
    ASSERT_EQ(expret, ret);                                                    \
    if (expret && ret) {                                                       \
      ASSERT_EQ(expval, val);                                                  \
      ASSERT_EQ(explen, len);                                                  \
    }                                                                          \
  } while (0)

TEST(sscanf, integers)
{
  const sscanf_test<char>* stc;
  char input[16];

  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C"), "C");

  memset(input, '\0', sizeof(input));

  // %b specifier
  for (stc = narrow_sscanf_tests; !stc->input.empty(); stc++) {
    strcpy(input + 1, stc->input.data());
    SSCANF_TEST(input + 1, "%b", stc->b.ret, stc->b.val, stc->b.len);
    input[0] = '+';
    SSCANF_TEST(
      input, "%b", stc->b.ret, stc->b.val, stc->b.len ? stc->b.len + 1 : 0);
    input[0] = '-';
    SSCANF_TEST(
      input, "%b", stc->b.ret, -stc->b.val, stc->b.len ? stc->b.len + 1 : 0);
  }

  memset(input, '\0', sizeof(input));

  // %o specifier
  for (stc = narrow_sscanf_tests; !stc->input.empty(); stc++) {
    strcpy(input + 1, stc->input.data());
    SSCANF_TEST(input + 1, "%o", stc->o.ret, stc->o.val, stc->o.len);
    input[0] = '+';
    SSCANF_TEST(
      input, "%o", stc->o.ret, stc->o.val, stc->o.len ? stc->o.len + 1 : 0);
    input[0] = '-';
    SSCANF_TEST(
      input, "%o", stc->o.ret, -stc->o.val, stc->o.len ? stc->o.len + 1 : 0);
  }

  memset(input, '\0', sizeof(input));

  // %u specifier
  for (stc = narrow_sscanf_tests; !stc->input.empty(); stc++) {
    strcpy(input + 1, stc->input.data());
    SSCANF_TEST(input + 1, "%u", stc->u.ret, stc->u.val, stc->u.len);
    input[0] = '+';
    SSCANF_TEST(
      input, "%u", stc->u.ret, stc->u.val, stc->u.len ? stc->u.len + 1 : 0);
    input[0] = '-';
    SSCANF_TEST(
      input, "%u", stc->u.ret, -stc->u.val, stc->u.len ? stc->u.len + 1 : 0);
  }

  memset(input, '\0', sizeof(input));

  // %x specifier
  for (stc = narrow_sscanf_tests; !stc->input.empty(); stc++) {
    strcpy(input + 1, stc->input.data());
    SSCANF_TEST(input + 1, "%x", stc->x.ret, stc->x.val, stc->x.len);
    input[0] = '+';
    SSCANF_TEST(
      input, "%x", stc->x.ret, stc->x.val, stc->x.len ? stc->x.len + 1 : 0);
    input[0] = '-';
    SSCANF_TEST(
      input, "%x", stc->x.ret, -stc->x.val, stc->x.len ? stc->x.len + 1 : 0);
  }

  memset(input, '\0', sizeof(input));

  // %d specifier
  for (stc = narrow_sscanf_tests; !stc->input.empty(); stc++) {
    strcpy(input + 1, stc->input.data());
    SSCANF_TEST(input + 1, "%d", stc->d.ret, stc->d.val, stc->d.len);
    input[0] = '+';
    SSCANF_TEST(
      input, "%d", stc->d.ret, stc->d.val, stc->d.len ? stc->d.len + 1 : 0);
    input[0] = '-';
    SSCANF_TEST(
      input, "%d", stc->d.ret, -stc->d.val, stc->d.len ? stc->d.len + 1 : 0);
  }

  memset(input, '\0', sizeof(input));

  // %i specifier
  for (stc = narrow_sscanf_tests; !stc->input.empty(); stc++) {
    strcpy(input + 1, stc->input.data());
    SSCANF_TEST(input + 1, "%i", stc->i.ret, stc->i.val, stc->i.len);
    input[0] = '+';
    SSCANF_TEST(
      input, "%i", stc->i.ret, stc->i.val, stc->i.len ? stc->i.len + 1 : 0);
    input[0] = '-';
    SSCANF_TEST(
      input, "%i", stc->i.ret, -stc->i.val, stc->i.len ? stc->i.len + 1 : 0);
  }

  int a = 0, b = 0, c = 0;
  char d = 0;

  ASSERT_EQ(4, rs_sscanf("3.1415", "%d%c%2d%d", &a, &d, &b, &c));
  ASSERT_EQ(3, a);
  ASSERT_EQ(14, b);
  ASSERT_EQ(15, c);
  ASSERT_EQ('.', d);

  // Length modifiers
  int ret_val;
  uintmax_t max_result = 0;
  int int_result = 0;
  char char_result = 0;

  ret_val = rs_sscanf("123", "%ju", &max_result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(max_result, uintmax_t(123));

  ret_val =
    rs_sscanf("999999999999999999999999999999999999", "%ju", &max_result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(max_result, std::numeric_limits<uintmax_t>::max());

  ret_val =
    rs_sscanf("-999999999999999999999999999999999999", "%ju", &max_result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(max_result, std::numeric_limits<uintmax_t>::max());

  ret_val = rs_sscanf("-18446744073709551616", "%ju", &max_result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(max_result, std::numeric_limits<uintmax_t>::max());

  ret_val = rs_sscanf("-1", "%ju", &max_result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(max_result, uintmax_t(-1));

  ret_val = rs_sscanf("-1", "%u", &int_result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(int_result, -1);

  max_result = 0xff00ff00ff00ff00;
  char_result = 0x6f;

  ret_val = rs_sscanf("8589967360", "%d", &int_result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(int_result, int(8589967360));

  ASSERT_EQ(max_result, uintmax_t(0xff00ff00ff00ff00));
  ASSERT_EQ(char_result, char(0x6f));

  ret_val = rs_sscanf("-8589967360", "%d", &int_result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(int_result, int(-8589967360));
  ASSERT_EQ(max_result, uintmax_t(0xff00ff00ff00ff00));
  ASSERT_EQ(char_result, char(0x6f));

  ret_val = rs_sscanf("25", "%hhd", &char_result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(char_result, char(25));
}

#define eq(type, a, b) _eq(type##_EPSILON, (a), (b))
static inline bool
_eq(long double epsilon, long double a, long double b)
{
  long double delta;

  delta = fabsl(a - b);
  return (delta <= epsilon);
}

TEST(sscanf, floats)
{
  char buf[128];
  long double ld = 0.0;
  double d = 0.0;
  float f = 0.0;

  buf[0] = '\0';
  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C"), "C");

  ASSERT_EQ(1, rs_sscanf("3.141592", "%e", &f));
  ASSERT_TRUE(eq(FLT, f, 3.141592));

  ASSERT_EQ(1, rs_sscanf("3.141592653589793", "%lf", &d));
  ASSERT_TRUE(eq(DBL, d, 3.141592653589793));

  ASSERT_EQ(1, rs_sscanf("1.234568e+06", "%E", &f));
  ASSERT_TRUE(eq(FLT, f, 1.234568e+06));

  ASSERT_EQ(1, rs_sscanf("-1.234568e6", "%lF", &d));
  ASSERT_TRUE(eq(DBL, d, -1.234568e6));

  ASSERT_EQ(1, rs_sscanf("+1.234568e-52", "%LG", &ld));
  ASSERT_TRUE(eq(LDBL, ld, 1.234568e-52L));

  ASSERT_EQ(1, rs_sscanf("0.1", "%la", &d));
  ASSERT_TRUE(eq(DBL, d, 0.1));

  ASSERT_EQ(1, rs_sscanf("00.2", "%lA", &d));
  ASSERT_TRUE(eq(DBL, d, 0.2));

  ASSERT_EQ(2, rs_sscanf("123456", "%5le%s", &d, buf));
  ASSERT_TRUE(eq(DBL, d, 12345.));
  ASSERT_TRUE(strcmp(buf, "6") == 0);

  ASSERT_EQ(1, rs_sscanf("1.0Q", "%*5le%s", buf));
  ASSERT_TRUE(strcmp(buf, "Q") == 0);

  ASSERT_EQ(2, rs_sscanf("-1.23e", "%e%s", &f, buf));
  ASSERT_TRUE(eq(FLT, f, -1.23));
  ASSERT_TRUE(strcmp(buf, "e") == 0);

  ASSERT_EQ(2, rs_sscanf("1.25e+", "%le%s", &d, buf));
  ASSERT_TRUE(eq(DBL, d, 1.25));
  ASSERT_TRUE(strcmp(buf, "e+") == 0);

  ASSERT_EQ(2, rs_sscanf("1.23E4E5", "%le%s", &d, buf));
  ASSERT_TRUE(eq(DBL, d, 1.23e4));
  ASSERT_TRUE(strcmp(buf, "E5") == 0);

  ASSERT_EQ(1, rs_sscanf("12e6", "%le", &d));
  ASSERT_TRUE(eq(DBL, d, 12e6));

  ASSERT_EQ(2, rs_sscanf("1.a", "%le%s", &d, buf));
  ASSERT_TRUE(eq(DBL, d, 1.0));
  ASSERT_TRUE(strcmp(buf, "a") == 0);

  ASSERT_EQ(2, rs_sscanf(".0p4", "%le%s", &d, buf));
  ASSERT_TRUE(eq(DBL, d, 0.0));
  ASSERT_TRUE(strcmp(buf, "p4") == 0);

  d = 0.25;
  ASSERT_EQ(0, rs_sscanf(".", "%le", &d));
  ASSERT_TRUE(d == 0.25);

  ASSERT_EQ(1, rs_sscanf("0x08", "%le", &d));
  ASSERT_TRUE(d == 0x8p0);

  ASSERT_EQ(2, rs_sscanf("0x90a.bcdefP+09a", "%le%s", &d, buf));
  ASSERT_TRUE(d == 0x90a.bcdefp+09);
  ASSERT_TRUE(strcmp(buf, "a") == 0);

  ASSERT_EQ(1, rs_sscanf("3.14159265358979323846", "%Lg", &ld));
  ASSERT_TRUE(eq(LDBL, ld, 3.14159265358979323846L));

  ASSERT_EQ(2, rs_sscanf("  0X.0123456789abcdefffp-3g", "%Le%s", &ld, buf));
  ASSERT_TRUE(ld == 0x0.0123456789abcdefffp-3L);
  ASSERT_TRUE(strcmp(buf, "g") == 0);

  ASSERT_EQ(2, rs_sscanf("0xg", "%le%s", &d, buf));
  ASSERT_TRUE(d == 0.0);
  ASSERT_TRUE(strcmp(buf, "xg") == 0);

  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "ru_RU.UTF-8"),
               "ru_RU.UTF-8"); /* decimalpoint==, */

  ASSERT_EQ(2, rs_sscanf("1.23", "%le%s", &d, buf));
  ASSERT_TRUE(d == 1.0);
  ASSERT_TRUE(strcmp(buf, ".23") == 0);

  ASSERT_EQ(1, rs_sscanf("1,23", "%le", &d));
  ASSERT_TRUE(d == 1.23);

  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C"), "C");

  int ret_val;
  float result = 0;

  float inf = std::numeric_limits<float>::infinity();

  ret_val = rs_sscanf("123", "%f", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, 123.0f);

  ret_val = rs_sscanf("456.1", "%a", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, 456.1f);

  ret_val = rs_sscanf("0x789.ap0", "%e", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, 0x789.ap0f);

  ret_val = rs_sscanf("0x.8", "%e", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, 0x0.8p0f);

  ret_val = rs_sscanf("0x8.", "%e", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, 0x8.0p0f);

  ret_val = rs_sscanf("+12.0e1", "%g", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, 12.0e1f);

  ret_val = rs_sscanf("inf", "%F", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, inf);

  ret_val = rs_sscanf("NaN", "%A", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_TRUE(std::isnan(result));

  ret_val = rs_sscanf("-InFiNiTy", "%E", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, -inf);

  ret_val = rs_sscanf("1e10", "%G", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, 1e10f);

  ret_val = rs_sscanf(".1", "%G", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, 0.1f);

  ret_val = rs_sscanf("1.", "%G", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, 1.0f);

  ret_val = rs_sscanf("0", "%f", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, 0.0f);

  ret_val = rs_sscanf("Not a float", "%f", &result);
  ASSERT_EQ(ret_val, 0);

  double d_result = 0;
  long double ld_result = 0;

  double d_inf = std::numeric_limits<double>::infinity();

  ret_val = rs_sscanf("123", "%lf", &d_result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(d_result, 123.0);

  ret_val = rs_sscanf("456.1", "%La", &ld_result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(ld_result, 456.1L);

  ret_val = rs_sscanf("inf", "%le", &d_result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(d_result, d_inf);

  ret_val = rs_sscanf("nan", "%Lg", &ld_result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_TRUE(std::isnan(ld_result));

  ret_val = rs_sscanf("1e-300", "%lF", &d_result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(d_result, 1e-300);

  ret_val = rs_sscanf("1.0e600", "%LA", &ld_result);
  ASSERT_EQ(ret_val, 1);
// 1e600 may be larger than the maximum long double (if long double is double).
// In that case both of these should be evaluated as inf.
#if LDBL_TYPE == LDBL_IS_F64 || LDBL_TYPE == LDBL_IS_PPC_DOUBLE
  ASSERT_EQ(ld_result, d_inf);
#else
  ASSERT_EQ(ld_result, 1.0e600L);
#endif

  ret_val = rs_sscanf("123456789012345678901234567890.0", "%f", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, 123456789012345678901234567890.0f);

  ret_val = rs_sscanf(
    "123456789012345678901234567890123456789012345678901234567890.000",
    "%la",
    &d_result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(d_result,
            123456789012345678901234567890123456789012345678901234567890.000);

  ret_val = rs_sscanf(
    "123456789012345678901234567890123456789012345678901234567890"
    "123456789012345678901234567890123456789012345678901234567890.0000000",
    "%le",
    &d_result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(
    d_result,
    123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890.0000000);

  ret_val = rs_sscanf("10000000000000000000000000000000"
                      "00000000000000000000000000000000"
                      "00000000000000000000000000000000"
                      "00000000000000000000000000000000"
                      "00000000000000000000000000000000"
                      "00000000000000000000000000000000"
                      "00000000000000000000000000000000"
                      "00000000000000000000000000000000",
                      "%lf",
                      &d_result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(d_result, 1e255);

  ret_val = rs_sscanf("10000000000000000000000000000000"
                      "00000000000000000000000000000000"
                      "00000000000000000000000000000000"
                      "00000000000000000000000000000000"
                      "00000000000000000000000000000000"
                      "00000000000000000000000000000000"
                      "00000000000000000000000000000000"
                      "00000000000000000000000000000000"
                      "00000000000000000000000000000000",
                      "%lf",
                      &d_result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(d_result, 1e287);

  ret_val = rs_sscanf("123", "%3f", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, 123.0f);

  ret_val = rs_sscanf("123", "%5f", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, 123.0f);

  ret_val = rs_sscanf("456", "%1f", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, 4.0f);

  ret_val = rs_sscanf("-789", "%1f", &result);
  ASSERT_EQ(ret_val, 0);

  ret_val = rs_sscanf("-123", "%2f", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, -1.0f);

  ret_val = rs_sscanf("inf", "%2f", &result);
  ASSERT_EQ(ret_val, 0);

  ret_val = rs_sscanf("nan", "%1f", &result);
  ASSERT_EQ(ret_val, 0);

  ret_val = rs_sscanf("-inf", "%3f", &result);
  ASSERT_EQ(ret_val, 0);

  ret_val = rs_sscanf("-nan", "%3f", &result);
  ASSERT_EQ(ret_val, 0);

  ret_val = rs_sscanf("infinite", "%3f", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, inf);

  ret_val = rs_sscanf("-infinite", "%4f", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, -inf);

  ret_val = rs_sscanf("01", "%1f", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, 0.0f);

  ret_val = rs_sscanf("0x1", "%2f", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, 0.0f);

  ret_val = rs_sscanf("100e", "%4f", &result);
  ASSERT_EQ(ret_val, 0);

  ret_val = rs_sscanf("100e+10", "%5f", &result);
  ASSERT_EQ(ret_val, 0);

  ret_val = rs_sscanf("100e10", "%5f", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, 100e1f);
}

TEST(sscanf, floats_inf_nan)
{
  char buf[128];
  long double ld = 0.0;
  double d = 0.0;
  float f = 0.0;

  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C"), "C");

  ASSERT_EQ(1, rs_sscanf("-Inf", "%le", &d));
  ASSERT_TRUE(d < 0.0 && std::isinf(d));

  ASSERT_EQ(2, rs_sscanf("iNfInItY and beyond", "%le%s", &d, buf));
  ASSERT_TRUE(d > 0.0 && std::isinf(d));
  ASSERT_TRUE(strcmp(buf, " and beyond"));

  ASSERT_EQ(1, rs_sscanf("NaN", "%le", &d));
  ASSERT_TRUE(std::isnan(d));

  ASSERT_EQ(2, rs_sscanf("NAN(123Y", "%le%s", &d, buf));
  ASSERT_TRUE(std::isnan(d));
  ASSERT_TRUE(strcmp(buf, "(123Y") == 0);

  ASSERT_EQ(2, rs_sscanf("nan(f00f)plugh", "%le%s", &d, buf));
  ASSERT_TRUE(std::isnan(d));
  ASSERT_TRUE(strcmp(buf, "plugh") == 0);

  ASSERT_EQ(1, rs_sscanf("-nan", "%le", &d));
  ASSERT_TRUE(std::isnan(d));

  ASSERT_EQ(1, rs_sscanf("NaN", "%e", &f));
  ASSERT_EQ(1, rs_sscanf("nan", "%le", &d));
  ASSERT_EQ(1, rs_sscanf("nan", "%Le", &ld));
  ASSERT_EQ(0, feclearexcept(FE_ALL_EXCEPT));
  ASSERT_TRUE(f != f);
  ASSERT_TRUE(d != d);
  ASSERT_TRUE(ld != ld);
  ASSERT_TRUE(fetestexcept(FE_INVALID) == 0);
  ASSERT_EQ(1, rs_sscanf("nan(1234)", "%e", &f));
  ASSERT_EQ(1, rs_sscanf("nan(1234)", "%le", &d));
  ASSERT_EQ(1, rs_sscanf("nan(1234)", "%Le", &ld));
  ASSERT_EQ(0, feclearexcept(FE_ALL_EXCEPT));
  ASSERT_TRUE(f != f);
  ASSERT_TRUE(d != d);
  ASSERT_TRUE(ld != ld);
  /* POSIX says we should only generate quiet NaNs. */
  ASSERT_TRUE(fetestexcept(FE_INVALID) == 0);
}

TEST(sscanf, floats_rounding)
{
  long double ld = 0.0;
  double d = 0.0;

  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C"), "C");

  ASSERT_EQ(0, fesetround(FE_DOWNWARD));

  ASSERT_EQ(1, rs_sscanf("1.999999999999999999999999999999999", "%le", &d));
  ASSERT_TRUE(d < 2.0);
  ASSERT_EQ(1, rs_sscanf("0x1.ffffffffffffffp0", "%le", &d));
  ASSERT_TRUE(d < 2.0);
  ASSERT_EQ(1, rs_sscanf("1.999999999999999999999999999999999", "%Le", &ld));
  ASSERT_TRUE(ld < 2.0);

  ASSERT_EQ(1, rs_sscanf("1.0571892669084007", "%le", &d));
  ASSERT_TRUE(d == 0x1.0ea3f4af0dc59p0);
  ASSERT_EQ(1, rs_sscanf("-1.0571892669084007", "%le", &d));
  ASSERT_TRUE(d == -0x1.0ea3f4af0dc5ap0);
  ASSERT_EQ(1, rs_sscanf("1.0571892669084010", "%le", &d));
  ASSERT_TRUE(d == 0x1.0ea3f4af0dc5ap0);

  ASSERT_EQ(1, rs_sscanf("0x1.23p-5000", "%le", &d));
  ASSERT_TRUE(d == 0.0);

  ASSERT_EQ(1, rs_sscanf("0x1.2345678p-1050", "%le", &d));
  ASSERT_TRUE(d == 0x1.234567p-1050);

  ASSERT_EQ(0, fesetround(FE_UPWARD));

  ASSERT_EQ(1, rs_sscanf("1.0571892669084007", "%le", &d));
  ASSERT_TRUE(d == 0x1.0ea3f4af0dc5ap0);
  ASSERT_EQ(1, rs_sscanf("-1.0571892669084007", "%le", &d));
  ASSERT_TRUE(d == -0x1.0ea3f4af0dc59p0);
  ASSERT_EQ(1, rs_sscanf("1.0571892669084010", "%le", &d));
  ASSERT_TRUE(d == 0x1.0ea3f4af0dc5bp0);

  ASSERT_EQ(1, rs_sscanf("0x1.23p-5000", "%le", &d));
  ASSERT_TRUE(d == 0x1p-1074);

  ASSERT_EQ(1, rs_sscanf("0x1.2345678p-1050", "%le", &d));
  ASSERT_TRUE(d == 0x1.234568p-1050);

  ASSERT_EQ(0, fesetround(FE_TOWARDZERO));

  ASSERT_EQ(1, rs_sscanf("1.0571892669084007", "%le", &d));
  ASSERT_TRUE(d == 0x1.0ea3f4af0dc59p0);
  ASSERT_EQ(1, rs_sscanf("-1.0571892669084007", "%le", &d));
  ASSERT_TRUE(d == -0x1.0ea3f4af0dc59p0);
  ASSERT_EQ(1, rs_sscanf("1.0571892669084010", "%le", &d));
  ASSERT_TRUE(d == 0x1.0ea3f4af0dc5ap0);

  ASSERT_EQ(1, rs_sscanf("0x1.23p-5000", "%le", &d));
  ASSERT_TRUE(d == 0.0);

  ASSERT_EQ(1, rs_sscanf("0x1.2345678p-1050", "%le", &d));
  ASSERT_TRUE(d == 0x1.234567p-1050);

  ASSERT_EQ(0, fesetround(FE_TONEAREST));

  /* 1.0571892669084007 is slightly closer to 0x1.0ea3f4af0dc59p0 */
  ASSERT_EQ(1, rs_sscanf("1.0571892669084007", "%le", &d));
  ASSERT_TRUE(d == 0x1.0ea3f4af0dc59p0);
  ASSERT_EQ(1, rs_sscanf("-1.0571892669084007", "%le", &d));
  ASSERT_TRUE(d == -0x1.0ea3f4af0dc59p0);
  ASSERT_EQ(1, rs_sscanf("1.0571892669084010", "%le", &d));
  ASSERT_TRUE(d == 0x1.0ea3f4af0dc5bp0);

  ASSERT_EQ(1, rs_sscanf("0x1.23p-5000", "%le", &d));
  ASSERT_TRUE(d == 0.0);

  /* Extra digits in a denormal shouldn't break anything. */
  ASSERT_EQ(1, rs_sscanf("0x1.2345678p-1050", "%le", &d));
  ASSERT_TRUE(d == 0x1.234568p-1050);
}

TEST(sscanf, http_version)
{
  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C"), "C");

  int major, minor;
  ASSERT_EQ(2,
            rs_sscanf("HTTP/1.0", "HTTP/%d.%d%c", &major, &minor, (char*)NULL));
  ASSERT_EQ(1, major);
  ASSERT_EQ(0, minor);
}

TEST(sscanf, numbered_arguments)
{
  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C"), "C");

  int value1, value2;
  ASSERT_EQ(2, rs_sscanf("12345 67890", "%1$d%2$d", &value1, &value2));
  ASSERT_EQ(12345, value1);
  ASSERT_EQ(67890, value2);

  ASSERT_EQ(2, rs_sscanf("23456 78901", "%2$d%1$d", &value1, &value2));
  ASSERT_EQ(78901, value1);
  ASSERT_EQ(23456, value2);
}

TEST(sscanf, whitespace)
{
  char str1[20], str2[20];

  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C"), "C");
  ASSERT_EQ(1, rs_sscanf("Hello\xe2\x80\xa8World", "%s%s", str1, str2));
  ASSERT_STREQ("Hello\xe2\x80\xa8World", str1);

  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "ru_RU.UTF-8"), "ru_RU.UTF-8");
  ASSERT_EQ(2, rs_sscanf("Hello\xe2\x80\xa8World", "%s%s", str1, str2));
  ASSERT_STREQ("Hello", str1);
  ASSERT_STREQ("World", str2);
}

TEST(sscanf, smoke)
{
  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C"), "C");

  {
    int i;
    float x;
    char name[50];
    ASSERT_EQ(3, rs_sscanf("25 54.32E-1 thompson", "%d%f%s", &i, &x, name));
    ASSERT_EQ(25, i);
    ASSERT_EQ(5.432f, x);
    ASSERT_STREQ("thompson", name);
  }

  {
    int i;
    float x;
    char name[50];
    ASSERT_EQ(
      3, rs_sscanf("56789 0123 56a72", "%2d%f%*d %[0123456789]", &i, &x, name));
    ASSERT_EQ(56, i);
    ASSERT_EQ(789.0f, x);
    ASSERT_STREQ("56", name);
  }

  {
    float quant;
    char units[21], item[21];
    ASSERT_EQ(
      3, rs_sscanf("2 quarts of oil", "%f%20s of %20s", &quant, units, item));
    ASSERT_EQ(2, quant);
    ASSERT_STREQ("quarts", units);
    ASSERT_STREQ("oil", item);

    ASSERT_EQ(
      2,
      rs_sscanf("-12.5degrees Celcius", "%f%20s of %20s", &quant, units, item));
    ASSERT_EQ(-12.5, quant);
    ASSERT_STREQ("degrees", units);

    ASSERT_EQ(0,
              rs_sscanf("lots of luck", "%f%20s of %20s", &quant, units, item));

    ASSERT_EQ(
      3,
      rs_sscanf("10.0LBS     of\ndirt", "%f%20s of %20s", &quant, units, item));
    ASSERT_EQ(10.0, quant);
    ASSERT_STREQ("LBS", units);
    ASSERT_STREQ("dirt", item);

    // For some reason, the standard requires that this fails to parse,
    // but our floating point literal parser is smart enough to only
    // parse the "100" part.
    ASSERT_EQ(
      3, rs_sscanf("100ergs of energy", "%f%20s of %20s", &quant, units, item));
    ASSERT_EQ(100, quant);
    ASSERT_STREQ("ergs", units);
    ASSERT_STREQ("energy", item);
  }

  {
    int d1, d2 = 12345, n1, n2;
    ASSERT_EQ(1, rs_sscanf("123", "%d%n%n%d", &d1, &n1, &n2, &d2));
    ASSERT_EQ(123, d1);
    ASSERT_EQ(3, n1);
    ASSERT_EQ(3, n2);
    ASSERT_EQ(12345, d2);
  }

  // WG14's N2033: %% should always skip leading whitespace. For literal
  // characters, this should only be done when also preceded by
  // whitespace.
  {
    int i;
    ASSERT_EQ(0, rs_sscanf("foo  %  bar  42", "foo%%bar%d", &i));
    ASSERT_EQ(1, rs_sscanf("foo  %  bar  42", "foo%% bar%d", &i));
    ASSERT_EQ(42, i);
  }
}

TEST(sscanf, character)
{
  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C"), "C");

  int result;
  char ch;
  int n;

  result = rs_sscanf("a", "%c%n", &ch, &n);
  ASSERT_EQ(ch, 'a');
  ASSERT_EQ(n, 1);
  ASSERT_EQ(result, 1);
}

TEST(sscanf, allocated_character)
{
  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C"), "C");

  int result;
  char* ch = nullptr;
  int n;

  result = rs_sscanf("a", "%mc%n", &ch, &n);
  ASSERT_EQ(ch[0], 'a');
  ASSERT_EQ(n, 1);
  ASSERT_EQ(result, 1);
  rs_free(ch);
}

TEST(sscanf, wide_character)
{
  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C.UTF-8"), "C.UTF-8");

  int result;
  wchar_t ch;
  int n;

  result = rs_sscanf("€", "%lc%n", &ch, &n);
  ASSERT_EQ(ch, L'€');
  ASSERT_EQ(n, 3);
  ASSERT_EQ(result, 1);

  result = rs_sscanf("😊", "%lc%n", &ch, &n);
  ASSERT_EQ(ch, L'😊');
  ASSERT_EQ(n, 4);
  ASSERT_EQ(result, 1);
}

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wformat"

TEST(sscanf, allocated_wide_character)
{
  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C.UTF-8"), "C.UTF-8");

  int result;
  wchar_t* ch = nullptr;
  int n;

  result = rs_sscanf("€", "%mlc%n", &ch, &n);
  ASSERT_EQ(ch[0], L'€');
  ASSERT_EQ(n, 3);
  ASSERT_EQ(result, 1);
  rs_free(ch);

  result = rs_sscanf("😊", "%mlc%n", &ch, &n);
  ASSERT_EQ(ch[0], L'😊');
  ASSERT_EQ(n, 4);
  ASSERT_EQ(result, 1);
  rs_free(ch);
}

#pragma clang diagnostic pop

TEST(sscanf, characters)
{
  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C"), "C");

  // Parse 5 characters at once.
  int before;
  char out[6];
  int after;
  ASSERT_EQ(1, rs_sscanf("Hello", "%n%5c%n", &before, out, &after));
  ASSERT_EQ(0, before);
  out[5] = '\0';
  ASSERT_STREQ("Hello", out);
  ASSERT_EQ(5, after);
}

TEST(sscanf, allocated_characters)
{
  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C"), "C");

  char* out;
  ASSERT_EQ(1, rs_sscanf("Hello", "%5mc", &out));
  ASSERT_EQ(out[0], 'H');
  ASSERT_EQ(out[1], 'e');
  ASSERT_EQ(out[2], 'l');
  ASSERT_EQ(out[3], 'l');
  ASSERT_EQ(out[4], 'o');
  rs_free(out);
}

TEST(sscanf, wide_characters)
{
  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C.UTF-8"), "C.UTF-8");

  // Parse 2 characters at once.
  int before;
  wchar_t out[3];
  int after;
  ASSERT_EQ(1, rs_sscanf("😊😎", "%n%2lc%n", &before, out, &after));
  ASSERT_EQ(0, before);
  out[2] = L'\0';
  ASSERT_TRUE(wcscmp(L"😊😎", out) == 0);
  ASSERT_EQ(8, after);
}

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wformat"

TEST(sscanf, allocated_wide_characters)
{
  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C.UTF-8"), "C.UTF-8");

  wchar_t* out;
  ASSERT_EQ(1, rs_sscanf("😊😎", "%2mlc", &out));
  ASSERT_EQ(out[0], L'😊');
  ASSERT_EQ(out[1], L'😎');
  rs_free(out);
}

#pragma clang diagnostic pop

TEST(sscanf, scanset)
{
  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C"), "C");

  {
    char p1[8], p2;
    ASSERT_EQ(2, rs_sscanf("Hello there!", "%[Helo t]here%c", p1, &p2));
    ASSERT_STREQ("Hello t", p1);
    ASSERT_EQ('!', p2);

    ASSERT_EQ(1, rs_sscanf("Hell[o] there!", "%[][Helot]here%c", p1, &p2));
    ASSERT_STREQ("Hell[o]", p1);
    ASSERT_EQ(1, rs_sscanf("Hello there!", "%[Helo t]ere%c", p1, &p2));
    ASSERT_STREQ("Hello t", p1);
  }

  {
    char p1[8], p2;
    ASSERT_EQ(2, rs_sscanf("Hello there!", "%[^]h]here%c", p1, &p2));
    ASSERT_STREQ("Hello t", p1);
    ASSERT_EQ('!', p2);

    ASSERT_EQ(1, rs_sscanf("Hell[o] there!", "%[^!@#)$(*# ]here%c", p1, &p2));
    ASSERT_STREQ("Hell[o]", p1);
    ASSERT_EQ(1, rs_sscanf("Hello there!", "%[^abcdfgh]ere%c", p1, &p2));
    ASSERT_STREQ("Hello t", p1);
  }
}

TEST(sscanf, allocated_scanset)
{
  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C"), "C");

  {
    char *p1, p2;
    ASSERT_EQ(2, rs_sscanf("Hello there!", "%m[Helo t]here%c", &p1, &p2));
    ASSERT_STREQ("Hello t", p1);
    ASSERT_EQ('!', p2);
    rs_free(p1);

    ASSERT_EQ(1, rs_sscanf("Hell[o] there!", "%m[][Helot]here%c", &p1, &p2));
    ASSERT_STREQ("Hell[o]", p1);
    rs_free(p1);

    ASSERT_EQ(1, rs_sscanf("Hello there!", "%m[Helo t]ere%c", &p1, &p2));
    ASSERT_STREQ("Hello t", p1);
    rs_free(p1);
  }

  {
    char *p1, p2;
    ASSERT_EQ(2, rs_sscanf("Hello there!", "%m[^]h]here%c", &p1, &p2));
    ASSERT_STREQ("Hello t", p1);
    ASSERT_EQ('!', p2);
    rs_free(p1);

    ASSERT_EQ(1, rs_sscanf("Hell[o] there!", "%m[^!@#)$(*# ]here%c", &p1, &p2));
    ASSERT_STREQ("Hell[o]", p1);
    rs_free(p1);

    ASSERT_EQ(1, rs_sscanf("Hello there!", "%m[^abcdfgh]ere%c", &p1, &p2));
    ASSERT_STREQ("Hello t", p1);
    rs_free(p1);
  }
}

TEST(sscanf, strings)
{
  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C"), "C");

  int ret_val;
  char buffer[10];
  char buffer2[10];
  char buffer3[3];
  ret_val = rs_sscanf("abc123", "abc %s", buffer);
  ASSERT_EQ(ret_val, 1);
  ASSERT_STREQ(buffer, "123");

  ret_val = rs_sscanf("abc123", "%3s %3s", buffer, buffer2);
  ASSERT_EQ(ret_val, 2);
  ASSERT_STREQ(buffer, "abc");
  ASSERT_STREQ(buffer2, "123");

  ret_val = rs_sscanf("abc 123", "%3s%3s", buffer, buffer2);
  ASSERT_EQ(ret_val, 2);
  ASSERT_STREQ(buffer, "abc");
  ASSERT_STREQ(buffer2, "123");

  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C.UTF-8"), "C.UTF-8");

  char str1[256] = { 0 }, str2[256] = { 0 };
  size_t n = 0;

  ret_val = rs_sscanf("abcd efgh", "%s%s%zn", str1, str2, &n);
  ASSERT_STREQ(str1, "abcd");
  ASSERT_STREQ(str2, "efgh");
  ASSERT_EQ(n, 9);
  ASSERT_EQ(ret_val, 2);

  ret_val = rs_sscanf("€", "%s%zn", str1, &n);
  ASSERT_STREQ(str1, "€");
  ASSERT_EQ(n, 3);
  ASSERT_EQ(ret_val, 1);

  ret_val = rs_sscanf("😊", "%s%zn", str2, &n);
  ASSERT_STREQ(str2, "😊");
  ASSERT_EQ(n, 4);
  ASSERT_EQ(ret_val, 1);
}

TEST(sscanf, allocated_strings)
{
  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C"), "C");

  int ret_val;
  char* buffer;
  char* buffer2;
  ret_val = rs_sscanf("abc123", "abc %ms", &buffer);
  ASSERT_EQ(ret_val, 1);
  ASSERT_STREQ(buffer, "123");
  rs_free(buffer);

  ret_val = rs_sscanf("abc123", "%3ms %3ms", &buffer, &buffer2);
  ASSERT_EQ(ret_val, 2);
  ASSERT_STREQ(buffer, "abc");
  ASSERT_STREQ(buffer2, "123");
  rs_free(buffer);
  rs_free(buffer2);

  ret_val = rs_sscanf("abc 123", "%3ms%3ms", &buffer, &buffer2);
  ASSERT_EQ(ret_val, 2);
  ASSERT_STREQ(buffer, "abc");
  ASSERT_STREQ(buffer2, "123");
  rs_free(buffer);
  rs_free(buffer2);

  char *str1 = nullptr, *str2 = nullptr;
  size_t n = 0;

  ret_val = rs_sscanf("abcd efgh", "%ms%ms%zn", &str1, &str2, &n);
  ASSERT_STREQ(str1, "abcd");
  ASSERT_STREQ(str2, "efgh");
  ASSERT_EQ(n, 9);
  ASSERT_EQ(ret_val, 2);
  rs_free(str1);
  rs_free(str2);

  ret_val = rs_sscanf("€", "%ms%zn", &str1, &n);
  ASSERT_STREQ(str1, "€");
  ASSERT_EQ(n, 3);
  ASSERT_EQ(ret_val, 1);
  rs_free(str1);

  ret_val = rs_sscanf("😊", "%ms%zn", &str2, &n);
  ASSERT_STREQ(str2, "😊");
  ASSERT_EQ(n, 4);
  ASSERT_EQ(ret_val, 1);
  rs_free(str2);
}

TEST(sscanf, wide_strings)
{
  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C.UTF-8"), "C.UTF-8");

  int ret_val;
  wchar_t str1[256] = { 0 }, str2[256] = { 0 };
  size_t n = 0;

  ret_val = rs_sscanf("abcd efgh", "%ls%ls%zn", str1, str2, &n);
  ASSERT_TRUE(wcscmp(str1, L"abcd") == 0);
  ASSERT_TRUE(wcscmp(str2, L"efgh") == 0);
  ASSERT_EQ(n, 9);
  ASSERT_EQ(ret_val, 2);

  ret_val = rs_sscanf("€", "%ls%zn", str1, &n);
  ASSERT_TRUE(wcscmp(str1, L"€") == 0);
  ASSERT_EQ(n, 3);
  ASSERT_EQ(ret_val, 1);

  ret_val = rs_sscanf("😊", "%ls%zn", str2, &n);
  ASSERT_TRUE(wcscmp(str2, L"😊") == 0);
  ASSERT_EQ(n, 4);
  ASSERT_EQ(ret_val, 1);
}

TEST(sscanf, allocated_wide_strings)
{
  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C.UTF-8"), "C.UTF-8");

  int ret_val;
  wchar_t *str1 = nullptr, *str2 = nullptr;
  size_t n = 0;
  const char* fmt_mlsmls = "%mls%mls%zn";
  const char* fmt_mls = "%mls%zn";

  ret_val = rs_sscanf("abcd efgh", fmt_mlsmls, &str1, &str2, &n);
  ASSERT_TRUE(wcscmp(str1, L"abcd") == 0);
  ASSERT_TRUE(wcscmp(str2, L"efgh") == 0);
  ASSERT_EQ(n, 9);
  ASSERT_EQ(ret_val, 2);
  rs_free(str1);
  rs_free(str2);

  ret_val = rs_sscanf("€", fmt_mls, &str1, &n);
  ASSERT_TRUE(wcscmp(str1, L"€") == 0);
  ASSERT_EQ(n, 3);
  ASSERT_EQ(ret_val, 1);
  rs_free(str1);

  ret_val = rs_sscanf("😊", fmt_mls, &str2, &n);
  ASSERT_TRUE(wcscmp(str2, L"😊") == 0);
  ASSERT_EQ(n, 4);
  ASSERT_EQ(ret_val, 1);
  rs_free(str2);
}

TEST(sscanf, pointers)
{
  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C"), "C");

  int ret_val;
  void* result;

  ret_val = rs_sscanf("0", "%p", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, reinterpret_cast<void*>(0));

  ret_val = rs_sscanf("100", "%p", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, reinterpret_cast<void*>(0x100));

  ret_val = rs_sscanf("-1", "%p", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, reinterpret_cast<void*>(-1));

  ret_val = rs_sscanf("0xabcDEFG", "%p", &result);
  ASSERT_EQ(ret_val, 1);
  ASSERT_EQ(result, reinterpret_cast<void*>(0xabcdef));
}

TEST(sscanf, read_bytes)
{
  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C"), "C");

  int status = 0;
  int result = 0;

  int i = 0, j = 0;
  double x = 0, y = 0;
  int n = 0, m = 0;

  result = rs_sscanf("123", "%2$2d %1$d%3$n", &i, &j, &n);
  ASSERT_EQ(i, 3);
  ASSERT_EQ(j, 12);
  ASSERT_EQ(n, 3);
  ASSERT_EQ(result, 2);

  result = rs_sscanf("123e457e-2", "%2$5lf%4$n%1$lf%3$n", &x, &y, &n, &m);
  ASSERT_EQ(x, 57e-2);
  ASSERT_EQ(y, 123e+4);
  ASSERT_EQ(n, 10);
  ASSERT_EQ(m, 5);
  ASSERT_EQ(result, 2);

  result =
    rs_sscanf("123 567 4e2", "%1$d %5$n %3$d %2$lf %4$n", &i, &x, &j, &n, &m);
  ASSERT_EQ(i, 123);
  ASSERT_EQ(j, 567);
  ASSERT_EQ(x, 4e2);
  ASSERT_EQ(n, 11);
  ASSERT_EQ(m, 4);
  ASSERT_EQ(result, 3);
}

TEST(sscanf, overflow)
{
  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C"), "C");

  int status = 0;
  int result = 0;

  int8_t i8 = 0;
  int16_t i16 = 0;
  int32_t i32 = 0;
  int64_t i64 = 0;
  intmax_t imax = 0;
  intptr_t iptr = 0;

  uint8_t u8 = 0;
  uint16_t u16 = 0;
  uint32_t u32 = 0;
  uint64_t u64 = 0;
  uintmax_t umax = 0;
  uintptr_t uptr = 0;
  size_t usize = 0;

  result = rs_sscanf("-128 -32768 -2147483648 -9223372036854775808 "
                     "-9223372036854775808 -9223372036854775808",
                     "%hhi %hi %i %li %ji %ti",
                     &i8,
                     &i16,
                     &i32,
                     &i64,
                     &imax,
                     &iptr);
  ASSERT_EQ(result, 6);
  ASSERT_EQ(i8, INT8_MIN);
  ASSERT_EQ(i16, INT16_MIN);
  ASSERT_EQ(i32, INT32_MIN);
  ASSERT_EQ(i64, INT64_MIN);
  ASSERT_EQ(imax, INTMAX_MIN);
  ASSERT_EQ(iptr, INTPTR_MIN);

  result = rs_sscanf("127 32767 2147483647 9223372036854775807 "
                     "9223372036854775807 9223372036854775807",
                     "%hhi %hi %i %li %ji %ti",
                     &i8,
                     &i16,
                     &i32,
                     &i64,
                     &imax,
                     &iptr);
  ASSERT_EQ(result, 6);
  ASSERT_EQ(i8, INT8_MAX);
  ASSERT_EQ(i16, INT16_MAX);
  ASSERT_EQ(i32, INT32_MAX);
  ASSERT_EQ(i64, INT64_MAX);
  ASSERT_EQ(imax, INTMAX_MAX);
  ASSERT_EQ(iptr, INTPTR_MAX);

  result =
    rs_sscanf("255 65535 4294967295 18446744073709551615 18446744073709551615 "
              "18446744073709551615 18446744073709551615",
              "%hhu %hu %u %lu %ju %tu %zu",
              &u8,
              &u16,
              &u32,
              &u64,
              &umax,
              &uptr,
              &usize);
  ASSERT_EQ(result, 7);
  ASSERT_EQ(u8, UINT8_MAX);
  ASSERT_EQ(u16, UINT16_MAX);
  ASSERT_EQ(u32, UINT32_MAX);
  ASSERT_EQ(u64, UINT64_MAX);
  ASSERT_EQ(umax, UINTMAX_MAX);
  ASSERT_EQ(uptr, UINTPTR_MAX);
  ASSERT_EQ(usize, UINT64_MAX);
}

static const struct
{
  const wchar_t* fmt;
  const wchar_t* wfmt;
  const wchar_t* arg;
  int retval;
  const char* res;
  const wchar_t* wres;
  int only_C_locale;
} swscanf_tests[] = {
  { L"%[abc]", L"%l[abc]", L"aabbccddaabb", 1, "aabbcc", L"aabbcc", 0 },
  { L"%[^def]", L"%l[^def]", L"aabbccddaabb", 1, "aabbcc", L"aabbcc", 0 },
  { L"%[^abc]", L"%l[^abc]", L"aabbccddaabb", 0, "", L"", 0 },
  { L"%[a-c]", L"%l[a-c]", L"aabbccddaabb", 1, "aabbcc", L"aabbcc", 1 },
  { L"%[^d-f]", L"%l[^d-f]", L"aabbccddaabb", 1, "aabbcc", L"aabbcc", 1 },
  { L"%[^a-c]", L"%l[^a-c]", L"aabbccddaabb", 0, "", L"", 1 },
  { L"%[^a-c]", L"%l[^a-c]", L"bbccddaabb", 0, "", L"", 1 }
};

static void
do_swscanf_test(const char* loc)
{
  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, loc), loc);

  for (size_t n = 0; n < sizeof(swscanf_tests) / sizeof(swscanf_tests[0]);
       ++n) {
    char buf[100];
    wchar_t wbuf[100];

    if (swscanf_tests[n].only_C_locale && strcmp(loc, "C") != 0)
      continue;

    ASSERT_EQ(rs_swscanf(swscanf_tests[n].arg, swscanf_tests[n].fmt, buf),
              swscanf_tests[n].retval);
    ASSERT_FALSE(swscanf_tests[n].retval != 0 &&
                 strcmp(buf, swscanf_tests[n].res) != 0);
    ASSERT_EQ(rs_swscanf(swscanf_tests[n].arg, swscanf_tests[n].wfmt, wbuf),
              swscanf_tests[n].retval);
    ASSERT_FALSE(swscanf_tests[n].retval != 0 &&
                 wcscmp(wbuf, swscanf_tests[n].wres) != 0);
  }
}

TEST(swscanf, example)
{
  do_swscanf_test("C");
  do_swscanf_test("nl_NL.UTF-8");

  ASSERT_STREQ(rs_setlocale(RS_LC_ALL, "C"), "C");

  {
    int i;
    float x;
    wchar_t name[50];
    ASSERT_EQ(3, rs_swscanf(L"25 54.32E-1 thompson", L"%d%f%ls", &i, &x, name));
    ASSERT_EQ(25, i);
    ASSERT_EQ(5.432f, x);
    ASSERT_STREQ(L"thompson", name);
  }

  {
    int i;
    float x;
    double y;
    ASSERT_EQ(3, rs_swscanf(L"56789 0123 56a72", L"%2d%f%*d %lf", &i, &x, &y));
    ASSERT_EQ(56, i);
    ASSERT_EQ(789.0f, x);
    ASSERT_EQ(56.0, y);
  }
}
