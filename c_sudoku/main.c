#include "board.h"
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
// #include "random.h"

int main() {
  srand(time(NULL));
  int seed = rand();
  printf("seed=%d\n", seed);
  srand(seed);
  board_t *b = board_init();
  board_t *filled = board_backtracking(b);
  if (filled)
    board_print(filled);
}
