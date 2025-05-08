#include <stdbool.h>

static const int SIZE = 9;
static const int TIER = 3;

typedef int Board[9][9];

// board
void b_print(Board *board);
void b_print_grid(Board *board);
void b_debug(Board *board, int iteration);
char *b_string(Board *board);
Board *b_clone(Board *board);
Board *b_init(void);
bool can_be_place(Board *board, int x, int y, int number);
bool is_complete(Board *board);

// backtracking
bool recursive_backtracking(Board *board, int iteration);
