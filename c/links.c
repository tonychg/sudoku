#include "links.h"
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

// Initialize head of torus linked list as described in
// Knuth paper https://arxiv.org/pdf/cs/0011047
links_T *links_create_torus()
{
	links_T *head = (links_T *)malloc(sizeof(links_T));
	head->left = head;
	head->right = head;
	head->up = head;
	head->down = head;
	head->size = 0;
	head->col = 0;
	head->row = 0;
	return head;
}

void links_free(links_T *head)
{
	int col = 0;
	int counter = 0;
	links_T *header = head->right;
	while (header != head) {
		links_T *row = header->down;
		int row_count = 0;
		while (row != header) {
			links_T *tmp = row->down;
			free(row);
			row = tmp;
			row_count++;
		}
		counter += row_count;
		links_T *column = header->right;
		free(header);
		header = column;
		col++;
	}
	free(head);
}

void links_destroy(links_T *head, links_T **rows, links_T **columns)
{
	int i, j;
	for (i = 0; rows[i]; i++)
		free(rows[i]);
	free(rows);
	for (j = 0; columns[j]; j++)
		free(columns[j]);
	free(columns);
	free(head);
}

links_T **links_columns_save(links_T *head)
{
	int i = 0;
	links_T *header = head->right;
	links_T **columns = (links_T **)malloc(sizeof(links_T *) * head->size);
	while (header != head) {
		columns[i] = header;
		header = header->right;
		i++;
	}
	columns[i] = NULL;
	return columns;
}

int count_columns(links_T *head)
{
	links_T *column;
	int counter = 0;
	for (column = head->right; column != head; column = column->right) {
		counter++;
	}
	return counter;
}

void debug_matrix(links_T *head)
{
	links_T *header;
	links_T *tmp;
	for (header = head->right; header != head; header = header->right) {
		for (tmp = header->down; tmp != header; tmp = tmp->down) {
			printf("col=%d row=%d\n", tmp->col + 1, tmp->row + 1);
		}
	}
}

void links_check(links_T *head)
{
	links_T *column, *row, *node;
	for (column = head->right; column != head; column = column->right) {
		printf("C%d Size:%d\n", column->col, column->size);
		for (row = column->down; row != column; row = row->down) {
			node = row;
			do {
				printf("R%dC%d", node->row, node->col);
				if (node->right != row) {
					printf(" -> ");
				}
				node = node->right;
			} while (node != row);
			printf("\n");
		}
	}
}

void links_debug(links_T *head)
{
	links_T *column, *row;
	int columns = 0;
	int total_rows = 0;
	for (column = head->right; column != head; column = column->right) {
		if (!column->right) {
			printf("Corrupted links column[%p]->right %p\n", column,
			       column->right);
			continue;
		}
		if (!column->left) {
			printf("Corrupted links column[%p]->left %p\n", column,
			       column->left);
			continue;
		}
		if (!column->up) {
			printf("Corrupted links column[%p]->up %p\n", column,
			       column->up);
			continue;
		}
		if (!column->down) {
			printf("Corrupted links column[%p]->down %p\n", column,
			       column->down);
			continue;
		}
		int rows = 0;
		for (row = column->down; row != column; row = row->down) {
			if (!row->right) {
				printf("Corrupted links row[%p]->right %p\n",
				       row, row->right);
				continue;
			}
			if (!row->left) {
				printf("Corrupted links row[%p]->left %p\n",
				       row, row->left);
				continue;
			}
			if (!row->up) {
				printf("Corrupted links row[%p]->up %p\n", row,
				       row->up);
				continue;
			}
			if (!row->down) {
				printf("Corrupted links row[%p]->down %p\n",
				       row, row->down);
				continue;
			}
			if (!row->column) {
				printf("Corrupted links row[%p]->column %p\n",
				       row, row->column);
				continue;
			}
			rows++;
		}
		total_rows += rows;
		columns++;
	}
	printf("Columns=%d Rows=%d\n", columns, total_rows);
}

// Add all column headers
void links_add_header(links_T *head, int width)
{
	int x;
	if (!head) {
		head = links_create_torus();
	}
	for (x = 0; x < width; x++) {
		links_T *header = (links_T *)malloc(sizeof(links_T));
		if (!x) {
			header->right = head;
			header->left = head;
			head->right = header;
			head->left = header;
		} else {
			header->right = head->right;
			header->left = head;
			head->right->left = header;
			head->right = header;
		}
		header->down = header;
		header->up = header;
		header->size = 0;
		header->col = (width - 1) - x;
		header->row = 0;
		head->size++;
	}
}

// Construct the torus associated with the exact cover of a 9x9 Sudoku
links_T *links_exact_cover(int width)
{
	links_T *head = links_create_torus();
	links_add_header(head, width);
	return head;
}

links_T **links_add_nodes(links_T *head, int width, int height, int **matrix)
{
	int x, y, i;
	links_T *header;
	links_T *first;
	links_T *tmp;
	links_T **rows =
		(links_T **)malloc(sizeof(links_T *) * (width * height));
	for (y = 0, i = 0; y < height; y++) {
		for (x = 0, header = head->right, first = NULL; x < width;
		     x++, header = header->right) {
			if (matrix[x][y] != 0) {
				links_T *new =
					(links_T *)malloc(sizeof(links_T));
				if (!header->size) {
					header->up = new;
					header->down = new;
					new->up = header;
					new->down = header;
				} else {
					links_T *last = header->up;
					last->down = new;
					header->up = new;
					new->down = header;
					new->up = last;
				}
				if (first == NULL) {
					new->right = new;
					new->left = new;
					first = new;
				} else {
					tmp = first->left;
					first->left = new;
					new->left = tmp;
					tmp->right = new;
					new->right = first;
				}
				new->col = x;
				new->row = y;
				new->indice = y / 9;
				new->n = y % 9;
				new->column = header;
				header->size++;
				rows[i] = new;
				i++;
			}
		}
	}
	rows[i] = NULL;
	rows = (links_T **)realloc(rows, i * sizeof(links_T *));
	return rows;
}

void links_cover(links_T *column)
{
	links_T *i, *j;
	for (i = column->down; i != column; i = i->down) {
		for (j = i->right; j != i; j = j->right) {
			j->down->up = j->up;
			j->up->down = j->down;
			j->column->size--;
		}
	}
	column->right->left = column->left;
	column->left->right = column->right;
}

void links_uncover(links_T *column)
{
	links_T *i, *j;
	for (i = column->up; i != column; i = i->up) {
		for (j = i->left; i != j; j = j->left) {
			j->column->size++;
			j->down->up = j;
			j->up->down = j;
		}
	}
	column->left->right = column;
	column->right->left = column;
}

links_T *links_select_column(links_T *head)
{
	links_T *header, *selected;
	int min = 1000000000;
	header = head->right;
	selected = header;
	while (header != head) {
		if (header->size < min) {
			min = header->size;
			selected = header;
		}
		header = header->right;
	}
	return selected;
}

links_T *links_random_column(links_T *head)
{
	links_T *header;
	int random_index = rand() % head->size - 1;
	header = head->right;
	while (random_index > 0) {
		header = header->right;
		random_index--;
	}
	return header;
}

links_T *links_select_row(links_T *head, int index)
{
	links_T *column, *row, *node;
	for (column = head->right; column != head; column = column->right) {
		for (row = column->down; row != column; row = row->down) {
			if (row->row == index) {
				node = row;
				do {
					links_cover(node->column);
					node = node->right;
				} while (node != row);
				break;
			}
		}
	}
	return row;
}

int *rebuild_grid(plist_T *o)
{
	links_T *tmp;
	int *grid = (int *)calloc(81, sizeof(int));
	for (int i = 0; i < o->size; i++) {
		tmp = o->p[i];
		if (tmp && tmp->indice < 81) {
			grid[tmp->indice] = tmp->n + 1;
		}
	}
	return grid;
}

void links_dancing(links_T *head, plist_T *o, int k, int limit,
		   int determinisic)
{
	links_T *column, *row, *j;
	if (head->right == head) {
		int *grid = rebuild_grid(o);
		list_push_tail(o->grids, grid);
		o->solutions++;
	}
	if (!determinisic && k < 1) {
		column = links_random_column(head);
	} else {
		column = links_select_column(head);
	}
	links_cover(column);
	for (row = column->down; row != column; row = row->down) {
		if (!determinisic && column->size > 0) {
			int random_index = rand() % column->size;
			while (random_index) {
				row = row->down;
				random_index--;
			}
			if (row == column)
				row = row->down;
		}
		o->p[k] = row;
		if (o->p[k]) {
			o->size++;
		}
		for (j = row->right; j != row; j = j->right) {
			links_cover(j->column);
		}
		if (limit == -1 || o->solutions < limit) {
			links_dancing(head, o, k + 1, limit, determinisic);
		}
		row = o->p[k];
		if (o->p[k]) {
			o->size--;
		}
		o->p[k] = NULL;
		column = row->column;
		for (j = row->left; j != row; j = j->left) {
			links_uncover(j->column);
		}
	}
	links_uncover(column);
	return;
}

plist_T *partial_new()
{
	plist_T *p = (plist_T *)malloc(sizeof(plist_T));
	p->solutions = 0;
	p->grids = list_create();
	p->size = 0;
	return p;
}

void partial_destroy(plist_T *o)
{
	list_T *next;
	list_T *tmp = o->grids->next;
	while (tmp != o->grids) {
		next = tmp->next;
		if (tmp->data)
			free(tmp->data);
		free(tmp);
		tmp = next;
	}
	if (tmp == o->grids->next) {
		free(o->grids->data);
	}
	free(o->grids);
	free(o);
}
