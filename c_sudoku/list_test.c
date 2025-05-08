#include "list.h"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

void test_list_new() {
  list_t *head = list_new();
  assert(head->size == 0);
  assert(head->first == NULL);
  printf("[OK] test_list_new\n");
}

void test_list_push() {
  int p1 = 10;
  int p2 = 11;
  int p3 = 12;
  list_t *list = list_new();
  list_push(list, (void *)(uintptr_t)p1);
  list_push(list, (void *)(uintptr_t)p2);
  list_push(list, (void *)(uintptr_t)p3);
  assert(list->size == 3);
  assert(list->first->data == p3);
  assert(list->first->prev->data == p1);
  assert(list->first->next->data == p2);
  printf("[OK] test_list_push\n");
}

void test_list_insert() {
  int p1 = 10;
  int p2 = 11;
  int p3 = 12;
  list_t *list = list_new();
  list_insert(list, (void *)(uintptr_t)p1);
  list_insert(list, (void *)(uintptr_t)p2);
  list_insert(list, (void *)(uintptr_t)p3);
  assert(list->size == 3);
  assert(list->first->data == p1);
  assert(list->first->prev->data == p3);
  assert(list->first->next->data == p2);
  printf("[OK] test_list_insert\n");
}

void test_list_del() {
  int p1 = 10;
  int p2 = 11;
  int p3 = 12;
  list_t *list = list_new();
  list_push(list, (void *)(uintptr_t)p1);
  list_push(list, (void *)(uintptr_t)p2);
  list_push(list, (void *)(uintptr_t)p3);
  list_del(list, (void *)(uintptr_t)p2);
  assert(list->first->next == list->first->prev);
  assert(list->first->prev == list->first->next);
  assert(list->size == 2);
  printf("[OK] test_list_del\n");
}

void test_list_pop() {
  int p1 = 10;
  int p2 = 11;
  int p3 = 12;
  list_t *list = list_new();
  list_push(list, (void *)(uintptr_t)p1);
  list_push(list, (void *)(uintptr_t)p2);
  list_push(list, (void *)(uintptr_t)p3);
  size_t *elem = list_pop(list);
  assert(*elem == p3);
  assert(list->first->data == p2);
  assert(list->first->prev->data == p1);
  printf("[OK] test_list_pop\n");
}

void test_complete() {
  list_t *list = list_new();
  for (int i = 0; i < 100; i++) {
    list_push(list, &i);
  }
  list_node_t *node = list->first;
  for (int i = 0; i < list->size; i++) {
    printf("%p -> %d\n", node, (int)node->data);
    node = node->next;
  }
}

int main() {
  printf("--> Testing list.h\n");
  test_list_new();
  test_list_push();
  test_list_insert();
  test_list_del();
  test_list_pop();
  // test_complete();
}
