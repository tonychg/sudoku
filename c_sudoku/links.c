#include "links.h"
#include <stdio.h>
#include <stdlib.h>

static const int LENGTH = 81;
static const int SIZE = 9;
static const int MAX_WIDTH = 324;
static const int MAX_HEIGHT = 729;
static const int CONSTRAINT_ROW_COL = 0;
static const int CONSTRAINT_ROW_NUMBER = 1;
static const int CONSTRAINT_COL_NUMBER = 2;
static const int CONSTRAINT_BOX_NUMBER = 3;

int **create_sparse_matrix() {
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

void print_sparse_matrix(int **matrix, int p) {
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

links_t *create_torus() {
  links_t *torus = (links_t *)malloc(sizeof(links_t));
  torus->left = torus;
  torus->right = torus;
  torus->up = torus;
  torus->down = torus;
  torus->size = 0;
  torus->col = 0;
  torus->row = 0;
  return torus;
}
