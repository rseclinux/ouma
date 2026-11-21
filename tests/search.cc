#include "common.h"
#include <gtest/gtest.h>

typedef struct rs_posix_tnode
{
  void* key;
  struct rs_posix_tnode* left;
  struct rs_posix_tnode* right;
} rs_posix_tnode;

typedef enum RS_VISIT
{
  endorder,
  leaf,
  postorder,
  preorder,
} RS_VISIT;

extern "C"
{
  void* rs_lsearch(const void* key,
                   void* base,
                   size_t* nelp,
                   size_t width,
                   int (*compar)(const void*, const void*));
  void* rs_lfind(const void* key,
                 const void* base,
                 size_t* nelp,
                 size_t width,
                 int (*compar)(const void*, const void*));
  void rs_insque(void* element, void* pred);
  void rs_remque(void* element);
  void* rs_tdelete(const void* key,
                   rs_posix_tnode** rootp,
                   int (*compar)(const void*, const void*));
  void rs_tdestroy(rs_posix_tnode* root, void (*free_node)(void*));
  rs_posix_tnode* rs_tfind(const void* key,
                           rs_posix_tnode* const* rootp,
                           int (*compar)(const void*, const void*));
  rs_posix_tnode* rs_tsearch(const void* key,
                             rs_posix_tnode** rootp,
                             int (*compar)(const void*, const void*));
  void rs_twalk(const rs_posix_tnode* rootp,
                void (*action)(const rs_posix_tnode*, RS_VISIT, int));
  void rs_twalk_r(const rs_posix_tnode* rootp,
                  void (*action)(const rs_posix_tnode*, RS_VISIT, void*),
                  void* closure);
}

TEST(lfind, zero)
{
  // Empty list should not be accessed.
  size_t nel = 0;
  ASSERT_EQ(NULL,
            rs_lfind((const void*)1,
                     (const void*)2,
                     &nel,
                     3,
                     (int (*)(const void*, const void*))4));
  ASSERT_EQ(0, nel);
}

static int
compar_l1(const void* a, const void* b)
{
  return *(int*)b - *(int*)a;
}

TEST(lfind, example)
{
  int v[10] = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 };
  size_t nel = std::size(v);

  for (int i = -nel; i < 2 * (int)nel; ++i) {
    if (i < 0 || i >= (int)nel)
      ASSERT_EQ(NULL, rs_lfind(&i, v, &nel, sizeof(i), compar_l1));
    else
      ASSERT_EQ(&v[i], rs_lfind(&i, v, &nel, sizeof(i), compar_l1));
    ASSERT_EQ(std::size(v), nel);
  }
}

static int
compar_l2(const void* a, const void* b)
{
  return *(int*)b - *(int*)a;
}

TEST(lsearch, example)
{
  int v[10];
  for (int i = 0; i < (int)std::size(v); ++i) {
    size_t nel = i;
    ASSERT_EQ(NULL, rs_lfind(&i, v, &nel, sizeof(int), compar_l2));
    ASSERT_EQ(i, nel);

    ASSERT_EQ(&v[i], rs_lsearch(&i, v, &nel, sizeof(int), compar_l2));
    ASSERT_EQ(i + 1, nel);

    for (int j = 0; j <= i; ++j) {
      ASSERT_EQ(&v[j], rs_lsearch(&j, v, &nel, sizeof(int), compar_l2));
      ASSERT_EQ(i + 1, nel);
    }
  }
}

struct que
{
  struct que* succ;
  struct que* pred;
};

TEST(insque, null)
{
  struct que element;
  rs_insque(&element, NULL);
  ASSERT_EQ(NULL, element.succ);
  ASSERT_EQ(NULL, element.pred);
}

TEST(insque, tail)
{
  struct que element;
  struct que pred = { .succ = NULL, .pred = (struct que*)42 };
  rs_insque(&element, &pred);
  ASSERT_EQ(NULL, element.succ);
  ASSERT_EQ(&pred, element.pred);
  ASSERT_EQ(&element, pred.succ);
  ASSERT_EQ((struct que*)42, pred.pred);
}

TEST(insque, middle)
{
  struct que element;
  struct que succ = { .succ = (struct que*)13, .pred = (struct que*)37 };
  struct que pred = { .succ = &succ, .pred = (struct que*)42 };
  rs_insque(&element, &pred);
  ASSERT_EQ(&succ, element.succ);
  ASSERT_EQ(&pred, element.pred);
  ASSERT_EQ((struct que*)13, succ.succ);
  ASSERT_EQ(&element, succ.pred);
  ASSERT_EQ(&element, pred.succ);
  ASSERT_EQ((struct que*)42, pred.pred);
}

TEST(remque, singleton)
{
  struct que element = { .succ = NULL, .pred = NULL };
  rs_remque(&element);
}

TEST(remque, head)
{
  struct que succ = { .succ = (struct que*)13, .pred = (struct que*)37 };
  struct que element = { .succ = &succ, .pred = NULL };
  rs_remque(&element);
  ASSERT_EQ((struct que*)13, succ.succ);
  ASSERT_EQ(NULL, succ.pred);
}

TEST(remque, tail)
{
  struct que pred = { .succ = (struct que*)13, .pred = (struct que*)37 };
  struct que element = { .succ = NULL, .pred = &pred };
  rs_remque(&element);
  ASSERT_EQ(NULL, pred.succ);
  ASSERT_EQ((struct que*)37, pred.pred);
}

TEST(remque, middle)
{
  struct que succ = { .succ = (struct que*)19, .pred = (struct que*)86 };
  struct que pred = { .succ = (struct que*)13, .pred = (struct que*)37 };
  struct que element = { .succ = &succ, .pred = &pred };
  rs_remque(&element);
  ASSERT_EQ((struct que*)19, succ.succ);
  ASSERT_EQ(&pred, succ.pred);
  ASSERT_EQ(&succ, pred.succ);
  ASSERT_EQ((struct que*)37, pred.pred);
}

struct node
{
  explicit node(const char* s)
    : s(strdup(s))
  {
  }

  char* s;
};

static int
node_cmp(const void* lhs, const void* rhs)
{
  return strcmp(reinterpret_cast<const node*>(lhs)->s,
                reinterpret_cast<const node*>(rhs)->s);
}

static std::vector<std::string> g_nodes;

static void
node_walk(const rs_posix_tnode* p, RS_VISIT order, int)
{
  const node* n = *reinterpret_cast<const node* const*>(p);
  if (order == postorder || order == leaf) {
    g_nodes.push_back(n->s);
  }
}

static size_t g_free_calls;

static void
node_free(void* p)
{
  node* n = reinterpret_cast<node*>(p);
  free(n->s);
  ++g_free_calls;
}

TEST(tfind_tsearch_tdestroy, example)
{
  rs_posix_tnode* root = nullptr;

  node n1("z");
  node n2("a");
  node n3("m");

  ASSERT_EQ(nullptr, rs_tfind(&n1, &root, node_cmp));
  ASSERT_EQ(nullptr, rs_tfind(&n2, &root, node_cmp));
  ASSERT_EQ(nullptr, rs_tfind(&n3, &root, node_cmp));

  void* i1 = rs_tsearch(&n1, &root, node_cmp);
  ASSERT_NE(nullptr, i1);

  ASSERT_EQ(i1, rs_tfind(&n1, &root, node_cmp));
  ASSERT_EQ(nullptr, rs_tfind(&n2, &root, node_cmp));
  ASSERT_EQ(nullptr, rs_tfind(&n3, &root, node_cmp));

  ASSERT_NE(nullptr, rs_tsearch(&n2, &root, node_cmp));
  ASSERT_NE(nullptr, rs_tsearch(&n3, &root, node_cmp));

  g_free_calls = 0;
  rs_tdestroy(root, node_free);
  ASSERT_EQ(3U, g_free_calls);
}

TEST(tdestroy, example)
{
  rs_tdestroy(nullptr, nullptr);
}

struct pod_node
{
  explicit pod_node(int i)
    : i(i)
  {
  }
  int i;
};

static int
pod_node_cmp(const void* lhs, const void* rhs)
{
  return reinterpret_cast<const pod_node*>(rhs)->i -
         reinterpret_cast<const pod_node*>(lhs)->i;
}

TEST(tdelete, example)
{
  rs_posix_tnode* root = nullptr;

  pod_node n1(123);
  ASSERT_NE(nullptr, rs_tsearch(&n1, &root, pod_node_cmp));

  pod_node not_there(456);
  ASSERT_EQ(nullptr, rs_tdelete(&not_there, &root, pod_node_cmp));
  ASSERT_NE(nullptr, rs_tdelete(&n1, &root, pod_node_cmp));
}

TEST(tsearch, null)
{
  ASSERT_EQ(NULL, rs_tdelete(NULL, NULL, NULL));
  ASSERT_EQ(NULL, rs_tfind(NULL, NULL, NULL));
  ASSERT_EQ(NULL, rs_tsearch(NULL, NULL, NULL));
}

static inline unsigned int
tnode_assert(const rs_posix_tnode* n)
{
  if (n == NULL)
    return 0;
  unsigned int height_left = tnode_assert(n->left);
  unsigned int height_right = tnode_assert(n->right);
  return (height_left > height_right ? height_left : height_right) + 1;
}

static int
compar(const void* a, const void* b)
{
  return *(int*)a - *(int*)b;
}

TEST(tsearch, random)
{
  int keys[1000];
  for (int i = 0; i < (int)std::size(keys); ++i)
    keys[i] = i;

  rs_posix_tnode* root = NULL;
  bool present[std::size(keys)] = {};
  for (int i = 0; i < 10000; ++i) {
    int key = arc4random_uniform(std::size(keys));
    switch (arc4random_uniform(3)) {
      case 0: // tdelete().
        if (present[key]) {
          ASSERT_TRUE(NULL != rs_tdelete(&key, &root, compar));
          present[key] = false;
        } else {
          ASSERT_TRUE(NULL == rs_tdelete(&key, &root, compar));
        }
        break;
      case 1: // tfind().
        if (present[key]) {
          ASSERT_TRUE(&keys[key] == rs_tfind(&key, &root, compar)->key);
        } else {
          ASSERT_TRUE(NULL == rs_tfind(&key, &root, compar));
        }
        break;
      case 2: // tsearch().
        if (present[key]) {
          ASSERT_TRUE(&keys[key] == rs_tsearch(&key, &root, compar)->key);
        } else {
          ASSERT_TRUE(&keys[key] == rs_tsearch(&keys[key], &root, compar)->key);
          present[key] = true;
        }
        break;
    }
    tnode_assert(root);
  }

  // Remove all entries from the tree.
  for (int key = 0; key < (int)std::size(keys); ++key)
    if (present[key])
      ASSERT_TRUE(NULL != rs_tdelete(&key, &root, compar));
  ASSERT_TRUE(NULL == root);
}

// helpers from llvm libc
static void*
encode(int val)
{
  return reinterpret_cast<void*>(static_cast<intptr_t>(val));
}

static int
decode(const void* val)
{
  return static_cast<int>(reinterpret_cast<intptr_t>(val));
}

static int
read_node(const rs_posix_tnode* node)
{
  return static_cast<int>(reinterpret_cast<intptr_t>(node->key));
}

static int
int_compare(const void* a, const void* b)
{
  int x = decode(a);
  int y = decode(b);
  return (x > y) - (x < y);
}

static void
noop_free(void*)
{
}

static int walk_sum;

static void
sum_action(const rs_posix_tnode* node, RS_VISIT visit, int)
{
  if (visit == leaf || visit == postorder)
    walk_sum += read_node(node);
}

TEST(twalk, empty_tree)
{
  walk_sum = 0;
  rs_twalk(nullptr, sum_action);
  ASSERT_EQ(walk_sum, 0);
}

TEST(twalk, single_node)
{
  rs_posix_tnode* root = nullptr;
  rs_tsearch(encode(99), &root, int_compare);

  walk_sum = 0;
  rs_twalk(root, sum_action);

  ASSERT_EQ(walk_sum, 99);
  rs_tdestroy(root, noop_free);
}

TEST(twalk, sums_correctly)
{
  rs_posix_tnode* root = nullptr;
  int expected = 0;
  for (int i = 1; i <= 10; ++i) {
    ASSERT_NE(rs_tsearch(encode(i), &root, int_compare), nullptr);
    expected += i;
  }

  walk_sum = 0;
  rs_twalk(root, sum_action);
  ASSERT_EQ(walk_sum, expected);
  rs_tdestroy(root, noop_free);
}

static int walk_prev;
static bool walk_sorted;

static void
sorted_action(const rs_posix_tnode* node, RS_VISIT visit, int)
{
  if (visit == leaf || visit == postorder) {
    int val = read_node(node);
    if (val <= walk_prev)
      walk_sorted = false;
    walk_prev = val;
  }
}

TEST(twalk, in_order)
{
  rs_posix_tnode* root = nullptr;

  constexpr int N = 32;
  for (int i = 0; i < N; ++i)
    ASSERT_NE(rs_tsearch(encode((i * 13 + 7) % N), &root, int_compare),
              nullptr);

  walk_prev = -1;
  walk_sorted = true;
  rs_twalk(root, sorted_action);
  ASSERT_TRUE(walk_sorted);
  rs_tdestroy(root, noop_free);
}

// Verify twalk depth parameter.
static int walk_max_depth;
static int walk_node_count;

static void
depth_action(const rs_posix_tnode*, RS_VISIT visit, int depth)
{
  if (visit == leaf || visit == postorder) {
    ++walk_node_count;
    if (depth > walk_max_depth)
      walk_max_depth = depth;
  }
}

TEST(twalk, reports_depth)
{
  rs_posix_tnode* root = nullptr;
  for (int i = 1; i <= 15; ++i)
    ASSERT_NE(rs_tsearch(encode(i), &root, int_compare), nullptr);

  walk_max_depth = -1;
  walk_node_count = 0;
  rs_twalk(root, depth_action);
  ASSERT_EQ(walk_node_count, 15);

  ASSERT_GT(walk_max_depth, 0);
  rs_tdestroy(root, noop_free);
}

static void
collect_action(const rs_posix_tnode* node, RS_VISIT visit, void* closure)
{
  if (visit == leaf || visit == postorder) {
    int** cursor = static_cast<int**>(closure);
    **cursor = read_node(node);
    *cursor += 1;
  }
}

TEST(twalk_r, produces_sorted_output)
{
  rs_posix_tnode* root = nullptr;
  constexpr int N = 64;
  for (int i = 0; i < N; ++i)
    ASSERT_NE(rs_tsearch(encode((i * 37) % N), &root, int_compare), nullptr);

  int sorted[N];
  int* cursor = sorted;
  rs_twalk_r(root, collect_action, &cursor);

  for (int i = 0; i < N; ++i)
    ASSERT_EQ(sorted[i], i);
  rs_tdestroy(root, noop_free);
}

TEST(twalk_r, empty_tree)
{
  int count = 0;
  auto counter = [](const rs_posix_tnode*, RS_VISIT, void* c) {
    ++*static_cast<int*>(c);
  };
  rs_twalk_r(nullptr, counter, &count);
  ASSERT_EQ(count, 0);
}
