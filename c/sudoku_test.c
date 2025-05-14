#include "sudoku.h"
#include <stdlib.h>

int main()
{
	int size = 9;
	int *grid = (int *)calloc(size * size, sizeof(int));
	grid[0] = 4;
	int **matrix = sudoku_sparse_build(grid);
	sudoku_print_matrix_constraint(matrix, 1);
}
