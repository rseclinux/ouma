#include "common.h"
#include "common_locale.h"

typedef int nl_item;

#define RS_CODESET 0
#define RS_D_T_FMT 1
#define RS_D_FMT 2
#define RS_T_FMT 3
#define RS_T_FMT_AMPM 4
#define RS_AM_STR 5
#define RS_PM_STR 6
#define RS_DAY_1 7
#define RS_DAY_2 8
#define RS_DAY_3 9
#define RS_DAY_4 10
#define RS_DAY_5 11
#define RS_DAY_6 12
#define RS_DAY_7 13
#define RS_ABDAY_1 14
#define RS_ABDAY_2 15
#define RS_ABDAY_3 16
#define RS_ABDAY_4 17
#define RS_ABDAY_5 18
#define RS_ABDAY_6 19
#define RS_ABDAY_7 20
#define RS_MON_1 21
#define RS_MON_2 22
#define RS_MON_3 23
#define RS_MON_4 24
#define RS_MON_5 25
#define RS_MON_6 26
#define RS_MON_7 27
#define RS_MON_8 28
#define RS_MON_9 29
#define RS_MON_10 30
#define RS_MON_11 31
#define RS_MON_12 32
#define RS_ALTMON_1 33
#define RS_ALTMON_2 34
#define RS_ALTMON_3 35
#define RS_ALTMON_4 36
#define RS_ALTMON_5 37
#define RS_ALTMON_6 38
#define RS_ALTMON_7 39
#define RS_ALTMON_8 40
#define RS_ALTMON_9 41
#define RS_ALTMON_10 42
#define RS_ALTMON_11 43
#define RS_ALTMON_12 44
#define RS_ABMON_1 45
#define RS_ABMON_2 46
#define RS_ABMON_3 47
#define RS_ABMON_4 48
#define RS_ABMON_5 49
#define RS_ABMON_6 50
#define RS_ABMON_7 51
#define RS_ABMON_8 52
#define RS_ABMON_9 53
#define RS_ABMON_10 54
#define RS_ABMON_11 55
#define RS_ABMON_12 56
#define RS_ABALTMON_1 57
#define RS_ABALTMON_2 58
#define RS_ABALTMON_3 59
#define RS_ABALTMON_4 60
#define RS_ABALTMON_5 61
#define RS_ABALTMON_6 62
#define RS_ABALTMON_7 63
#define RS_ABALTMON_8 64
#define RS_ABALTMON_9 65
#define RS_ABALTMON_10 66
#define RS_ABALTMON_11 67
#define RS_ABALTMON_12 68
#define RS_ERA 69
#define RS_ERA_D_FMT 70
#define RS_ERA_D_T_FMT 71
#define RS_ERA_T_FMT 72
#define RS_ALT_DIGITS 73
#define RS_RADIXCHAR 74
#define RS_THOUSEP 75
#define RS_YESEXPR 76
#define RS_NOEXPR 77
#define RS_CRNCYSTR 78

extern "C"
{
  char* rs_nl_langinfo(nl_item);
  char* rs_nl_langinfo_l(nl_item, ouma_locale_t);
}

TEST(nl_langinfo, posix)
{
  REVERT_TO_C_LOCALE();

  ASSERT_STREQ("US-ASCII", rs_nl_langinfo(RS_CODESET));
  ASSERT_STREQ("%a %b %e %H:%M:%S %Y", rs_nl_langinfo(RS_D_T_FMT));
  ASSERT_STREQ("%m/%d/%y", rs_nl_langinfo(RS_D_FMT));
  ASSERT_STREQ("%H:%M:%S", rs_nl_langinfo(RS_T_FMT));
  ASSERT_STREQ("%I:%M:%S %p", rs_nl_langinfo(RS_T_FMT_AMPM));
  ASSERT_STREQ("AM", rs_nl_langinfo(RS_AM_STR));
  ASSERT_STREQ("PM", rs_nl_langinfo(RS_PM_STR));
  ASSERT_STREQ("Sunday", rs_nl_langinfo(RS_DAY_1));
  ASSERT_STREQ("Monday", rs_nl_langinfo(RS_DAY_2));
  ASSERT_STREQ("Tuesday", rs_nl_langinfo(RS_DAY_3));
  ASSERT_STREQ("Wednesday", rs_nl_langinfo(RS_DAY_4));
  ASSERT_STREQ("Thursday", rs_nl_langinfo(RS_DAY_5));
  ASSERT_STREQ("Friday", rs_nl_langinfo(RS_DAY_6));
  ASSERT_STREQ("Saturday", rs_nl_langinfo(RS_DAY_7));
  ASSERT_STREQ("Sun", rs_nl_langinfo(RS_ABDAY_1));
  ASSERT_STREQ("Mon", rs_nl_langinfo(RS_ABDAY_2));
  ASSERT_STREQ("Tue", rs_nl_langinfo(RS_ABDAY_3));
  ASSERT_STREQ("Wed", rs_nl_langinfo(RS_ABDAY_4));
  ASSERT_STREQ("Thu", rs_nl_langinfo(RS_ABDAY_5));
  ASSERT_STREQ("Fri", rs_nl_langinfo(RS_ABDAY_6));
  ASSERT_STREQ("Sat", rs_nl_langinfo(RS_ABDAY_7));
  ASSERT_STREQ("January", rs_nl_langinfo(RS_MON_1));
  ASSERT_STREQ("February", rs_nl_langinfo(RS_MON_2));
  ASSERT_STREQ("March", rs_nl_langinfo(RS_MON_3));
  ASSERT_STREQ("April", rs_nl_langinfo(RS_MON_4));
  ASSERT_STREQ("May", rs_nl_langinfo(RS_MON_5));
  ASSERT_STREQ("June", rs_nl_langinfo(RS_MON_6));
  ASSERT_STREQ("July", rs_nl_langinfo(RS_MON_7));
  ASSERT_STREQ("August", rs_nl_langinfo(RS_MON_8));
  ASSERT_STREQ("September", rs_nl_langinfo(RS_MON_9));
  ASSERT_STREQ("October", rs_nl_langinfo(RS_MON_10));
  ASSERT_STREQ("November", rs_nl_langinfo(RS_MON_11));
  ASSERT_STREQ("December", rs_nl_langinfo(RS_MON_12));
  ASSERT_STREQ("Jan", rs_nl_langinfo(RS_ABMON_1));
  ASSERT_STREQ("Feb", rs_nl_langinfo(RS_ABMON_2));
  ASSERT_STREQ("Mar", rs_nl_langinfo(RS_ABMON_3));
  ASSERT_STREQ("Apr", rs_nl_langinfo(RS_ABMON_4));
  ASSERT_STREQ("May", rs_nl_langinfo(RS_ABMON_5));
  ASSERT_STREQ("Jun", rs_nl_langinfo(RS_ABMON_6));
  ASSERT_STREQ("Jul", rs_nl_langinfo(RS_ABMON_7));
  ASSERT_STREQ("Aug", rs_nl_langinfo(RS_ABMON_8));
  ASSERT_STREQ("Sep", rs_nl_langinfo(RS_ABMON_9));
  ASSERT_STREQ("Oct", rs_nl_langinfo(RS_ABMON_10));
  ASSERT_STREQ("Nov", rs_nl_langinfo(RS_ABMON_11));
  ASSERT_STREQ("Dec", rs_nl_langinfo(RS_ABMON_12));
  ASSERT_STREQ("", rs_nl_langinfo(RS_ERA));
  ASSERT_STREQ("", rs_nl_langinfo(RS_ERA_D_FMT));
  ASSERT_STREQ("", rs_nl_langinfo(RS_ERA_D_T_FMT));
  ASSERT_STREQ("", rs_nl_langinfo(RS_ERA_T_FMT));
  ASSERT_STREQ("", rs_nl_langinfo(RS_ALT_DIGITS));
  ASSERT_STREQ(".", rs_nl_langinfo(RS_RADIXCHAR));
  ASSERT_STREQ("", rs_nl_langinfo(RS_THOUSEP));
  ASSERT_STREQ("^[+1yY]", rs_nl_langinfo(RS_YESEXPR));
  ASSERT_STREQ("^[-0nN]", rs_nl_langinfo(RS_NOEXPR));
  ASSERT_STREQ("", rs_nl_langinfo(RS_CRNCYSTR));
  ASSERT_STREQ("", rs_nl_langinfo(0xdeadc0de));
}

TEST(nl_langinfo, en_us)
{
  ouma_locale_t locale = rs_newlocale(RS_LC_ALL_MASK, "en_US.UTF-8", 0);
  ASSERT_STREQ("UTF-8", rs_nl_langinfo_l(RS_CODESET, locale));
  ASSERT_STREQ("%a, %b %e, %Y, %l:%M:%S %p",
               rs_nl_langinfo_l(RS_D_T_FMT, locale));
  ASSERT_STREQ("%b %e, %Y", rs_nl_langinfo_l(RS_D_FMT, locale));
  ASSERT_STREQ("%l:%M:%S %p", rs_nl_langinfo_l(RS_T_FMT, locale));
  ASSERT_STREQ("%l:%M:%S %p", rs_nl_langinfo_l(RS_T_FMT_AMPM, locale));
  ASSERT_STREQ("AM", rs_nl_langinfo_l(RS_AM_STR, locale));
  ASSERT_STREQ("PM", rs_nl_langinfo_l(RS_PM_STR, locale));
  ASSERT_STREQ("Sunday", rs_nl_langinfo_l(RS_DAY_1, locale));
  ASSERT_STREQ("Monday", rs_nl_langinfo_l(RS_DAY_2, locale));
  ASSERT_STREQ("Tuesday", rs_nl_langinfo_l(RS_DAY_3, locale));
  ASSERT_STREQ("Wednesday", rs_nl_langinfo_l(RS_DAY_4, locale));
  ASSERT_STREQ("Thursday", rs_nl_langinfo_l(RS_DAY_5, locale));
  ASSERT_STREQ("Friday", rs_nl_langinfo_l(RS_DAY_6, locale));
  ASSERT_STREQ("Saturday", rs_nl_langinfo_l(RS_DAY_7, locale));
  ASSERT_STREQ("Sun", rs_nl_langinfo_l(RS_ABDAY_1, locale));
  ASSERT_STREQ("Mon", rs_nl_langinfo_l(RS_ABDAY_2, locale));
  ASSERT_STREQ("Tue", rs_nl_langinfo_l(RS_ABDAY_3, locale));
  ASSERT_STREQ("Wed", rs_nl_langinfo_l(RS_ABDAY_4, locale));
  ASSERT_STREQ("Thu", rs_nl_langinfo_l(RS_ABDAY_5, locale));
  ASSERT_STREQ("Fri", rs_nl_langinfo_l(RS_ABDAY_6, locale));
  ASSERT_STREQ("Sat", rs_nl_langinfo_l(RS_ABDAY_7, locale));
  ASSERT_STREQ("January", rs_nl_langinfo_l(RS_MON_1, locale));
  ASSERT_STREQ("February", rs_nl_langinfo_l(RS_MON_2, locale));
  ASSERT_STREQ("March", rs_nl_langinfo_l(RS_MON_3, locale));
  ASSERT_STREQ("April", rs_nl_langinfo_l(RS_MON_4, locale));
  ASSERT_STREQ("May", rs_nl_langinfo_l(RS_MON_5, locale));
  ASSERT_STREQ("June", rs_nl_langinfo_l(RS_MON_6, locale));
  ASSERT_STREQ("July", rs_nl_langinfo_l(RS_MON_7, locale));
  ASSERT_STREQ("August", rs_nl_langinfo_l(RS_MON_8, locale));
  ASSERT_STREQ("September", rs_nl_langinfo_l(RS_MON_9, locale));
  ASSERT_STREQ("October", rs_nl_langinfo_l(RS_MON_10, locale));
  ASSERT_STREQ("November", rs_nl_langinfo_l(RS_MON_11, locale));
  ASSERT_STREQ("December", rs_nl_langinfo_l(RS_MON_12, locale));
  ASSERT_STREQ("Jan", rs_nl_langinfo_l(RS_ABMON_1, locale));
  ASSERT_STREQ("Feb", rs_nl_langinfo_l(RS_ABMON_2, locale));
  ASSERT_STREQ("Mar", rs_nl_langinfo_l(RS_ABMON_3, locale));
  ASSERT_STREQ("Apr", rs_nl_langinfo_l(RS_ABMON_4, locale));
  ASSERT_STREQ("May", rs_nl_langinfo_l(RS_ABMON_5, locale));
  ASSERT_STREQ("Jun", rs_nl_langinfo_l(RS_ABMON_6, locale));
  ASSERT_STREQ("Jul", rs_nl_langinfo_l(RS_ABMON_7, locale));
  ASSERT_STREQ("Aug", rs_nl_langinfo_l(RS_ABMON_8, locale));
  ASSERT_STREQ("Sep", rs_nl_langinfo_l(RS_ABMON_9, locale));
  ASSERT_STREQ("Oct", rs_nl_langinfo_l(RS_ABMON_10, locale));
  ASSERT_STREQ("Nov", rs_nl_langinfo_l(RS_ABMON_11, locale));
  ASSERT_STREQ("Dec", rs_nl_langinfo_l(RS_ABMON_12, locale));
  ASSERT_STREQ("", rs_nl_langinfo_l(RS_ERA, locale));
  ASSERT_STREQ("", rs_nl_langinfo_l(RS_ERA_D_FMT, locale));
  ASSERT_STREQ("", rs_nl_langinfo_l(RS_ERA_D_T_FMT, locale));
  ASSERT_STREQ("", rs_nl_langinfo_l(RS_ERA_T_FMT, locale));
  ASSERT_STREQ("", rs_nl_langinfo_l(RS_ALT_DIGITS, locale));
  ASSERT_STREQ(".", rs_nl_langinfo_l(RS_RADIXCHAR, locale));
  ASSERT_STREQ(",", rs_nl_langinfo_l(RS_THOUSEP, locale));
  ASSERT_STREQ("^[+1yY]", rs_nl_langinfo(RS_YESEXPR));
  ASSERT_STREQ("^[-0nN]", rs_nl_langinfo(RS_NOEXPR));
  ASSERT_STREQ("-$", rs_nl_langinfo_l(RS_CRNCYSTR, locale));
  ASSERT_STREQ("", rs_nl_langinfo_l(0xdeadc0de, locale));
  rs_freelocale(locale);
}

TEST(nl_langinfo, uk_ua)
{
  ouma_locale_t locale = rs_newlocale(RS_LC_ALL_MASK, "uk_UA.utf8", 0);
  ASSERT_STREQ("UTF-8", rs_nl_langinfo_l(RS_CODESET, locale));
  ASSERT_STREQ("%a, %e %b %Y р., %H:%M:%S",
               rs_nl_langinfo_l(RS_D_T_FMT, locale));
  ASSERT_STREQ("%e %b %Y р.", rs_nl_langinfo_l(RS_D_FMT, locale));
  ASSERT_STREQ("%H:%M:%S", rs_nl_langinfo_l(RS_T_FMT, locale));
  ASSERT_STREQ("", rs_nl_langinfo_l(RS_T_FMT_AMPM, locale));
  ASSERT_STREQ("", rs_nl_langinfo_l(RS_AM_STR, locale));
  ASSERT_STREQ("", rs_nl_langinfo_l(RS_PM_STR, locale));
  ASSERT_STREQ("неділя", rs_nl_langinfo_l(RS_DAY_1, locale));
  ASSERT_STREQ("понеділок", rs_nl_langinfo_l(RS_DAY_2, locale));
  ASSERT_STREQ("вівторок", rs_nl_langinfo_l(RS_DAY_3, locale));
  ASSERT_STREQ("середа", rs_nl_langinfo_l(RS_DAY_4, locale));
  ASSERT_STREQ("четвер", rs_nl_langinfo_l(RS_DAY_5, locale));
  ASSERT_STREQ("пʼятниця", rs_nl_langinfo_l(RS_DAY_6, locale));
  ASSERT_STREQ("субота", rs_nl_langinfo_l(RS_DAY_7, locale));
  ASSERT_STREQ("нд", rs_nl_langinfo_l(RS_ABDAY_1, locale));
  ASSERT_STREQ("пн", rs_nl_langinfo_l(RS_ABDAY_2, locale));
  ASSERT_STREQ("вт", rs_nl_langinfo_l(RS_ABDAY_3, locale));
  ASSERT_STREQ("ср", rs_nl_langinfo_l(RS_ABDAY_4, locale));
  ASSERT_STREQ("чт", rs_nl_langinfo_l(RS_ABDAY_5, locale));
  ASSERT_STREQ("пт", rs_nl_langinfo_l(RS_ABDAY_6, locale));
  ASSERT_STREQ("сб", rs_nl_langinfo_l(RS_ABDAY_7, locale));
  ASSERT_STREQ("січня", rs_nl_langinfo_l(RS_MON_1, locale));
  ASSERT_STREQ("лютого", rs_nl_langinfo_l(RS_MON_2, locale));
  ASSERT_STREQ("березня", rs_nl_langinfo_l(RS_MON_3, locale));
  ASSERT_STREQ("квітня", rs_nl_langinfo_l(RS_MON_4, locale));
  ASSERT_STREQ("травня", rs_nl_langinfo_l(RS_MON_5, locale));
  ASSERT_STREQ("червня", rs_nl_langinfo_l(RS_MON_6, locale));
  ASSERT_STREQ("липня", rs_nl_langinfo_l(RS_MON_7, locale));
  ASSERT_STREQ("серпня", rs_nl_langinfo_l(RS_MON_8, locale));
  ASSERT_STREQ("вересня", rs_nl_langinfo_l(RS_MON_9, locale));
  ASSERT_STREQ("жовтня", rs_nl_langinfo_l(RS_MON_10, locale));
  ASSERT_STREQ("листопада", rs_nl_langinfo_l(RS_MON_11, locale));
  ASSERT_STREQ("грудня", rs_nl_langinfo_l(RS_MON_12, locale));
  ASSERT_STREQ("січень", rs_nl_langinfo_l(RS_ALTMON_1, locale));
  ASSERT_STREQ("лютий", rs_nl_langinfo_l(RS_ALTMON_2, locale));
  ASSERT_STREQ("березень", rs_nl_langinfo_l(RS_ALTMON_3, locale));
  ASSERT_STREQ("квітень", rs_nl_langinfo_l(RS_ALTMON_4, locale));
  ASSERT_STREQ("травень", rs_nl_langinfo_l(RS_ALTMON_5, locale));
  ASSERT_STREQ("червень", rs_nl_langinfo_l(RS_ALTMON_6, locale));
  ASSERT_STREQ("липень", rs_nl_langinfo_l(RS_ALTMON_7, locale));
  ASSERT_STREQ("серпень", rs_nl_langinfo_l(RS_ALTMON_8, locale));
  ASSERT_STREQ("вересень", rs_nl_langinfo_l(RS_ALTMON_9, locale));
  ASSERT_STREQ("жовтень", rs_nl_langinfo_l(RS_ALTMON_10, locale));
  ASSERT_STREQ("листопад", rs_nl_langinfo_l(RS_ALTMON_11, locale));
  ASSERT_STREQ("грудень", rs_nl_langinfo_l(RS_ALTMON_12, locale));
  ASSERT_STREQ("січ", rs_nl_langinfo_l(RS_ABMON_1, locale));
  ASSERT_STREQ("лют", rs_nl_langinfo_l(RS_ABMON_2, locale));
  ASSERT_STREQ("бер", rs_nl_langinfo_l(RS_ABMON_3, locale));
  ASSERT_STREQ("квіт", rs_nl_langinfo_l(RS_ABMON_4, locale));
  ASSERT_STREQ("трав", rs_nl_langinfo_l(RS_ABMON_5, locale));
  ASSERT_STREQ("черв", rs_nl_langinfo_l(RS_ABMON_6, locale));
  ASSERT_STREQ("лип", rs_nl_langinfo_l(RS_ABMON_7, locale));
  ASSERT_STREQ("серп", rs_nl_langinfo_l(RS_ABMON_8, locale));
  ASSERT_STREQ("вер", rs_nl_langinfo_l(RS_ABMON_9, locale));
  ASSERT_STREQ("жовт", rs_nl_langinfo_l(RS_ABMON_10, locale));
  ASSERT_STREQ("лист", rs_nl_langinfo_l(RS_ABMON_11, locale));
  ASSERT_STREQ("груд", rs_nl_langinfo_l(RS_ABMON_12, locale));
  ASSERT_STREQ("січ.", rs_nl_langinfo_l(RS_ABALTMON_1, locale));
  ASSERT_STREQ("лют.", rs_nl_langinfo_l(RS_ABALTMON_2, locale));
  ASSERT_STREQ("бер.", rs_nl_langinfo_l(RS_ABALTMON_3, locale));
  ASSERT_STREQ("квіт.", rs_nl_langinfo_l(RS_ABALTMON_4, locale));
  ASSERT_STREQ("трав.", rs_nl_langinfo_l(RS_ABALTMON_5, locale));
  ASSERT_STREQ("черв.", rs_nl_langinfo_l(RS_ABALTMON_6, locale));
  ASSERT_STREQ("лип.", rs_nl_langinfo_l(RS_ABALTMON_7, locale));
  ASSERT_STREQ("серп.", rs_nl_langinfo_l(RS_ABALTMON_8, locale));
  ASSERT_STREQ("вер.", rs_nl_langinfo_l(RS_ABALTMON_9, locale));
  ASSERT_STREQ("жовт.", rs_nl_langinfo_l(RS_ABALTMON_10, locale));
  ASSERT_STREQ("лист.", rs_nl_langinfo_l(RS_ABALTMON_11, locale));
  ASSERT_STREQ("груд.", rs_nl_langinfo_l(RS_ABALTMON_12, locale));
  ASSERT_STREQ("", rs_nl_langinfo_l(RS_ERA, locale));
  ASSERT_STREQ("", rs_nl_langinfo_l(RS_ERA_D_FMT, locale));
  ASSERT_STREQ("", rs_nl_langinfo_l(RS_ERA_D_T_FMT, locale));
  ASSERT_STREQ("", rs_nl_langinfo_l(RS_ERA_T_FMT, locale));
  ASSERT_STREQ("", rs_nl_langinfo_l(RS_ALT_DIGITS, locale));
  ASSERT_STREQ(",", rs_nl_langinfo_l(RS_RADIXCHAR, locale));
  ASSERT_STREQ(" ", rs_nl_langinfo_l(RS_THOUSEP, locale));
  ASSERT_STREQ("^([+1Yy]|[Тт][Аа][Кк]?)$",
               rs_nl_langinfo_l(RS_YESEXPR, locale));
  ASSERT_STREQ("^([-0Nn]|[Нн][Іі])$", rs_nl_langinfo_l(RS_NOEXPR, locale));
  ASSERT_STREQ("+₴", rs_nl_langinfo_l(RS_CRNCYSTR, locale));
  ASSERT_STREQ("", rs_nl_langinfo_l(0xdeadc0de, locale));
  rs_freelocale(locale);
}

TEST(nl_langinfo, ja_jp)
{
  ouma_locale_t locale = rs_newlocale(RS_LC_ALL_MASK, "ja_JP.utf8", 0);
  ASSERT_STREQ("〇", rs_nl_langinfo_l(RS_ALT_DIGITS, locale));
  rs_freelocale(locale);
}
