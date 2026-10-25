#include "common.h"
#include "common_float.h"
#include "common_locale.h"

#include <cfenv>
#include <cfloat>
#include <cstdint>
#include <cstdio>
#include <gtest/gtest.h>

extern "C"
{
  double rs_atof(const char*);
  int rs_atoi(const char*);
  long int rs_atol(const char*);
  long long int rs_atoll(const char*);
  double rs_strtod(const char* __restrict__, char** __restrict__);
  float rs_strtof(const char* __restrict__, char** __restrict__);
  long double rs_strtold(const char* __restrict__, char** __restrict__);
  long int rs_strtol(const char* __restrict__, char** __restrict__, int);
  long long int rs_strtoll(const char* __restrict__, char** __restrict__, int);
  unsigned long int rs_strtoul(const char* __restrict__,
                               char** __restrict__,
                               int);
  unsigned long long int rs_strtoull(const char* __restrict__,
                                     char** __restrict__,
                                     int);
  int rs_mblen(const char*, size_t);
  int rs_mbtowc(wchar_t* __restrict__, const char* __restrict__, size_t);
  int rs_wctomb(char*, wchar_t wc);
  size_t rs_mbstowcs(wchar_t* __restrict__s, const char* __restrict__, size_t);
  size_t rs_wcstombs(char* __restrict__, const wchar_t* __restrict__s, size_t);
  div_t rs_div(int numerator, int denominator);
  ldiv_t rs_ldiv(long numerator, long denominator);
  lldiv_t rs_lldiv(long long numerator, long long denominator);
  int rs_abs(int num);
  long rs_labs(long num);
  long long rs_llabs(long long num);
  int rs_strfromf(char* s, size_t n, const char* format, float fp);
  int rs_strfromd(char* s, size_t n, const char* format, double fp);
  int rs_strfroml(char* s, size_t n, const char* format, long double fp);
  void rs_qsort(void* ptr,
                size_t count,
                size_t size,
                int (*comp)(const void*, const void*));
  void rs_qsort_r(void* base,
                  size_t nel,
                  size_t width,
                  int (*compar)(const void*, const void*, void*),
                  void* arg);
  long rs_a64l(const char* str64);
  int rs_l64a_r(long int l, char* buffer, int buflen);
  char* rs_l64a(long int l);
}

TEST(atof, examples)
{
  REVERT_TO_C_LOCALE();

  char buf[128];

  // TODO: replace with rs_snprintf
  (void)snprintf(buf, sizeof(buf), "%f\n", DBL_MAX);

  ASSERT_EQ(rs_atof("0"), rs_strtod("0", NULL));
  ASSERT_EQ(rs_atof("-1"), rs_strtod("-1", NULL));
  ASSERT_EQ(rs_atof(buf), rs_strtod(buf, NULL));
}

TEST(atoi, examples)
{
  ASSERT_EQ(12, rs_atoi("  12"));
  ASSERT_EQ(-3, rs_atoi("-03"));
  ASSERT_EQ(0, rs_atoi("0x5"));
}

TEST(atol, examples)
{
  ASSERT_EQ(123456, rs_atol("  123456"));
  ASSERT_EQ(-8860, rs_atol("-08860"));
}

TEST(atoll, examples)
{
  ASSERT_EQ(5050505, rs_atoll(" 5050505 "));
}

// Taken from musl libc-test: https://wiki.musl-libc.org/libc-test
FloatTestData<float, char> tests_float[] = {
  { ".700649232162408535461864791644958065640130970938257885878534141944895541"
    "3429303e-45",
    0 },
  { ".700649232162408535461864791644958065640130970938257885878534141944895541"
    "3429304e-45",
    0x1p-149 },
  { ".210194769648722560638559437493487419692039291281477365763560242583468662"
    "4028790e-44",
    0x1p-149 },
  { ".210194769648722560638559437493487419692039291281477365763560242583468662"
    "4028791e-44",
    0x1p-148 },
  { ".117549442088721072420959008340872484231447212078518461533454029413183145"
    "3944281e-37",
    0x1p-126 },
  { ".117549442088721072420959008340872484231447212078518461533454029413183145"
    "3944282e-37",
    0x1.000002p-126 },
  { "340282356779733661637539395458142568447.9999999999999999999",
    0x1.fffffep127 },
  { "340282356779733661637539395458142568448", INFINITY },
};

FloatTestData<double, char> tests_double[] = {
  { "0", 0.0 },
  { "00.00", 0.0 },
  { "-.00000", -0.0 },
  { "1e+1000000", INFINITY },
  { "1e-1000000", 0 },
  { ".247032822920623272088284396434110686182529901307162382212792841250337753"
    "6351043e-323",
    0 },
  { ".247032822920623272088284396434110686182529901307162382212792841250337753"
    "6351044e-323",
    0x1p-1074 },
  { ".741098468761869816264853189302332058547589703921487146638378523751013260"
    "9053131e-323",
    0x1p-1074 },
  { ".741098468761869816264853189302332058547589703921487146638378523751013260"
    "9053132e-323",
    0x1p-1073 },
  { ".222507385850720163012305563795567615250361241457301801308322872404958664"
    "7606759e-307",
    0x1p-1022 },
  { ".222507385850720163012305563795567615250361241457301801308322872404958664"
    "7606760e-307",
    0x1.0000000000001p-1022 },
  { "17976931348623158079372897140530341507993413271003782693617377898044"
    "49682927647509466490179775872070963302864166928879109465555478519404"
    "02630657488671505820681908902000708383676273854845817711531764475730"
    "27006985557136695962284291481986083493647529271907416844436551070434"
    "2711559699508093042880177904174497791.999999999999999999999999999999",
    0x1.fffffffffffffp1023 },
  { "17976931348623158079372897140530341507993413271003782693617377898044"
    "49682927647509466490179775872070963302864166928879109465555478519404"
    "02630657488671505820681908902000708383676273854845817711531764475730"
    "27006985557136695962284291481986083493647529271907416844436551070434"
    "2711559699508093042880177904174497792",
    INFINITY },
  { ".5961860348131807091861002266453941950428e00", 0.59618603481318067 },
  { "1.815013169218038729887460898733526957442e-1", 0.18150131692180388 },
  { "42.07082357534453600681618685682257590772e-2", 0.42070823575344535 },
  { "665.4686306516261456328973225579833470816e-3", 0.66546863065162609 },
  { "6101.852922970868621786690495485449831753e-4", 0.61018529229708685 },
  { "76966.95208236968077849464348875471158549e-5", 0.76966952082369677 },
  { "250506.5322228682496132604807222923702304e-6", 0.25050653222286823 },
  { "2740037.230228005325852424697698331177377e-7", 0.27400372302280052 },
  { "20723093.50049742645941529268715428324490e-8", 0.20723093500497428 },
  { "0.7900280238081604956226011047460238748912e1", 7.9002802380816046 },
  { "0.9822860653737296848190558448760465863597e2", 98.228606537372968 },
  { "0.7468949723190370809405570560160405324869e3", 746.89497231903704 },
  { "0.1630268320282728475980459844271031751665e4", 1630.2683202827284 },
  { "0.4637168629719170695109918769645492022088e5", 46371.686297191707 },
  { "0.6537805944497711554209461686415872067523e6", 653780.59444977110 },
  { "0.2346324356502437045212230713960457676531e6", 234632.43565024371 },
  { "0.9709481716420048341897258980454298205278e8", 97094817.164200485 },
  { "0.4996908522051874110779982354932499499602e9", 499690852.20518744 },
};

FloatTestData<long double, char> tests_long_double[] = {
  { "0", 0.0 },
  { "12.345", 12.345L },
  { "1.2345e1", 12.345L },
  { "1e+1000000", INFINITY },
  { "1e-1000000", 0 },
#if LDBL_TYPE == LDBL_IS_F64
  { ".247032822920623272088284396434110686182529901307162382212792841250337753"
    "6351043e-323",
    0 },
  { ".247032822920623272088284396434110686182529901307162382212792841250337753"
    "6351044e-323",
    0x1p-1074 },
  { ".741098468761869816264853189302332058547589703921487146638378523751013260"
    "9053131e-323",
    0x1p-1074 },
  { ".741098468761869816264853189302332058547589703921487146638378523751013260"
    "9053132e-323",
    0x1p-1073 },
  { ".222507385850720163012305563795567615250361241457301801308322872404958664"
    "7606759e-307",
    0x1p-1022 },
  { ".222507385850720163012305563795567615250361241457301801308322872404958664"
    "7606760e-307",
    0x1.0000000000001p-1022 },
  { "17976931348623158079372897140530341507993413271003782693617377898044"
    "49682927647509466490179775872070963302864166928879109465555478519404"
    "02630657488671505820681908902000708383676273854845817711531764475730"
    "27006985557136695962284291481986083493647529271907416844436551070434"
    "2711559699508093042880177904174497791.999999999999999999999999999999",
    0x1.fffffffffffffp1023 },
  { "17976931348623158079372897140530341507993413271003782693617377898044"
    "49682927647509466490179775872070963302864166928879109465555478519404"
    "02630657488671505820681908902000708383676273854845817711531764475730"
    "27006985557136695962284291481986083493647529271907416844436551070434"
    "2711559699508093042880177904174497792",
    INFINITY },
  { ".5961860348131807091861002266453941950428e00", 0.59618603481318067 },
  { "1.815013169218038729887460898733526957442e-1", 0.18150131692180388 },
  { "42.07082357534453600681618685682257590772e-2", 0.42070823575344535 },
  { "665.4686306516261456328973225579833470816e-3", 0.66546863065162609 },
  { "6101.852922970868621786690495485449831753e-4", 0.61018529229708685 },
  { "76966.95208236968077849464348875471158549e-5", 0.76966952082369677 },
  { "250506.5322228682496132604807222923702304e-6", 0.25050653222286823 },
  { "2740037.230228005325852424697698331177377e-7", 0.27400372302280052 },
  { "20723093.50049742645941529268715428324490e-8", 0.20723093500497428 },
  { "0.7900280238081604956226011047460238748912e1", 7.9002802380816046 },
  { "0.9822860653737296848190558448760465863597e2", 98.228606537372968 },
  { "0.7468949723190370809405570560160405324869e3", 746.89497231903704 },
  { "0.1630268320282728475980459844271031751665e4", 1630.2683202827284 },
  { "0.4637168629719170695109918769645492022088e5", 46371.686297191707 },
  { "0.6537805944497711554209461686415872067523e6", 653780.59444977110 },
  { "0.2346324356502437045212230713960457676531e6", 234632.43565024371 },
  { "0.9709481716420048341897258980454298205278e8", 97094817.164200485 },
  { "0.4996908522051874110779982354932499499602e9", 499690852.20518744 },
#elif LDBL_TYPE == LDBL_IS_F80
  { ".182259976594123730126420296680970990819952540784678167186049024351418584"
    "4316698e-4950",
    0 },
  { ".182259976594123730126420296680970990819952540784678167186049024351418584"
    "4316699e-4950",
    0x1p-16445L },
  { ".546779929782371190379260890042912972459857622354034501558147073054255753"
    "2950096e-4950",
    0x1p-16445L },
  { ".546779929782371190379260890042912972459857622354034501558147073054255753"
    "2950097e-4950",
    0x1p-16444L },
  { ".336210314311209350644493779391587633272449964152744223092877977059342086"
    "6576777e-4931",
    0x1p-16382L },
  { ".336210314311209350644493779391587633272449964152744223092877977059342086"
    "6576778e-4931",
    0x1.0000000000000002p-16382L },
  { "118973149535723176505351158982948.86679662540046955672e4900",
    0x1.fffffffffffffffep16383L },
  { "118973149535723176505351158982948.86679662540046955673e4900", INFINITY },
#endif
};

TEST(strtof, dec)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  for (size_t i = 0; i < std::size(tests_float); i++) {
    ASSERT_EQ(rs_strtof(tests_float[i].name.data(), nullptr),
              tests_float[i].value);
  }
}

TEST(strtof, dec1)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* str = "0.0625";
  char* endptr;
  ASSERT_EQ(0.0625f, rs_strtof(str, NULL));
  ASSERT_EQ(0.0625f, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 6, endptr);
}

TEST(strtof, dec2)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* str = "12800e-2";
  char* endptr;
  ASSERT_EQ(128.0f, rs_strtof(str, NULL));
  ASSERT_EQ(128.0f, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 8, endptr);
}

TEST(strtof, dec3)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* str = "7,5";
  char* endptr;
  ASSERT_EQ(7.0f, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 1, endptr);

  SET_GLOBAL_LOCALE("nl_NL.UTF-8");

  rs_errno = 0;

  ASSERT_EQ(7.5f, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 3, endptr);

  str = ",75";
  ASSERT_EQ(.75f, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 3, endptr);
}

TEST(strtof, hex1)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* str = "  0xcaf.eff";
  char* endptr;
  ASSERT_EQ(0xcaf.effp0, rs_strtof(str, NULL));
  ASSERT_EQ(0xcaf.effp0, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 11, endptr);
}

TEST(strtof, hex2)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* str = "0x0p99999999999999999999";
  char* endptr;
  ASSERT_EQ(0.0, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 24, endptr);
}

TEST(strtof, hex3)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* str = "\t0x1p+30000";
  char* endptr;
  ASSERT_EQ(HUGE_VALF, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 11, endptr);
}

TEST(strtof, hex4)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* str = "\n0X1P-30000 ";
  char* endptr;
  ASSERT_EQ(0.0f, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 11, endptr);
}

TEST(strtof, hex5)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* str = "-0x123xyz";
  char* endptr;
  ASSERT_EQ(-0x123.0p0, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 6, endptr);
}

TEST(strtof, hex6)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* str = "0x";
  char* endptr;
  ASSERT_EQ(0, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 1, endptr);
}

TEST(strtof, hex7)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* str = "0x.8";
  char* endptr;
  ASSERT_EQ(0.5, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 4, endptr);
}

TEST(strtof, hex8)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* below = "-0x0.ffffffffffffffffffffffffffffffffffffffffffffffffff";
  const char* exact = "-0x1.0";
  const char* above = "-0x1.00000000000000000000000000000000000000000000000001";
  float low = 0x1.fffffep-1f;
  float high = 0x1.000002p+0f;

  ASSERT_EQ(0, fesetround(FE_DOWNWARD));
  ASSERT_EQ(low, rs_strtof(below + 1, NULL));
  ASSERT_EQ(1.0f, rs_strtof(exact + 1, NULL));
  ASSERT_EQ(1.0f, rs_strtof(above + 1, NULL));
  ASSERT_EQ(-1.0f, rs_strtof(below, NULL));
  ASSERT_EQ(-1.0f, rs_strtof(exact, NULL));
  ASSERT_EQ(-high, rs_strtof(above, NULL));

  ASSERT_EQ(0, fesetround(FE_TONEAREST));
  ASSERT_EQ(1.0f, rs_strtof(below + 1, NULL));
  ASSERT_EQ(1.0f, rs_strtof(exact + 1, NULL));
  ASSERT_EQ(1.0f, rs_strtof(above + 1, NULL));
  ASSERT_EQ(-1.0f, rs_strtof(below, NULL));
  ASSERT_EQ(-1.0f, rs_strtof(exact, NULL));
  ASSERT_EQ(-1.0f, rs_strtof(above, NULL));

  ASSERT_EQ(0, fesetround(FE_TOWARDZERO));
  ASSERT_EQ(low, rs_strtof(below + 1, NULL));
  ASSERT_EQ(1.0f, rs_strtof(exact + 1, NULL));
  ASSERT_EQ(1.0f, rs_strtof(above + 1, NULL));
  ASSERT_EQ(-low, rs_strtof(below, NULL));
  ASSERT_EQ(-1.0f, rs_strtof(exact, NULL));
  ASSERT_EQ(-1.0f, rs_strtof(above, NULL));

  ASSERT_EQ(0, fesetround(FE_UPWARD));
  ASSERT_EQ(1.0f, rs_strtof(below + 1, NULL));
  ASSERT_EQ(1.0f, rs_strtof(exact + 1, NULL));
  ASSERT_EQ(high, rs_strtof(above + 1, NULL));
  ASSERT_EQ(-low, rs_strtof(below, NULL));
  ASSERT_EQ(-1.0f, rs_strtof(exact, NULL));
  ASSERT_EQ(-1.0f, rs_strtof(above, NULL));

  ASSERT_EQ(0, fesetround(FE_TONEAREST));
}

TEST(strtof, hex9)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* normal = "0x1p-126";
  const char* highest_subnormal = "0x1.fffffcp-127";
  float high = 0x1.fffffcp-127;
  const char* lowest_subnormal = "0x1p-149";
  const char* underflow = "0x1p-150";

  ASSERT_EQ(FLT_MIN, rs_strtof(normal, NULL));
  ASSERT_EQ(0, rs_errno);
  ASSERT_EQ(high, rs_strtof(highest_subnormal, NULL));
  ASSERT_EQ(0, rs_errno);
  ASSERT_EQ(FLT_TRUE_MIN, rs_strtof(lowest_subnormal, NULL));
  ASSERT_EQ(0, rs_errno);
  ASSERT_EQ(0.0, rs_strtof(underflow, NULL));
  ASSERT_EQ(ERANGE, rs_errno);
}

TEST(strtof, nan1)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* str = "NaN(Hello";
  char* endptr;
  ASSERT_TRUE(std::isnan(rs_strtof(str, &endptr)));
  ASSERT_EQ(str + 3, endptr);
}

TEST(strtof, nan2)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* str = "NaN(Hello world) :-)";
  char* endptr;
  ASSERT_TRUE(std::isnan(rs_strtof(str, &endptr)));
  ASSERT_EQ(str + 16, endptr);
}

TEST(strtof, inf1)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* str = "INFINITE";
  char* endptr;
  ASSERT_EQ(INFINITY, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 3, endptr);
}

TEST(strtof, inf2)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* str = "-INFINITY";
  char* endptr;
  ASSERT_EQ(-INFINITY, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 9, endptr);
}

TEST(strtof, huge_val1)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* str =
    "10000000000000000000000000000000000000000000000000000000000000000000000"
    "00000000000000000000000000000000000000000000000000000000000000000000000"
    "00000000000000000000000000000000000000000000000000000000000000000000000";
  char* endptr;
  ASSERT_EQ(HUGE_VALF, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 213, endptr);
  ASSERT_EQ(ERANGE, rs_errno);
}

TEST(strtof, huge_val2)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* str = "-1e3000";
  char* endptr;
  ASSERT_EQ(-HUGE_VALF, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 7, endptr);
  ASSERT_EQ(ERANGE, rs_errno);
}

TEST(strtof, zero1)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* str =
    "0.000000000000000000000000000000000000000000000000000000000000000000000"
    "00000000000000000000000000000000000000000000000000000000000000000000000"
    "00000000000000000000000000000000000000000000000000000000000000000000001";
  char* endptr;
  float v = rs_strtof(str, &endptr);
  ASSERT_EQ(0.0, v);
  ASSERT_FALSE(std::signbit(v));
  ASSERT_EQ(str + 213, endptr);
  ASSERT_EQ(ERANGE, rs_errno);
}

TEST(strtof, zero2)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* str = "-1e-3000";
  char* endptr;
  float v = rs_strtof(str, &endptr);
  ASSERT_EQ(0.0, v);
  ASSERT_TRUE(std::signbit(v));
  ASSERT_EQ(str + 8, endptr);
  ASSERT_EQ(ERANGE, rs_errno);
}

TEST(strtof, zero3)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* str = "0.0";
  char* endptr;
  float v = rs_strtof(str, &endptr);
  ASSERT_EQ(0.0, v);
  ASSERT_FALSE(std::signbit(v));
  ASSERT_EQ(str + 3, endptr);
  ASSERT_EQ(0, rs_errno);
}

TEST(strtof, zero4)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* str = "-0.0";
  char* endptr;
  float v = rs_strtof(str, &endptr);
  ASSERT_EQ(0.0, v);
  ASSERT_TRUE(std::signbit(v));
  ASSERT_EQ(str + 4, endptr);
  ASSERT_EQ(0, rs_errno);
}

TEST(strtod, dec)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  for (size_t i = 0; i < std::size(tests_double); i++) {
    ASSERT_EQ(rs_strtod(tests_double[i].name.data(), nullptr),
              tests_double[i].value);
  }
}

TEST(strtod, hex1)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* below = "-0x0.ffffffffffffffffffffffffffffffffffffffffffffffffff";
  const char* exact = "-0x1.0";
  const char* above = "-0x1.00000000000000000000000000000000000000000000000001";
  double low = 0x1.fffffffffffffp-1;
  double high = 0x1.0000000000001p+0;

  ASSERT_EQ(0, fesetround(FE_DOWNWARD));
  ASSERT_EQ(low, rs_strtod(below + 1, NULL));
  ASSERT_EQ(1.0, rs_strtod(exact + 1, NULL));
  ASSERT_EQ(1.0, rs_strtod(above + 1, NULL));
  ASSERT_EQ(-1.0, rs_strtod(below, NULL));
  ASSERT_EQ(-1.0, rs_strtod(exact, NULL));
  ASSERT_EQ(-high, rs_strtod(above, NULL));

  ASSERT_EQ(0, fesetround(FE_TONEAREST));
  ASSERT_EQ(1.0, rs_strtod(below + 1, NULL));
  ASSERT_EQ(1.0, rs_strtod(exact + 1, NULL));
  ASSERT_EQ(1.0, rs_strtod(above + 1, NULL));
  ASSERT_EQ(-1.0, rs_strtod(below, NULL));
  ASSERT_EQ(-1.0, rs_strtod(exact, NULL));
  ASSERT_EQ(-1.0, rs_strtod(above, NULL));

  ASSERT_EQ(0, fesetround(FE_TOWARDZERO));
  ASSERT_EQ(low, rs_strtod(below + 1, NULL));
  ASSERT_EQ(1.0, rs_strtod(exact + 1, NULL));
  ASSERT_EQ(1.0, rs_strtod(above + 1, NULL));
  ASSERT_EQ(-low, rs_strtod(below, NULL));
  ASSERT_EQ(-1.0, rs_strtod(exact, NULL));
  ASSERT_EQ(-1.0, rs_strtod(above, NULL));

  ASSERT_EQ(0, fesetround(FE_UPWARD));
  ASSERT_EQ(1.0, rs_strtod(below + 1, NULL));
  ASSERT_EQ(1.0, rs_strtod(exact + 1, NULL));
  ASSERT_EQ(high, rs_strtod(above + 1, NULL));
  ASSERT_EQ(-low, rs_strtod(below, NULL));
  ASSERT_EQ(-1.0, rs_strtod(exact, NULL));
  ASSERT_EQ(-1.0, rs_strtod(above, NULL));

  ASSERT_EQ(0, fesetround(FE_TONEAREST));
}

TEST(strtod, hex2)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* normal = "0x1p-1022";
  const char* highest_subnormal = "0X1.fFfFfFfFfFfFEP-1023";
  double high = 0x1.ffffffffffffep-1023;
  const char* lowest_subnormal = "0x1p-1074";
  const char* underflow = "0x1p-1075";
  const char* above_subnormal = "0x1.ffffffffffffe000001p-1023";

  ASSERT_EQ(DBL_MIN, rs_strtod(normal, NULL));
  ASSERT_EQ(0, rs_errno);
  ASSERT_EQ(high, rs_strtod(highest_subnormal, NULL));
  ASSERT_EQ(0, rs_errno);
  ASSERT_EQ(DBL_TRUE_MIN, rs_strtod(lowest_subnormal, NULL));
  ASSERT_EQ(0, rs_errno);
  ASSERT_EQ(0.0, rs_strtod(underflow, NULL));
  ASSERT_EQ(ERANGE, rs_errno);

  ASSERT_EQ(0, fesetround(FE_DOWNWARD));
  ASSERT_EQ(high, rs_strtod(above_subnormal, NULL));
  ASSERT_EQ(0, fesetround(FE_TONEAREST));
  ASSERT_EQ(high, rs_strtod(above_subnormal, NULL));
  ASSERT_EQ(0, fesetround(FE_TOWARDZERO));
  ASSERT_EQ(high, rs_strtod(above_subnormal, NULL));
  ASSERT_EQ(0, fesetround(FE_UPWARD));
  ASSERT_EQ(DBL_MIN, rs_strtod(above_subnormal, NULL));

  ASSERT_EQ(0, fesetround(FE_TONEAREST));
}

TEST(strtold, dec)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  for (size_t i = 0; i < std::size(tests_long_double); i++) {
    ASSERT_TRUE(rs_strtold(tests_long_double[i].name.data(), nullptr) ==
                tests_long_double[i].value);
  }
}

TEST(strtold, hex1)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  const char* below = "-0x0.ffffffffffffffffffffffffffffffffffffffffffffffffff";
  const char* exact = "-0x1.0";
  const char* above = "-0x1.00000000000000000000000000000000000000000000000001";
  long double low = nexttowardl(1.0L, 0.0L);
  long double high = nexttowardl(1.0L, 2.0L);

  ASSERT_EQ(0, fesetround(FE_DOWNWARD));
  ASSERT_EQ(low, rs_strtold(below + 1, NULL));
  ASSERT_EQ(1.0L, rs_strtold(exact + 1, NULL));
  ASSERT_EQ(1.0L, rs_strtold(above + 1, NULL));
  ASSERT_EQ(-1.0L, rs_strtold(below, NULL));
  ASSERT_EQ(-1.0L, rs_strtold(exact, NULL));
  ASSERT_EQ(-high, rs_strtold(above, NULL));

  ASSERT_EQ(0, fesetround(FE_TONEAREST));
  ASSERT_EQ(1.0L, rs_strtold(below + 1, NULL));
  ASSERT_EQ(1.0L, rs_strtold(exact + 1, NULL));
  ASSERT_EQ(1.0L, rs_strtold(above + 1, NULL));
  ASSERT_EQ(-1.0L, rs_strtold(below, NULL));
  ASSERT_EQ(-1.0L, rs_strtold(exact, NULL));
  ASSERT_EQ(-1.0L, rs_strtold(above, NULL));

  ASSERT_EQ(0, fesetround(FE_TOWARDZERO));
  ASSERT_EQ(low, rs_strtold(below + 1, NULL));
  ASSERT_EQ(1.0L, rs_strtold(exact + 1, NULL));
  ASSERT_EQ(1.0L, rs_strtold(above + 1, NULL));
  ASSERT_EQ(-low, rs_strtold(below, NULL));
  ASSERT_EQ(-1.0L, rs_strtold(exact, NULL));
  ASSERT_EQ(-1.0L, rs_strtold(above, NULL));

  ASSERT_EQ(0, fesetround(FE_UPWARD));
  ASSERT_EQ(1.0L, rs_strtold(below + 1, NULL));
  ASSERT_EQ(1.0L, rs_strtold(exact + 1, NULL));
  ASSERT_EQ(high, rs_strtold(above + 1, NULL));
  ASSERT_EQ(-low, rs_strtold(below, NULL));
  ASSERT_EQ(-1.0L, rs_strtold(exact, NULL));
  ASSERT_EQ(-1.0L, rs_strtold(above, NULL));

  ASSERT_EQ(0, fesetround(FE_TONEAREST));
}

TEST(strtold, hex2)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

#if LDBL_TYPE == LDBL_IS_F64
  const char* normal = "0x1p-1022";
  const char* highest_subnormal = "0x1.ffffffffffffep-1023";
  const char* lowest_subnormal = "0x1p-1074";
  const char* underflow = "0x1p-1075";
#elif LDBL_TYPE == LDBL_IS_F80
  const char* normal = "0x1p-16382";
  const char* highest_subnormal = "0x1.fffffffffffffffcp-16383";
  const char* lowest_subnormal = "0x1p-16445";
  const char* underflow = "0x1p-16446";
#elif LDBL_TYPE == LDBL_IS_F128
  const char* normal = "0x1p-16382";
  const char* highest_subnormal = "0x1.fffffffffffffffffffffffffffep-16383";
  const char* lowest_subnormal = "0x1p-16494";
  const char* underflow = "0x1p-16495";
#endif

  ASSERT_EQ(LDBL_MIN, rs_strtold(normal, NULL));
  ASSERT_EQ(0, rs_errno);
  ASSERT_EQ(nexttowardl(LDBL_MIN, 0.0L), rs_strtold(highest_subnormal, NULL));
  ASSERT_EQ(0, rs_errno);
  ASSERT_EQ(LDBL_TRUE_MIN, rs_strtold(lowest_subnormal, NULL));
  ASSERT_EQ(0, rs_errno);
  ASSERT_EQ(0.0, rs_strtold(underflow, NULL));
  ASSERT_EQ(ERANGE, rs_errno);
}

struct strto_test
{
  const char* str;
  int64_t res;
  int base;
  const char* end;
};

static void
check_end(const strto_test& t, const char* end)
{
  if (t.end != nullptr)
    ASSERT_STREQ(t.end, end);
  else
    ASSERT_EQ('\0', *end);
}

TEST(strtol, base)
{
  const strto_test tests[] = {
    { "123456789", 123456789, 0, nullptr },
    { "111010110111100110100010101", 123456789, 2, nullptr },
    { "22121022020212200", 123456789, 3, nullptr },
    { "13112330310111", 123456789, 4, nullptr },
    { "223101104124", 123456789, 5, nullptr },
    { "20130035113", 123456789, 6, nullptr },
    { "3026236221", 123456789, 7, nullptr },
    { "726746425", 123456789, 8, nullptr },
    { "277266780", 123456789, 9, nullptr },
    { "123456789", 123456789, 10, nullptr },
    { "63762A05", 123456789, 11, nullptr },
    { "35418A99", 123456789, 12, nullptr },
    { "1C767471", 123456789, 13, nullptr },
    { "12579781", 123456789, 14, nullptr },
    { "AC89BC9", 123456789, 15, nullptr },
    { "75BCD15", 123456789, 16, nullptr },
    { "1234567", 342391, 8, nullptr },
    { "01234567", 342391, 0, nullptr },
    { "0123456789", 123456789, 10, nullptr },
    { "0x75bcd15", 123456789, 0, nullptr },
    { " 0xX", 0, 0, "xX" },
    { " 0xX", 0, 16, "xX" },
    { " 0XX", 0, 0, "XX" },
    { " 0XX", 0, 16, "XX" },
  };

  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  for (size_t i = 0; i < sizeof(tests) / sizeof(tests[0]); ++i) {
    char* end;
    long result = rs_strtol(tests[i].str, &end, tests[i].base);
    ASSERT_EQ(tests[i].res, int64_t(result));
    check_end(tests[i], end);
  }
}

TEST(strtoul, base)
{
  const strto_test tests[] = {
    { "123456789", 123456789, 0, nullptr },
    { "111010110111100110100010101", 123456789, 2, nullptr },
    { "22121022020212200", 123456789, 3, nullptr },
    { "13112330310111", 123456789, 4, nullptr },
    { "223101104124", 123456789, 5, nullptr },
    { "20130035113", 123456789, 6, nullptr },
    { "3026236221", 123456789, 7, nullptr },
    { "726746425", 123456789, 8, nullptr },
    { "277266780", 123456789, 9, nullptr },
    { "123456789", 123456789, 10, nullptr },
    { "63762A05", 123456789, 11, nullptr },
    { "35418A99", 123456789, 12, nullptr },
    { "1C767471", 123456789, 13, nullptr },
    { "12579781", 123456789, 14, nullptr },
    { "AC89BC9", 123456789, 15, nullptr },
    { "75BCD15", 123456789, 16, nullptr },
    { "1234567", 342391, 8, nullptr },
    { "01234567", 342391, 0, nullptr },
    { "0123456789", 123456789, 10, nullptr },
    { "0x75bcd15", 123456789, 0, nullptr },
    { " 0xX", 0, 0, "xX" },
    { " 0xX", 0, 16, "xX" },
    { " 0XX", 0, 0, "XX" },
    { " 0XX", 0, 16, "XX" },
  };

  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  for (size_t i = 0; i < sizeof(tests) / sizeof(tests[0]); ++i) {
    char* end;
    unsigned long result = rs_strtoul(tests[i].str, &end, tests[i].base);
    ASSERT_EQ(uint64_t(tests[i].res), uint64_t(result));
    check_end(tests[i], end);
  }
}

TEST(strtoll, base)
{
  const strto_test tests[] = {
    { "123456789", 123456789, 0, nullptr },
    { "111010110111100110100010101", 123456789, 2, nullptr },
    { "22121022020212200", 123456789, 3, nullptr },
    { "13112330310111", 123456789, 4, nullptr },
    { "223101104124", 123456789, 5, nullptr },
    { "20130035113", 123456789, 6, nullptr },
    { "3026236221", 123456789, 7, nullptr },
    { "726746425", 123456789, 8, nullptr },
    { "277266780", 123456789, 9, nullptr },
    { "123456789", 123456789, 10, nullptr },
    { "63762A05", 123456789, 11, nullptr },
    { "35418A99", 123456789, 12, nullptr },
    { "1C767471", 123456789, 13, nullptr },
    { "12579781", 123456789, 14, nullptr },
    { "AC89BC9", 123456789, 15, nullptr },
    { "75BCD15", 123456789, 16, nullptr },
    { "1234567", 342391, 8, nullptr },
    { "01234567", 342391, 0, nullptr },
    { "0123456789", 123456789, 10, nullptr },
    { "0x75bcd15", 123456789, 0, nullptr },
    { " 0xX", 0, 0, "xX" },
    { " 0xX", 0, 16, "xX" },
    { " 0XX", 0, 0, "XX" },
    { " 0XX", 0, 16, "XX" },
  };

  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  for (size_t i = 0; i < sizeof(tests) / sizeof(tests[0]); ++i) {
    char* end;
    long long result = rs_strtoll(tests[i].str, &end, tests[i].base);
    ASSERT_EQ(tests[i].res, int64_t(result));
    check_end(tests[i], end);
  }
}

TEST(strtoull, base)
{
  const strto_test tests[] = {
    { "123456789", 123456789, 0, nullptr },
    { "111010110111100110100010101", 123456789, 2, nullptr },
    { "22121022020212200", 123456789, 3, nullptr },
    { "13112330310111", 123456789, 4, nullptr },
    { "223101104124", 123456789, 5, nullptr },
    { "20130035113", 123456789, 6, nullptr },
    { "3026236221", 123456789, 7, nullptr },
    { "726746425", 123456789, 8, nullptr },
    { "277266780", 123456789, 9, nullptr },
    { "123456789", 123456789, 10, nullptr },
    { "63762A05", 123456789, 11, nullptr },
    { "35418A99", 123456789, 12, nullptr },
    { "1C767471", 123456789, 13, nullptr },
    { "12579781", 123456789, 14, nullptr },
    { "AC89BC9", 123456789, 15, nullptr },
    { "75BCD15", 123456789, 16, nullptr },
    { "1234567", 342391, 8, nullptr },
    { "01234567", 342391, 0, nullptr },
    { "0123456789", 123456789, 10, nullptr },
    { "0x75bcd15", 123456789, 0, nullptr },
    { " 0xX", 0, 0, "xX" },
    { " 0xX", 0, 16, "xX" },
    { " 0XX", 0, 0, "XX" },
    { " 0XX", 0, 16, "XX" },
  };

  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  for (size_t i = 0; i < sizeof(tests) / sizeof(tests[0]); ++i) {
    char* end;
    unsigned long long result = rs_strtoull(tests[i].str, &end, tests[i].base);
    ASSERT_EQ(uint64_t(tests[i].res), uint64_t(result));
    check_end(tests[i], end);
  }
}

TEST(strtol, invbase)
{
  char boo[] = "boo";
  const char str[] = "1";
  char* end = boo;

  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  long result = rs_strtol(str, &end, -2);
  ASSERT_STREQ(str, end);
  ASSERT_EQ(0, result);
  ASSERT_EQ(EINVAL, rs_errno);
}

TEST(strtoul, invbase)
{
  char boo[] = "boo";
  const char str[] = "1";
  char* end = boo;

  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  unsigned long result = rs_strtoul(str, &end, -2);
  ASSERT_STREQ(str, end);
  ASSERT_EQ(0UL, result);
  ASSERT_EQ(EINVAL, rs_errno);
}

TEST(strtoll, invbase)
{
  char boo[] = "boo";
  const char str[] = "1";
  char* end = boo;

  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  long long result = rs_strtoll(str, &end, -2);
  ASSERT_STREQ(str, end);
  ASSERT_EQ(0LL, result);
  ASSERT_EQ(EINVAL, rs_errno);
}

TEST(strtoull, invbase)
{
  char boo[] = "boo";
  const char str[] = "1";
  char* end = boo;

  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  unsigned long long result = rs_strtoull(str, &end, -2);
  ASSERT_STREQ(str, end);
  ASSERT_EQ(0ULL, result);
  ASSERT_EQ(EINVAL, rs_errno);
}

TEST(strtol, case_insensitive)
{
  const strto_test tests[] = {
    { "abcd", 0xabcd, 16, nullptr },
    { "     dcba", 0xdcba, 16, nullptr },
    { "abcd dcba", 0xabcd, 16, " dcba" },
    { "abc0x123", 0xabc0, 16, "x123" },
    { "abcd\0x123", 0xabcd, 16, "\0x123" },
    { "ABCD", 0xabcd, 16, nullptr },
    { "aBcD", 0xabcd, 16, nullptr },
    { "0xABCD", 0xabcd, 16, nullptr },
    { "0xABCDX", 0xabcd, 16, "X" },
  };

  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  for (size_t i = 0; i < sizeof(tests) / sizeof(tests[0]); ++i) {
    char* end;
    long result = rs_strtol(tests[i].str, &end, tests[i].base);
    ASSERT_EQ(tests[i].res, int64_t(result));
    check_end(tests[i], end);
  }
}

TEST(strtoul, case_insensitive)
{
  const strto_test tests[] = {
    { "abcd", 0xabcd, 16, nullptr },
    { "     dcba", 0xdcba, 16, nullptr },
    { "abcd dcba", 0xabcd, 16, " dcba" },
    { "abc0x123", 0xabc0, 16, "x123" },
    { "abcd\0x123", 0xabcd, 16, "\0x123" },
    { "ABCD", 0xabcd, 16, nullptr },
    { "aBcD", 0xabcd, 16, nullptr },
    { "0xABCD", 0xabcd, 16, nullptr },
    { "0xABCDX", 0xabcd, 16, "X" },
  };

  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  for (size_t i = 0; i < sizeof(tests) / sizeof(tests[0]); ++i) {
    char* end;
    unsigned long result = rs_strtoul(tests[i].str, &end, tests[i].base);
    ASSERT_EQ(uint64_t(tests[i].res), uint64_t(result));
    check_end(tests[i], end);
  }
}

TEST(strtoll, case_insensitive)
{
  const strto_test tests[] = {
    { "abcd", 0xabcd, 16, nullptr },
    { "     dcba", 0xdcba, 16, nullptr },
    { "abcd dcba", 0xabcd, 16, " dcba" },
    { "abc0x123", 0xabc0, 16, "x123" },
    { "abcd\0x123", 0xabcd, 16, "\0x123" },
    { "ABCD", 0xabcd, 16, nullptr },
    { "aBcD", 0xabcd, 16, nullptr },
    { "0xABCD", 0xabcd, 16, nullptr },
    { "0xABCDX", 0xabcd, 16, "X" },
  };

  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  for (size_t i = 0; i < sizeof(tests) / sizeof(tests[0]); ++i) {
    char* end;
    long long result = rs_strtoll(tests[i].str, &end, tests[i].base);
    ASSERT_EQ(tests[i].res, int64_t(result));
    check_end(tests[i], end);
  }
}

TEST(strtoull, case_insensitive)
{
  const strto_test tests[] = {
    { "abcd", 0xabcd, 16, nullptr },
    { "     dcba", 0xdcba, 16, nullptr },
    { "abcd dcba", 0xabcd, 16, " dcba" },
    { "abc0x123", 0xabc0, 16, "x123" },
    { "abcd\0x123", 0xabcd, 16, "\0x123" },
    { "ABCD", 0xabcd, 16, nullptr },
    { "aBcD", 0xabcd, 16, nullptr },
    { "0xABCD", 0xabcd, 16, nullptr },
    { "0xABCDX", 0xabcd, 16, "X" },
  };

  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  for (size_t i = 0; i < sizeof(tests) / sizeof(tests[0]); ++i) {
    char* end;
    unsigned long long result = rs_strtoull(tests[i].str, &end, tests[i].base);
    ASSERT_EQ(uint64_t(tests[i].res), uint64_t(result));
    check_end(tests[i], end);
  }
}

TEST(strtol, range)
{
#if LONG_MAX == 0x7fffffff
  const strto_test tests[] = {
    { "20000000000", 2147483647, 8, nullptr },
    { "2147483648", 2147483647, 10, nullptr },
    { "80000000", 2147483647, 16, nullptr },
  };
#else
  const strto_test tests[] = {
    { "1000000000000000000000", 9223372036854775807LL, 8, nullptr },
    { "9223372036854775808", 9223372036854775807LL, 10, nullptr },
    { "8000000000000000", 9223372036854775807LL, 16, nullptr },
  };
#endif

  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  for (size_t i = 0; i < sizeof(tests) / sizeof(tests[0]); ++i) {
    rs_errno = 0;
    char* end;
    long result = rs_strtol(tests[i].str, &end, tests[i].base);
    ASSERT_EQ(ERANGE, rs_errno);
    ASSERT_EQ(tests[i].res, int64_t(result));
    check_end(tests[i], end);
  }
}

TEST(strtoul, range)
{
  const struct
  {
    const char* str;
    unsigned long res;
    int base;
  } tests[] = {
    { "18446744073709551616", ULONG_MAX, 10 },
    { "10000000000000000000000", ULONG_MAX, 8 },
    { "10000000000000000", ULONG_MAX, 16 },
  };

  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  for (size_t i = 0; i < sizeof(tests) / sizeof(tests[0]); ++i) {
    rs_errno = 0;
    char* end;
    unsigned long result = rs_strtoul(tests[i].str, &end, tests[i].base);
    ASSERT_EQ(ERANGE, rs_errno);
    ASSERT_EQ(tests[i].res, result);
    ASSERT_EQ('\0', *end);
  }
}

TEST(strtoull, range)
{
  const struct
  {
    const char* str;
    unsigned long long res;
    int base;
  } tests[] = {
    { "18446744073709551616", ULLONG_MAX, 10 },
    { "10000000000000000000000", ULLONG_MAX, 8 },
    { "10000000000000000", ULLONG_MAX, 16 },
  };

  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  for (size_t i = 0; i < sizeof(tests) / sizeof(tests[0]); ++i) {
    rs_errno = 0;
    char* end;
    unsigned long long result = rs_strtoull(tests[i].str, &end, tests[i].base);
    ASSERT_EQ(ERANGE, rs_errno);
    ASSERT_EQ(tests[i].res, result);
    ASSERT_EQ('\0', *end);
  }
}

TEST(strtol, signed)
{
  const strto_test tests[] = {
    { "1", 1, 0, nullptr },      { " 2", 2, 0, nullptr },
    { "  3", 3, 0, nullptr },    { " -3", -3, 0, nullptr },
    { "--1", 0, 0, "--1" },      { "+-2", 0, 0, "+-2" },
    { "++3", 0, 0, "++3" },      { "+9", 9, 0, nullptr },
    { "+123", 123, 0, nullptr }, { "-1 3", -1, 0, " 3" },
    { "-1.3", -1, 0, ".3" },     { "-  3", 0, 0, "-  3" },
    { "+33.", 33, 0, "." },      { "30x0", 30, 0, "x0" },
  };

  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  for (size_t i = 0; i < sizeof(tests) / sizeof(tests[0]); ++i) {
    char* end;
    long result = rs_strtol(tests[i].str, &end, tests[i].base);
    ASSERT_EQ(tests[i].res, int64_t(result));
    check_end(tests[i], end);
  }
}

TEST(strtoul, signed)
{
  const struct
  {
    const char* str;
    uint64_t res;
    int base;
    const char* end;
  } tests[] = {
    { "1", 1, 0, nullptr },
    { " 2", 2, 0, nullptr },
    { "  3", 3, 0, nullptr },
    { " -3", uint64_t(-3LL), 0, nullptr },
    { "--1", 0, 0, "--1" },
    { "+-2", 0, 0, "+-2" },
    { "++3", 0, 0, "++3" },
    { "+9", 9, 0, nullptr },
    { "+123", 123, 0, nullptr },
    { "-1 3", uint64_t(-1LL), 0, " 3" },
    { "-1.3", uint64_t(-1LL), 0, ".3" },
    { "-  3", 0, 0, "-  3" },
    { "+33.", 33, 0, "." },
    { "30x0", 30, 0, "x0" },
  };

  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  for (size_t i = 0; i < sizeof(tests) / sizeof(tests[0]); ++i) {
    char* end;
    unsigned long result = rs_strtoul(tests[i].str, &end, tests[i].base);
    ASSERT_EQ(tests[i].res, uint64_t(result));
    if (tests[i].end != nullptr)
      ASSERT_STREQ(tests[i].end, end);
    else
      ASSERT_EQ('\0', *end);
  }
}

TEST(strtoll, signed)
{
  const strto_test tests[] = {
    { "1", 1, 0, nullptr },      { " 2", 2, 0, nullptr },
    { "  3", 3, 0, nullptr },    { " -3", -3, 0, nullptr },
    { "--1", 0, 0, "--1" },      { "+-2", 0, 0, "+-2" },
    { "++3", 0, 0, "++3" },      { "+9", 9, 0, nullptr },
    { "+123", 123, 0, nullptr }, { "-1 3", -1, 0, " 3" },
    { "-1.3", -1, 0, ".3" },     { "-  3", 0, 0, "-  3" },
    { "+33.", 33, 0, "." },      { "30x0", 30, 0, "x0" },
  };

  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  for (size_t i = 0; i < sizeof(tests) / sizeof(tests[0]); ++i) {
    char* end;
    long long result = rs_strtoll(tests[i].str, &end, tests[i].base);
    ASSERT_EQ(tests[i].res, int64_t(result));
    check_end(tests[i], end);
  }
}

TEST(strtoull, signed)
{
  const struct
  {
    const char* str;
    uint64_t res;
    int base;
    const char* end;
  } tests[] = {
    { "1", 1, 0, nullptr },
    { " 2", 2, 0, nullptr },
    { "  3", 3, 0, nullptr },
    { " -3", uint64_t(-3LL), 0, nullptr },
    { "--1", 0, 0, "--1" },
    { "+-2", 0, 0, "+-2" },
    { "++3", 0, 0, "++3" },
    { "+9", 9, 0, nullptr },
    { "+123", 123, 0, nullptr },
    { "-1 3", uint64_t(-1LL), 0, " 3" },
    { "-1.3", uint64_t(-1LL), 0, ".3" },
    { "-  3", 0, 0, "-  3" },
    { "+33.", 33, 0, "." },
    { "30x0", 30, 0, "x0" },
  };

  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  for (size_t i = 0; i < sizeof(tests) / sizeof(tests[0]); ++i) {
    char* end;
    unsigned long long result = rs_strtoull(tests[i].str, &end, tests[i].base);
    ASSERT_EQ(tests[i].res, uint64_t(result));
    if (tests[i].end != nullptr)
      ASSERT_STREQ(tests[i].end, end);
    else
      ASSERT_EQ('\0', *end);
  }
}

TEST(mblen, bad)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  ASSERT_EQ(-1, rs_mblen("", 0));
  ASSERT_EQ(EILSEQ, rs_errno);
  ASSERT_EQ(-1, rs_mblen("Hello", 0));
  ASSERT_EQ(EILSEQ, rs_errno);
}

TEST(mblen, ascii)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  ASSERT_EQ(0, rs_mblen(NULL, 12345));

  char c = 0;
  ASSERT_EQ(0, rs_mblen(&c, 12345));
  for (int i = 1; i < 128; ++i) {
    SCOPED_TRACE(i);
    c = i;
    ASSERT_EQ(1, rs_mblen(&c, 12345));
  }
  for (int i = 128; i < 256; ++i) {
    SCOPED_TRACE(i);
    c = i;
    ASSERT_EQ(-1, rs_mblen(&c, 12345));
    ASSERT_EQ(EILSEQ, rs_errno);
  }
}

TEST(mblen, unicode)
{
  REVERT_TO_UNICODE_LOCALE();

  rs_errno = 0;

  char euro[] = "€";
  for (size_t i = 0; i < sizeof(euro) - 1; ++i) {
    ASSERT_EQ(-1, rs_mblen(euro, i));
    ASSERT_EQ(EILSEQ, rs_errno);
  }

  ASSERT_EQ(sizeof(euro) - 1, rs_mblen(euro, sizeof(euro) - 1));
  ASSERT_EQ(sizeof(euro) - 1, rs_mblen(euro, sizeof(euro)));
}

TEST(mbtowc, bad)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  ASSERT_EQ(-1, rs_mbtowc(NULL, "", 0));
  ASSERT_EQ(EILSEQ, rs_errno);
  ASSERT_EQ(-1, rs_mbtowc(NULL, "Hello", 0));
  ASSERT_EQ(EILSEQ, rs_errno);
}

TEST(mbtowc, ascii)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  ASSERT_EQ(0, rs_mbtowc(NULL, NULL, 12345));

  wchar_t wc;
  char c = 0;
  ASSERT_EQ(0, rs_mbtowc(&wc, &c, 12345));
  ASSERT_EQ(0, wc);
  for (int i = 1; i < 128; ++i) {
    SCOPED_TRACE(i);
    c = i;
    ASSERT_EQ(1, rs_mbtowc(&wc, &c, 12345));
    ASSERT_EQ(i, wc);
  }
  for (int i = 128; i < 256; ++i) {
    SCOPED_TRACE(i);
    c = i;
    ASSERT_EQ(-1, rs_mbtowc(NULL, &c, 12345));
    ASSERT_EQ(EILSEQ, rs_errno);
  }
}

TEST(mbtowc, unicode)
{
  REVERT_TO_UNICODE_LOCALE();

  rs_errno = 0;

  char euro[] = "€";
  for (size_t i = 0; i < sizeof(euro) - 1; ++i) {
    ASSERT_EQ(-1, rs_mbtowc(NULL, euro, i));
    ASSERT_EQ(EILSEQ, rs_errno);
  }

  wchar_t wc;
  ASSERT_EQ(sizeof(euro) - 1, rs_mbtowc(&wc, euro, sizeof(euro) - 1));
  ASSERT_EQ(L'€', wc);
  ASSERT_EQ(sizeof(euro) - 1, rs_mbtowc(&wc, euro, sizeof(euro)));
  ASSERT_EQ(L'€', wc);
}

TEST(mbstowcs, bad)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  ASSERT_EQ(-1, rs_mbstowcs(NULL, "München", 42));
  ASSERT_EQ(EILSEQ, rs_errno);
}

TEST(mbstowcs, zero)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  ASSERT_EQ(0, rs_mbstowcs((wchar_t*)0x42, "Hello", 0));
}

TEST(mbstowcs, length)
{
  SET_GLOBAL_LOCALE("nl_NL.UTF-8");

  rs_errno = 0;

  ASSERT_EQ(10, rs_mbstowcs(NULL, "Düsseldorf", 0));
  ASSERT_EQ(10, rs_mbstowcs(NULL, "Düsseldorf", 5));
  ASSERT_EQ(10, rs_mbstowcs(NULL, "Düsseldorf", 40));
  ASSERT_EQ(10, rs_mbstowcs(NULL, "Düsseldorf", SIZE_MAX));
}

TEST(mbstowcs, convert)
{
  REVERT_TO_UNICODE_LOCALE();

  rs_errno = 0;

  {
    wchar_t buf[] = L"AAAAAAAAAAAA";
    ASSERT_EQ(0, rs_mbstowcs(buf, "Düsseldorf", 0));
    ASSERT_THAT(buf, testing::ElementsAreArray(L"AAAAAAAAAAAA"));
  }
  {
    wchar_t buf[] = L"AAAAAAAAAAAA";
    ASSERT_EQ(4, rs_mbstowcs(buf, "Düsseldorf", 4));
    ASSERT_THAT(buf, testing::ElementsAreArray(L"DüssAAAAAAAA"));
  }
  {
    wchar_t buf[] = L"AAAAAAAAAAAA";
    ASSERT_EQ(9, rs_mbstowcs(buf, "Düsseldorf", 9));
    ASSERT_THAT(buf, testing::ElementsAreArray(L"DüsseldorAAA"));
  }
  {
    wchar_t buf[] = L"AAAAAAAAAAAA";
    ASSERT_EQ(10, rs_mbstowcs(buf, "Düsseldorf", 10));
    ASSERT_THAT(buf, testing::ElementsAreArray(L"DüsseldorfAA"));
  }
  {
    wchar_t buf[] = L"AAAAAAAAAAAA";
    ASSERT_EQ(10, rs_mbstowcs(buf, "Düsseldorf", 11));
    ASSERT_THAT(buf, testing::ElementsAreArray(L"Düsseldorf\0A"));
  }
  {
    wchar_t buf[] = L"AAAAAAAAAAAA";
    ASSERT_EQ(10, rs_mbstowcs(buf, "Düsseldorf", 12));
    ASSERT_THAT(buf, testing::ElementsAreArray(L"Düsseldorf\0A"));
  }
}
TEST(wctomb, ascii)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  ASSERT_EQ(0, rs_wctomb(NULL, L'€'));

  for (int i = 0; i < 128; ++i) {
    SCOPED_TRACE(i);
    char c;
    ASSERT_EQ(1, rs_wctomb(&c, i));
    ASSERT_EQ(i, c);
  }
  for (int i = 128; i < 256; ++i) {
    SCOPED_TRACE(i);
    char c;
    ASSERT_EQ(-1, rs_wctomb(&c, i));
    ASSERT_EQ(EILSEQ, rs_errno);
  }
}

TEST(wctomb, unicode)
{
  REVERT_TO_UNICODE_LOCALE();

  rs_errno = 0;

  char buf[MB_LEN_MAX];
  ASSERT_EQ(sizeof("€") - 1, rs_wctomb(buf, L'€'));
  ASSERT_THAT(buf, testing::StartsWith("€"));
}

TEST(wcstombs, bad)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  ASSERT_EQ(-1, rs_wcstombs(NULL, L"München", 42));
  ASSERT_EQ(EILSEQ, rs_errno);
}

TEST(wcstombs, zero)
{
  REVERT_TO_C_LOCALE();

  rs_errno = 0;

  ASSERT_EQ(0, rs_wcstombs((char*)0x42, L"Hello", 0));
}

TEST(wcstombs, length)
{
  REVERT_TO_UNICODE_LOCALE();

  rs_errno = 0;

  ASSERT_EQ(11, rs_wcstombs(NULL, L"Düsseldorf", 0));
  ASSERT_EQ(11, rs_wcstombs(NULL, L"Düsseldorf", 5));
  ASSERT_EQ(11, rs_wcstombs(NULL, L"Düsseldorf", 40));
  ASSERT_EQ(11, rs_wcstombs(NULL, L"Düsseldorf", SIZE_MAX));
}

TEST(wcstombs, convert)
{
  REVERT_TO_UNICODE_LOCALE();

  rs_errno = 0;

  {
    char buf[] = "AAAAAAAAAAAAA";
    ASSERT_EQ(0, rs_wcstombs(buf, L"Düsseldorf", 0));
    ASSERT_THAT(buf, testing::ElementsAreArray("AAAAAAAAAAAAA"));
  }
  {
    char buf[] = "AAAAAAAAAAAAA";
    ASSERT_EQ(1, rs_wcstombs(buf, L"Düsseldorf", 1));
    ASSERT_THAT(buf, testing::ElementsAreArray("DAAAAAAAAAAAA"));
  }
  {
    char buf[] = "AAAAAAAAAAAAA";
    ASSERT_EQ(1, rs_wcstombs(buf, L"Düsseldorf", 2));
    ASSERT_THAT(buf, testing::ElementsAreArray("DAAAAAAAAAAAA"));
  }
  {
    char buf[] = "AAAAAAAAAAAAA";
    ASSERT_EQ(3, rs_wcstombs(buf, L"Düsseldorf", 3));
    ASSERT_THAT(buf, testing::ElementsAreArray("DüAAAAAAAAAA"));
  }
  {
    char buf[] = "AAAAAAAAAAAAA";
    ASSERT_EQ(10, rs_wcstombs(buf, L"Düsseldorf", 10));
    ASSERT_THAT(buf, testing::ElementsAreArray("DüsseldorAAA"));
  }
  {
    char buf[] = "AAAAAAAAAAAAA";
    ASSERT_EQ(11, rs_wcstombs(buf, L"Düsseldorf", 11));
    ASSERT_THAT(buf, testing::ElementsAreArray("DüsseldorfAA"));
  }
  {
    char buf[] = "AAAAAAAAAAAAA";
    ASSERT_EQ(11, rs_wcstombs(buf, L"Düsseldorf", 12));
    ASSERT_THAT(buf, testing::ElementsAreArray("Düsseldorf\0A"));
  }
  {
    char buf[] = "AAAAAAAAAAAAA";
    ASSERT_EQ(11, rs_wcstombs(buf, L"Düsseldorf", 13));
    ASSERT_THAT(buf, testing::ElementsAreArray("Düsseldorf\0A"));
  }
}

TEST(div, examples)
{
  div_t r = rs_div(5, 2);
  ASSERT_EQ(2, r.quot);
  ASSERT_EQ(1, r.rem);

  r = rs_div(-11, 2);
  ASSERT_EQ(-5, r.quot);
  ASSERT_EQ(-1, r.rem);

  r = rs_div(-43, -3);
  ASSERT_EQ(14, r.quot);
  ASSERT_EQ(-1, r.rem);

  r = rs_div(18, -7);
  ASSERT_EQ(-2, r.quot);
  ASSERT_EQ(4, r.rem);
}

TEST(ldiv, examples)
{
  ldiv_t r = rs_ldiv(5, 2);
  ASSERT_EQ(2, r.quot);
  ASSERT_EQ(1, r.rem);

  r = rs_ldiv(-11, 2);
  ASSERT_EQ(-5, r.quot);
  ASSERT_EQ(-1, r.rem);

  r = rs_ldiv(-43, -3);
  ASSERT_EQ(14, r.quot);
  ASSERT_EQ(-1, r.rem);

  r = rs_ldiv(18, -7);
  ASSERT_EQ(-2, r.quot);
  ASSERT_EQ(4, r.rem);
}

TEST(lldiv, examples)
{
  lldiv_t r = rs_lldiv(5, 2);
  ASSERT_EQ(2, r.quot);
  ASSERT_EQ(1, r.rem);

  r = rs_lldiv(-11, 2);
  ASSERT_EQ(-5, r.quot);
  ASSERT_EQ(-1, r.rem);

  r = rs_lldiv(-43, -3);
  ASSERT_EQ(14, r.quot);
  ASSERT_EQ(-1, r.rem);

  r = rs_lldiv(18, -7);
  ASSERT_EQ(-2, r.quot);
  ASSERT_EQ(4, r.rem);
}

TEST(abs, example)
{
  ASSERT_EQ(0, rs_abs(0));
  ASSERT_EQ(42, rs_abs(42));
  ASSERT_EQ(42, rs_abs(-42));
  ASSERT_EQ(INT_MAX, rs_abs(INT_MAX));
  ASSERT_EQ(INT_MAX, rs_abs(-INT_MAX));
}

TEST(labs, example)
{
  ASSERT_EQ(0, rs_labs(0));
  ASSERT_EQ(42, rs_labs(42));
  ASSERT_EQ(42, rs_labs(-42));
  ASSERT_EQ(LONG_MAX, rs_labs(LONG_MAX));
  ASSERT_EQ(LONG_MAX, rs_labs(-LONG_MAX));
}

TEST(llabs, example)
{
  ASSERT_EQ(0, rs_llabs(0));
  ASSERT_EQ(42, rs_llabs(42));
  ASSERT_EQ(42, rs_llabs(-42));
  ASSERT_EQ(LLONG_MAX, rs_llabs(LLONG_MAX));
  ASSERT_EQ(LLONG_MAX, rs_llabs(-LLONG_MAX));
}

/*
 * Copyright (c) 2026 Faraz Vahedi <kfv@kfv.io>
 *
 * SPDX-License-Identifier: BSD-2-Clause
 */

#define FBUF 8192

static inline void
check_d(const char* fmt, double val)
{
  char got[FBUF], ref[FBUF];
  int rlen, glen;

  // TODO: replace with rs_snprintf
  rlen = snprintf(ref, sizeof(ref), fmt, val);
  glen = rs_strfromd(got, sizeof(got), fmt, val);
  ASSERT_STREQ(ref, got);
  ASSERT_EQ(glen, rlen);
}

static inline void
check_f(const char* fmt, float val)
{
  char got[FBUF], ref[FBUF];
  int rlen, glen;

  // TODO: replace with rs_snprintf
  rlen = snprintf(ref, sizeof(ref), fmt, (double)val);
  glen = rs_strfromf(got, sizeof(got), fmt, val);
  ASSERT_STREQ(ref, got);
  ASSERT_EQ(glen, rlen);
}

static inline void
check_l(const char* fmt, long double val)
{
  fesetround(FE_TONEAREST);

  char got[FBUF], ref[FBUF], lfmt[32];
  int rlen, glen;

  /*
   * snprintf requires the 'L' length modifier for long double.
   */
  // TODO: replace with rs_snprintf
  snprintf(lfmt,
           sizeof(lfmt),
           "%.*sL%c",
           (int)(strlen(fmt) - 1),
           fmt,
           fmt[strlen(fmt) - 1]);
  rlen = snprintf(ref, sizeof(ref), lfmt, val);
  glen = rs_strfroml(got, sizeof(got), fmt, val);
  ASSERT_STREQ(ref, got);
  ASSERT_EQ(glen, rlen);
}

TEST(strfromf, fixed)
{
  REVERT_TO_C_LOCALE();

  check_f("%f", 0.0f);
  check_f("%f", 1.0f);
  check_f("%f", -1.0f);
  check_f("%f", 3.14159f);
  check_f("%.0f", 3.5f);
  check_f("%.2f", 3.14159f);
  check_f("%.9f", 1.0f / 3.0f);
  check_f("%f", FLT_MAX);
  check_f("%f", FLT_MIN);
}

TEST(strfromf, scientific)
{
  char buf[FBUF];

  REVERT_TO_C_LOCALE();

  check_f("%e", 0.0f);
  check_f("%e", 1.0f);
  check_f("%e", -1.0f);
  check_f("%e", 3.14159f);
  check_f("%E", 2.71828f);
  check_f("%.2e", 12345.6f);
  check_f("%.0e", 3.14f);

  rs_strfromf(buf, sizeof(buf), "%e", 1.0f);
  ASSERT_STREQ("1.000000e+00", buf);
}

TEST(strfromf, automatic)
{
  char buf[FBUF];

  REVERT_TO_C_LOCALE();

  check_f("%g", 0.0f);
  check_f("%g", 1.0f);
  check_f("%g", 100000.0f);
  check_f("%g", 1000000.0f);
  check_f("%g", 0.0001f);
  check_f("%g", 0.00001f);
  check_f("%G", 1.23e10f);
  check_f("%.2g", 3.14159f);

  rs_strfromf(buf, sizeof(buf), "%g", 1.0f);
  ASSERT_STREQ("1", buf);
}

TEST(strfromf, hexadecimal)
{
  char buf[FBUF];

  REVERT_TO_C_LOCALE();

  check_f("%a", 0.0f);
  check_f("%a", 1.0f);
  check_f("%a", 0.5f);
  check_f("%a", -1.0f);
  check_f("%A", 1.0f);
  check_f("%.2a", 1.0f / 3.0f);

  rs_strfromf(buf, sizeof(buf), "%a", 1.0f);
  ASSERT_STREQ("0x1p+0", buf);
}

TEST(strfromf, inf_nan)
{
  REVERT_TO_C_LOCALE();

  check_f("%f", INFINITY);
  check_f("%f", -INFINITY);
  check_f("%f", NAN);
  check_f("%e", INFINITY);
  check_f("%g", INFINITY);
  check_f("%a", INFINITY);
  check_f("%F", INFINITY);
  check_f("%f", -0.0f);
  check_f("%g", -0.0f);
}

TEST(strfromf, truncation)
{
  REVERT_TO_C_LOCALE();

  char buf[FBUF];
  int ret;

  buf[0] = 'X';
  ret = rs_strfromf(buf, 0, "%f", 1.0f);
  ASSERT_GT(ret, 0);
  ASSERT_EQ(buf[0], 'X');

  ret = rs_strfromf(buf, 5, "%.6f", 1.0f);
  ASSERT_EQ(ret, 8);
  ASSERT_EQ(buf[4], '\0');
}

TEST(strfromd, fixed)
{
  char buf[FBUF];

  REVERT_TO_C_LOCALE();

  check_d("%f", 0.0);
  check_d("%f", 1.0);
  check_d("%f", -1.0);
  check_d("%f", 3.14159265358979);
  check_d("%f", 1.0 / 3.0);
  check_d("%f", 1e10);
  check_d("%f", 1e-10);
  check_d("%f", DBL_MAX);
  check_d("%f", DBL_MIN);
  check_d("%.0f", 3.14);
  check_d("%.0f", 3.5);
  check_d("%.0f", 4.5);
  check_d("%.2f", 3.14159);
  check_d("%.10f", 1.0 / 7.0);
  check_d("%.20f", 1.0);

  rs_strfromd(buf, sizeof(buf), "%.6f", 0.0);
  ASSERT_STREQ("0.000000", buf);
  rs_strfromd(buf, sizeof(buf), "%.6f", -1.5);
  ASSERT_STREQ("-1.500000", buf);
}

TEST(strfromd, scientific)
{
  char buf[FBUF];

  REVERT_TO_C_LOCALE();

  check_d("%e", 0.0);
  check_d("%e", 1.0);
  check_d("%e", -1.0);
  check_d("%e", 3.14159265358979);
  check_d("%e", 1e100);
  check_d("%e", 1e-100);
  check_d("%E", 2.718281828);
  check_d("%.0e", 3.14);
  check_d("%.2e", 12345.6789);
  check_d("%.15e", 1.0 / 3.0);

  rs_strfromd(buf, sizeof(buf), "%e", 1.0);
  ASSERT_STREQ("1.000000e+00", buf);
  rs_strfromd(buf, sizeof(buf), "%E", 1.0);
  ASSERT_STREQ("1.000000E+00", buf);
}

TEST(strfromd, automatic)
{
  char buf[FBUF];

  REVERT_TO_C_LOCALE();

  check_d("%g", 0.0);
  check_d("%g", 1.0);
  check_d("%g", 100000.0);
  check_d("%g", 1000000.0); /* exponent >= prec:   hence %e */
  check_d("%g", 0.0001);    /* exponent == -4:     hence %f */
  check_d("%g", 0.00001);   /* exponent < -4:      hence %e */
  check_d("%g", 3.14159265358979);
  check_d("%G", 1.23e10);
  check_d("%.2g", 3.14159);
  check_d("%.0g", 3.14159);
  check_d("%.10g", 1.0 / 7.0);
  check_d("%g", DBL_MIN);
  check_d("%g", DBL_MAX);

  rs_strfromd(buf, sizeof(buf), "%g", 1.0);
  ASSERT_STREQ("1", buf);
  rs_strfromd(buf, sizeof(buf), "%g", 100.0);
  ASSERT_STREQ("100", buf);
}

TEST(strfromd, hexadecimal)
{
  char buf[FBUF];

  REVERT_TO_C_LOCALE();

  check_d("%a", 0.0);
  check_d("%a", 1.0);
  check_d("%a", -1.0);
  check_d("%a", 0.5);
  check_d("%a", 1.5);
  check_d("%a", DBL_MIN);
  check_d("%a", DBL_MAX);
  check_d("%A", 1.0);
  check_d("%.4a", 1.0 / 3.0);
  check_d("%.0a", 1.5);

  rs_strfromd(buf, sizeof(buf), "%a", 1.0);
  ASSERT_STREQ("0x1p+0", buf);
  rs_strfromd(buf, sizeof(buf), "%a", 0.5);
  ASSERT_STREQ("0x1p-1", buf);
  rs_strfromd(buf, sizeof(buf), "%A", 1.0);
  ASSERT_STREQ("0X1P+0", buf);
}

TEST(strfromd, inf_nan)
{
  char buf[FBUF];

  REVERT_TO_C_LOCALE();

  check_d("%f", INFINITY);
  check_d("%f", -INFINITY);
  check_d("%f", NAN);
  check_d("%e", INFINITY);
  check_d("%e", -INFINITY);
  check_d("%e", NAN);
  check_d("%g", INFINITY);
  check_d("%g", -INFINITY);
  check_d("%g", NAN);
  check_d("%a", INFINITY);
  check_d("%a", -INFINITY);
  check_d("%a", NAN);
  check_d("%F", INFINITY);
  check_d("%E", INFINITY);
  check_d("%G", INFINITY);
  check_d("%A", INFINITY);
  check_d("%f", -0.0);
  check_d("%g", -0.0);
  check_d("%e", -0.0);
  check_d("%a", -0.0);

  rs_strfromd(buf, sizeof(buf), "%f", INFINITY);
  ASSERT_STREQ("inf", buf);
  rs_strfromd(buf, sizeof(buf), "%F", INFINITY);
  ASSERT_STREQ("INF", buf);
  rs_strfromd(buf, sizeof(buf), "%f", -INFINITY);
  ASSERT_STREQ("-inf", buf);
  rs_strfromd(buf, sizeof(buf), "%f", NAN);
  ASSERT_STREQ("nan", buf);
  rs_strfromd(buf, sizeof(buf), "%F", NAN);
  ASSERT_STREQ("NAN", buf);
}

TEST(strfromd, truncation)
{
  REVERT_TO_C_LOCALE();

  char buf[FBUF];
  int ret;

  buf[0] = 'X';
  ret = rs_strfromd(buf, 0, "%f", 1.0);
  ASSERT_GT(ret, 0);
  ASSERT_EQ(buf[0], 'X');

  ret = rs_strfromd(buf, 5, "%.6f", 1.0);
  ASSERT_EQ(ret, 8);
  ASSERT_EQ(buf[4], '\0');
}

TEST(strfroml, fixed)
{
  check_l("%f", 0.0L);
  check_l("%f", 1.0L);
  check_l("%f", -1.0L);
  check_l("%f", 3.14159265358979323846L);
  check_l("%.0f", 3.5L);
  check_l("%.2f", 3.14159L);
  check_l("%.15f", 1.0L / 3.0L);
  check_l("%f", LDBL_MAX);
  check_l("%f", LDBL_MIN);
}

TEST(strfroml, scientific)
{
  char buf[FBUF];

  REVERT_TO_C_LOCALE();

  check_l("%e", 0.0L);
  check_l("%e", 1.0L);
  check_l("%e", -1.0L);
  check_l("%e", 3.14159265358979323846L);
  check_l("%E", 2.71828182845904523536L);
  check_l("%.2e", 12345.6789L);
  check_l("%.0e", 3.14L);
  check_l("%e", LDBL_MAX);
  check_l("%e", LDBL_MIN);

  rs_strfroml(buf, sizeof(buf), "%e", 1.0L);
  ASSERT_STREQ("1.000000e+00", buf);
}

TEST(strfroml, automatic)
{
  char buf[FBUF];

  REVERT_TO_C_LOCALE();

  check_l("%g", 0.0L);
  check_l("%g", 1.0L);
  check_l("%g", 100000.0L);
  check_l("%g", 1000000.0L);
  check_l("%g", 0.0001L);
  check_l("%g", 0.00001L);
  check_l("%G", 1.23e10L);
  check_l("%.2g", 3.14159L);
  check_l("%.0g", 3.14159L);

  rs_strfroml(buf, sizeof(buf), "%g", 1.0L);
  ASSERT_STREQ("1", buf);
}

TEST(strfroml, hexadecimal)
{
  char buf[FBUF];

  REVERT_TO_C_LOCALE();

  check_l("%a", 0.0L);
  check_l("%a", 1.0L);
  check_l("%a", 0.5L);
  check_l("%a", -1.0L);
  check_l("%A", 1.0L);
  check_l("%.4a", 1.0L / 3.0L);
  check_l("%.0a", 1.5L);
  check_l("%a", LDBL_MAX);
  check_l("%a", LDBL_MIN);

  rs_strfroml(buf, sizeof(buf), "%a", 1.0L);
#if LDBL_TYPE == LDBL_IS_F80
  ASSERT_STREQ("0x8p-3", buf);
#else
  ASSERT_STREQ("0x1p+0", buf);
#endif
}

TEST(strfroml, inf_nan)
{
  REVERT_TO_C_LOCALE();

  check_l("%f", INFINITY);
  check_l("%f", -INFINITY);
  check_l("%f", NAN);
  check_l("%e", INFINITY);
  check_l("%g", INFINITY);
  check_l("%a", INFINITY);
  check_l("%F", INFINITY);
  check_l("%f", -0.0L);
  check_l("%g", -0.0L);
#ifdef __HAVE_LONG_DOUBLE
  check_l("%.20f", 1.0L / 3.0L);
  check_l("%.20e", 1.0L / 7.0L);
#endif
}

TEST(strfroml, truncation)
{
  REVERT_TO_C_LOCALE();

  char buf[FBUF];
  int ret;

  buf[0] = 'X';
  ret = rs_strfroml(buf, 0, "%f", 1.0L);
  ASSERT_GT(ret, 0);
  ASSERT_EQ(buf[0], 'X');

  ret = rs_strfroml(buf, 5, "%.6f", 1.0L);
  ASSERT_EQ(ret, 8);
  ASSERT_EQ(buf[4], '\0');
}

TEST(strfrom_misc, precision_forms)
{
  REVERT_TO_C_LOCALE();

  check_d("%.f", 2.5);
  check_d("%.E", 12.345e19);
  check_d("%.0g", 3.14159);
  check_d("%.15e", 1.0 / 3.0);
  check_f("%.2a", 1.0f / 3.0f);
  check_l("%.3g", 1.0L);
}

TEST(strfrom_misc, truncation_contract)
{
  char buf[FBUF];
  int ret;

  REVERT_TO_C_LOCALE();

  buf[0] = 'X';
  ret = rs_strfromd(buf, 0, "%f", 1.0);
  ASSERT_EQ(ret, 8);
  ASSERT_EQ(buf[0], 'X');

  ret = rs_strfromd(buf, 5, "%.6f", 1.0);
  ASSERT_EQ(ret, 8);
  ASSERT_STREQ(buf, "1.00");

  ret = rs_strfromf(buf, 5, "%.6f", 1.0f);
  ASSERT_EQ(ret, 8);
  ASSERT_STREQ(buf, "1.00");

  ret = rs_strfroml(buf, 5, "%.6f", 1.0L);
  ASSERT_EQ(ret, 8);
  ASSERT_STREQ(buf, "1.00");
}

TEST(strfrom_misc, errno_preserved_on_success)
{
  char buf[FBUF];

  REVERT_TO_C_LOCALE();

  rs_errno = 0;
  rs_strfromd(buf, sizeof(buf), "%f", 1.0);
  ASSERT_EQ(rs_errno, 0);
  rs_strfromf(buf, sizeof(buf), "%a", 1.0f);
  ASSERT_EQ(rs_errno, 0);
  rs_strfroml(buf, sizeof(buf), "%.3g", 1.0L);
  ASSERT_EQ(rs_errno, 0);
}

static void
check_rejects(const char* fmt, int variant)
{
  char buf[FBUF];
  buf[0] = 'X';
  buf[1] = '\0';
  rs_errno = 0;

  switch (variant) {
    case 0:
      rs_strfromd(buf, sizeof(buf), fmt, 1.0);
      break;
    case 1:
      rs_strfromf(buf, sizeof(buf), fmt, 1.0f);
      break;
    default:
      rs_strfroml(buf, sizeof(buf), fmt, 1.0L);
      break;
  }

  ASSERT_STREQ(buf, "");
  ASSERT_EQ(rs_errno, EINVAL);
}

TEST(strfrom_ext, rejects_malformed_formats)
{
  REVERT_TO_C_LOCALE();

  check_rejects("", 0);
  check_rejects("%q", 0);
  check_rejects("%.2q", 1);
  check_rejects("%", 2);
  check_rejects("f", 0);
  check_rejects("%*f", 0);
  check_rejects("%f ", 0);
}

static int
compare_uint32(const void* a, const void* b)
{
  if (*(uint32_t*)a < *(uint32_t*)b)
    return -1;
  if (*(uint32_t*)a > *(uint32_t*)b)
    return 1;
  return 0;
}

TEST(qsort, example)
{
  // Sort an example dataset.
  uint32_t input[100] = {
    0x125aa004, 0x6a366171, 0x8910ab73, 0x87aa8e00, 0xa7d1d701, 0x4f16f856,
    0xd81691a2, 0x617c4d0f, 0xf3706fb4, 0x2e817498, 0xe6437087, 0x57a8e6d8,
    0x2865ffa2, 0xac77426d, 0x981f32c9, 0x0dd3aed5, 0x36529288, 0xde0ad5d5,
    0xfdbdac63, 0xe45814ac, 0xe11827cd, 0x6c4518f0, 0x0b8c4287, 0x661ed74a,
    0xfb069a52, 0x1813dfd1, 0x68dfb692, 0xdd15518e, 0xa618025d, 0xc6a71fc1,
    0x5d22a254, 0x974e0c15, 0xa14fb4c0, 0x5d9623c3, 0x45f78eea, 0x5af342b0,
    0xe112e4cc, 0xead1adca, 0x4e83c0aa, 0x2a687581, 0xf2d30c8a, 0x3bec4ece,
    0x3cf25f27, 0x770c63ed, 0xd9dfa913, 0x2498fd54, 0x41d44f7f, 0x95d5d188,
    0x88a624e7, 0x594fd520, 0xa38fc8a3, 0x82f06732, 0xc8b85da4, 0x5d92acef,
    0x05d95fbf, 0xcb18850a, 0xa4baf2b7, 0x891617c1, 0xe7bcd2f7, 0x916ec264,
    0xf2fee0dd, 0x0e93db5b, 0xd62f0803, 0x8ea4bfe6, 0x4994670f, 0x457f7eed,
    0x3da4920c, 0xc075ece6, 0x2b41b0e8, 0x9c217654, 0xf8f79dae, 0xb193f044,
    0x98805700, 0xbf95e340, 0x845dc034, 0x86c52f67, 0x193f575d, 0xd4d28c25,
    0x2132ccf9, 0x20825bf8, 0x27b515b6, 0x3d1bf61a, 0xdd100644, 0x661f0546,
    0xb5e81966, 0x0bfc4974, 0x7c3f232b, 0x95f8f870, 0x3ebaed5a, 0xe679fbc6,
    0x3553fb8f, 0xc2dac073, 0x2c698e2e, 0x585bf5b1, 0x3205a3d1, 0x17894379,
    0xb030abca, 0x51e50386, 0xa63132b8, 0x11ce3177,
  };
  const uint32_t expected_output[100] = {
    0x05d95fbf, 0x0b8c4287, 0x0bfc4974, 0x0dd3aed5, 0x0e93db5b, 0x11ce3177,
    0x125aa004, 0x17894379, 0x1813dfd1, 0x193f575d, 0x20825bf8, 0x2132ccf9,
    0x2498fd54, 0x27b515b6, 0x2865ffa2, 0x2a687581, 0x2b41b0e8, 0x2c698e2e,
    0x2e817498, 0x3205a3d1, 0x3553fb8f, 0x36529288, 0x3bec4ece, 0x3cf25f27,
    0x3d1bf61a, 0x3da4920c, 0x3ebaed5a, 0x41d44f7f, 0x457f7eed, 0x45f78eea,
    0x4994670f, 0x4e83c0aa, 0x4f16f856, 0x51e50386, 0x57a8e6d8, 0x585bf5b1,
    0x594fd520, 0x5af342b0, 0x5d22a254, 0x5d92acef, 0x5d9623c3, 0x617c4d0f,
    0x661ed74a, 0x661f0546, 0x68dfb692, 0x6a366171, 0x6c4518f0, 0x770c63ed,
    0x7c3f232b, 0x82f06732, 0x845dc034, 0x86c52f67, 0x87aa8e00, 0x88a624e7,
    0x8910ab73, 0x891617c1, 0x8ea4bfe6, 0x916ec264, 0x95d5d188, 0x95f8f870,
    0x974e0c15, 0x981f32c9, 0x98805700, 0x9c217654, 0xa14fb4c0, 0xa38fc8a3,
    0xa4baf2b7, 0xa618025d, 0xa63132b8, 0xa7d1d701, 0xac77426d, 0xb030abca,
    0xb193f044, 0xb5e81966, 0xbf95e340, 0xc075ece6, 0xc2dac073, 0xc6a71fc1,
    0xc8b85da4, 0xcb18850a, 0xd4d28c25, 0xd62f0803, 0xd81691a2, 0xd9dfa913,
    0xdd100644, 0xdd15518e, 0xde0ad5d5, 0xe112e4cc, 0xe11827cd, 0xe45814ac,
    0xe6437087, 0xe679fbc6, 0xe7bcd2f7, 0xead1adca, 0xf2d30c8a, 0xf2fee0dd,
    0xf3706fb4, 0xf8f79dae, 0xfb069a52, 0xfdbdac63,
  };
  rs_qsort(input, 100, sizeof(uint32_t), compare_uint32);
  ASSERT_THAT(input, testing::ElementsAreArray(expected_output));
}

TEST(qsort, empty)
{
  // Both the list and the compare function should not be accessed.
  rs_qsort_r(NULL, 0, 123, NULL, NULL);
}

TEST(a64l_l64r, examples)
{
#define TEST_BASE64(v, str)                                                    \
  do {                                                                         \
    ASSERT_EQ(v, rs_a64l(str));                                                \
    ASSERT_EQ(v, rs_a64l(str "| Trailing garbage"));                           \
                                                                               \
    char buf[sizeof(str)];                                                     \
    ASSERT_EQ(-1, rs_l64a_r(v, buf, sizeof(buf) - 1));                         \
    ASSERT_EQ(0, rs_l64a_r(v, buf, sizeof(buf)));                              \
    ASSERT_STREQ(str, buf);                                                    \
  } while (0)
  // Individual characters.
  TEST_BASE64(0, "");
  TEST_BASE64(1, "/");
  TEST_BASE64(2, "0");
  TEST_BASE64(11, "9");
  TEST_BASE64(12, "A");
  TEST_BASE64(37, "Z");
  TEST_BASE64(38, "a");
  TEST_BASE64(63, "z");

  TEST_BASE64(-798039413, "98iPE1");
  TEST_BASE64(-457520467, "hegiY1");
  TEST_BASE64(-130063910, "O5qDs1");
  TEST_BASE64(1146677630, "yZCK2/");
  TEST_BASE64(1378758273, "/uW9G/");
  TEST_BASE64(1388004411, "vEoiG/");
  TEST_BASE64(1464607185, "F50HL/");
  TEST_BASE64(1717698504, "6zTMa/");
  TEST_BASE64(1783681590, "q6BIe/");
  TEST_BASE64(1812451478, "K0x/g/");
#undef TEST_BASE64
}

TEST(a64l_l64r, random)
{
  for (int i = 0; i < 1000; ++i) {
    long v;
    arc4random_buf(&v, sizeof(v));
    char buf[7];
    ASSERT_EQ(0, rs_l64a_r(v, buf, sizeof(buf)));
    ASSERT_EQ((int32_t)v, rs_a64l(buf));
  }
}

TEST(a64l_l64, random)
{
  for (int i = 0; i < 1000; ++i) {
    long v;
    arc4random_buf(&v, sizeof(v));
    char* buf = rs_l64a(v);
    ASSERT_NE(nullptr, buf);
    ASSERT_EQ((int32_t)v, rs_a64l(buf));
  }
}
