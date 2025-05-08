#include "grid.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int *new_grid(void) {
  int *grid;

  grid = (int *)malloc(LENGTH * sizeof(int));
  for (int i = 0; i < LENGTH; i++) {
    grid[i] = 0;
  }
  return (grid);
}

int *clone_grid(int *grid) {
  int *new_grid;

  new_grid = (int *)malloc(LENGTH * sizeof(int));
  memcpy(new_grid, grid, LENGTH * sizeof(int));
  return (new_grid);
}

void print_grid(int *grid) {
  for (int i = 0; i < LENGTH; i++) {
    printf("%d", grid[i]);
  }
  printf("\n");
}

void print_pretty_grid(int *grid) {
  char line[] = "+---+---+---+";

  for (int y = 0; y < SIZE; y++) {
    if (y == 0) {
      printf("%s\n", line);
    }
    for (int x = 0; x < SIZE; x++) {
      int index = y * SIZE + x;
      char num_char = grid[index] + '0';
      if (grid[index] == 0) {
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

void free_grid(int *grid) { free(grid); }

char *grid_as_str(int *grid) {
  char *str = (char *)malloc((LENGTH + 1) * sizeof(char));
  int i = 0;
  for (i = 0; i < LENGTH; i++) {
    str[i] = grid[i] + '0';
  }
  str[i] = '\0';
  return str;
}

unsigned long hash(int *grid) {
  unsigned long hash = 5381;
  for (int i = 0; i < LENGTH; i++) {
    hash = ((hash << 5) + hash) + grid[i];
  }
  return hash;
}
