#include "common.h"
#include "common_float.h"
#include "common_locale.h"

#include <cstdio>
#include <gtest/gtest.h>

extern "C" {
double rs_atof(const char *);
int rs_atoi(const char *);
long int rs_atol(const char *);
long long int rs_atoll(const char *);
double rs_strtod(const char *__restrict__, char **__restrict__);
float rs_strtof(const char *__restrict__, char **__restrict__);
long double rs_strtold(const char *__restrict__, char **__restrict__);
long int rs_strtol(const char *__restrict__, char **__restrict__, int);
long long int rs_strtoll(const char *__restrict__, char **__restrict__, int);
unsigned long int rs_strtoul(const char *__restrict__, char **__restrict__,
                             int);
unsigned long long int rs_strtoull(const char *__restrict__,
                                   char **__restrict__, int);
int rs_mblen(const char *, size_t);
int rs_mbtowc(wchar_t *__restrict__, const char *__restrict__, size_t);
int rs_wctomb(char *, wchar_t wc);
size_t rs_mbstowcs(wchar_t *__restrict__s, const char *__restrict__, size_t);
size_t rs_wcstombs(char *__restrict__, const wchar_t *__restrict__s, size_t);
}

TEST(atof, examples) {
  rs_setlocale(RS_LC_ALL, "C");

  char buf[128];

  // TODO: replace with rs_snprintf
  (void)snprintf(buf, sizeof(buf), "%f\n", DBL_MAX);

  ASSERT_EQ(rs_atof("0"), rs_strtod("0", NULL));
  ASSERT_EQ(rs_atof("-1"), rs_strtod("-1", NULL));
  ASSERT_EQ(rs_atof(buf), rs_strtod(buf, NULL));
}

TEST(atoi, examples) {
  ASSERT_EQ(12, rs_atoi("  12"));
  ASSERT_EQ(-3, rs_atoi("-03"));
  ASSERT_EQ(0, rs_atoi("0x5"));
}

TEST(atol, examples) {
  ASSERT_EQ(123456, rs_atol("  123456"));
  ASSERT_EQ(-8860, rs_atol("-08860"));
}

TEST(atoll, examples) { ASSERT_EQ(5050505, rs_atoll(" 5050505 ")); }

// Taken from musl libc-test: https://wiki.musl-libc.org/libc-test
FloatTestData<float, char> tests_float[] = {
    {".700649232162408535461864791644958065640130970938257885878534141944895541"
     "3429303e-45",
     0},
    {".700649232162408535461864791644958065640130970938257885878534141944895541"
     "3429304e-45",
     0x1p-149},
    {".210194769648722560638559437493487419692039291281477365763560242583468662"
     "4028790e-44",
     0x1p-149},
    {".210194769648722560638559437493487419692039291281477365763560242583468662"
     "4028791e-44",
     0x1p-148},
    {".117549442088721072420959008340872484231447212078518461533454029413183145"
     "3944281e-37",
     0x1p-126},
    {".117549442088721072420959008340872484231447212078518461533454029413183145"
     "3944282e-37",
     0x1.000002p-126},
    {"340282356779733661637539395458142568447.9999999999999999999",
     0x1.fffffep127},
    {"340282356779733661637539395458142568448", INFINITY},
};

FloatTestData<double, char> tests_double[] = {
    {"0", 0.0},
    {"00.00", 0.0},
    {"-.00000", -0.0},
    {"1e+1000000", INFINITY},
    {"1e-1000000", 0},
    {".247032822920623272088284396434110686182529901307162382212792841250337753"
     "6351043e-323",
     0},
    {".247032822920623272088284396434110686182529901307162382212792841250337753"
     "6351044e-323",
     0x1p-1074},
    {".741098468761869816264853189302332058547589703921487146638378523751013260"
     "9053131e-323",
     0x1p-1074},
    {".741098468761869816264853189302332058547589703921487146638378523751013260"
     "9053132e-323",
     0x1p-1073},
    {".222507385850720163012305563795567615250361241457301801308322872404958664"
     "7606759e-307",
     0x1p-1022},
    {".222507385850720163012305563795567615250361241457301801308322872404958664"
     "7606760e-307",
     0x1.0000000000001p-1022},
    {"17976931348623158079372897140530341507993413271003782693617377898044"
     "49682927647509466490179775872070963302864166928879109465555478519404"
     "02630657488671505820681908902000708383676273854845817711531764475730"
     "27006985557136695962284291481986083493647529271907416844436551070434"
     "2711559699508093042880177904174497791.999999999999999999999999999999",
     0x1.fffffffffffffp1023},
    {"17976931348623158079372897140530341507993413271003782693617377898044"
     "49682927647509466490179775872070963302864166928879109465555478519404"
     "02630657488671505820681908902000708383676273854845817711531764475730"
     "27006985557136695962284291481986083493647529271907416844436551070434"
     "2711559699508093042880177904174497792",
     INFINITY},
    {".5961860348131807091861002266453941950428e00", 0.59618603481318067},
    {"1.815013169218038729887460898733526957442e-1", 0.18150131692180388},
    {"42.07082357534453600681618685682257590772e-2", 0.42070823575344535},
    {"665.4686306516261456328973225579833470816e-3", 0.66546863065162609},
    {"6101.852922970868621786690495485449831753e-4", 0.61018529229708685},
    {"76966.95208236968077849464348875471158549e-5", 0.76966952082369677},
    {"250506.5322228682496132604807222923702304e-6", 0.25050653222286823},
    {"2740037.230228005325852424697698331177377e-7", 0.27400372302280052},
    {"20723093.50049742645941529268715428324490e-8", 0.20723093500497428},
    {"0.7900280238081604956226011047460238748912e1", 7.9002802380816046},
    {"0.9822860653737296848190558448760465863597e2", 98.228606537372968},
    {"0.7468949723190370809405570560160405324869e3", 746.89497231903704},
    {"0.1630268320282728475980459844271031751665e4", 1630.2683202827284},
    {"0.4637168629719170695109918769645492022088e5", 46371.686297191707},
    {"0.6537805944497711554209461686415872067523e6", 653780.59444977110},
    {"0.2346324356502437045212230713960457676531e6", 234632.43565024371},
    {"0.9709481716420048341897258980454298205278e8", 97094817.164200485},
    {"0.4996908522051874110779982354932499499602e9", 499690852.20518744},
};

FloatTestData<long double, char> tests_long_double[] = {
    {"0", 0.0},
    {"12.345", 12.345L},
    {"1.2345e1", 12.345L},
    {"1e+1000000", INFINITY},
    {"1e-1000000", 0},
#if LDBL_TYPE == LDBL_IS_F64
    {".247032822920623272088284396434110686182529901307162382212792841250337753"
     "6351043e-323",
     0},
    {".247032822920623272088284396434110686182529901307162382212792841250337753"
     "6351044e-323",
     0x1p-1074},
    {".741098468761869816264853189302332058547589703921487146638378523751013260"
     "9053131e-323",
     0x1p-1074},
    {".741098468761869816264853189302332058547589703921487146638378523751013260"
     "9053132e-323",
     0x1p-1073},
    {".222507385850720163012305563795567615250361241457301801308322872404958664"
     "7606759e-307",
     0x1p-1022},
    {".222507385850720163012305563795567615250361241457301801308322872404958664"
     "7606760e-307",
     0x1.0000000000001p-1022},
    {"17976931348623158079372897140530341507993413271003782693617377898044"
     "49682927647509466490179775872070963302864166928879109465555478519404"
     "02630657488671505820681908902000708383676273854845817711531764475730"
     "27006985557136695962284291481986083493647529271907416844436551070434"
     "2711559699508093042880177904174497791.999999999999999999999999999999",
     0x1.fffffffffffffp1023},
    {"17976931348623158079372897140530341507993413271003782693617377898044"
     "49682927647509466490179775872070963302864166928879109465555478519404"
     "02630657488671505820681908902000708383676273854845817711531764475730"
     "27006985557136695962284291481986083493647529271907416844436551070434"
     "2711559699508093042880177904174497792",
     INFINITY},
    {".5961860348131807091861002266453941950428e00", 0.59618603481318067},
    {"1.815013169218038729887460898733526957442e-1", 0.18150131692180388},
    {"42.07082357534453600681618685682257590772e-2", 0.42070823575344535},
    {"665.4686306516261456328973225579833470816e-3", 0.66546863065162609},
    {"6101.852922970868621786690495485449831753e-4", 0.61018529229708685},
    {"76966.95208236968077849464348875471158549e-5", 0.76966952082369677},
    {"250506.5322228682496132604807222923702304e-6", 0.25050653222286823},
    {"2740037.230228005325852424697698331177377e-7", 0.27400372302280052},
    {"20723093.50049742645941529268715428324490e-8", 0.20723093500497428},
    {"0.7900280238081604956226011047460238748912e1", 7.9002802380816046},
    {"0.9822860653737296848190558448760465863597e2", 98.228606537372968},
    {"0.7468949723190370809405570560160405324869e3", 746.89497231903704},
    {"0.1630268320282728475980459844271031751665e4", 1630.2683202827284},
    {"0.4637168629719170695109918769645492022088e5", 46371.686297191707},
    {"0.6537805944497711554209461686415872067523e6", 653780.59444977110},
    {"0.2346324356502437045212230713960457676531e6", 234632.43565024371},
    {"0.9709481716420048341897258980454298205278e8", 97094817.164200485},
    {"0.4996908522051874110779982354932499499602e9", 499690852.20518744},
#elif LDBL_TYPE == LDBL_IS_F80
    {".182259976594123730126420296680970990819952540784678167186049024351418584"
     "4316698e-4950",
     0},
    {".182259976594123730126420296680970990819952540784678167186049024351418584"
     "4316699e-4950",
     0x1p-16445L},
    {".546779929782371190379260890042912972459857622354034501558147073054255753"
     "2950096e-4950",
     0x1p-16445L},
    {".546779929782371190379260890042912972459857622354034501558147073054255753"
     "2950097e-4950",
     0x1p-16444L},
    {".336210314311209350644493779391587633272449964152744223092877977059342086"
     "6576777e-4931",
     0x1p-16382L},
    {".336210314311209350644493779391587633272449964152744223092877977059342086"
     "6576778e-4931",
     0x1.0000000000000002p-16382L},
    {"118973149535723176505351158982948.86679662540046955672e4900",
     0x1.fffffffffffffffep16383L},
    {"118973149535723176505351158982948.86679662540046955673e4900", INFINITY},
#endif
};

TEST(strtof, dec) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  for (size_t i = 0; i < std::size(tests_float); i++) {
    ASSERT_EQ(rs_strtof(tests_float[i].name.data(), nullptr),
              tests_float[i].value);
  }
}

TEST(strtof, dec1) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str = "0.0625";
  char *endptr;
  ASSERT_EQ(0.0625f, rs_strtof(str, NULL));
  ASSERT_EQ(0.0625f, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 6, endptr);
}

TEST(strtof, dec2) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str = "12800e-2";
  char *endptr;
  ASSERT_EQ(128.0f, rs_strtof(str, NULL));
  ASSERT_EQ(128.0f, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 8, endptr);
}

TEST(strtof, dec3) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str = "7,5";
  char *endptr;
  ASSERT_EQ(7.0f, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 1, endptr);

  rs_setlocale(RS_LC_ALL, "nl_NL.UTF-8");
  rs_errno = 0;

  ASSERT_EQ(7.5f, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 3, endptr);

  str = ",75";
  ASSERT_EQ(.75f, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 3, endptr);
}

TEST(strtof, hex1) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str = "  0xcaf.eff";
  char *endptr;
  ASSERT_EQ(0xcaf.effp0, rs_strtof(str, NULL));
  ASSERT_EQ(0xcaf.effp0, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 11, endptr);
}

TEST(strtof, hex2) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str = "0x0p99999999999999999999";
  char *endptr;
  ASSERT_EQ(0.0, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 24, endptr);
}

TEST(strtof, hex3) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str = "\t0x1p+30000";
  char *endptr;
  ASSERT_EQ(HUGE_VALF, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 11, endptr);
}

TEST(strtof, hex4) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str = "\n0X1P-30000 ";
  char *endptr;
  ASSERT_EQ(0.0f, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 11, endptr);
}

TEST(strtof, hex5) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str = "-0x123xyz";
  char *endptr;
  ASSERT_EQ(-0x123.0p0, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 6, endptr);
}

TEST(strtof, hex6) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str = "0x";
  char *endptr;
  ASSERT_EQ(0, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 1, endptr);
}

TEST(strtof, hex7) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str = "0x.8";
  char *endptr;
  ASSERT_EQ(0.5, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 4, endptr);
}

TEST(strtof, hex8) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *below = "-0x0.ffffffffffffffffffffffffffffffffffffffffffffffffff";
  const char *exact = "-0x1.0";
  const char *above = "-0x1.00000000000000000000000000000000000000000000000001";
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

TEST(strtof, hex9) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *normal = "0x1p-126";
  const char *highest_subnormal = "0x1.fffffcp-127";
  float high = 0x1.fffffcp-127;
  const char *lowest_subnormal = "0x1p-149";
  const char *underflow = "0x1p-150";

  rs_errno = 0;
  ASSERT_EQ(FLT_MIN, rs_strtof(normal, NULL));
  ASSERT_EQ(0, rs_errno);
  ASSERT_EQ(high, rs_strtof(highest_subnormal, NULL));
  ASSERT_EQ(0, rs_errno);
  ASSERT_EQ(FLT_TRUE_MIN, rs_strtof(lowest_subnormal, NULL));
  ASSERT_EQ(0, rs_errno);
  ASSERT_EQ(0.0, rs_strtof(underflow, NULL));
  ASSERT_EQ(ERANGE, rs_errno);
}

TEST(strtof, nan1) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str = "NaN(Hello";
  char *endptr;
  ASSERT_TRUE(std::isnan(rs_strtof(str, &endptr)));
  ASSERT_EQ(str + 3, endptr);
}

TEST(strtof, nan2) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str = "NaN(Hello world) :-)";
  char *endptr;
  ASSERT_TRUE(std::isnan(rs_strtof(str, &endptr)));
  ASSERT_EQ(str + 16, endptr);
}

TEST(strtof, inf1) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str = "INFINITE";
  char *endptr;
  ASSERT_EQ(INFINITY, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 3, endptr);
}

TEST(strtof, inf2) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str = "-INFINITY";
  char *endptr;
  ASSERT_EQ(-INFINITY, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 9, endptr);
}

TEST(strtof, huge_val1) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str =
      "10000000000000000000000000000000000000000000000000000000000000000000000"
      "00000000000000000000000000000000000000000000000000000000000000000000000"
      "00000000000000000000000000000000000000000000000000000000000000000000000";
  char *endptr;
  ASSERT_EQ(HUGE_VALF, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 213, endptr);
  ASSERT_EQ(ERANGE, rs_errno);
}

TEST(strtof, huge_val2) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str = "-1e3000";
  char *endptr;
  ASSERT_EQ(-HUGE_VALF, rs_strtof(str, &endptr));
  ASSERT_EQ(str + 7, endptr);
  ASSERT_EQ(ERANGE, rs_errno);
}

TEST(strtof, zero1) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str =
      "0.000000000000000000000000000000000000000000000000000000000000000000000"
      "00000000000000000000000000000000000000000000000000000000000000000000000"
      "00000000000000000000000000000000000000000000000000000000000000000000001";
  char *endptr;
  float v = rs_strtof(str, &endptr);
  ASSERT_EQ(0.0, v);
  ASSERT_FALSE(std::signbit(v));
  ASSERT_EQ(str + 213, endptr);
  ASSERT_EQ(ERANGE, rs_errno);
}

TEST(strtof, zero2) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str = "-1e-3000";
  char *endptr;
  float v = rs_strtof(str, &endptr);
  ASSERT_EQ(0.0, v);
  ASSERT_TRUE(std::signbit(v));
  ASSERT_EQ(str + 8, endptr);
  ASSERT_EQ(ERANGE, rs_errno);
}

TEST(strtof, zero3) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str = "0.0";
  char *endptr;
  float v = rs_strtof(str, &endptr);
  ASSERT_EQ(0.0, v);
  ASSERT_FALSE(std::signbit(v));
  ASSERT_EQ(str + 3, endptr);
  ASSERT_EQ(0, rs_errno);
}

TEST(strtof, zero4) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str = "-0.0";
  char *endptr;
  float v = rs_strtof(str, &endptr);
  ASSERT_EQ(0.0, v);
  ASSERT_TRUE(std::signbit(v));
  ASSERT_EQ(str + 4, endptr);
  ASSERT_EQ(0, rs_errno);
}

TEST(strtod, dec) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  for (size_t i = 0; i < std::size(tests_double); i++) {
    ASSERT_EQ(rs_strtod(tests_double[i].name.data(), nullptr),
              tests_double[i].value);
  }
}

TEST(strtod, hex1) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *below = "-0x0.ffffffffffffffffffffffffffffffffffffffffffffffffff";
  const char *exact = "-0x1.0";
  const char *above = "-0x1.00000000000000000000000000000000000000000000000001";
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

TEST(strtod, hex2) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *normal = "0x1p-1022";
  const char *highest_subnormal = "0X1.fFfFfFfFfFfFEP-1023";
  double high = 0x1.ffffffffffffep-1023;
  const char *lowest_subnormal = "0x1p-1074";
  const char *underflow = "0x1p-1075";
  const char *above_subnormal = "0x1.ffffffffffffe000001p-1023";

  rs_errno = 0;
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

TEST(strtold, dec) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  for (size_t i = 0; i < std::size(tests_long_double); i++) {
    ASSERT_FLOAT_EQ(rs_strtold(tests_long_double[i].name.data(), nullptr),
                    tests_long_double[i].value);
  }
}

TEST(strtold, hex1) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *below = "-0x0.ffffffffffffffffffffffffffffffffffffffffffffffffff";
  const char *exact = "-0x1.0";
  const char *above = "-0x1.00000000000000000000000000000000000000000000000001";
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

TEST(strtold, hex2) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

#if LDBL_TYPE == LDBL_IS_F64
  const char *normal = "0x1p-1022";
  const char *highest_subnormal = "0x1.ffffffffffffep-1023";
  const char *lowest_subnormal = "0x1p-1074";
  const char *underflow = "0x1p-1075";
#elif LDBL_TYPE == LDBL_IS_F80
  const char *normal = "0x1p-16382";
  const char *highest_subnormal = "0x1.fffffffffffffffcp-16383";
  const char *lowest_subnormal = "0x1p-16445";
  const char *underflow = "0x1p-16446";
#elif LDBL_TYPE == LDBL_IS_F128
  const char *normal = "0x1p-16382";
  const char *highest_subnormal = "0x1.fffffffffffffffffffffffffffep-16383";
  const char *lowest_subnormal = "0x1p-16494";
  const char *underflow = "0x1p-16495";
#endif

  errno = 0;
  ASSERT_EQ(LDBL_MIN, rs_strtold(normal, NULL));
  ASSERT_EQ(0, rs_errno);
  ASSERT_EQ(nexttowardl(LDBL_MIN, 0.0L), rs_strtold(highest_subnormal, NULL));
  ASSERT_EQ(0, rs_errno);
  ASSERT_EQ(LDBL_TRUE_MIN, rs_strtold(lowest_subnormal, NULL));
  ASSERT_EQ(0, rs_errno);
  ASSERT_EQ(0.0, rs_strtold(underflow, NULL));
  ASSERT_EQ(ERANGE, rs_errno);
}

TEST(strtold, extremes) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  char max[500];
  char min[500];

  // TODO: use rs_snprintf
  snprintf(max, sizeof(max), "%Le", LDBL_MAX);
  snprintf(min, sizeof(min), "%Le", LDBL_MIN);

  ASSERT_FLOAT_EQ(rs_strtold(max, nullptr), LDBL_MAX);
  ASSERT_FLOAT_EQ(rs_strtold(min, nullptr), LDBL_MIN);
}

TEST(strtol, positive) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str;
  char *endptr;

  str = "0";
  ASSERT_EQ(0, rs_strtol(str, NULL, 0));
  ASSERT_EQ(0, rs_errno);

  str = "1";
  ASSERT_EQ(1, rs_strtol(str, NULL, 0));
  ASSERT_EQ(0, rs_errno);

  str = "0x7ffffffffffffffe";
  errno = 0;
  ASSERT_EQ(LONG_MAX - 1, rs_strtol(str, &endptr, 0));
  ASSERT_EQ(str + 18, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "0x7fffffffffffffff";
  ASSERT_EQ(LONG_MAX, rs_strtol(str, &endptr, 0));
  ASSERT_EQ(str + 18, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "0x8000000000000000";
  ASSERT_EQ(LONG_MAX, rs_strtol(str, &endptr, 0));
  ASSERT_EQ(str + 18, endptr);
  ASSERT_EQ(ERANGE, rs_errno);
}

TEST(strtol, negative) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str;
  char *endptr;

  str = "-0";
  ASSERT_EQ(0, rs_strtol(str, NULL, 0));
  ASSERT_EQ(0, rs_errno);

  str = "-1";
  ASSERT_EQ(-1, rs_strtol(str, NULL, 0));
  ASSERT_EQ(0, rs_errno);

  str = "-0x7fffffffffffffff";
  errno = 0;
  ASSERT_EQ(LONG_MIN + 1, rs_strtol(str, &endptr, 0));
  ASSERT_EQ(str + 19, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "-0x8000000000000000";
  ASSERT_EQ(LONG_MIN, rs_strtol(str, &endptr, 0));
  ASSERT_EQ(str + 19, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "-0x8000000000000001";
  ASSERT_EQ(LONG_MIN, rs_strtol(str, &endptr, 0));
  ASSERT_EQ(str + 19, endptr);
  ASSERT_EQ(ERANGE, rs_errno);
}

TEST(strtoll, positive) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str;
  char *endptr;

  str = "0";
  ASSERT_EQ(0, rs_strtoll(str, NULL, 0));
  ASSERT_EQ(0, rs_errno);

  str = "1";
  ASSERT_EQ(1, rs_strtoll(str, NULL, 0));
  ASSERT_EQ(0, rs_errno);

  str = "0x7ffffffffffffffe";
  ASSERT_EQ(LLONG_MAX - 1, rs_strtoll(str, &endptr, 0));
  ASSERT_EQ(str + 18, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "0x7fffffffffffffff";
  ASSERT_EQ(LLONG_MAX, rs_strtoll(str, &endptr, 0));
  ASSERT_EQ(str + 18, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "0x8000000000000000";
  ASSERT_EQ(LLONG_MAX, rs_strtoll(str, &endptr, 0));
  ASSERT_EQ(str + 18, endptr);
  ASSERT_EQ(ERANGE, rs_errno);
}

TEST(strtoll, negative) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  const char *str;
  char *endptr;

  str = "-0";
  ASSERT_EQ(0, rs_strtoll(str, NULL, 0));
  ASSERT_EQ(0, rs_errno);

  str = "-1";
  ASSERT_EQ(-1, rs_strtoll(str, NULL, 0));
  ASSERT_EQ(0, rs_errno);

  str = "-0x7fffffffffffffff";
  ASSERT_EQ(LLONG_MIN + 1, rs_strtoll(str, &endptr, 0));
  ASSERT_EQ(str + 19, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "-0x8000000000000000";
  ASSERT_EQ(LLONG_MIN, rs_strtoll(str, &endptr, 0));
  ASSERT_EQ(str + 19, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "-0x8000000000000001";
  ASSERT_EQ(LLONG_MIN, rs_strtoll(str, &endptr, 0));
  ASSERT_EQ(str + 19, endptr);
  ASSERT_EQ(ERANGE, rs_errno);
}

TEST(strtoul, examples) {
  rs_setlocale(RS_LC_ALL, "C");

  const char *str = "  57";
  char *endptr;
  rs_errno = 0;
  ASSERT_EQ(57, rs_strtoul(str, NULL, 10));
  ASSERT_EQ(0, rs_errno);

  str = "          ";
  ASSERT_EQ(0, rs_strtoul(str, &endptr, 10));
  ASSERT_EQ(str, endptr);
  ASSERT_EQ(EINVAL, rs_errno);

  str = "  01234hello";
  rs_errno = 0;
  ASSERT_EQ(1234, rs_strtoul(str, &endptr, 10));
  ASSERT_EQ(str + 7, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "  01234hello";
  ASSERT_EQ(194, rs_strtoul(str, &endptr, 5));
  ASSERT_EQ(str + 7, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "  01234hello";
  ASSERT_EQ(01234, rs_strtoul(str, &endptr, 0));
  ASSERT_EQ(str + 7, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "Hello!";
  ASSERT_EQ(29234652, rs_strtoul(str, &endptr, 36));
  ASSERT_EQ(str + 5, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "\n-42boom";
  ASSERT_EQ((unsigned long)-26, rs_strtoul(str, &endptr, 6));
  ASSERT_EQ(str + 4, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "\t-000000";
  rs_errno = 0;
  ASSERT_EQ(0, rs_strtoul(str, &endptr, 6));
  ASSERT_EQ(str + 8, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "0x123";
  ASSERT_EQ(0x123, rs_strtoul(str, &endptr, 0));
  ASSERT_EQ(str + 5, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "456";
  ASSERT_EQ(0x456, rs_strtoul(str, &endptr, 16));
  ASSERT_EQ(str + 3, endptr);
  ASSERT_EQ(0, rs_errno);
}

TEST(strtoull, positive) {
  rs_setlocale(RS_LC_ALL, "C");

  const char *str;
  char *endptr;

  rs_errno = 0;
  str = "0xfffffffffffffffe";
  ASSERT_EQ(ULLONG_MAX - 1, rs_strtoull(str, &endptr, 0));
  ASSERT_EQ(str + 18, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "0xffffffffffffffff";
  ASSERT_EQ(ULLONG_MAX, rs_strtoull(str, &endptr, 0));
  ASSERT_EQ(str + 18, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "0x10000000000000000";
  ASSERT_EQ(ULLONG_MAX, rs_strtoull(str, &endptr, 0));
  ASSERT_EQ(str + 19, endptr);
  ASSERT_EQ(ERANGE, rs_errno);

  str = "0xfffffffffffffffff";
  rs_errno = 0;
  ASSERT_EQ(ULLONG_MAX, rs_strtoull(str, &endptr, 0));
  ASSERT_EQ(str + 19, endptr);
  ASSERT_EQ(ERANGE, rs_errno);
}

TEST(strtoull, negative) {
  rs_setlocale(RS_LC_ALL, "C");

  const char *str;
  char *endptr;

  rs_errno = 0;
  str = "0";
  ASSERT_EQ(0, rs_strtoull(str, &endptr, 0));
  ASSERT_EQ(str + 1, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "-0";
  ASSERT_EQ(0, rs_strtoull(str, &endptr, 0));
  ASSERT_EQ(str + 2, endptr);
  ASSERT_EQ(0, rs_errno);

  str = "-1";
  ASSERT_EQ(ULLONG_MAX, rs_strtoull(str, &endptr, 0));
  ASSERT_EQ(str + 2, endptr);
  ASSERT_EQ(0, rs_errno);
}

TEST(mblen, bad) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  ASSERT_EQ(-1, rs_mblen("", 0));
  ASSERT_EQ(EILSEQ, rs_errno);
  ASSERT_EQ(-1, rs_mblen("Hello", 0));
  ASSERT_EQ(EILSEQ, rs_errno);
}

TEST(mblen, ascii) {
  rs_setlocale(RS_LC_ALL, "C");
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

TEST(mblen, unicode) {
  rs_setlocale(RS_LC_ALL, "C.UTF-8");
  rs_errno = 0;

  char euro[] = "€";
  for (size_t i = 0; i < sizeof(euro) - 1; ++i) {
    ASSERT_EQ(-1, rs_mblen(euro, i));
    ASSERT_EQ(EILSEQ, rs_errno);
  }

  ASSERT_EQ(sizeof(euro) - 1, rs_mblen(euro, sizeof(euro) - 1));
  ASSERT_EQ(sizeof(euro) - 1, rs_mblen(euro, sizeof(euro)));
}

TEST(mbtowc, bad) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  ASSERT_EQ(-1, rs_mbtowc(NULL, "", 0));
  ASSERT_EQ(EILSEQ, rs_errno);
  ASSERT_EQ(-1, rs_mbtowc(NULL, "Hello", 0));
  ASSERT_EQ(EILSEQ, rs_errno);
}

TEST(mbtowc, ascii) {
  rs_setlocale(RS_LC_ALL, "C");
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

TEST(mbtowc, unicode) {
  rs_setlocale(RS_LC_ALL, "C.UTF-8");
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

TEST(mbstowcs, bad) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  ASSERT_EQ(-1, rs_mbstowcs(NULL, "München", 42));
  ASSERT_EQ(EILSEQ, rs_errno);
}

TEST(mbstowcs, zero) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  ASSERT_EQ(0, rs_mbstowcs((wchar_t *)0x42, "Hello", 0));
}

TEST(mbstowcs, length) {
  rs_setlocale(RS_LC_ALL, "nl_NL.UTF-8");
  rs_errno = 0;

  ASSERT_EQ(10, rs_mbstowcs(NULL, "Düsseldorf", 0));
  ASSERT_EQ(10, rs_mbstowcs(NULL, "Düsseldorf", 5));
  ASSERT_EQ(10, rs_mbstowcs(NULL, "Düsseldorf", 40));
  ASSERT_EQ(10, rs_mbstowcs(NULL, "Düsseldorf", SIZE_MAX));
}

TEST(mbstowcs, convert) {
  rs_setlocale(RS_LC_ALL, "nl_NL.UTF-8");
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
TEST(wctomb, ascii) {
  rs_setlocale(RS_LC_ALL, "C");
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

TEST(wctomb, unicode) {
  rs_setlocale(RS_LC_ALL, "C.UTF-8");
  rs_errno = 0;

  char buf[MB_LEN_MAX];
  ASSERT_EQ(sizeof("€") - 1, rs_wctomb(buf, L'€'));
  ASSERT_THAT(buf, testing::StartsWith("€"));
}

TEST(wcstombs, bad) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  ASSERT_EQ(-1, rs_wcstombs(NULL, L"München", 42));
  ASSERT_EQ(EILSEQ, rs_errno);
}

TEST(wcstombs, zero) {
  rs_setlocale(RS_LC_ALL, "C");
  rs_errno = 0;

  ASSERT_EQ(0, rs_wcstombs((char *)0x42, L"Hello", 0));
}

TEST(wcstombs, length) {
  rs_setlocale(RS_LC_ALL, "C.UTF-8");
  rs_errno = 0;

  ASSERT_EQ(11, rs_wcstombs(NULL, L"Düsseldorf", 0));
  ASSERT_EQ(11, rs_wcstombs(NULL, L"Düsseldorf", 5));
  ASSERT_EQ(11, rs_wcstombs(NULL, L"Düsseldorf", 40));
  ASSERT_EQ(11, rs_wcstombs(NULL, L"Düsseldorf", SIZE_MAX));
}

TEST(wcstombs, convert) {
  rs_setlocale(RS_LC_ALL, "C.UTF-8");
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
