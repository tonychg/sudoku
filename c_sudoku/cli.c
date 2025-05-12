#include "cli.h"
#include "random.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct args_generate *parse_args_generate(int argc, char **argv) {
  struct args_generate *args =
      (struct args_generate *)malloc(sizeof(struct args_generate));
  args->human_readable = false;
  args->seed = random_seed();
  args->dest = NULL;
  args->mode = "dfs";
  if (argc <= 2) {
    return args;
  }
  for (int i = 2; i < argc; i++) {
    if ((!strcmp(argv[i], "--destination") || !strcmp(argv[i], "-d")) &&
        i + 1 < argc) {
      args->dest = argv[i + 1];
    }
    if (!strcmp(argv[i], "--human") || !strcmp(argv[i], "-h")) {
      args->human_readable = true;
    }
    if ((!strcmp(argv[i], "--seed") || !strcmp(argv[i], "-s")) &&
        i + 1 < argc) {
      args->seed = atoi(argv[i + 1]);
    }
    if ((!strcmp(argv[i], "--mode") || !strcmp(argv[i], "-m")) &&
        i + 1 < argc) {
      args->mode = argv[i + 1];
    }
  }
  return args;
}

struct args_sparse *parse_args_sparse(int argc, char **argv) {
  struct args_sparse *args =
      (struct args_sparse *)malloc(sizeof(struct args_sparse));
  if (argc == 2) {
    args->dest = "m.out";
  } else {
    args->dest = argv[2];
  }
  return args;
}

struct args_solve *parse_args_solve(int argc, char **argv) {
  struct args_solve *args =
      (struct args_solve *)malloc(sizeof(struct args_solve));
  args->grid = NULL;
  args->from_stdin = false;
  args->limit = -1;
  if (argc <= 2) {
    return args;
  }
  for (int i = 2; i < argc; i++) {
    if ((!strcmp(argv[i], "--grid") || !strcmp(argv[i], "-g")) &&
        i + 1 < argc) {
      args->grid = argv[i + 1];
    }
    if (!strcmp(argv[i], "--stdin") || !strcmp(argv[i], "-s")) {
      args->from_stdin = true;
    }
    if (!strcmp(argv[i], "--limit") || !strcmp(argv[i], "-l")) {
      args->limit = atoi(argv[i + 1]);
    }
  }
  return args;
}

struct command *parse_command(int argc, char **argv) {
  struct command *cmd = (struct command *)malloc(sizeof(struct command));
  cmd->name = argv[1];
  if (!strcmp(cmd->name, "sparse")) {
    cmd->args = parse_args_sparse(argc, argv);
  }
  if (!strcmp(cmd->name, "generate")) {
    cmd->args = parse_args_generate(argc, argv);
  }
  if (!strcmp(cmd->name, "solve")) {
    cmd->args = parse_args_solve(argc, argv);
  }
  return cmd;
}

struct command *parse_args(int argc, char **argv) {
  if (argc < 2) {
    printf(
        "Usage: sudoku [sparse,generate,solve] [-d/--destination] [-h/--human] "
        "[-m/--mode] [-s/--stdin] [-g/--grid]\n");
    return NULL;
  } else {
    return parse_command(argc, argv);
  }
}
