#include "common.h"

#include <wctype.h>

#include <iostream>

extern "C"
{
  int rs_iswascii(wint_t);
  int rs_iswascii_l(wint_t, strogino_locale_t);
  int rs_iswalnum(wint_t);
  int rs_iswalnum_l(wint_t, strogino_locale_t);
  int rs_iswalpha(wint_t);
  int rs_iswalpha_l(wint_t, strogino_locale_t);
  int rs_iswblank(wint_t);
  int rs_iswblank_l(wint_t, strogino_locale_t);
  int rs_iswcntrl(wint_t);
  int rs_iswcntrl_l(wint_t, strogino_locale_t);
  int rs_iswctype(wint_t, wctype_t);
  int rs_iswctype_l(wint_t, wctype_t, strogino_locale_t);
  int rs_iswdigit(wint_t);
  int rs_iswdigit_l(wint_t, strogino_locale_t);
  int rs_iswgraph(wint_t);
  int rs_iswgraph_l(wint_t, strogino_locale_t);
  int rs_iswlower(wint_t);
  int rs_iswlower_l(wint_t, strogino_locale_t);
  int rs_iswprint(wint_t);
  int rs_iswprint_l(wint_t, strogino_locale_t);
  int rs_iswpunct(wint_t);
  int rs_iswpunct_l(wint_t, strogino_locale_t);
  int rs_iswspace(wint_t);
  int rs_iswspace_l(wint_t, strogino_locale_t);
  int rs_iswupper(wint_t);
  int rs_iswupper_l(wint_t, strogino_locale_t);
  int rs_iswxdigit(wint_t);
  int rs_iswxdigit_l(wint_t, strogino_locale_t);
  wint_t rs_towctrans(wint_t, wctrans_t);
  wint_t rs_towctrans_l(wint_t, wctrans_t, strogino_locale_t);
  wint_t rs_towlower(wint_t);
  wint_t rs_towlower_l(wint_t, strogino_locale_t);
  wint_t rs_towupper(wint_t);
  wint_t rs_towupper_l(wint_t, strogino_locale_t);
  wctrans_t rs_wctrans(const char*);
  wctrans_t rs_wctrans_l(const char*, strogino_locale_t);
  wctype_t rs_wctype(const char*);
  wctype_t rs_wctype_l(const char*, strogino_locale_t);

  int rs_isalnum(int);
  int rs_isalnum_l(int, strogino_locale_t);
  int rs_isalpha(int);
  int rs_isalpha_l(int, strogino_locale_t);
  int rs_isascii(int);
  int rs_isascii_l(int, strogino_locale_t);
  int rs_isblank(int);
  int rs_isblank_l(int, strogino_locale_t);
  int rs_iscntrl(int);
  int rs_iscntrl_l(int, strogino_locale_t);
  int rs_isdigit(int);
  int rs_isdigit_l(int, strogino_locale_t);
  int rs_isgraph(int);
  int rs_isgraph_l(int, strogino_locale_t);
  int rs_islower(int);
  int rs_islower_l(int, strogino_locale_t);
  int rs_isprint(int);
  int rs_isprint_l(int, strogino_locale_t);
  int rs_ispunct(int);
  int rs_ispunct_l(int, strogino_locale_t);
  int rs_isspace(int);
  int rs_isspace_l(int, strogino_locale_t);
  int rs_isupper(int);
  int rs_isupper_l(int, strogino_locale_t);
  int rs_isxdigit(int);
  int rs_isxdigit_l(int, strogino_locale_t);
  int rs_toascii(int);
  int rs_toascii_l(int, strogino_locale_t);
  int rs_tolower(int);
  int rs_tolower_l(int, strogino_locale_t);
  int rs_toupper(int);
  int rs_toupper_l(int, strogino_locale_t);

  strogino_locale_t rs_newlocale(int, const char*, strogino_locale_t);
  void rs_freelocale(strogino_locale_t);
  const char* rs_getlocalename_l(int, strogino_locale_t);
}

TEST(iswascii, examples)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C"), nullptr);

  strogino_locale_t loc = rs_newlocale(LC_CTYPE_MASK, "nl_BE.UTF-8", 0);
  ASSERT_NE(nullptr, loc);
  ASSERT_NE(ENOENT, rs_errno);
  ASSERT_STREQ("nl_BE.UTF-8", rs_getlocalename_l(LC_CTYPE, loc));

  ASSERT_FALSE(rs_iswascii(WEOF));

  ASSERT_TRUE(rs_iswascii(L'0'));
  ASSERT_TRUE(rs_iswascii(L'A'));
  ASSERT_TRUE(rs_iswascii(L'a'));
  ASSERT_TRUE(rs_iswascii_l(L'B', loc));
  ASSERT_FALSE(rs_iswascii_l(L'Å', loc));

  ASSERT_FALSE(rs_iswascii(L'€'));
  ASSERT_FALSE(rs_iswascii_l(L'€', loc));

  rs_freelocale(loc);
}

TEST(iswalnum, examples)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C"), nullptr);

  strogino_locale_t loc = rs_newlocale(LC_CTYPE_MASK, "nl_BE.UTF-8", 0);
  ASSERT_NE(nullptr, loc);
  ASSERT_NE(ENOENT, rs_errno);
  ASSERT_STREQ("nl_BE.UTF-8", rs_getlocalename_l(LC_CTYPE, loc));

  ASSERT_FALSE(rs_iswalnum(WEOF));

  ASSERT_TRUE(rs_iswalnum(L'0'));
  ASSERT_TRUE(rs_iswalnum(L'A'));
  ASSERT_TRUE(rs_iswalnum(L'a'));
  ASSERT_TRUE(rs_iswalnum_l(L'Å', loc));
  ASSERT_TRUE(rs_iswalnum_l(L'Ω', loc));
  ASSERT_TRUE(rs_iswalnum_l(L'д', loc));

  ASSERT_FALSE(rs_iswalnum(L' '));
  ASSERT_FALSE(rs_iswalnum(L'.'));
  ASSERT_FALSE(rs_iswalnum_l(L'€', loc));

  rs_freelocale(loc);
}

TEST(iswalpha, examples)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C"), nullptr);

  strogino_locale_t loc = rs_newlocale(LC_CTYPE_MASK, "nl_BE.UTF-8", 0);
  ASSERT_NE(nullptr, loc);
  ASSERT_NE(ENOENT, rs_errno);
  ASSERT_STREQ("nl_BE.UTF-8", rs_getlocalename_l(LC_CTYPE, loc));

  ASSERT_FALSE(rs_iswalpha(WEOF));

  ASSERT_TRUE(rs_iswalpha(L'A'));
  ASSERT_TRUE(rs_iswalpha(L'a'));
  ASSERT_TRUE(rs_iswalpha_l(L'Å', loc));
  ASSERT_TRUE(rs_iswalpha_l(L'Ω', loc));
  ASSERT_TRUE(rs_iswalpha_l(L'д', loc));

  ASSERT_FALSE(rs_iswalpha(L'0'));
  ASSERT_FALSE(rs_iswalpha(L' '));
  ASSERT_FALSE(rs_iswalpha(L'.'));
  ASSERT_FALSE(rs_iswalpha_l(L'€', loc));

  rs_freelocale(loc);
}

TEST(iswblank, examples)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C"), nullptr);

  strogino_locale_t loc = rs_newlocale(LC_CTYPE_MASK, "nl_BE.UTF-8", 0);
  ASSERT_NE(nullptr, loc);
  ASSERT_NE(ENOENT, rs_errno);
  ASSERT_STREQ("nl_BE.UTF-8", rs_getlocalename_l(LC_CTYPE, loc));

  ASSERT_FALSE(rs_iswblank(WEOF));

  ASSERT_TRUE(rs_iswblank(L'\t'));
  ASSERT_TRUE(rs_iswblank(L' '));
  ASSERT_TRUE(rs_iswblank_l(L'\u2001', loc));

  ASSERT_FALSE(rs_iswblank(L'\n'));
  ASSERT_FALSE(rs_iswblank(L'A'));
  ASSERT_FALSE(rs_iswblank(L'.'));
  ASSERT_FALSE(rs_iswblank_l(L'€', loc));

  rs_freelocale(loc);
}

TEST(iswcntrl, examples)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C"), nullptr);

  strogino_locale_t loc = rs_newlocale(LC_CTYPE_MASK, "nl_BE.UTF-8", 0);
  ASSERT_NE(nullptr, loc);
  ASSERT_NE(ENOENT, rs_errno);
  ASSERT_STREQ("nl_BE.UTF-8", rs_getlocalename_l(LC_CTYPE, loc));

  ASSERT_FALSE(rs_iswcntrl(WEOF));

  ASSERT_TRUE(rs_iswcntrl(L'\0'));
  ASSERT_TRUE(rs_iswcntrl(L'\n'));
  ASSERT_TRUE(rs_iswcntrl(L'\t'));

  ASSERT_FALSE(rs_iswcntrl(L'0'));
  ASSERT_FALSE(rs_iswcntrl(L'A'));
  ASSERT_FALSE(rs_iswcntrl(L' '));
  ASSERT_FALSE(rs_iswcntrl(L'.'));
  ASSERT_FALSE(rs_iswcntrl_l(L'€', loc));

  rs_freelocale(loc);
}

TEST(iswctype, good)
{
  wctype_t wt = rs_wctype("upper");
  ASSERT_NE((wctype_t)0, wt);
  ASSERT_TRUE(rs_iswctype('A', wt));
  ASSERT_FALSE(rs_iswctype('a', wt));
}

TEST(iswctype, bad)
{
  wctype_t wt = rs_wctype("banana");
  ASSERT_EQ((wctype_t)0, wt);
  ASSERT_FALSE(rs_iswctype('p', wt));
}

TEST(iswdigit, examples)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C"), nullptr);

  strogino_locale_t loc = rs_newlocale(LC_CTYPE_MASK, "nl_BE.UTF-8", 0);
  ASSERT_NE(nullptr, loc);
  ASSERT_NE(ENOENT, rs_errno);
  ASSERT_STREQ("nl_BE.UTF-8", rs_getlocalename_l(LC_CTYPE, loc));

  ASSERT_FALSE(rs_iswdigit(WEOF));

  ASSERT_TRUE(rs_iswdigit(L'0'));
  ASSERT_TRUE(rs_iswdigit(L'1'));
  ASSERT_TRUE(rs_iswdigit(L'2'));
  ASSERT_TRUE(rs_iswdigit(L'3'));
  ASSERT_TRUE(rs_iswdigit(L'4'));
  ASSERT_TRUE(rs_iswdigit(L'5'));
  ASSERT_TRUE(rs_iswdigit(L'6'));
  ASSERT_TRUE(rs_iswdigit(L'7'));
  ASSERT_TRUE(rs_iswdigit(L'8'));
  ASSERT_TRUE(rs_iswdigit(L'9'));

  ASSERT_FALSE(rs_iswdigit(L'A'));
  ASSERT_FALSE(rs_iswdigit_l(L'①', loc));
  ASSERT_FALSE(rs_iswdigit(L'?'));

  rs_freelocale(loc);
}

TEST(iswgraph, examples)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C"), nullptr);

  strogino_locale_t loc = rs_newlocale(LC_CTYPE_MASK, "nl_BE.UTF-8", 0);
  ASSERT_NE(nullptr, loc);
  ASSERT_NE(ENOENT, rs_errno);
  ASSERT_STREQ("nl_BE.UTF-8", rs_getlocalename_l(LC_CTYPE, loc));

  ASSERT_FALSE(rs_iswgraph(WEOF));

  ASSERT_TRUE(rs_iswgraph(L'0'));
  ASSERT_TRUE(rs_iswgraph(L'A'));
  ASSERT_TRUE(rs_iswgraph(L'a'));
  ASSERT_TRUE(rs_iswgraph_l(L'Å', loc));
  ASSERT_TRUE(rs_iswgraph_l(L'Ω', loc));
  ASSERT_TRUE(rs_iswgraph_l(L'д', loc));
  ASSERT_TRUE(rs_iswgraph(L'.'));
  ASSERT_TRUE(rs_iswgraph_l(L'€', loc));
  ASSERT_FALSE(rs_iswgraph(L' '));

  rs_freelocale(loc);
}

TEST(iswlower, examples)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C.UTF-8"), nullptr);

  for (wint_t wc = 0; wc <= 0x10ffff; ++wc) {
    SCOPED_TRACE(wc);
    ASSERT_EQ(wc != rs_towupper(wc), rs_iswlower(wc));
  }
}

TEST(iswprint, examples)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C"), nullptr);

  strogino_locale_t loc = rs_newlocale(LC_CTYPE_MASK, "nl_BE.UTF-8", 0);
  ASSERT_NE(nullptr, loc);
  ASSERT_NE(ENOENT, rs_errno);
  ASSERT_STREQ("nl_BE.UTF-8", rs_getlocalename_l(LC_CTYPE, loc));

  ASSERT_FALSE(rs_iswprint(WEOF));

  ASSERT_TRUE(rs_iswprint(L' '));
  ASSERT_TRUE(rs_iswprint_l(L'\u2001', loc));
  ASSERT_TRUE(rs_iswprint_l(L'\u00a0', loc));
  ASSERT_TRUE(rs_iswprint(L'A'));
  ASSERT_TRUE(rs_iswprint(L'.'));
  ASSERT_TRUE(rs_iswprint_l(L'€', loc));

  ASSERT_FALSE(rs_iswprint(L'\t'));
  ASSERT_FALSE(rs_iswprint(L'\n'));

  rs_freelocale(loc);
}

TEST(iswpunct, examples)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C"), nullptr);

  strogino_locale_t loc = rs_newlocale(LC_CTYPE_MASK, "nl_BE.UTF-8", 0);
  ASSERT_NE(nullptr, loc);
  ASSERT_NE(ENOENT, rs_errno);
  ASSERT_STREQ("nl_BE.UTF-8", rs_getlocalename_l(LC_CTYPE, loc));

  ASSERT_FALSE(rs_iswpunct(WEOF));

  ASSERT_TRUE(rs_iswpunct(L'.'));

  ASSERT_FALSE(rs_iswpunct_l(L'€', loc));
  ASSERT_FALSE(rs_iswpunct(L'A'));
  ASSERT_FALSE(rs_iswpunct(L'\t'));
  ASSERT_FALSE(rs_iswpunct(L' '));
  ASSERT_FALSE(rs_iswpunct_l(L'\u2001', loc));
  ASSERT_FALSE(rs_iswpunct(L'\n'));

  rs_freelocale(loc);
}

TEST(iswspace, examples)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C"), nullptr);

  strogino_locale_t loc = rs_newlocale(LC_CTYPE_MASK, "nl_BE.UTF-8", 0);
  ASSERT_NE(nullptr, loc);
  ASSERT_NE(ENOENT, rs_errno);
  ASSERT_STREQ("nl_BE.UTF-8", rs_getlocalename_l(LC_CTYPE, loc));

  ASSERT_FALSE(rs_iswspace(WEOF));

  ASSERT_TRUE(rs_iswspace(L'\t'));
  ASSERT_TRUE(rs_iswspace(L' '));
  ASSERT_TRUE(rs_iswspace_l(L'\u2001', loc));
  ASSERT_TRUE(rs_iswspace(L'\n'));

  ASSERT_FALSE(rs_iswspace(L'A'));
  ASSERT_FALSE(rs_iswspace(L'.'));
  ASSERT_FALSE(rs_iswspace_l(L'€', loc));

  rs_freelocale(loc);
}

TEST(iswupper, examples)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C.UTF-8"), nullptr);

  for (wint_t wc = 0; wc <= 0x10ffff; ++wc) {
    SCOPED_TRACE(wc);
    ASSERT_EQ(wc != rs_towlower(wc), rs_iswupper(wc));
  }
}

TEST(iswxdigit, examples)
{
  ASSERT_FALSE(rs_iswalnum(WEOF));

  ASSERT_TRUE(rs_iswxdigit(L'0'));
  ASSERT_TRUE(rs_iswxdigit(L'1'));
  ASSERT_TRUE(rs_iswxdigit(L'2'));
  ASSERT_TRUE(rs_iswxdigit(L'3'));
  ASSERT_TRUE(rs_iswxdigit(L'4'));
  ASSERT_TRUE(rs_iswxdigit(L'5'));
  ASSERT_TRUE(rs_iswxdigit(L'6'));
  ASSERT_TRUE(rs_iswxdigit(L'7'));
  ASSERT_TRUE(rs_iswxdigit(L'8'));
  ASSERT_TRUE(rs_iswxdigit(L'9'));

  ASSERT_TRUE(rs_iswxdigit(L'A'));
  ASSERT_TRUE(rs_iswxdigit(L'a'));
  ASSERT_TRUE(rs_iswxdigit(L'B'));
  ASSERT_TRUE(rs_iswxdigit(L'b'));
  ASSERT_TRUE(rs_iswxdigit(L'C'));
  ASSERT_TRUE(rs_iswxdigit(L'c'));
  ASSERT_TRUE(rs_iswxdigit(L'D'));
  ASSERT_TRUE(rs_iswxdigit(L'd'));
  ASSERT_TRUE(rs_iswxdigit(L'E'));
  ASSERT_TRUE(rs_iswxdigit(L'e'));
  ASSERT_TRUE(rs_iswxdigit(L'F'));
  ASSERT_TRUE(rs_iswxdigit(L'f'));

  ASSERT_FALSE(rs_iswxdigit(L'g'));
  ASSERT_FALSE(rs_iswxdigit(L'１'));
}

TEST(towlower, examples)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C"), nullptr);

  strogino_locale_t loc = rs_newlocale(LC_CTYPE_MASK, "nl_BE.UTF-8", 0);
  ASSERT_NE(nullptr, loc);
  ASSERT_NE(ENOENT, rs_errno);
  ASSERT_STREQ("nl_BE.UTF-8", rs_getlocalename_l(LC_CTYPE, loc));

  ASSERT_EQ(WEOF, rs_towlower(WEOF));

  ASSERT_EQ(L' ', rs_towlower(L' '));
  ASSERT_EQ(L'€', rs_towlower_l(L'€', loc));

  ASSERT_EQ(L'a', rs_towlower(L'A'));
  ASSERT_EQ(L'a', rs_towlower(L'a'));

  ASSERT_EQ(L'ä', rs_towlower_l(L'Ä', loc));
  ASSERT_EQ(L'ä', rs_towlower_l(L'ä', loc));

  ASSERT_EQ(L'λ', rs_towlower_l(L'Λ', loc));
  ASSERT_EQ(L'λ', rs_towlower_l(L'λ', loc));

  ASSERT_EQ(L'𐐷', rs_towlower_l(L'𐐏', loc));
  ASSERT_EQ(L'𐐷', rs_towlower_l(L'𐐷', loc));

  rs_freelocale(loc);
}

TEST(towupper, examples)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C"), nullptr);

  strogino_locale_t loc = rs_newlocale(LC_CTYPE_MASK, "nl_BE.UTF-8", 0);
  ASSERT_NE(nullptr, loc);
  ASSERT_NE(ENOENT, rs_errno);
  ASSERT_STREQ("nl_BE.UTF-8", rs_getlocalename_l(LC_CTYPE, loc));

  ASSERT_EQ(WEOF, towupper(WEOF));

  ASSERT_EQ(L' ', towupper(L' '));
  ASSERT_EQ(L'€', rs_towupper_l(L'€', loc));

  ASSERT_EQ(L'A', towupper(L'A'));
  ASSERT_EQ(L'A', towupper(L'a'));

  ASSERT_EQ(L'Ä', rs_towupper_l(L'Ä', loc));
  ASSERT_EQ(L'Ä', rs_towupper_l(L'ä', loc));

  ASSERT_EQ(L'Ÿ', rs_towupper_l(L'Ÿ', loc));
  ASSERT_EQ(L'Ÿ', rs_towupper_l(L'ÿ', loc));

  ASSERT_EQ(L'Λ', rs_towupper_l(L'Λ', loc));
  ASSERT_EQ(L'Λ', rs_towupper_l(L'λ', loc));

  ASSERT_EQ(L'𐐏', rs_towupper_l(L'𐐏', loc));
  ASSERT_EQ(L'𐐏', rs_towupper_l(L'𐐷', loc));

  rs_freelocale(loc);
}

TEST(towctrans, examples)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C"), nullptr);

  ASSERT_EQ(wint_t('a'), rs_towctrans(L'A', rs_wctrans("tolower")));
  ASSERT_EQ(WEOF, rs_towctrans(WEOF, rs_wctrans("tolower")));
  ASSERT_EQ(wint_t('A'), rs_towctrans(L'a', rs_wctrans("toupper")));
  ASSERT_EQ(WEOF, rs_towctrans(WEOF, rs_wctrans("toupper")));
}

TEST(wctrans, example)
{
  ASSERT_TRUE(rs_wctrans("tolower") != 0);
  ASSERT_TRUE(rs_wctrans("toupper") != 0);
  ASSERT_TRUE(rs_wctrans("monkeys") == 0);
}

TEST(wctype, classes)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C"), nullptr);

  for (wint_t wc = 0; wc <= 0x10ffff; ++wc) {
    SCOPED_TRACE(wc);

    bool alnum = rs_iswalnum(wc);
    bool alpha = rs_iswalpha(wc);
    bool blank = rs_iswblank(wc);
    bool cntrl = rs_iswcntrl(wc);
    bool digit = rs_iswdigit(wc);
    bool graph = rs_iswgraph(wc);
    bool lower = rs_iswlower(wc);
    bool print = rs_iswprint(wc);
    bool punct = rs_iswpunct(wc);
    bool space = rs_iswspace(wc);
    bool upper = rs_iswupper(wc);
    bool xdigit = rs_iswxdigit(wc);

    ASSERT_EQ(alnum, alpha || digit);
    ASSERT_EQ(graph, alnum || punct);

    ASSERT_TRUE(!upper || alpha);
    ASSERT_TRUE(!upper || graph);
    ASSERT_TRUE(!upper || print);
    ASSERT_TRUE(!lower || alpha);
    ASSERT_TRUE(!lower || graph);
    ASSERT_TRUE(!lower || print);
    ASSERT_TRUE(!alpha || graph);
    ASSERT_TRUE(!alpha || print);
    ASSERT_TRUE(!digit || graph);
    ASSERT_TRUE(!digit || print);
    ASSERT_TRUE(!digit || xdigit);
    ASSERT_TRUE(!punct || graph);
    ASSERT_TRUE(!punct || print);
    ASSERT_TRUE(!graph || print);
    ASSERT_TRUE(!xdigit || graph);
    ASSERT_TRUE(!xdigit || print);
    ASSERT_TRUE(!blank || space);

    ASSERT_FALSE(upper && digit);
    ASSERT_FALSE(upper && space);
    ASSERT_FALSE(upper && cntrl);
    ASSERT_FALSE(upper && punct);
    ASSERT_FALSE(upper && blank);
    ASSERT_FALSE(lower && digit);
    ASSERT_FALSE(lower && space);
    ASSERT_FALSE(lower && cntrl);
    ASSERT_FALSE(lower && punct);
    ASSERT_FALSE(lower && blank);
    ASSERT_FALSE(alpha && digit);
    ASSERT_FALSE(alpha && space);
    ASSERT_FALSE(alpha && cntrl);
    ASSERT_FALSE(alpha && punct);
    ASSERT_FALSE(alpha && blank);
    ASSERT_FALSE(digit && space);
    ASSERT_FALSE(digit && cntrl);
    ASSERT_FALSE(digit && punct);
    ASSERT_FALSE(digit && blank);
    ASSERT_FALSE(space && xdigit);
    ASSERT_FALSE(cntrl && punct);
    ASSERT_FALSE(cntrl && graph);
    ASSERT_FALSE(cntrl && print);
    ASSERT_FALSE(cntrl && xdigit);
    ASSERT_FALSE(punct && xdigit);
    ASSERT_FALSE(xdigit && blank);
  }
}

TEST(ctype, eof)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C"), nullptr);

  ASSERT_EQ(0, rs_isalnum(EOF));
  ASSERT_EQ(0, rs_isalpha(EOF));
  ASSERT_FALSE(rs_isascii(EOF));
  ASSERT_EQ(0, rs_isblank(EOF));
  ASSERT_EQ(0, rs_iscntrl(EOF));
  ASSERT_EQ(0, rs_isdigit(EOF));
  ASSERT_EQ(0, rs_isgraph(EOF));
  ASSERT_EQ(0, rs_islower(EOF));
  ASSERT_EQ(0, rs_isprint(EOF));
  ASSERT_EQ(0, rs_ispunct(EOF));
  ASSERT_EQ(0, rs_isspace(EOF));
  ASSERT_EQ(0, rs_isupper(EOF));
  ASSERT_EQ(0, rs_isxdigit(EOF));
  ASSERT_EQ(EOF, rs_tolower(EOF));
  ASSERT_EQ(EOF, rs_toupper(EOF));
}

TEST(ctype, ascii_to_wchar)
{
  ASSERT_NE(rs_setlocale(LC_ALL, "C"), nullptr);

  for (int ch = 0; ch <= 127; ++ch) {
    SCOPED_TRACE(ch);

    ASSERT_EQ(rs_iswalnum(ch), rs_isalnum(ch));
    ASSERT_EQ(rs_iswalpha(ch), rs_isalpha(ch));
    ASSERT_EQ(rs_iswascii(ch), rs_isascii(ch));
    ASSERT_EQ(rs_iswblank(ch), rs_isblank(ch));
    ASSERT_EQ(rs_iswcntrl(ch), rs_iscntrl(ch));
    ASSERT_EQ(rs_iswdigit(ch), rs_isdigit(ch));
    ASSERT_EQ(rs_iswgraph(ch), rs_isgraph(ch));
    ASSERT_EQ(rs_iswlower(ch), rs_islower(ch));
    ASSERT_EQ(rs_iswprint(ch), rs_isprint(ch));
    ASSERT_EQ(rs_iswpunct(ch), rs_ispunct(ch));
    ASSERT_EQ(rs_iswspace(ch), rs_isspace(ch));
    ASSERT_EQ(rs_iswupper(ch), rs_isupper(ch));
    ASSERT_EQ(rs_iswxdigit(ch), rs_isxdigit(ch));
    ASSERT_EQ(rs_towlower(ch), rs_tolower(ch));
    ASSERT_EQ(rs_towupper(ch), rs_toupper(ch));
  }

  for (int ch = 128; ch <= 255; ++ch) {
    SCOPED_TRACE(ch);

    ASSERT_EQ(0, rs_isalnum(ch));
    ASSERT_EQ(0, rs_isalpha(ch));
    ASSERT_FALSE(rs_isascii(ch));
    ASSERT_EQ(0, rs_isblank(ch));
    ASSERT_EQ(0, rs_iscntrl(ch));
    ASSERT_EQ(0, rs_isdigit(ch));
    ASSERT_EQ(0, rs_isgraph(ch));
    ASSERT_EQ(0, rs_islower(ch));
    ASSERT_EQ(0, rs_isprint(ch));
    ASSERT_EQ(0, rs_ispunct(ch));
    ASSERT_EQ(0, rs_isspace(ch));
    ASSERT_EQ(0, rs_isupper(ch));
    ASSERT_EQ(0, rs_isxdigit(ch));
    ASSERT_EQ(ch, rs_tolower(ch));
    ASSERT_EQ(ch, rs_toupper(ch));
  }
}

TEST(ctype, unicode_equality)
{
  ASSERT_NE(rs_setlocale(LC_CTYPE, "C.UTF-8"), nullptr);

  for (int ch = 0; ch <= UCHAR_MAX; ++ch) {
    SCOPED_TRACE(ch);

    ASSERT_EQ(rs_iswalnum(ch), rs_isalnum(ch));
    ASSERT_EQ(rs_iswalpha(ch), rs_isalpha(ch));
    ASSERT_EQ(rs_iswblank(ch), rs_isblank(ch));
    ASSERT_EQ(rs_iswcntrl(ch), rs_iscntrl(ch));
    ASSERT_EQ(rs_iswdigit(ch), rs_isdigit(ch));
    ASSERT_EQ(rs_iswgraph(ch), rs_isgraph(ch));
    ASSERT_EQ(rs_iswlower(ch), rs_islower(ch));
    ASSERT_EQ(rs_iswprint(ch), rs_isprint(ch));
    ASSERT_EQ(rs_iswpunct(ch), rs_ispunct(ch));
    ASSERT_EQ(rs_iswspace(ch), rs_isspace(ch));
    ASSERT_EQ(rs_iswupper(ch), rs_isupper(ch));
    ASSERT_EQ(rs_iswxdigit(ch), rs_isxdigit(ch));
    ASSERT_EQ(rs_towlower(ch), rs_tolower(ch));
    ASSERT_EQ(rs_towupper(ch), rs_toupper(ch));
  }
}
