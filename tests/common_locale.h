typedef void *strogino_locale_t;

#define RS_LC_CTYPE 0
#define RS_LC_NUMERIC 1
#define RS_LC_TIME 2
#define RS_LC_COLLATE 3
#define RS_LC_MONETARY 4
#define RS_LC_MESSAGES 5
#define RS_LC_ALL 6

#define RS_LC_CTYPE_MASK (1 << RS_LC_CTYPE)
#define RS_LC_NUMERIC_MASK (1 << RS_LC_NUMERIC)
#define RS_LC_TIME_MASK (1 << RS_LC_TIME)
#define RS_LC_COLLATE_MASK (1 << RS_LC_COLLATE)
#define RS_LC_MONETARY_MASK (1 << RS_LC_MONETARY)
#define RS_LC_MESSAGES_MASK (1 << RS_LC_MESSAGES)
#define RS_LC_ALL_MASK 0x7fffffff

#define RS_LC_GLOBAL_LOCALE ((strogino_locale_t)(-1))

extern "C" {
    char *rs_setlocale(int, const char *);
    strogino_locale_t rs_duplocale(strogino_locale_t);
    void rs_freelocale(strogino_locale_t);
    const char *rs_getlocalename_l(int, strogino_locale_t);
    strogino_locale_t rs_newlocale(int, const char *, strogino_locale_t);
    strogino_locale_t rs_uselocale(strogino_locale_t);
    struct lconv *rs_localeconv(void);
    struct lconv *rs_localeconv_l(strogino_locale_t);
}
