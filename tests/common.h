#include <gmock/gmock.h>

extern "C" {
    int *__rs_errno_location(void);
}

#define rs_errno (*__rs_errno_location())
