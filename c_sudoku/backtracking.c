#include "grid.h"
#include "random.h"

bool recursive_backtracking(int *grid, int iteration) {
  if (is_complete(grid)) {
    return true;
  }
  int x = random_index();
  int y = random_index();
  int i = y * SIZE + x;
  if (grid[i] != 0) {
    return recursive_backtracking(grid, iteration + 1);
  }
  for (int num = 1; num <= SIZE; num++) {
    if (can_be_place(grid, x, y, num)) {
      grid[i] = num;
      if (recursive_backtracking(grid, iteration + 1)) {
        return true;
      }
    }
  }
  grid[i] = 0;
  return false;
}
