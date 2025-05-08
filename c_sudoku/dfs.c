#include "grid.h"
#include "random.h"
#include <stdio.h>

typedef struct node {
  int *grid;
  struct node *next;
} node_t;

void push_front(node_t **head, int *grid) {
  node_t *node = (node_t *)malloc(sizeof(node_t));

  node->grid = grid;
  if (*head == NULL) {
    node->next = NULL;
  } else {
    node->next = *head;
  }
  *head = node;
}

node_t *pop_front(node_t **head) {
  node_t *next_node = NULL;
  if (*head == NULL) {
    return NULL;
  }
  next_node = (*head)->next;
  node_t *current = (*head);
  // next_node->prev = current->prev;
  *head = next_node;
  return current;
}

void free_stack(node_t **head) {
  node_t *current = (*head);
  if (current != NULL) {
    while (current->next != NULL) {
      node_t *ptr = current;
      current = current->next;
      free(ptr);
    }
  }
}

void free_node(node_t *node) {
  free(node->grid);
  free(node);
}

void debug_stack(node_t **head) {
  node_t *current = (*head);
  int i = 0;
  if (current != NULL) {
    while (current->next != NULL) {
      printf("i=%d, *grid=%p\n", i, current->grid);
      current = current->next;
      i++;
    }
  }
}

int stack_length(node_t **head) {
  int length = 0;
  node_t *current = (*head);
  if (current != NULL) {
    while (current->next != NULL) {
      current = current->next;
      length++;
    }
  }
  return length;
}

int *dfs(int *grid) {
  node_t *stack = NULL;
  node_t *current = NULL;
  int depth = 0;

  push_front(&stack, grid);
  do {
    current = pop_front(&stack);
    // if (depth % 10000000 == 0) {
    //   printf("Stack size=%d depth=%d\n", stack_length(&stack), depth);
    //   print_pretty_grid(current->grid);
    // }
    if (current == NULL) {
      return NULL;
    }
    if (is_complete(current->grid)) {
      free_stack(&stack);
      return current->grid;
    }
    int x = random_index();
    int y = random_index();
    int i = y * SIZE + x;
    while (current->grid[i] != 0) {
      x = random_index();
      y = random_index();
      i = y * SIZE + x;
    }
    for (int num = 1; num <= SIZE; num++) {
      if (can_be_place(current->grid, x, y, num)) {
        int *neighbor = clone_grid(current->grid);
        neighbor[i] = num;
        push_front(&stack, neighbor);
      }
    }
    depth++;
    free_node(current);
  } while (current != NULL);

  free_stack(&stack);
  return NULL;
}
