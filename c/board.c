#include "board.h"
#include "list.h"
#include "random.h"
#include <stdio.h>
#include <string.h>

board_t *board_init() {
  board_t *b = (board_t *)malloc(sizeof(board_t));
  for (int i = 0; i < LENGTH; i++)
    b->grid[i] = 0;
  b->cols = 0;
  b->rows = 0;
  b->quads = 0;
  return b;
}

char *board_export(board_t *b) {
  char *bstr = (char *)calloc(LENGTH, sizeof(char));
  for (int i = 0; i < LENGTH; i++) {
    bstr[i] = b->grid[i] + '0';
  }
  return bstr;
}

void board_write(board_t *b, int seed, char *dest) {
  FILE *fptr = fopen(dest, "w");
  char *bstr = board_export(b);
  if (!fptr)
    return;
  fprintf(fptr, "%d:%s\n", seed, bstr);
  fclose(fptr);
  free(bstr);
}

void board_print(board_t *b) {
  char *bstr = board_export(b);
  printf("%s\n", bstr);
  free(bstr);
}

void board_pretty_print(board_t *b) {
  char line[] = "+---+---+---+";

  for (int y = 0; y < SIZE; y++) {
    if (y == 0) {
      printf("%s\n", line);
    }
    for (int x = 0; x < SIZE; x++) {
      int index = y * SIZE + x;
      char num_char = b->grid[index] + '0';
      if (b->grid[index] == 0) {
        num_char = ' ';
      }
      if (x % TIER == 2) {
        printf("%c|", num_char);
      } else if (x == 0) {
        printf("|%c", num_char);
      } else if (y % TIER == 2 && x == SIZE - 1) {
        printf("%c", num_char);
      } else {
        printf("%c", num_char);
      }
    }
    printf("\n");
    if (y % TIER == 2) {
      printf("%s\n", line);
    }
  }
}

void board_set_number(board_t *b, u8 x, u8 y, u8 num) {
  u128 key_quads = (y / TIER * TIER + x / TIER) * SIZE;
  u128 key_cols = x * SIZE;
  u128 key_rows = y * SIZE;
  u8 dest_num = b->grid[y * SIZE + x];
  if (dest_num > 0) {
    u128 key = 1 << (dest_num - 1);
    b->cols ^= key << key_cols;
    b->rows ^= key << key_rows;
    b->quads ^= key << key_quads;
  }
  if (num) {
    u128 key = 1 << (num - 1);
    b->cols |= key << key_cols;
    b->rows |= key << key_rows;
    b->quads |= key << key_quads;
  }
  b->grid[y * SIZE + x] = num;
}

bool board_can_place(board_t *b, u8 x, u8 y, u8 num) {
  u128 key = 1 << (num - 1);
  u128 mask_rows = b->rows >> y * SIZE;
  u128 mask_cols = b->cols >> x * SIZE;
  u128 mask_quads = b->quads >> ((y / TIER * TIER + x / TIER) * SIZE);
  u128 mask = mask_rows | mask_cols | mask_quads;
  return (mask & key) == 0;
}

bool board_is_complete(board_t *b) {
  for (int i = 0; i < LENGTH; i++)
    if (b->grid[i] == 0)
      return false;
  return true;
}

u8 board_next_empty(board_t *b) {
  int x = random_index();
  int y = random_index();
  while (b->grid[y * SIZE + x] != 0) {
    x = random_index();
    y = random_index();
  }
  return y * SIZE + x;
}

board_t *board_clone(board_t *b) {
  board_t *new = (board_t *)malloc(sizeof(board_t));
  memcpy(new->grid, b->grid, LENGTH);
  new->cols = b->cols;
  new->rows = b->rows;
  new->quads = b->quads;
  return new;
}

board_t *board_backtracking(board_t *b) {
  list_t *stack = list_new();
  int depth = 0;
  list_push(stack, (void *)(board_t *)b);
  while (stack->size) {
    board_t *current = (board_t *)list_pop(stack);
    if (board_is_complete(current)) {
      return current;
    }
    if (depth > 500) {
      stack->first = stack->first->prev;
      depth = 0;
    }
    u8 i = board_next_empty(current);
    u8 x = i % SIZE;
    u8 y = i / SIZE;
    for (int n = 1; n <= SIZE; n++) {
      if (board_can_place(current, x, y, n)) {
        board_t *neighbor = board_clone(current);
        board_set_number(neighbor, x, y, n);
        list_push(stack, (void *)(board_t *)neighbor);
      }
    }
    free(current);
    depth++;
  }
  return NULL;
}

bool board_backtracking_recursive(board_t *b) {
  if (board_is_complete(b)) {
    return true;
  }
  int i = board_next_empty(b);
  int x = i % SIZE;
  int y = i / SIZE;
  for (int num = 1; num <= SIZE; num++) {
    if (board_can_place(b, x, y, num)) {
      board_set_number(b, x, y, num);
      if (board_backtracking_recursive(b)) {
        return true;
      }
    }
  }
  board_set_number(b, x, y, 0);
  return false;
}
