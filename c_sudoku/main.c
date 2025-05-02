#include "board.h"
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

int random_index() { return rand() % SIZE; }

int random_seed() { return rand() % 2147483647; }

bool solve(Board *board, int iteration) {
  if (is_complete(board)) {
    return false;
  }
  // printf("i=%d\n", iteration);
  int x = random_index();
  int y = random_index();
  if (*board[y][x] != 0) {
    return solve(board, iteration + 1);
  }
  for (int num = 1; num <= SIZE; num++) {
    if (can_be_place(board, x, y, num)) {
      *board[y][x] = num;
      if (solve(board, iteration + 1)) {
        return true;
      }
      b_debug(board, iteration);
    }
  }
  *board[y][x] = 0;
  return false;
}

int main() {
  Board *board = b_init();
  unsigned int seed = random_seed();
  srand(seed);
  solve(board, 0);
}
