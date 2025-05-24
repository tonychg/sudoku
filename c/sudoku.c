#include "sudoku.h"
#include "links.h"
#include "list.h"
#include "hashtable.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int *sudoku_propagate_clue(int x, int y, int n)
{
	int j = x * SIZE + y * LENGTH + (n + 1);
	int *position = malloc(sizeof(int) * (SIZE - 1));
	printf("j=%d\n", j);
	return position;
}

// int **sudoku_sparse_build(char *grid)
// {
// 	int j, indice, number, row, col, box, n, s = SIZE / 3;
// 	int **matrix = (int **)malloc(MAX_HEIGHT * sizeof(int *));
// 	for (j = 0; j < MAX_HEIGHT; j++) {
// 		matrix[j] = (int *)calloc(MAX_WIDTH, sizeof(int));
// 	}
// 	for (j = 0; j < MAX_HEIGHT; j++) {
// 		number = j % SIZE;
// 		indice = j / SIZE;
// 		row = indice % SIZE;
// 		col = indice / SIZE;
// 		box = (row / s * s + col / s) * SIZE;
// 		matrix[j][indice] = number + 1;
// 		matrix[j][number + (col * SIZE) + LENGTH] = number + 1;
// 		matrix[j][number + (row * SIZE) + LENGTH * 2] = number + 1;
// 		matrix[j][number + box + LENGTH * 3] = number + 1;
// 	}
// 	if (grid) {
// 		for (indice = 0; indice < LENGTH; indice++) {
// 			if (grid[indice]) {
// 				number = grid[indice];
// 				row = indice / SIZE;
// 				col = indice % SIZE;
// 				box = (row / s * s + col / s) * SIZE;
// 				j = indice * SIZE;
// 			}
// 		}
// 	}
// 	return matrix;
// }

void sudoku_print_matrix_constraint(int **matrix, int constraint)
{
	for (int j = 0; j < MAX_HEIGHT; j++) {
		for (int i = 0; i < LENGTH; i++) {
			int offset = constraint * LENGTH;
			if (matrix[j][i + offset]) {
				printf("%d", matrix[j][i + offset]);
			} else {
				printf(" ");
			}
		}
		printf("\n");
	}
}

void sudoku_print_matrix(int **matrix)
{
	for (int j = 0; j < MAX_HEIGHT; j++) {
		for (int i = 0; i < MAX_WIDTH; i++) {
			if (matrix[j][i]) {
				printf("%d", matrix[j][i]);
			} else {
				printf(" ");
			}
		}
		printf("\n");
	}
}

int **sudoku_sparse_create()
{
	int **matrix = (int **)malloc(MAX_WIDTH * sizeof(int *));
	for (int x = 0; x < MAX_WIDTH; x++) {
		matrix[x] = (int *)calloc(MAX_HEIGHT, sizeof(int));
	}
	int offset;
	int i = 0;
	int t1 = 0;
	int t2 = 0;
	for (int x = 0; x < LENGTH; x++) {
		for (int y = 0; y < SIZE; y++) {
			offset = CONSTRAINT_ROW_COL * LENGTH;
			matrix[x + offset][y + (SIZE * x)] = y + 1;
			offset = CONSTRAINT_ROW_NUMBER * LENGTH;
			matrix[y + (x / SIZE * SIZE) + offset][y + (x * SIZE)] =
				y + 1;
			offset = CONSTRAINT_COL_NUMBER * LENGTH;
			matrix[i % LENGTH + offset][y + (x * SIZE)] = y + 1;
			t1 = i / (LENGTH / 3) % 3 * SIZE;
			t2 = i / (MAX_HEIGHT / 3) * (SIZE * 3);
			offset = CONSTRAINT_BOX_NUMBER * LENGTH + t1 + t2;
			matrix[(i % LENGTH % SIZE) + offset][y + (x * SIZE)] =
				y + 1;
			i++;
		}
	}
	return matrix;
}

void sudoku_sparse_destroy(int **matrix)
{
	for (int x = 0; x < MAX_WIDTH; x++) {
		free(matrix[x]);
	}
	free(matrix);
}

void sudoku_sparse_print(int **matrix, int p)
{
	for (int y = 0; y < MAX_HEIGHT; y++) {
		for (int x = 0; x < LENGTH; x++) {
			if (matrix[x + LENGTH * p][y]) {
				printf("%d", matrix[x + LENGTH * p][y]);
			} else {
				printf(" ");
			}
		}
		printf("\n");
	}
}

void sudoku_sparse_write(int **matrix, char *dest)
{
	FILE *fptr;
	int x = 0;

	fptr = fopen(dest, "w");
	if (fptr == NULL) {
		return;
	}
	for (int y = 0; y < MAX_HEIGHT; y++) {
		char *line = (char *)calloc(MAX_WIDTH, sizeof(char));
		for (x = 0; x < MAX_WIDTH; x++) {
			line[x] = matrix[x][y] + '0';
		}
		fprintf(fptr, "%s\n", line);
		free(line);
	}
	fclose(fptr);
}

char *sudoku_grid_stdin(void)
{
	int i;
	char buf[LENGTH + 2];
	char *grid = (char *)malloc(LENGTH * sizeof(char));
	fgets(buf, LENGTH + 2, stdin);
	for (i = 0; i < LENGTH; i++) {
		grid[i] = buf[i];
	}
	grid[i] = '\0';
	return grid;
}

int *sudoku_grid_from_str(char *str)
{
	int *grid = (int *)calloc(LENGTH, sizeof(int));
	for (int i = 0; i < LENGTH; i++) {
		grid[i] = str[i] - '0';
	}
	return grid;
}

char *sudoku_grid_to_str(int *grid)
{
	int i;
	char *str = (char *)malloc(LENGTH * sizeof(char));
	for (i = 0; i < LENGTH; i++) {
		str[i] = grid[i] + '0';
	}
	str[i] = '\0';
	return str;
}

void sudoku_grid_print(char *grid, char *solution)
{
	char number;
	char line[] = "+---+---+---+";

	for (int y = 0; y < SIZE; y++) {
		if (y == 0) {
			printf("%s\n", line);
		}
		for (int x = 0; x < SIZE; x++) {
			int index = y * SIZE + x;
			if (solution != NULL) {
				number = grid[index] != '0' ? grid[index] :
							      solution[index];
			} else {
				number = grid[index];
			}
			if (number == '0') {
				number = ' ';
			}
			if (x % TIER == 2) {
				printf("%c|", number);
			} else if (x == 0) {
				printf("|%c", number);
			} else if (y % TIER == 2 && x == SIZE - 1) {
				printf("%c", number);
			} else {
				printf("%c", number);
			}
		}
		printf("\n");
		if (y % TIER == 2) {
			printf("%s\n", line);
		}
	}
}

int sudoku_update_matrix(links_T *head, char *grid, plist_T *o)
{
	int k = 0;
	for (int i = 0; i < LENGTH; i++) {
		if (grid[i] != '0') {
			int x = i % SIZE;
			int y = i / SIZE;
			int number = grid[i] - '0';
			int row_index = y * 81 + x * 9 + (number - 1);
			o->p[k] = links_select_row(head, row_index);
			o->size++;
			k++;
		}
	}
	return k;
}

plist_T *sudoku_x(char *grid, int limit, bool deterministic)
{
	int **matrix;
	links_T *head, **cols, **rows;
	plist_T *result;
	result = partial_new();
	matrix = sudoku_sparse_create();
	head = links_exact_cover(MAX_WIDTH);
	cols = links_columns_save(head);
	rows = links_add_nodes(head, MAX_WIDTH, MAX_HEIGHT, matrix);
	sudoku_sparse_destroy(matrix);
	if (grid)
		sudoku_update_matrix(head, grid, result);
	links_dancing(head, result, 0, limit, deterministic);
	links_destroy(head, rows, cols);
	return result;
}

void sudoku_solve(char *grid, int limit)
{
	plist_T *result = sudoku_x(grid, limit, true);
	list_T *tmp;
	for (tmp = result->grids->next; tmp != result->grids; tmp = tmp->next)
		sudoku_grid_print(grid, sudoku_grid_to_str(tmp->data));
	partial_destroy(result);
}

int sudoku_count_solution(char *grid)
{
	plist_T *result = sudoku_x(grid, 2, true);
	int solutions = result->solutions;
	partial_destroy(result);
	return solutions;
}

int sudoku_count_solution_with_limit(char *grid, int limit)
{
	plist_T *result = sudoku_x(grid, limit, true);
	int solutions = result->solutions;
	partial_destroy(result);
	return solutions;
}

int sudoku_next_random(char *grid)
{
	int i;
	do {
		i = rand() % LENGTH;
	} while (grid[i] == '0');
	return i;
}

bool sudoku_make_playable(char *grid, int clues)
{
	if (!clues) {
		return true;
	}
	for (int i = 0; i < LENGTH; i++) {
		if (grid[i] != '0') {
			char number = grid[i];
			grid[i] = '0';
			int solutions = sudoku_count_solution(grid);
			if (solutions == 1) {
				if (sudoku_make_playable(grid, clues - 1)) {
					return true;
				}
			}
			grid[i] = number;
		}
	}
	return false;
}

int count_clues(char *grid)
{
	int counter;
	for (counter = 0; *grid; grid++)
		if (*grid != '0')
			counter++;
	return counter;
}

char **fisher_yates(char **array, int n)
{
	int i, j, upper_bound;
	char *tmp;
	char **result = (char **)malloc(sizeof(char *) * n);
	for (i = n - 1; i > 0; i--) {
		upper_bound = RAND_MAX - ((RAND_MAX % (i + 1)) + 1);
		do {
			j = rand() % (i + 1);
		} while (j > upper_bound);
		tmp = array[j];
		array[j] = array[i];
		array[i] = tmp;
	}
	return result;
}

char **get_neighbors(char *grid, int clues)
{
	char **neighbors = (char **)malloc(sizeof(char *) * clues);
	int i, j = 0;
	for (i = 0; grid[i]; i++) {
		if (grid[i] != '0') {
			char *neighbor = strdup(grid);
			neighbor[i] = '0';
			int n = sudoku_count_solution(neighbor);
			if (n == 1) {
				neighbors[j] = neighbor;
				j++;
			} else {
				free(neighbor);
			}
		}
	}
	neighbors[j] = NULL;
	fisher_yates(neighbors, j);
	return neighbors;
}

char *sudoku_backtracking_playable(char *grid, int clues)
{
	list_T *node;
	list_T *stack = list_create();
	hashtable_T *visited = hashtable_create();
	char *result;
	int curr_clues, depth = 0;
	list_push(stack, grid);
	while (stack->next != stack) {
		node = list_pop(stack);
		curr_clues = count_clues(node->data);
		if (!(depth % 100)) {
			printf("-> %7d Current clues: %d\n", depth, curr_clues);
			sudoku_grid_print(node->data, NULL);
		}
		if (curr_clues == clues) {
			result = strdup((char *)node->data);
			break;
		}
		if (hashtable_lookup(visited, node->data))
			continue;
		result = strdup((char *)node->data);
		char **neighbors = get_neighbors(result, curr_clues);
		for (int i = 0; neighbors[i]; i++) {
			if (!hashtable_lookup(visited, neighbors[i])) {
				list_push(stack, neighbors[i]);
			} else {
				free(neighbors[i]);
			}
		}
		free(neighbors);
		hashtable_insert(visited, result);
		depth++;
	}
	hashtable_destroy(visited);
	list_free(stack);
	return result;
}

char *sudoku_create_random_grid(char *grid, int clues)
{
	char *result = strdup(grid);
	int removed = 0;
	while (LENGTH - removed != clues) {
		int indice = sudoku_next_random(result);
		result[indice] = 0;
		removed++;
	}
	return result;
}

void sudoku_make_playable_full(char *grid, int clues)
{
	int solutions = 0;
	char *result;
	while (solutions != 2) {
		result = sudoku_create_random_grid(grid, clues);
		solutions = sudoku_count_solution_with_limit(result, 100);
		printf("solutions=%d\n", solutions);
		free(result);
	}
	sudoku_grid_print(result, NULL);
}

char *sudoku_new_grid()
{
	int i;
	char *grid = (char *)malloc(sizeof(char) * LENGTH);
	for (i = 0; i < LENGTH; i++) {
		grid[i] = '0';
	}
	grid[i] = '\0';
	return grid;
}

char *sudoku_generate_complete()
{
	char *grid = sudoku_new_grid();
	printf("Run X algorithm on empty grid\n");
	plist_T *result = sudoku_x(grid, 1, false);
	grid = sudoku_grid_to_str(result->grids->next->data);
	partial_destroy(result);
	return grid;
}

void sudoku_generate(int clues, bool human)
{
	char *base_grid, *holed;
	base_grid = sudoku_generate_complete();
	printf("Complete board generated\n");
	printf("Make the board playable\n");
	holed = sudoku_backtracking_playable(base_grid, clues);
	printf("Board is playable\n");
	if (human) {
		sudoku_grid_print(base_grid, NULL);
		sudoku_grid_print(holed, NULL);
	} else {
		printf("%s\n", base_grid);
		printf("%s\n", holed);
	}
	free(base_grid);
	free(holed);
}
