#include <cfenv>
#include <cfloat>
#include <string_view>
#include <type_traits>

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

template <typename F, typename Char, typename = void> class FloatTestData;

template <typename F, typename Char>
class FloatTestData<F, Char, std::enable_if_t<std::is_floating_point_v<F>>> {
public:
  std::basic_string_view<Char> name;
  F value{};
};
