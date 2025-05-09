#include "board.h"
#include "sudoku.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

void command_generate(char *dest) {
  srand(time(NULL));
  int seed = rand();
  srand(seed);
  board_t *b = board_init();
  board_t *filled = board_backtracking(b);
  if (filled) {
    if (!dest) {
      printf("%d:", seed);
      board_print(filled);
    } else {
      board_write(filled, seed, dest);
    }
  }
}

void command_sparse(char *dest) {
  int **matrix = sudoku_sparse_create();
  sudoku_sparse_write(matrix, dest);
}

int main(int argc, char **argv) {
  if (argc <= 1) {
    printf("Usage: sudoku [sparse,generate] [DESTINATION]\n");
  } else if (!strcmp(argv[1], "sparse")) {
    char *dest;
    if (argc != 3) {
      printf("Use default output m.out\n");
      dest = "m.out";
    } else {
      dest = argv[2];
    }
    command_sparse(dest);
  } else if (!strcmp(argv[1], "generate")) {
    if (argc != 3) {
      command_generate(NULL);
    } else {
      command_generate(argv[2]);
    }
  }
  exit(0);
}
