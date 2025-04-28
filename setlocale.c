#define LC_CTYPE 0
#define LC_NUMERIC 1
#define LC_TIME 2
#define LC_COLLATE 3
#define LC_MONETARY 4
#define LC_MESSAGES 5
#define LC_ALL 6

#include <stdio.h>
#include <string.h>
#include <stddef.h>

char *rs_setlocale(int, const char *);

int main() {
  printf("%s\n", rs_setlocale(LC_CTYPE, "C"));
  printf("%s\n", rs_setlocale(LC_CTYPE, "POSIX"));
  printf("%s\n", rs_setlocale(LC_ALL, "C.UTF-8"));
  printf("%s\n", rs_setlocale(LC_CTYPE, "C.utf8"));
  printf("%s\n", rs_setlocale(LC_COLLATE, "en_US.UTF-8"));
  printf("%s\n", rs_setlocale(LC_ALL, "en_US.Us-aScIi"));
  printf("%s\n", rs_setlocale(LC_CTYPE, ".UTF8"));
  printf("%s\n", rs_setlocale(LC_COLLATE, "cz_CZ"));
  printf("%s\n", rs_setlocale(LC_COLLATE, "C"));
  printf("%s\n", rs_setlocale(LC_COLLATE, "POSIX"));

  return 0;
}
