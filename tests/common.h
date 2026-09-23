#include <gmock/gmock.h>

extern "C"
{
  int* __rs_errno_location(void);
  void* rs_malloc(size_t size);
  void* rs_aligned_alloc(size_t alignment, size_t size);
  void* rs_calloc(size_t n, size_t size);
  void* rs_realloc(void* ptr, size_t new_size);
  void* rs_reallocarray(void* ptr, size_t nelem, size_t size);
  void rs_free(void* ptr);
  void rs_free_sized(void* ptr, size_t size);
  void rs_free_aligned_sized(void* ptr, size_t align, size_t size);
}

#define rs_errno (*__rs_errno_location())
