#include "common.h"

#include <vector>

#include <wchar.h>

extern "C" {
void rs_bzero(void *, size_t);
void rs_explicit_bzero(void *, size_t);
int rs_ffs(int);
int rs_ffsl(long);
int rs_ffsll(long long);
int rs_strcasecmp(const char *, const char *);
int rs_strcasecmp_l(const char *, const char *, strogino_locale_t);
int rs_strncasecmp(const char *, const char *, size_t);
int rs_strncasecmp_l(const char *, const char *, size_t, strogino_locale_t);
}

static constexpr unsigned char kCanary = 0xA5;

static bool buf_is_zero(const void *buf, size_t n) {
  const auto *p = static_cast<const unsigned char *>(buf);
  for (size_t i = 0; i < n; i++)
    if (p[i] != 0)
      return false;
  return true;
}

static void fill(void *buf, size_t n, unsigned char val = kCanary) {
  memset(buf, val, n);
}

TEST(bzero, fill_buffer) {
  unsigned char buf[64];
  fill(buf, sizeof buf);
  rs_bzero(buf, sizeof buf);
  EXPECT_TRUE(buf_is_zero(buf, sizeof buf));
}

TEST(bzero, zero_len_noop) {
  unsigned char buf[8];
  fill(buf, sizeof buf);
  rs_bzero(buf, 0);
  for (size_t i = 0; i < sizeof buf; i++)
    EXPECT_EQ(buf[i], kCanary) << "byte " << i << " was modified";
}

TEST(bzero, single_byte_middle) {
  unsigned char buf[4];
  fill(buf, sizeof buf);
  rs_bzero(buf + 2, 1);
  EXPECT_EQ(buf[0], kCanary);
  EXPECT_EQ(buf[1], kCanary);
  EXPECT_EQ(buf[2], 0);
  EXPECT_EQ(buf[3], kCanary);
}

TEST(bzero, partial_prefix) {
  unsigned char buf[16];
  fill(buf, sizeof buf);
  rs_bzero(buf, 4);
  EXPECT_TRUE(buf_is_zero(buf, 4));
  for (size_t i = 4; i < sizeof buf; i++)
    EXPECT_EQ(buf[i], kCanary) << "byte " << i << " should be untouched";
}

TEST(bzero, partial_suffix) {
  unsigned char buf[16];
  fill(buf, sizeof buf);
  rs_bzero(buf + 12, 4);
  for (size_t i = 0; i < 12; i++)
    EXPECT_EQ(buf[i], kCanary) << "byte " << i << " should be untouched";
  EXPECT_TRUE(buf_is_zero(buf + 12, 4));
}

TEST(bzero, unaligned_interior_window) {
  unsigned char buf[32];
  fill(buf, sizeof buf);
  rs_bzero(buf + 3, 17);
  for (size_t i = 0; i < 3; i++)
    EXPECT_EQ(buf[i], kCanary) << "byte " << i << " should be untouched";
  EXPECT_TRUE(buf_is_zero(buf + 3, 17));
  for (size_t i = 20; i < sizeof buf; i++)
    EXPECT_EQ(buf[i], kCanary) << "byte " << i << " should be untouched";
}

TEST(bzero, large_buffer) {
  const size_t n = 1u << 20; /* 1 MiB */
  std::vector<unsigned char> buf(n, kCanary);
  rs_bzero(buf.data(), n);
  EXPECT_TRUE(buf_is_zero(buf.data(), n));
}

TEST(bzero, various_sizes_with_canary_guard) {
  static const size_t sizes[] = {
      1,  2,  3,  4,   7,   8,   9,   15,  16,  17,  31,  32,  33,
      63, 64, 65, 127, 128, 129, 255, 256, 257, 511, 512, 513,
  };
  for (size_t n : sizes) {
    std::vector<unsigned char> buf(n + 2, kCanary);
    rs_bzero(buf.data() + 1, n);
    EXPECT_EQ(buf[0], kCanary) << "leading canary clobbered for n=" << n;
    EXPECT_TRUE(buf_is_zero(buf.data() + 1, n)) << "not zeroed for n=" << n;
    EXPECT_EQ(buf[n + 1], kCanary) << "trailing canary clobbered for n=" << n;
  }
}

TEST(bzero, struct) {
  struct S {
    int a;
    long b;
    char c[7];
    double d;
  };
  S s;
  memset(&s, kCanary, sizeof s);
  rs_bzero(&s, sizeof s);
  EXPECT_TRUE(buf_is_zero(&s, sizeof s));
}

TEST(explicit_bzero, basic_full_buffer) {
  unsigned char buf[64];
  fill(buf, sizeof buf);
  rs_explicit_bzero(buf, sizeof buf);
  EXPECT_TRUE(buf_is_zero(buf, sizeof buf));
}

TEST(explicit_bzero, zero_len_noop) {
  unsigned char buf[8];
  fill(buf, sizeof buf);
  rs_explicit_bzero(buf, 0);
  for (size_t i = 0; i < sizeof buf; i++)
    EXPECT_EQ(buf[i], kCanary) << "byte " << i << " was modified";
}

TEST(explicit_bzero, single_byte) {
  unsigned char buf[4];
  fill(buf, sizeof buf);
  rs_explicit_bzero(buf + 1, 1);
  EXPECT_EQ(buf[0], kCanary);
  EXPECT_EQ(buf[1], 0);
  EXPECT_EQ(buf[2], kCanary);
  EXPECT_EQ(buf[3], kCanary);
}

TEST(explicit_bzero, partial_interior) {
  unsigned char buf[32];
  fill(buf, sizeof buf);
  rs_explicit_bzero(buf + 8, 16);
  for (size_t i = 0; i < 8; i++)
    EXPECT_EQ(buf[i], kCanary) << "byte " << i << " should be untouched";
  EXPECT_TRUE(buf_is_zero(buf + 8, 16));
  for (size_t i = 24; i < sizeof buf; i++)
    EXPECT_EQ(buf[i], kCanary) << "byte " << i << " should be untouched";
}

TEST(explicit_bzero, unaligned_window) {
  unsigned char buf[32];
  fill(buf, sizeof buf);
  rs_explicit_bzero(buf + 5, 13);
  for (size_t i = 0; i < 5; i++)
    EXPECT_EQ(buf[i], kCanary) << "byte " << i << " should be untouched";
  EXPECT_TRUE(buf_is_zero(buf + 5, 13));
  for (size_t i = 18; i < sizeof buf; i++)
    EXPECT_EQ(buf[i], kCanary) << "byte " << i << " should be untouched";
}

TEST(explicit_bzero, sensitive_pattern) {
  char password[32];
  memcpy(password, "s3cr3t_p@ssw0rd!s3cr3t_p@ssw0rd!", sizeof password);
  rs_explicit_bzero(password, sizeof password);
  EXPECT_TRUE(buf_is_zero(password, sizeof password));
}

TEST(explicit_bzero, heap_buffer) {
  const size_t n = 4096;
  std::vector<unsigned char> buf(n, kCanary);
  rs_explicit_bzero(buf.data(), n);
  EXPECT_TRUE(buf_is_zero(buf.data(), n));
}

TEST(explicit_bzero, various_sizes_with_canary_guard) {
  static const size_t sizes[] = {
      1,  2,  3,  4,  7,  8,   9,   15,  16,  17,  31,
      32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 257,
  };
  for (size_t n : sizes) {
    std::vector<unsigned char> buf(n + 2, kCanary);
    rs_explicit_bzero(buf.data() + 1, n);
    EXPECT_EQ(buf[0], kCanary) << "leading canary clobbered for n=" << n;
    EXPECT_TRUE(buf_is_zero(buf.data() + 1, n)) << "not zeroed for n=" << n;
    EXPECT_EQ(buf[n + 1], kCanary) << "trailing canary clobbered for n=" << n;
  }
}

TEST(ffs, examples) {
  ASSERT_EQ(0, rs_ffs(0x0));
  ASSERT_EQ(1, rs_ffs(0x3211));
  ASSERT_EQ(2, rs_ffs(0xabc2));
  ASSERT_EQ(3, rs_ffs(0x79224));

  ASSERT_EQ(WORD_BIT - 3, rs_ffs(INT_MIN >> 3));
  ASSERT_EQ(WORD_BIT, rs_ffs(INT_MIN));
}

TEST(ffsl, examples) {
  ASSERT_EQ(0, rs_ffsl(0x0));
  ASSERT_EQ(1, rs_ffsl(0x3211));
  ASSERT_EQ(2, rs_ffsl(0xabc2));
  ASSERT_EQ(3, rs_ffsl(0x79224));

  ASSERT_EQ(LONG_BIT - 3, rs_ffsl(LONG_MIN >> 3));
  ASSERT_EQ(LONG_BIT, rs_ffsl(LONG_MIN));
}

TEST(ffsll, examples) {
  ASSERT_EQ(0, rs_ffsll(0x0));
  ASSERT_EQ(1, rs_ffsll(0x3211));
  ASSERT_EQ(2, rs_ffsll(0xabc2));
  ASSERT_EQ(3, rs_ffsll(0x79224));

  ASSERT_EQ(sizeof(long long) * 8 - 3, rs_ffsll(LLONG_MIN >> 3));
  ASSERT_EQ(sizeof(long long) * 8, rs_ffsll(LLONG_MIN));
}

TEST(strcasecmp, example) {
  ASSERT_STREQ("C", rs_setlocale(LC_ALL, "C"));

  ASSERT_EQ(rs_strcasecmp(nullptr, nullptr), 0);
  ASSERT_EQ(rs_strcasecmp("", ""), 0);
  ASSERT_EQ(rs_strcasecmp("abc", "abc"), 0);
  ASSERT_EQ(rs_strcasecmp("ABC", "ABC"), 0);
  ASSERT_EQ(rs_strcasecmp("abc", "ABC"), 0);
  ASSERT_EQ(rs_strcasecmp("ABC", "abc"), 0);
  ASSERT_NE(rs_strcasecmp("abc", "xyz"), 0);
  ASSERT_NE(rs_strcasecmp("ABC", "xyz"), 0);
  ASSERT_NE(rs_strcasecmp("abc", "XYZ"), 0);
  ASSERT_NE(rs_strcasecmp("ABC", "XYZ"), 0);
  ASSERT_NE(rs_strcasecmp("xyz", "abc"), 0);
  ASSERT_NE(rs_strcasecmp("XYZ", "abc"), 0);
  ASSERT_NE(rs_strcasecmp("xyz", "ABC"), 0);
  ASSERT_NE(rs_strcasecmp("XYZ", "ABC"), 0);
  ASSERT_NE(rs_strcasecmp("abc", "ABCD"), 0);
  ASSERT_NE(rs_strcasecmp("ABC", "abcd"), 0);
  ASSERT_NE(rs_strcasecmp("abcd", "ABC"), 0);
  ASSERT_NE(rs_strcasecmp("ABCD", "abc"), 0);
}

TEST(strcasecmp, unicode) {
  strogino_locale_t loc = rs_newlocale(LC_CTYPE_MASK, "en_US.UTF-8", 0);
  ASSERT_NE(nullptr, loc);
  ASSERT_STREQ("en_US.UTF-8", rs_getlocalename_l(LC_CTYPE, loc));

  ASSERT_EQ(rs_strcasecmp_l("λ", "Λ", loc), 0);
  ASSERT_NE(rs_strcasecmp_l("λ", "Ω", loc), 0);
  ASSERT_NE(rs_strcasecmp_l("Ω", "λ", loc), 0);

  rs_freelocale(loc);
}

TEST(strncasecmp, example) {
  ASSERT_STREQ("C", rs_setlocale(LC_ALL, "C"));

  ASSERT_EQ(rs_strncasecmp(nullptr, nullptr, 0), 0);
  ASSERT_EQ(rs_strncasecmp("", "", 50), 0);
  ASSERT_NE(rs_strncasecmp("abc", "ABCD", 4), 0);
  ASSERT_NE(rs_strncasecmp("ABC", "abcd", 4), 0);
  ASSERT_NE(rs_strncasecmp("abcd", "ABC", 4), 0);
  ASSERT_NE(rs_strncasecmp("ABCD", "abc", 4), 0);
  ASSERT_EQ(rs_strncasecmp("abc", "ABCD", 3), 0);
  ASSERT_EQ(rs_strncasecmp("ABC", "abcd", 3), 0);
}

TEST(strncasecmp, unicode) {
  strogino_locale_t loc = rs_newlocale(LC_CTYPE_MASK, "en_US.UTF-8", 0);
  ASSERT_NE(nullptr, loc);
  ASSERT_STREQ("en_US.UTF-8", rs_getlocalename_l(LC_CTYPE, loc));

  ASSERT_EQ(rs_strncasecmp_l("λ", "Λ", 1, loc), 0);
  ASSERT_NE(rs_strncasecmp_l("λ", "Ω", 1, loc), 0);
  ASSERT_NE(rs_strncasecmp_l("Ω", "λ", 1, loc), 0);

  rs_freelocale(loc);
}
