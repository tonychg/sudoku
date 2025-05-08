#include "grid.h"
#include "random.h"
#include <stdio.h>

int main() {
  int seed = random_seed();
  int *grid = new_grid();
  srand(seed);
  printf("%d\n", random_index());
  printf("%d\n", random_index());
  printf("%d\n", random_index());
  printf("%d\n", random_index());
  printf("%d\n", random_index());
  printf("%d\n", random_index());
  int *solution = dfs(grid);
  printf("seed=%d\n", seed);
  print_pretty_grid(solution);
  printf("hash=%lu\n", hash(grid));
  printf("hash=%lu\n", hash(solution));
  free_grid(grid);
  free_grid(solution);
}
