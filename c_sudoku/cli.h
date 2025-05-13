#ifndef __SUDOKU_CLI__
#define __SUDOKU_CLI__

#include <stdbool.h>

struct command {
  char *name;
  void *args;
};

struct args_generate {
  char *dest;
  bool human_readable;
  int seed;
  int clues;
};

struct args_sparse {
  char *dest;
};

struct args_solve {
  bool from_stdin;
  int limit;
  char *grid;
};

struct command *parse_args(int argc, char **argv);

#endif
