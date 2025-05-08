#include "board.h"
#include "random.h"

int main() {
  Board *board = b_init();
  // unsigned int seed = random_seed();
  srand(1200);
  recursive_backtracking(board, 0);
  b_print_grid(board);
}
