#include "common.h"
#include "common_locale.h"
#include <cerrno>

extern "C" {
void *rs_memccpy(void *__restrict, const void *__restrict, int, size_t);
void *rs_memchr(const void *, int, size_t);
int rs_memcmp(const void *, const void *, size_t);
void *rs_memcpy(void *__restrict, const void *__restrict, size_t);
void *rs_memmem(const void *, size_t, const void *, size_t);
void *rs_memmove(void *, const void *, size_t);
void *rs_memset(void *, int, size_t);
void *rs_memset_explicit(void *, int, size_t);
char *rs_strchr(const char *, int);
char *rs_stpcpy(char *__restrict, const char *__restrict);
char *rs_stpncpy(char *__restrict, const char *__restrict, size_t);
char *rs_strncat(char *__restrict, const char *__restrict, size_t);
int rs_strncmp(const char *, const char *, size_t);
char *rs_strncpy(char *__restrict, const char *__restrict, size_t);
char *rs_strcat(char *__restrict, const char *__restrict);
int rs_strcmp(const char *, const char *);
char *rs_strcpy(char *__restrict, const char *__restrict);
size_t rs_strnlen(const char *, size_t);
size_t rs_strlen(const char *);
size_t rs_strcspn(const char *, const char *);
size_t rs_strspn(const char *, const char *);
char *rs_strpbrk(const char *, const char *);
char *rs_strrchr(const char *, int);
char *rs_strstr(const char *, const char *);
char *rs_strtok_r(char *__restrict, const char *__restrict, char **__restrict);
char *rs_strtok(char *__restrict, const char *__restrict);
int rs_strcoll(const char *, const char *);
size_t rs_strxfrm(char *__restrict, const char *__restrict, size_t);
int rs___xpg_strerror_r(int, char *, size_t);
char *rs_strerror_r(int, char *, size_t);
char *rs_strerror(int);
char *rs_strerror_l(int, ouma_locale_t);
char *rs_strsignal(int);
char *rs_strsignal_l(int, ouma_locale_t);
char *rs_strndup(const char *, size_t);
char *rs_strdup(const char *);
size_t rs_strlcat(char *, const char *, size_t);
size_t rs_strlcpy(char *, const char *t, size_t);
const char *rs_strerrorname_np(int errnum);
const char *rs_strerrordesc_np(int errnum);
}

TEST(memccpy, null) {
  ASSERT_EQ(NULL, rs_memccpy((char *)456, (char *)789, 'A', 0));
}

TEST(memccpy, example) {
  const char buf1[13] = "Test\0string!";
  char buf2[] = "AAAAAAAAA";
  ASSERT_EQ(&buf2[8], rs_memccpy(buf2, buf1, 'r', 9999));
  ASSERT_THAT(buf2, testing::ElementsAreArray("Test\0strA"));
}

TEST(memchr, null) { ASSERT_EQ(NULL, rs_memchr((char *)nullptr, 'A', 0)); }

TEST(memchr, match) {
  char buf[] = "Foo bar baz";
  ASSERT_EQ(buf + 5, rs_memchr(buf, 'a', sizeof(buf)));
}

TEST(memchr, nomatch) {
  char buf[] = "Foo bar baz";
  ASSERT_EQ(NULL, rs_memchr(buf, 'x', sizeof(buf)));
}

TEST(memcmp, null) { ASSERT_EQ(0, rs_memcmp(NULL, NULL, 0)); }

TEST(memcmp, example) {
  const char buf1[] = "Hello";
  const char buf2[] = "Helxo";
  ASSERT_EQ(0, rs_memcmp(buf1, buf1, sizeof(buf1)));
  ASSERT_GT(0, rs_memcmp(buf1, buf2, sizeof(buf1)));
  ASSERT_LT(0, rs_memcmp(buf2, buf1, sizeof(buf1)));
}

TEST(memcpy, null) {
  ASSERT_EQ((char *)42, rs_memcpy((char *)42, (char *)123, 0));
}

TEST(memcpy, example) {
  const char buf1[8] = "Foo\0Bar";
  char buf2[8];
  ASSERT_EQ(buf2, rs_memcpy(buf2, buf1, sizeof(buf1)));
  ASSERT_THAT(buf2, testing::ElementsAreArray(buf1));
}

TEST(memmem, empty_haystack_empty_needle_returns_haystack) {
  char *h = nullptr;
  char *n = nullptr;
  void *result = rs_memmem(h, 0, n, 0);
  ASSERT_EQ(static_cast<char *>(result), h);
}

TEST(memmem, empty_haystack_non_empty_needle_ret_null) {
  char *h = nullptr;
  char n[] = {'a', 'b', 'c'};
  void *result = rs_memmem(h, 0, n, sizeof(n));
  ASSERT_EQ(result, static_cast<void *>(nullptr));
}

TEST(memmem, empty_needle_returns_haystack) {
  char h[] = {'a', 'b', 'c'};
  char *n = nullptr;
  void *result = rs_memmem(h, sizeof(h), n, 0);
  ASSERT_EQ(static_cast<char *>(result), h + 0);
}

TEST(memmem, exact_match_returns_haystack) {
  char h[] = {'a', 'b', 'c'};
  char n[] = {'a', 'b', 'c'};
  void *result = rs_memmem(h, sizeof(h), n, sizeof(n));
  ASSERT_EQ(static_cast<char *>(result), h + 0);
}

TEST(memmem, return_first_match_of_needle) {
  char h[] = {'a', 'a', 'b', 'c'};
  char n[] = {'a'};
  void *result = rs_memmem(h, sizeof(h), n, sizeof(n));
  ASSERT_EQ(static_cast<char *>(result), h + 0);
}

TEST(memmem, return_first_exact_match_of_needle) {
  {
    char h[] = {'a', 'b', 'a', 'c', 'a', 'a'};
    char n[] = {'a', 'a'};
    void *result = rs_memmem(h, sizeof(h), n, sizeof(n));
    ASSERT_EQ(static_cast<char *>(result), h + 4);
  }
  {
    char h[] = {'a', 'a', 'b', 'a', 'b', 'a'};
    char n[] = {'a', 'b', 'a'};
    void *result = rs_memmem(h, sizeof(h), n, sizeof(n));
    ASSERT_EQ(static_cast<char *>(result), h + 1);
  }
}

TEST(memmem, null_terminator_doesnt_interrupt_match) {
  char h[] = {'\0', 'a', 'b'};
  char n[] = {'a', 'b'};
  void *result = rs_memmem(h, sizeof(h), n, sizeof(n));
  ASSERT_EQ(static_cast<char *>(result), h + 1);
}

TEST(memmem, return_null_on_no_exact_match) {
  {
    char h[] = {'a'};
    char n[] = {'a', 'a'};
    void *result = rs_memmem(h, sizeof(h), n, sizeof(n));
    ASSERT_EQ(result, static_cast<void *>(nullptr));
  }
  {
    char h[] = {'a', 'A'};
    char n[] = {'a', 'a'};
    void *result = rs_memmem(h, sizeof(h), n, sizeof(n));
    ASSERT_EQ(result, static_cast<void *>(nullptr));
  }
  {
    char h[] = {'a'};
    char n[] = {'a', '\0'};
    void *result = rs_memmem(h, sizeof(h), n, sizeof(n));
    ASSERT_EQ(result, static_cast<void *>(nullptr));
  }
  {
    char h[] = {'\0'};
    char n[] = {'\0', '\0'};
    void *result = rs_memmem(h, sizeof(h), n, sizeof(n));
    ASSERT_EQ(result, static_cast<void *>(nullptr));
  }
}

TEST(memmem, return_match_of_specified_needle_len) {
  {
    char h[] = {'a', 'b', 'c'};
    char n[] = {'x', 'y', 'z'};
    void *result = rs_memmem(h, sizeof(h), n, 0);
    ASSERT_EQ(static_cast<char *>(result), h + 0);
  }
  {
    char h[] = {'a', 'b', 'c'};
    char n[] = {'b', 'c', 'a'};
    void *result = rs_memmem(h, sizeof(h), n, 2);
    ASSERT_EQ(static_cast<char *>(result), h + 1);
  }
}

TEST(memmem, return_null_if_inadequate_haystack_len) {
  {
    char h[] = {'a', 'b', 'c'};
    char n[] = {'c'};
    void *result = rs_memmem(h, 2, n, sizeof(n));
    ASSERT_EQ(result, static_cast<void *>(nullptr));
  }
  {
    char h[] = {'a', 'b', 'c'};
    char n[] = {'a', 'b', 'c'};
    void *result = rs_memmem(h, 2, n, sizeof(n));
    ASSERT_EQ(result, static_cast<void *>(nullptr));
  }
}

TEST(memmove, null) {
  ASSERT_EQ((char *)42, rs_memmove((char *)42, (char *)34, 0));
}

TEST(memmove, example1) {
  char buf[] = "abcdefghijkl";
  ASSERT_EQ(buf, rs_memmove(buf, buf + 4, 8));
  ASSERT_STREQ("efghijklijkl", buf);
}

TEST(memmove, example2) {
  char buf[] = "abcdefghijkl";
  ASSERT_EQ(buf + 4, rs_memmove(buf + 4, buf, 8));
  ASSERT_STREQ("abcdabcdefgh", buf);
}

TEST(memset, null) { ASSERT_EQ((char *)5, rs_memset((char *)5, 'A', 0)); }

TEST(memset, example_small) {
  char buf[11];
  ASSERT_EQ(buf, rs_memset(buf, '!', 10));
  buf[10] = '\0';
  ASSERT_STREQ("!!!!!!!!!!", buf);
}

TEST(memset, example_large) {
  char buf[101];
  ASSERT_EQ(buf, rs_memset(buf, '!', 100));
  buf[100] = '\0';
  ASSERT_THAT(buf,
              testing::ElementsAreArray("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
                                        "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
                                        "!!!!!!!!!!!!!!!!!!!!!!!!"));
}

TEST(memset, explicit) {
  char buf[32];
  rs_memset_explicit(buf, 'x', sizeof(buf));
  ASSERT_TRUE(rs_memcmp(buf, "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", sizeof(buf)) ==
              0);
}

TEST(stpcpy, example) {
  char buf[] = "AAAAAAAAAA";
  ASSERT_EQ(buf, rs_stpcpy(buf, ""));
  ASSERT_THAT(buf, testing::ElementsAreArray("\0AAAAAAAAA"));
  ASSERT_EQ(buf + 5, rs_stpcpy(buf, "Hello"));
  ASSERT_THAT(buf, testing::ElementsAreArray("Hello\0AAAA"));
  ASSERT_EQ(buf, rs_stpcpy(buf, ""));
  ASSERT_THAT(buf, testing::ElementsAreArray("\0ello\0AAAA"));
  ASSERT_EQ(buf + 9, rs_stpcpy(buf, "Example!!"));
  ASSERT_THAT(buf, testing::ElementsAreArray("Example!!\0"));
}

TEST(stpncpy, null) {
  ASSERT_EQ((char *)12, rs_stpncpy((char *)12, (char *)500, 0));
}

TEST(stpncpy, example1) {
  char buf[] = "AAAAAAAAAAAA";
  ASSERT_EQ(buf + 5, rs_stpncpy(buf, "Hello", 12));
  ASSERT_THAT(buf, testing::ElementsAreArray("Hello\0\0\0\0\0\0\0"));
}

TEST(stpncpy, example2) {
  char buf[] = "AAAAAAAAAAAA";
  ASSERT_EQ(buf + 12, rs_stpncpy(buf, "This is a very long string", 12));
  ASSERT_THAT(buf, testing::ElementsAreArray("This is a ve"));
}

TEST(strcat, example) {
  char buf[] = "\0AAAAAAAAA";
  ASSERT_EQ(buf, rs_strcat(buf, ""));
  ASSERT_THAT(buf, testing::ElementsAreArray("\0AAAAAAAAA"));
  ASSERT_EQ(buf, rs_strcat(buf, "Hello"));
  ASSERT_THAT(buf, testing::ElementsAreArray("Hello\0AAAA"));
  ASSERT_EQ(buf, rs_strcat(buf, ""));
  ASSERT_THAT(buf, testing::ElementsAreArray("Hello\0AAAA"));
  ASSERT_EQ(buf, rs_strcat(buf, "!!!!"));
  ASSERT_THAT(buf, testing::ElementsAreArray("Hello!!!!\0"));
}

TEST(strchr, examples) {
  const char *str = "Hello, world";
  ASSERT_EQ(NULL, rs_strchr(str, 'A'));
  ASSERT_EQ(str + 4, rs_strchr(str, 'o'));
  ASSERT_EQ(str + 12, rs_strchr(str, '\0'));
}

TEST(strcmp, examples) {
  ASSERT_EQ(0, rs_strcmp("", ""));
  ASSERT_EQ(0, rs_strcmp("Hello", "Hello"));

  ASSERT_GT(0, rs_strcmp("Hello", "Hello, world"));
  ASSERT_LT(0, rs_strcmp("Hello, world", "Hello"));

  ASSERT_GT(0, rs_strcmp("Hello!", "Hello."));
  ASSERT_LT(0, rs_strcmp("Hello.", "Hello!"));
}

struct coll_data {
  const char *a;
  const char *b;
  int result;
};

static int sign(int a) {
  if (a < 0)
    return -1;
  if (a > 0)
    return 1;
  return 0;
}

static void test_strcoll(const coll_data *coll) {
  for (unsigned int i = 0; coll[i].a != NULL; ++i) {
    int result = sign(rs_strcoll(coll[i].a, coll[i].b));
    ASSERT_EQ(result, coll[i].result);
  }
}

static void test_strxfrm(const coll_data *coll) {
  for (unsigned int i = 0; coll[i].a != NULL; ++i) {
    int result = 0;
    char sortKeyA[100], sortKeyB[100];
    rs_strxfrm(sortKeyA, coll[i].a, 100);
    rs_strxfrm(sortKeyB, coll[i].b, 100);
    result = sign(rs_strcmp(sortKeyA, sortKeyB));
    ASSERT_EQ(result, coll[i].result);
  }
}

TEST(strcoll, posix) {
  ASSERT_STREQ("C", rs_setlocale(RS_LC_COLLATE, "C"));

  const coll_data coll[] = {
      {"", "", 0},         {"test", "test", 0}, {"tester", "tester", 0},
      {"côté", "côté", 0}, {NULL, NULL, 0},
  };

  test_strcoll(coll);
  test_strxfrm(coll);
}

TEST(strcoll, uca) {
  ASSERT_STREQ("en_US", rs_setlocale(RS_LC_COLLATE, "en_US"));

  const coll_data coll[] = {
      {"", "", 0},           {"test", "test", 0},    {"tester", "test", 1},
      {"tEst", "test", 1},   {"test", "tester", -1}, {"täst", "täst", 0},
      {"tast", "täst", -1},  {"tbst", "täst", 1},    {"tbst", "tæst", 1},
      {"täst", "tÄst", -1},  {"tBst", "tÄst", 1},    {"tBst", "täst", 1},
      {"taest", "tæst", -1}, {"tafst", "tæst", 1},   {"taa", "täa", -1},
      {"tab", "täb", -1},    {"tad", "täd", -1},     {"tae", "täe", -1},
      {"taf", "täf", -1},    {"cote", "coté", -1},   {"coté", "côte", -1},
      {"côte", "côté", -1},  {NULL, NULL, 0},
  };

  test_strcoll(coll);
  test_strxfrm(coll);
}

TEST(strcpy, example) {
  char buf[] = "AAAAAAAAAA";
  ASSERT_EQ(buf, rs_strcpy(buf, ""));
  ASSERT_THAT(buf, testing::ElementsAreArray("\0AAAAAAAAA"));
  ASSERT_EQ(buf, rs_strcpy(buf, "Hello"));
  ASSERT_THAT(buf, testing::ElementsAreArray("Hello\0AAAA"));
  ASSERT_EQ(buf, rs_strcpy(buf, ""));
  ASSERT_THAT(buf, testing::ElementsAreArray("\0ello\0AAAA"));
  ASSERT_EQ(buf, rs_strcpy(buf, "Example!!"));
  ASSERT_THAT(buf, testing::ElementsAreArray("Example!!\0"));
}

TEST(strcspn, example) {
  const char *str = "Hello, world";
  ASSERT_EQ(0, rs_strcspn(str, "H"));
  ASSERT_EQ(7, rs_strcspn(str, "rdw"));
  ASSERT_EQ(12, rs_strcspn(str, "XYZ"));
}

TEST(strlen, all) {
  ASSERT_EQ(0, rs_strlen(""));
  ASSERT_EQ(12, rs_strlen("Hello, world"));
}

TEST(strncat, example) {
  char buf[] = "\0AAAAAAAAA";
  ASSERT_EQ(buf, rs_strncat(buf, "", 0));
  ASSERT_THAT(buf, testing::ElementsAreArray("\0AAAAAAAAA"));
  ASSERT_EQ(buf, rs_strncat(buf, "Hello", 99999));
  ASSERT_THAT(buf, testing::ElementsAreArray("Hello\0AAAA"));
  ASSERT_EQ(buf, rs_strncat(buf, "", 1));
  ASSERT_THAT(buf, testing::ElementsAreArray("Hello\0AAAA"));
  ASSERT_EQ(buf, rs_strncat(buf, "!!!!!!!!!!!!", 3));
  ASSERT_THAT(buf, testing::ElementsAreArray("Hello!!!\0A"));
}

TEST(strncmp, null) { ASSERT_EQ(0, rs_strncmp(NULL, NULL, 0)); }

TEST(strncmp, examples) {
  ASSERT_EQ(0, rs_strncmp("", "", 100));
  ASSERT_EQ(0, rs_strncmp("Hello", "Hello", 100));

  ASSERT_EQ(0, rs_strncmp("Hello", "Hello, world", 5));
  ASSERT_GT(0, rs_strncmp("Hello", "Hello, world", 6));
  ASSERT_LT(0, rs_strncmp("Hello, world", "Hello", 100));

  ASSERT_EQ(0, rs_strncmp("Hello!", "Hello.", 5));
  ASSERT_GT(0, rs_strncmp("Hello!", "Hello.", 6));
  ASSERT_LT(0, rs_strncmp("Hello.", "Hello!", 100));
}

TEST(strncpy, null) {
  ASSERT_EQ((char *)12, rs_strncpy((char *)12, (char *)500, 0));
}

TEST(strncpy, example1) {
  char buf[] = "AAAAAAAAAAAA";
  ASSERT_EQ(buf, rs_strncpy(buf, "Hello", 12));
  ASSERT_THAT(buf, testing::ElementsAreArray("Hello\0\0\0\0\0\0\0"));
}

TEST(strncpy, example2) {
  char buf[13];
  ASSERT_EQ(buf, rs_strncpy(buf, "This is a very long string", 12));
  buf[12] = '\0';
  ASSERT_THAT(buf, testing::ElementsAreArray("This is a ve"));
}

TEST(strnlen, null) {
  ASSERT_EQ(0, rs_strnlen(NULL, 0));
  ASSERT_EQ(0, rs_strnlen("", 100));
  ASSERT_EQ(7, rs_strnlen("Hello, world", 7));
}

TEST(strpbrk, example) {
  const char *str = "Hello, world";
  ASSERT_EQ(str, rs_strpbrk(str, "H"));
  ASSERT_EQ(str + 7, rs_strpbrk(str, "rdw"));
  ASSERT_EQ(NULL, rs_strpbrk(str, "XYZ"));
}

TEST(strrchr, examples) {
  const char *str = "Hello, world";
  ASSERT_EQ(NULL, rs_strrchr(str, 'A'));
  ASSERT_EQ(str + 8, rs_strrchr(str, 'o'));
  ASSERT_EQ(str + 12, rs_strrchr(str, '\0'));
}

TEST(strspn, example) {
  const char *str = "Hello, world";
  ASSERT_EQ(0, rs_strspn(str, ""));
  ASSERT_EQ(0, rs_strspn(str, "Foo"));
  ASSERT_EQ(5, rs_strspn(str, "olHe"));
  ASSERT_EQ(12, rs_strspn(str, "Helo, wrld"));
}

TEST(strstr, examples) {
  const char *str = (const char *)0x42;
  ASSERT_EQ(str, rs_strstr(str, ""));

  str = "Hello world";
  ASSERT_EQ(str + 2, rs_strstr(str, "ll"));
  ASSERT_EQ(str + 4, rs_strstr(str, "o worl"));
  ASSERT_EQ(str + 6, rs_strstr(str, "world"));
  ASSERT_EQ(str + 10, rs_strstr(str, "d"));
  ASSERT_EQ(NULL, rs_strstr(str, "word"));
  ASSERT_EQ(NULL, rs_strstr(str, "world!"));
}

TEST(strtok, example) {
  char line[] = "LINE  TO BE\t\tSEPARATED\n";
  const char *split = " \t\n";
  char *lasts;
  ASSERT_STREQ("LINE", rs_strtok(line, split));
  ASSERT_STREQ("TO", rs_strtok(NULL, split));
  ASSERT_STREQ("BE", rs_strtok(NULL, split));
  ASSERT_STREQ("SEPARATED", rs_strtok(NULL, split));
  ASSERT_EQ(NULL, rs_strtok(NULL, split));
}

TEST(strtok_r, example) {
  char line[] = "LINE  TO BE\t\tSEPARATED\n";
  const char *split = " \t\n";
  char *lasts;
  ASSERT_STREQ("LINE", rs_strtok_r(line, split, &lasts));
  ASSERT_STREQ("TO", rs_strtok_r(NULL, split, &lasts));
  ASSERT_STREQ("BE", rs_strtok_r(NULL, split, &lasts));
  ASSERT_STREQ("SEPARATED", rs_strtok_r(NULL, split, &lasts));
  ASSERT_EQ(NULL, rs_strtok_r(NULL, split, &lasts));
}

TEST(strlcat, null) { ASSERT_EQ(5, rs_strlcat(NULL, "Hello", 0)); }

TEST(strlcat, one) {
  char buf = '\0';
  ASSERT_EQ(6, rs_strlcat(&buf, "Banana", 1));
  ASSERT_EQ('\0', buf);

  buf = 'A';
  ASSERT_EQ(7, rs_strlcat(&buf, "Banana", 1));
  ASSERT_EQ('A', buf);
}

TEST(strlcat, longer) {
  char buf[] = "AAAAAAAAAAAA";
  ASSERT_EQ(15, rs_strlcat(buf, "Foo", sizeof(buf) - 1));
  ASSERT_THAT(buf, testing::ElementsAreArray("AAAAAAAAAAAA"));

  buf[4] = '\0';
  ASSERT_EQ(7, rs_strlcat(buf, "Bar", sizeof(buf) - 1));
  ASSERT_THAT(buf, testing::ElementsAreArray("AAAABar\0AAAA"));

  ASSERT_EQ(16, rs_strlcat(buf, "Very long", sizeof(buf) - 1));
  ASSERT_THAT(buf, testing::ElementsAreArray("AAAABarVery\0"));
}

TEST(strlcpy, null) { ASSERT_EQ(5, rs_strlcpy(NULL, "Hello", 0)); }

TEST(strlcpy, one) {
  char buf;
  ASSERT_EQ(6, rs_strlcpy(&buf, "Banana", 1));
  ASSERT_EQ('\0', buf);
}

TEST(strlcpy, longer) {
  char buf[] = "AAAAAAAAAA";
  ASSERT_EQ(3, rs_strlcpy(buf, "Dog", sizeof(buf)));
  ASSERT_THAT(buf, testing::ElementsAreArray("Dog\0AAAAAA"));
}

TEST(strlcpy, longest) {
  char buf[12];
  ASSERT_EQ(23, rs_strlcpy(buf, "This is a long sentence", sizeof(buf)));
  ASSERT_STREQ("This is a l", buf);
}

TEST(strerror, example) {
  rs_setlocale(RS_LC_MESSAGES, "POSIX");
  ASSERT_STREQ(rs_strerror(0), "Success");

  const char *message_array[] = {
      "Success",
      "Operation not permitted",
      "No such file or directory",
      "No such process",
      "Interrupted system call",
      "Input/output error",
      "No such device or address",
      "Argument list too long",
      "Exec format error",
      "Bad file descriptor",
      "No child processes",
      "Resource temporarily unavailable",
      "Cannot allocate memory",
      "Permission denied",
      "Bad address",
      "Block device required",
      "Device or resource busy",
      "File exists",
      "Invalid cross-device link",
      "No such device",
      "Not a directory",
      "Is a directory",
      "Invalid argument",
      "Too many open files in system",
      "Too many open files",
      "Inappropriate ioctl for device",
      "Text file busy",
      "File too large",
      "No space left on device",
      "Illegal seek",
      "Read-only file system",
      "Too many links",
      "Broken pipe",
      "Numerical argument out of domain",
      "Numerical result out of range",
      "Resource deadlock avoided",
      "File name too long",
      "No locks available",
      "Function not implemented",
      "Directory not empty",
      "Too many levels of symbolic links",
      "Unknown error 41", // Unknown
      "No message of desired type",
      "Identifier removed",
      "Channel number out of range",
      "Level 2 not synchronized",
      "Level 3 halted",
      "Level 3 reset",
      "Link number out of range",
      "Protocol driver not attached",
      "No CSI structure available",
      "Level 2 halted",
      "Invalid exchange",
      "Invalid request descriptor",
      "Exchange full",
      "No anode",
      "Invalid request code",
      "Invalid slot",
      "Unknown error 58", // Unknown
      "Bad font file format",
      "Device not a stream",
      "No data available",
      "Timer expired",
      "Out of streams resources",
      "Machine is not on the network",
      "Package not installed",
      "Object is remote",
      "Link has been severed",
      "Advertise error",
      "Srmount error",
      "Communication error on send",
      "Protocol error",
      "Multihop attempted",
      "RFS specific error",
      "Bad message",
      "Value too large for defined data type",
      "Name not unique on network",
      "File descriptor in bad state",
      "Remote address changed",
      "Can not access a needed shared library",
      "Accessing a corrupted shared library",
      ".lib section in a.out corrupted",
      "Attempting to link in too many shared libraries",
      "Cannot exec a shared library directly",
      "Invalid or incomplete multibyte or wide character",
      "Interrupted system call should be restarted",
      "Streams pipe error",
      "Too many users",
      "Socket operation on non-socket",
      "Destination address required",
      "Message too long",
      "Protocol wrong type for socket",
      "Protocol not available",
      "Protocol not supported",
      "Socket type not supported",
      "Operation not supported",
      "Protocol family not supported",
      "Address family not supported by protocol",
      "Address already in use",
      "Cannot assign requested address",
      "Network is down",
      "Network is unreachable",
      "Network dropped connection on reset",
      "Software caused connection abort",
      "Connection reset by peer",
      "No buffer space available",
      "Transport endpoint is already connected",
      "Transport endpoint is not connected",
      "Cannot send after transport endpoint shutdown",
      "Too many references: cannot splice",
      "Connection timed out",
      "Connection refused",
      "Host is down",
      "No route to host",
      "Operation already in progress",
      "Operation now in progress",
      "Stale file handle",
      "Structure needs cleaning",
      "Not a XENIX named type file",
      "No XENIX semaphores available",
      "Is a named type file",
      "Remote I/O error",
      "Disk quota exceeded",
      "No medium found",
      "Wrong medium type",
      "Operation canceled",
      "Required key not available",
      "Key has expired",
      "Key has been revoked",
      "Key was rejected by service",
      "Owner died",
      "State not recoverable",
      "Operation not possible due to RF-kill",
      "Memory page has hardware error",
  };

  for (size_t i = 0; i < (sizeof(message_array) / sizeof(char *)); ++i) {
    EXPECT_STREQ(rs_strerror(static_cast<int>(i)), message_array[i]);
  }

  ASSERT_STREQ(rs_strerror(-1), "Unknown error -1");
  ASSERT_STREQ(rs_strerror(134), "Unknown error 134");
  ASSERT_STREQ(rs_strerror(2147483647), "Unknown error 2147483647");
  ASSERT_STREQ(rs_strerror(-2147483648), "Unknown error -2147483648");
}

TEST(strerror, korean) {
  rs_errno = 0;

  ouma_locale_t loc = rs_newlocale(RS_LC_MESSAGES_MASK, "ko_KR.UTF-8", 0);
  ASSERT_NE(nullptr, loc);
  ASSERT_NE(ENOENT, rs_errno);
  ASSERT_STREQ("ko_KR.UTF-8", rs_getlocalename_l(RS_LC_MESSAGES, loc));

  ASSERT_STREQ(rs_strerror_l(0, loc), "성공");

  rs_freelocale(loc);
}

TEST(strerror_r, posix) {
  rs_setlocale(RS_LC_MESSAGES, "POSIX");

  char buf[256];
  rs_memset(buf, '\0', sizeof(buf));
  int ret = rs___xpg_strerror_r(-1, buf, sizeof(buf));
  ASSERT_STREQ(buf, "Unknown error -1");
  ASSERT_EQ(ret, EINVAL);

  char mini_buf[5];
  rs_memset(mini_buf, '\0', sizeof(mini_buf));
  int ret2 = rs___xpg_strerror_r(ERANGE, mini_buf, sizeof(mini_buf));
  ASSERT_STREQ(mini_buf, "");
  ASSERT_EQ(ret2, ERANGE);

  char good_buf[512];
  rs_memset(good_buf, '\0', sizeof(good_buf));
  int ret3 = rs___xpg_strerror_r(EACCES, good_buf, sizeof(good_buf));
  ASSERT_STREQ(good_buf, "Permission denied");
  ASSERT_EQ(ret3, 0);
}

TEST(strerror_r, gnu) {
  rs_setlocale(RS_LC_MESSAGES, "POSIX");

  const size_t BUFF_SIZE = 128;
  char buffer[BUFF_SIZE];
  buffer[0] = '\0';
  ASSERT_STREQ(rs_strerror_r(0, buffer, BUFF_SIZE), "Success");
  ASSERT_NE(buffer[0], '\0');

  ASSERT_STREQ(rs_strerror_r(-1, buffer, BUFF_SIZE), "Unknown error -1");
  ASSERT_STREQ(buffer, "Unknown error -1");
}

TEST(strerrorname_np, example) {
  ASSERT_STREQ(rs_strerrorname_np(0), "0");
  ASSERT_STREQ(rs_strerrorname_np(EPERM), "EPERM");
  ASSERT_STREQ(rs_strerrorname_np(ENOENT), "ENOENT");
  ASSERT_STREQ(rs_strerrorname_np(ESRCH), "ESRCH");
  ASSERT_STREQ(rs_strerrorname_np(EINTR), "EINTR");
  ASSERT_STREQ(rs_strerrorname_np(EIO), "EIO");
  ASSERT_STREQ(rs_strerrorname_np(ENXIO), "ENXIO");
  ASSERT_STREQ(rs_strerrorname_np(E2BIG), "E2BIG");
  ASSERT_STREQ(rs_strerrorname_np(ENOEXEC), "ENOEXEC");
  ASSERT_STREQ(rs_strerrorname_np(EBADF), "EBADF");
  ASSERT_STREQ(rs_strerrorname_np(ECHILD), "ECHILD");
  ASSERT_STREQ(rs_strerrorname_np(EDEADLK), "EDEADLK");
  ASSERT_STREQ(rs_strerrorname_np(ENOMEM), "ENOMEM");
  ASSERT_STREQ(rs_strerrorname_np(EACCES), "EACCES");
  ASSERT_STREQ(rs_strerrorname_np(EFAULT), "EFAULT");
  ASSERT_STREQ(rs_strerrorname_np(ENOTBLK), "ENOTBLK");
  ASSERT_STREQ(rs_strerrorname_np(EBUSY), "EBUSY");
  ASSERT_STREQ(rs_strerrorname_np(EEXIST), "EEXIST");
  ASSERT_STREQ(rs_strerrorname_np(EXDEV), "EXDEV");
  ASSERT_STREQ(rs_strerrorname_np(ENODEV), "ENODEV");
  ASSERT_STREQ(rs_strerrorname_np(ENOTDIR), "ENOTDIR");
  ASSERT_STREQ(rs_strerrorname_np(EISDIR), "EISDIR");
  ASSERT_STREQ(rs_strerrorname_np(EINVAL), "EINVAL");
  ASSERT_STREQ(rs_strerrorname_np(EMFILE), "EMFILE");
  ASSERT_STREQ(rs_strerrorname_np(ENFILE), "ENFILE");
  ASSERT_STREQ(rs_strerrorname_np(ENOTTY), "ENOTTY");
  ASSERT_STREQ(rs_strerrorname_np(ETXTBSY), "ETXTBSY");
  ASSERT_STREQ(rs_strerrorname_np(EFBIG), "EFBIG");
  ASSERT_STREQ(rs_strerrorname_np(ENOSPC), "ENOSPC");
  ASSERT_STREQ(rs_strerrorname_np(ESPIPE), "ESPIPE");
  ASSERT_STREQ(rs_strerrorname_np(EROFS), "EROFS");
  ASSERT_STREQ(rs_strerrorname_np(EMLINK), "EMLINK");
  ASSERT_STREQ(rs_strerrorname_np(EPIPE), "EPIPE");
  ASSERT_STREQ(rs_strerrorname_np(EDOM), "EDOM");
  ASSERT_STREQ(rs_strerrorname_np(ERANGE), "ERANGE");
  ASSERT_STREQ(rs_strerrorname_np(EINPROGRESS), "EINPROGRESS");
  ASSERT_STREQ(rs_strerrorname_np(EALREADY), "EALREADY");
  ASSERT_STREQ(rs_strerrorname_np(ENOTSOCK), "ENOTSOCK");
  ASSERT_STREQ(rs_strerrorname_np(EMSGSIZE), "EMSGSIZE");
  ASSERT_STREQ(rs_strerrorname_np(EPROTOTYPE), "EPROTOTYPE");
  ASSERT_STREQ(rs_strerrorname_np(ENOPROTOOPT), "ENOPROTOOPT");
  ASSERT_STREQ(rs_strerrorname_np(EPROTONOSUPPORT), "EPROTONOSUPPORT");
  ASSERT_STREQ(rs_strerrorname_np(ESOCKTNOSUPPORT), "ESOCKTNOSUPPORT");
  ASSERT_STREQ(rs_strerrorname_np(EOPNOTSUPP), "EOPNOTSUPP");
  ASSERT_STREQ(rs_strerrorname_np(EPFNOSUPPORT), "EPFNOSUPPORT");
  ASSERT_STREQ(rs_strerrorname_np(EAFNOSUPPORT), "EAFNOSUPPORT");
  ASSERT_STREQ(rs_strerrorname_np(EADDRINUSE), "EADDRINUSE");
  ASSERT_STREQ(rs_strerrorname_np(EADDRNOTAVAIL), "EADDRNOTAVAIL");
  ASSERT_STREQ(rs_strerrorname_np(ENETDOWN), "ENETDOWN");
  ASSERT_STREQ(rs_strerrorname_np(ENETUNREACH), "ENETUNREACH");
  ASSERT_STREQ(rs_strerrorname_np(ENETRESET), "ENETRESET");
  ASSERT_STREQ(rs_strerrorname_np(ECONNABORTED), "ECONNABORTED");
  ASSERT_STREQ(rs_strerrorname_np(ECONNRESET), "ECONNRESET");
  ASSERT_STREQ(rs_strerrorname_np(ENOBUFS), "ENOBUFS");
  ASSERT_STREQ(rs_strerrorname_np(EISCONN), "EISCONN");
  ASSERT_STREQ(rs_strerrorname_np(ENOTCONN), "ENOTCONN");
  ASSERT_STREQ(rs_strerrorname_np(EDESTADDRREQ), "EDESTADDRREQ");
  ASSERT_STREQ(rs_strerrorname_np(ESHUTDOWN), "ESHUTDOWN");
  ASSERT_STREQ(rs_strerrorname_np(ETOOMANYREFS), "ETOOMANYREFS");
  ASSERT_STREQ(rs_strerrorname_np(ETIMEDOUT), "ETIMEDOUT");
  ASSERT_STREQ(rs_strerrorname_np(ECONNREFUSED), "ECONNREFUSED");
  ASSERT_STREQ(rs_strerrorname_np(ELOOP), "ELOOP");
  ASSERT_STREQ(rs_strerrorname_np(ENAMETOOLONG), "ENAMETOOLONG");
  ASSERT_STREQ(rs_strerrorname_np(EHOSTDOWN), "EHOSTDOWN");
  ASSERT_STREQ(rs_strerrorname_np(EHOSTUNREACH), "EHOSTUNREACH");
  ASSERT_STREQ(rs_strerrorname_np(ENOTEMPTY), "ENOTEMPTY");
  ASSERT_STREQ(rs_strerrorname_np(EUSERS), "EUSERS");
  ASSERT_STREQ(rs_strerrorname_np(EDQUOT), "EDQUOT");
  ASSERT_STREQ(rs_strerrorname_np(ESTALE), "ESTALE");
  ASSERT_STREQ(rs_strerrorname_np(EREMOTE), "EREMOTE");
  ASSERT_STREQ(rs_strerrorname_np(ENOLCK), "ENOLCK");
  ASSERT_STREQ(rs_strerrorname_np(ENOSYS), "ENOSYS");
  ASSERT_STREQ(rs_strerrorname_np(EILSEQ), "EILSEQ");
  ASSERT_STREQ(rs_strerrorname_np(EBADMSG), "EBADMSG");
  ASSERT_STREQ(rs_strerrorname_np(EIDRM), "EIDRM");
  ASSERT_STREQ(rs_strerrorname_np(EMULTIHOP), "EMULTIHOP");
  ASSERT_STREQ(rs_strerrorname_np(ENODATA), "ENODATA");
  ASSERT_STREQ(rs_strerrorname_np(ENOLINK), "ENOLINK");
  ASSERT_STREQ(rs_strerrorname_np(ENOMSG), "ENOMSG");
  ASSERT_STREQ(rs_strerrorname_np(ENOSR), "ENOSR");
  ASSERT_STREQ(rs_strerrorname_np(ENOSTR), "ENOSTR");
  ASSERT_STREQ(rs_strerrorname_np(EOVERFLOW), "EOVERFLOW");
  ASSERT_STREQ(rs_strerrorname_np(EPROTO), "EPROTO");
  ASSERT_STREQ(rs_strerrorname_np(ETIME), "ETIME");
  ASSERT_STREQ(rs_strerrorname_np(ECANCELED), "ECANCELED");
  ASSERT_STREQ(rs_strerrorname_np(EOWNERDEAD), "EOWNERDEAD");
  ASSERT_STREQ(rs_strerrorname_np(ENOTRECOVERABLE), "ENOTRECOVERABLE");
  ASSERT_STREQ(rs_strerrorname_np(ERESTART), "ERESTART");
  ASSERT_STREQ(rs_strerrorname_np(ECHRNG), "ECHRNG");
  ASSERT_STREQ(rs_strerrorname_np(EL2NSYNC), "EL2NSYNC");
  ASSERT_STREQ(rs_strerrorname_np(EL3HLT), "EL3HLT");
  ASSERT_STREQ(rs_strerrorname_np(EL3RST), "EL3RST");
  ASSERT_STREQ(rs_strerrorname_np(ELNRNG), "ELNRNG");
  ASSERT_STREQ(rs_strerrorname_np(EUNATCH), "EUNATCH");
  ASSERT_STREQ(rs_strerrorname_np(ENOCSI), "ENOCSI");
  ASSERT_STREQ(rs_strerrorname_np(EL2HLT), "EL2HLT");
  ASSERT_STREQ(rs_strerrorname_np(EBADE), "EBADE");
  ASSERT_STREQ(rs_strerrorname_np(EBADR), "EBADR");
  ASSERT_STREQ(rs_strerrorname_np(EXFULL), "EXFULL");
  ASSERT_STREQ(rs_strerrorname_np(ENOANO), "ENOANO");
  ASSERT_STREQ(rs_strerrorname_np(EBADRQC), "EBADRQC");
  ASSERT_STREQ(rs_strerrorname_np(EBADSLT), "EBADSLT");
  ASSERT_STREQ(rs_strerrorname_np(EBFONT), "EBFONT");
  ASSERT_STREQ(rs_strerrorname_np(ENONET), "ENONET");
  ASSERT_STREQ(rs_strerrorname_np(ENOPKG), "ENOPKG");
  ASSERT_STREQ(rs_strerrorname_np(EADV), "EADV");
  ASSERT_STREQ(rs_strerrorname_np(ESRMNT), "ESRMNT");
  ASSERT_STREQ(rs_strerrorname_np(ECOMM), "ECOMM");
  ASSERT_STREQ(rs_strerrorname_np(EDOTDOT), "EDOTDOT");
  ASSERT_STREQ(rs_strerrorname_np(ENOTUNIQ), "ENOTUNIQ");
  ASSERT_STREQ(rs_strerrorname_np(EBADFD), "EBADFD");
  ASSERT_STREQ(rs_strerrorname_np(EREMCHG), "EREMCHG");
  ASSERT_STREQ(rs_strerrorname_np(ELIBACC), "ELIBACC");
  ASSERT_STREQ(rs_strerrorname_np(ELIBBAD), "ELIBBAD");
  ASSERT_STREQ(rs_strerrorname_np(ELIBSCN), "ELIBSCN");
  ASSERT_STREQ(rs_strerrorname_np(ELIBMAX), "ELIBMAX");
  ASSERT_STREQ(rs_strerrorname_np(ELIBEXEC), "ELIBEXEC");
  ASSERT_STREQ(rs_strerrorname_np(ESTRPIPE), "ESTRPIPE");
  ASSERT_STREQ(rs_strerrorname_np(EUCLEAN), "EUCLEAN");
  ASSERT_STREQ(rs_strerrorname_np(ENOTNAM), "ENOTNAM");
  ASSERT_STREQ(rs_strerrorname_np(ENAVAIL), "ENAVAIL");
  ASSERT_STREQ(rs_strerrorname_np(EISNAM), "EISNAM");
  ASSERT_STREQ(rs_strerrorname_np(EREMOTEIO), "EREMOTEIO");
  ASSERT_STREQ(rs_strerrorname_np(ENOMEDIUM), "ENOMEDIUM");
  ASSERT_STREQ(rs_strerrorname_np(EMEDIUMTYPE), "EMEDIUMTYPE");
  ASSERT_STREQ(rs_strerrorname_np(ENOKEY), "ENOKEY");
  ASSERT_STREQ(rs_strerrorname_np(EKEYEXPIRED), "EKEYEXPIRED");
  ASSERT_STREQ(rs_strerrorname_np(EKEYREVOKED), "EKEYREVOKED");
  ASSERT_STREQ(rs_strerrorname_np(EKEYREJECTED), "EKEYREJECTED");
  ASSERT_STREQ(rs_strerrorname_np(ERFKILL), "ERFKILL");
  ASSERT_STREQ(rs_strerrorname_np(EHWPOISON), "EHWPOISON");
}

TEST(strerrordesc_np, example) {
  const char *message_array[] = {
      "Success",
      "Operation not permitted",
      "No such file or directory",
      "No such process",
      "Interrupted system call",
      "Input/output error",
      "No such device or address",
      "Argument list too long",
      "Exec format error",
      "Bad file descriptor",
      "No child processes",
      "Resource temporarily unavailable",
      "Cannot allocate memory",
      "Permission denied",
      "Bad address",
      "Block device required",
      "Device or resource busy",
      "File exists",
      "Invalid cross-device link",
      "No such device",
      "Not a directory",
      "Is a directory",
      "Invalid argument",
      "Too many open files in system",
      "Too many open files",
      "Inappropriate ioctl for device",
      "Text file busy",
      "File too large",
      "No space left on device",
      "Illegal seek",
      "Read-only file system",
      "Too many links",
      "Broken pipe",
      "Numerical argument out of domain",
      "Numerical result out of range",
      "Resource deadlock avoided",
      "File name too long",
      "No locks available",
      "Function not implemented",
      "Directory not empty",
      "Too many levels of symbolic links",
      "Unknown error 41", // Unknown
      "No message of desired type",
      "Identifier removed",
      "Channel number out of range",
      "Level 2 not synchronized",
      "Level 3 halted",
      "Level 3 reset",
      "Link number out of range",
      "Protocol driver not attached",
      "No CSI structure available",
      "Level 2 halted",
      "Invalid exchange",
      "Invalid request descriptor",
      "Exchange full",
      "No anode",
      "Invalid request code",
      "Invalid slot",
      "Unknown error 58", // Unknown
      "Bad font file format",
      "Device not a stream",
      "No data available",
      "Timer expired",
      "Out of streams resources",
      "Machine is not on the network",
      "Package not installed",
      "Object is remote",
      "Link has been severed",
      "Advertise error",
      "Srmount error",
      "Communication error on send",
      "Protocol error",
      "Multihop attempted",
      "RFS specific error",
      "Bad message",
      "Value too large for defined data type",
      "Name not unique on network",
      "File descriptor in bad state",
      "Remote address changed",
      "Can not access a needed shared library",
      "Accessing a corrupted shared library",
      ".lib section in a.out corrupted",
      "Attempting to link in too many shared libraries",
      "Cannot exec a shared library directly",
      "Invalid or incomplete multibyte or wide character",
      "Interrupted system call should be restarted",
      "Streams pipe error",
      "Too many users",
      "Socket operation on non-socket",
      "Destination address required",
      "Message too long",
      "Protocol wrong type for socket",
      "Protocol not available",
      "Protocol not supported",
      "Socket type not supported",
      "Operation not supported",
      "Protocol family not supported",
      "Address family not supported by protocol",
      "Address already in use",
      "Cannot assign requested address",
      "Network is down",
      "Network is unreachable",
      "Network dropped connection on reset",
      "Software caused connection abort",
      "Connection reset by peer",
      "No buffer space available",
      "Transport endpoint is already connected",
      "Transport endpoint is not connected",
      "Cannot send after transport endpoint shutdown",
      "Too many references: cannot splice",
      "Connection timed out",
      "Connection refused",
      "Host is down",
      "No route to host",
      "Operation already in progress",
      "Operation now in progress",
      "Stale file handle",
      "Structure needs cleaning",
      "Not a XENIX named type file",
      "No XENIX semaphores available",
      "Is a named type file",
      "Remote I/O error",
      "Disk quota exceeded",
      "No medium found",
      "Wrong medium type",
      "Operation canceled",
      "Required key not available",
      "Key has expired",
      "Key has been revoked",
      "Key was rejected by service",
      "Owner died",
      "State not recoverable",
      "Operation not possible due to RF-kill",
      "Memory page has hardware error",
  };

  for (size_t i = 0; i < (sizeof(message_array) / sizeof(char *)); ++i) {
    EXPECT_STREQ(rs_strerrordesc_np(static_cast<int>(i)), message_array[i]);
  }
}
