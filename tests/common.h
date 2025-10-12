#include <gmock/gmock.h>
#include <gtest/gtest.h>

extern "C" {
int *__rs_errno_location(void);
char *rs_setlocale(int, const char *);
}

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

typedef void* strogino_locale_t;

#define STROGINO_LC_GLOBAL_LOCALE ((strogino_locale_t) -1L)
#define rs_errno (*__rs_errno_location())
