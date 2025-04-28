#include <gmock/gmock.h>
#include <gtest/gtest.h>

extern "C" {
int *__rs_errno_location(void);
}
#define rs_errno (*__rs_errno_location())

typedef struct {
  char16_t __surrogate;
  unsigned int __bytesleft;
  char32_t __partial;
  char32_t __lowerbound;
  char32_t codeunit;
  uint8_t __byte[4];
  unsigned int count;
} StroginoMBState;
typedef StroginoMBState strogino_mbstate_t;
