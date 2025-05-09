#ifndef __SUDOKU_BOARD__
#define __SUDOKU_BOARD__

#include <stdint.h>
#include <stdbool.h>

static const int SIZE = 9;
static const int TIER = 3;
static const int LENGTH = 81;

typedef uint8_t u8;
typedef unsigned __int128 u128;

typedef struct board {
    u8 grid[81];
    u128 cols;
    u128 rows;
    u128 quads;
} board_t;

void board_print(board_t *b);
char *board_export(board_t *b);
void board_write(board_t *b, int seed, char *dest);
board_t *board_init();
board_t *board_clone(board_t *b);
board_t *board_backtracking(board_t *b);
bool board_backtracking_recursive(board_t *b);

#endif
