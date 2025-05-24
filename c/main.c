#include "cli.h"
#include "sudoku.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

void command_generate(struct args_generate *args)
{
	srand(args->seed);
	sudoku_generate(args->clues, args->human_readable);
}

void command_sparse(struct args_sparse *args)
{
	int **matrix = sudoku_sparse_create();
	sudoku_sparse_write(matrix, args->dest);
	free(args);
}

void command_solve(struct args_solve *args)
{
	char *grid;
	if (args->from_stdin) {
		grid = sudoku_grid_stdin();
	} else {
		grid = args->grid;
	}
	sudoku_grid_print(grid, NULL);
	sudoku_solve(grid, args->limit);
}

void command_run(struct command *cmd)
{
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

int main(int argc, char **argv)
{
	struct command *cmd = parse_args(argc, argv);
	if (cmd) {
		command_run(cmd);
	}
	exit(0);
}
