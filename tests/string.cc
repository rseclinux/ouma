#include "common.h"

extern "C" {
void *rs_memccpy(void *__restrict, const void *__restrict, int, size_t);
void *rs_memchr(const void *, int, size_t);
int rs_memcmp(const void *, const void *, size_t);
void *rs_memcpy(void *__restrict, const void *__restrict, size_t);
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
int rs_posix_strerror_r(int, char *, size_t);
char *rs_gnu_strerror_r(int, char *, size_t);
char *rs_strerror(int);
char *rs_strsignal(int);
char *rs_strndup(const char *, size_t);
char *rs_strdup(const char *);

char *rs_setlocale(int, const char *);
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

/*
TEST(strerror, example) {
  rs_setlocale(LC_MESSAGES, "POSIX");
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
*/
