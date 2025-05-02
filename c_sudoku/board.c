#include "board.h"
#include <stdio.h>
#include <stdlib.h>

void b_print(Board *board) {
  int y = 0;
  int x = 0;

  for (y = 0; y < SIZE; y++) {
    for (x = 0; x < SIZE; x++) {
      printf("%d", *board[y][x]);
    }
    printf("\n");
  }
}

void b_print_grid(Board *board) {
  char line[] = "+---+---+---+";

  for (int y = 0; y < SIZE; y++) {
    if (y == 0) {
      printf("%s\n", line);
    }
    for (int x = 0; x < SIZE; x++) {
      char num_char = *board[y][x] + '0';
      if (*board[y][x] == 0) {
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

void b_debug(Board *board, int iteration) {
  char *b_str = b_string(board);
  printf("%4d: %s\n", iteration, b_str);
  b_print_grid(board);
  free(b_str);
}

char *b_string(Board *board) {
  int y = 0;
  int x = 0;
  int i = 0;
  char *buffer;

  buffer = malloc(sizeof(char) * (SIZE * SIZE));
  if (buffer == NULL) {
    return NULL;
  }
  for (y = 0; y < SIZE; y++) {
    for (x = 0; x < SIZE; x++) {
      buffer[i] = *board[y][x] + '0';
      i++;
    }
  }
  return buffer;
}

Board *b_clone(Board *board) {
  int y = 0;
  int x = 0;
  static Board copy = {0};

  for (y = 0; y < SIZE; y++) {
    for (x = 0; x < SIZE; x++) {
      copy[y][x] = *board[y][x];
    }
  }
  return &copy;
}

Board *b_init(void) {
  int y = 0;
  int x = 0;
  static Board board = {0};

  for (y = 0; y < SIZE; y++) {
    for (x = 0; x < SIZE; x++) {
      board[y][x] = 0;
    }
  }
  return &board;
}

bool can_be_place(Board *board, int x, int y, int number) {
  int i = 0;
  int first_x = x / TIER * TIER;
  int first_y = y / TIER * TIER;

  for (i = 0; i < SIZE; i++) {
    if (*board[y][i] == number) {
      return false;
    }
    if (*board[i][x] == number) {
      return false;
    }
    if (*board[first_y + i / TIER][first_x + i % TIER] == number) {
      return false;
    }
  }
  return true;
}

bool is_complete(Board *board) {
  for (int y = 0; y < SIZE; y++) {
    for (int x = 0; x < SIZE; x++) {
      if (*board[y][x] == 0) {
        return false;
      }
    }
  }
  return true;
}
