#include <fenv.h>
#include <float.h>

#define LDBL_IS_F64 0x1
#define LDBL_IS_F80 0x2
#define LDBL_IS_F128 0x3

#if LDBL_MANT_DIG == 113
#define LDBL_TYPE LDBL_IS_F128
#elif LDBL_MANT_DIG == 64
#define LDBL_TYPE LDBL_IS_F80
#elif LDBL_MANT_DIG == 53
#define LDBL_TYPE LDBL_IS_F64
#else
#error long double not supported on this platform
#endif
