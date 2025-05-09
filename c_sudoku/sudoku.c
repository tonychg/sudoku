#include "sudoku.h"
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
