#include "grid.h"

bool can_be_place(int *grid, int x, int y, int num) {
  int k = 0;
  int fx = x / TIER * TIER;
  int fy = y / TIER * TIER;

  for (k = 0; k < SIZE; k++) {
    if (grid[y * SIZE + k] == num) {
      return false;
    }
    if (grid[k * SIZE + x] == num) {
      return false;
    }
    if (grid[(fy + k / TIER) * SIZE + (fx + k % TIER)] == num) {
      return false;
    }
  }
  return true;
}

bool is_complete(int *grid) {
  for (int i = 0; i < LENGTH; i++) {
    if (grid[i] == 0) {
      return false;
    }
  }
  return true;
}
