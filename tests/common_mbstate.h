#include <cstdint>
#include <stdint.h>

typedef struct {
  uint32_t ch;
  uintptr_t bytesleft;
  uint32_t partial;
  uint32_t lowerbound;
  uint8_t u8_buffer[4];
  uintptr_t u8_position;
  uint16_t u16_buffer[2];
  uint16_t u16_surrogate;
} strogino_mbstate_t;

extern "C" {
  int rs_mbsinit(const strogino_mbstate_t *);
}
