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
  char *mode;
  int seed;
};

struct args_sparse {
  char *dest;
};

struct command *parse_args(int argc, char **argv);

#endif
