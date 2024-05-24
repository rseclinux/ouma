#include <gtest/gtest.h>
#include <gmock/gmock.h>

struct StroginoLocale;
typedef struct StroginoLocale* strogino_locale_t;

typedef struct {
  char16_t __surrogate;
  unsigned int __bytesleft;
  char32_t __partial;
  char32_t __lowerbound;
} StroginoMBState;
typedef StroginoMBState strogino_mbstate_t;
