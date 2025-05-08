#include "grid.h"
#include "list.h"
#include "random.h"
#include <stdio.h>

int *dfs(int *grid) {
  list_t *stack = list_new();
  int *current = NULL;
  int depth = 0;

  // list_push(stack, grid);
  // do {
  //   current = pop_front(&stack);
  //   // if (depth % 10000000 == 0) {
  //   //   printf("Stack size=%d depth=%d\n", stack_length(&stack), depth);
  //   //   print_pretty_grid(current->grid);
  //   // }
  //   if (current == NULL) {
  //     return NULL;
  //   }
  //   if (is_complete(current->grid)) {
  //     return current->grid;
  //   }
  //   int x = random_index();
  //   int y = random_index();
  //   int i = y * SIZE + x;
  //   while (current->grid[i] != 0) {
  //     x = random_index();
  //     y = random_index();
  //     i = y * SIZE + x;
  //   }
  //   for (int num = 1; num <= SIZE; num++) {
  //     if (can_be_place(current->grid, x, y, num)) {
  //       int *neighbor = clone_grid(current->grid);
  //       neighbor[i] = num;
  //       push_front(&stack, neighbor);
  //     }
  //   }
  //   depth++;
  // } while (current != NULL);

  return NULL;
}
