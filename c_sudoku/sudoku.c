#include "sudoku.h"
#include "links.h"
#include <stdio.h>
#include <stdlib.h>

int **sudoku_sparse_create() {
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
      matrix[y + (x / SIZE * SIZE) + offset][y + (x * SIZE)] = y + 1;
      offset = CONSTRAINT_COL_NUMBER * LENGTH;
      matrix[i % LENGTH + offset][y + (x * SIZE)] = y + 1;
      t1 = i / (LENGTH / 3) % 3 * SIZE;
      t2 = i / (MAX_HEIGHT / 3) * (SIZE * 3);
      offset = CONSTRAINT_BOX_NUMBER * LENGTH + t1 + t2;
      matrix[(i % LENGTH % SIZE) + offset][y + (x * SIZE)] = y + 1;
      i++;
    }
  }
  return matrix;
}

void sudoku_sparse_print(int **matrix, int p) {
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

void sudoku_sparse_write(int **matrix, char *dest) {
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

int *sudoku_grid_stdin(void) {
  char buf[LENGTH + 2];
  int *grid = (int *)calloc(LENGTH, sizeof(int));
  fgets(buf, LENGTH + 2, stdin);
  for (int i = 0; i < LENGTH; i++) {
    grid[i] = buf[i] - '0';
  }
  return grid;
}

int *sudoku_grid_from_str(char *str) {
  int *grid = (int *)calloc(LENGTH, sizeof(int));
  for (int i = 0; i < LENGTH; i++) {
    grid[i] = str[i] - '0';
  }
  return grid;
}

void sudoku_grid_print(int *grid, int *solution) {
  int number;
  char line[] = "+---+---+---+";

  for (int y = 0; y < SIZE; y++) {
    if (y == 0) {
      printf("%s\n", line);
    }
    for (int x = 0; x < SIZE; x++) {
      int index = y * SIZE + x;
      if (solution != NULL) {
        number = grid[index] ? grid[index] : solution[index];
      } else {
        number = grid[index];
      }
      char num_char = number + '0';
      if (number == 0) {
        num_char = ' ';
      }
      if (x % TIER == 2) {
        printf("%c|", num_char);
      } else if (x == 0) {
        printf("|%c", num_char);
      } else if (y % TIER == 2 && x == SIZE - 1) {
        printf("%c", num_char);
      } else {
        printf("%c", num_char);
      }
    }
    printf("\n");
    if (y % TIER == 2) {
      printf("%s\n", line);
    }
  }
}

int sudoku_update_matrix(struct links *head, int *grid, struct plist *o) {
  int k = 0;
  for (int i = 0; i < LENGTH; i++) {
    if (grid[i]) {
      int x = i % SIZE;
      int y = i / SIZE;
      int number = grid[i];
      int row_index = y * 81 + x * 9 + (number - 1);
      o->p[k] = links_select_row(head, row_index);
      o->size++;
      k++;
    }
  }
  return k;
}

int **sudoku_build_grid() {
  int y;
  int **grid = (int **)malloc(sizeof(int *));
  for (y = 0; y < SIZE; y++) {
    grid[y] = (int *)calloc(SIZE, sizeof(int));
  }
  return grid;
}

void sudoku_solve(int *grid, int limit) {
  struct links *head = links_exact_cover(MAX_WIDTH);
  struct plist *o = partial_new();
  struct slist *s;
  int **matrix = sudoku_sparse_create();
  links_add_nodes(head, MAX_WIDTH, MAX_HEIGHT, matrix);
  int k = sudoku_update_matrix(head, grid, o);
  links_dancing(head, o, k, -1);
  for (s = o->s; s != NULL; s = s->next) {
    sudoku_grid_print(grid, s->grid);
  }
  links_destroy(head);
  partial_destroy(o);
}

void sudoku_generate(int limit) {
  struct links *head = links_exact_cover(MAX_WIDTH);
  struct plist *o = partial_new();
  struct slist *s;
  int **matrix = sudoku_sparse_create();
  links_add_nodes(head, MAX_WIDTH, MAX_HEIGHT, matrix);
  links_dancing_non_deterministic(head, o, 0, limit);
  for (s = o->s; s != NULL; s = s->next)
    sudoku_grid_print(s->grid, NULL);
  links_destroy(head);
  partial_destroy(o);
}
