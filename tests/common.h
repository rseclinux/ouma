#include <gmock/gmock.h>
#include <gtest/gtest.h>

extern "C"
{
  int* __rs_errno_location(void);
  char* rs_setlocale(int, const char*);
}

typedef struct
{
  char32_t ch;
  uintptr_t bytesleft;
  char32_t partial;
  char32_t lowerbound;
  char8_t u8_buffer[4];
  uintptr_t u8_position;
  char16_t u16_buffer[2];
  char16_t u16_surrogate;
} strogino_mbstate_t;

typedef void* strogino_locale_t;

#define STROGINO_LC_GLOBAL_LOCALE ((strogino_locale_t) - 1L)
#define rs_errno (*__rs_errno_location())
