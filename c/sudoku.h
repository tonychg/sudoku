#ifndef __SUDOKU__
#define __SUDOKU__

#include "board.h"

static const int MAX_WIDTH = 324;
static const int MAX_HEIGHT = 729;
static const int CONSTRAINT_ROW_COL = 0;
static const int CONSTRAINT_ROW_NUMBER = 1;
static const int CONSTRAINT_COL_NUMBER = 2;
static const int CONSTRAINT_BOX_NUMBER = 3;

int **sudoku_sparse_build(char *grid);
int **sudoku_sparse_create();
void sudoku_sparse_print(int **matrix, int p);
void sudoku_print_matrix(int **matrix);
void sudoku_print_matrix_constraint(int **matrix, int constraint);
void sudoku_sparse_write(int **matrix, char *dest);
char *sudoku_grid_stdin(void);
int *sudoku_grid_from_str(char *str);
void sudoku_grid_print(char *grid, char *solution);
void sudoku_solve(char *grid, int limit);
void sudoku_generate(int clues, bool human);

#endif
