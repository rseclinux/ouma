#include "common.h"
#include "gtest/gtest.h"

extern "C"
{
  [[noreturn]] void ouma_assertion_failed(const char* __func,
                                          const char* __file,
                                          int __line,
                                          const char* __exp);
}

#define rs_assert(expression)                                                  \
  ouma_assertion_failed(__func__, __FILE__, __LINE__, #expression)

TEST(assert, example)
{
  ASSERT_DEATH(rs_assert(0), "");
  ASSERT_DEATH(rs_assert(0 && "This doesn't work"), "");
  ASSERT_DEATH(rs_assert(NULL), "");
  ASSERT_DEATH(rs_assert(1 > 0 == false), "");

  int i = 0;
  ASSERT_DEATH(rs_assert((i = 1)), "");
  ASSERT_EQ(0, i);
}
