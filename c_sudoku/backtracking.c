#include "board.h"
#include "random.h"

bool recursive_backtracking(Board *board, int iteration) {
  if (is_complete(board)) {
    return true;
  }
  // printf("i=%d\n", iteration);
  int x = random_index();
  int y = random_index();
  if (*board[y][x] != 0) {
    return recursive_backtracking(board, iteration + 1);
  }
  for (int num = 1; num <= SIZE; num++) {
    if (can_be_place(board, x, y, num)) {
      *board[y][x] = num;
      if (recursive_backtracking(board, iteration + 1)) {
        return true;
      }
      // b_debug(board, iteration);
      // *board[y][x] = 0;
    }
  }
  *board[y][x] = 0;
  return false;
}
