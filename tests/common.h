#include <gmock/gmock.h>
#include <gtest/gtest.h>

struct StroginoLocale;
typedef struct StroginoLocale *strogino_locale_t;

typedef struct {
  char16_t __surrogate;
  unsigned int __bytesleft;
  char32_t __partial;
  char32_t __lowerbound;
  uint8_t __byte[4];
  unsigned int count;
} StroginoMBState;
typedef StroginoMBState strogino_mbstate_t;
