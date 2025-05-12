#include "cli.h"
#include "sudoku.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

void command_generate(struct args_generate *args) {
  srand(args->seed);
  board_t *b = board_init();
  board_t *filled;
  if (!strcmp(args->mode, "dfs")) {
    filled = board_backtracking(b);
  } else if (!strcmp(args->mode, "recursive")) {
    board_backtracking_recursive(filled);
  }
  if (filled) {
    if (!args->dest && args->human_readable) {
      board_pretty_print(filled);
    } else if (!args->dest && !args->human_readable) {
      printf("%d:", args->seed);
      board_print(filled);
    } else if (args->dest) {
      board_write(filled, args->seed, args->dest);
    }
  }
  free(args);
}

void command_sparse(struct args_sparse *args) {
  int **matrix = sudoku_sparse_create();
  sudoku_sparse_write(matrix, args->dest);
  free(args);
}

void command_solve(struct args_solve *args) {
  int *grid;
  if (args->from_stdin) {
    grid = sudoku_grid_stdin();
  } else {
    grid = sudoku_grid_from_str(args->grid);
  }
  sudoku_grid_print(grid, NULL);
  sudoku_solve(grid, args->limit);
  free(grid);
}

void command_run(struct command *cmd) {
  if (!strcmp(cmd->name, "generate")) {
    command_generate(cmd->args);
  }
  if (!strcmp(cmd->name, "sparse")) {
    command_sparse(cmd->args);
  }
  if (!strcmp(cmd->name, "solve")) {
    command_solve(cmd->args);
  }
  free(cmd);
}

int main(int argc, char **argv) {
  struct command *cmd = parse_args(argc, argv);
  if (cmd) {
    command_run(cmd);
  }
  exit(0);
}
