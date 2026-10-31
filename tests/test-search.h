/*
 * Copyright (c) 2026 Faraz Vahedi <kfv@FreeBSD.org>
 *
 * SPDX-License-Identifier: BSD-2-Clause
 */

#ifndef _TEST_SEARCH_H
#define _TEST_SEARCH_H

#include <sys/param.h>

#include <stddef.h>

#include <gtest/gtest.h>

#define SVEC_LEN 1024

typedef const void*
search_int_t(const int*, const int*, size_t, void*);

static int
searchhelp(const void* a, const void* b)
{
  const int *oa = static_cast<const int*>(a), *ob = static_cast<const int*>(b);

  return ((*oa > *ob) - (*oa < *ob));
}

/*
 * Fill v[i] = i, then confirm every element is found and -1 and n are not.
 */
static void
check_sorted_search(search_int_t* search, void* ctx, int* v, size_t n)
{
  size_t i;
  int key;

  for (i = 0; i < n; i++)
    v[i] = (int)i;
  for (i = 0; i < n; i++) {
    key = v[i];
    EXPECT_TRUE(search(&key, v, n, ctx) == &v[i]);
  }
  if (n != 0) {
    key = -1;
    EXPECT_TRUE(search(&key, v, n, ctx) == nullptr);
    key = (int)n;
    EXPECT_TRUE(search(&key, v, n, ctx) == nullptr);
  }
}

#endif /* !_TEST_SEARCH_H */
