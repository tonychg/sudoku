#include <stdbool.h>

static const int SIZE = 9;
static const int TIER = 3;
static const int LENGTH = 81;

// grid
int *new_grid(void);
int *clone_grid(int *grid);
void print_grid(int *grid);
void print_pretty_grid(int *grid);
void free_grid(int *grid);
char *grid_as_str(int *grid);
unsigned long hash(int *grid);

// heuristic
bool can_be_place(int *grid, int x, int y, int num);
bool is_complete(int *grid);

// backtracking
bool recursive_backtracking(int *grid, int iteration);

// dfs
int *dfs(int *grid);
