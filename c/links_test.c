#include "links.h"
#include "random.h"
#include "sudoku.h"
#include <stdio.h>

void debug_column(struct links *head, int x)
{
	int i;
	struct links *header;
	struct links *elem;
	for (i = 0, header = head->right; i < x; i++, header = header->right)
		;
	for (elem = header->down; elem != header; elem = elem->down)
		printf("row=%d col=%d\n", elem->row, elem->col);
}

void debug_matrix_y(struct links *head)
{
	struct links *header;
	struct links *tmp;
	for (header = head->right; header != head; header = header->right) {
		for (tmp = header->down; tmp != header; tmp = tmp->down) {
			printf("col=%d row=%d\n", tmp->col + 1, tmp->row + 1);
		}
	}
}

void debug_matrix_x(struct links *head)
{
	struct links *header, *tmp, *last;
	for (header = head->right; header != head; header = header->right) {
		last = header->down->left;
		tmp = last->right;
		do {
			printf("row=%d col=%d\n", tmp->row + 1, tmp->col + 1);
			tmp = tmp->right;
		} while (tmp != last->right);
	}
}

void debug_headers(struct links *head)
{
	struct links *h;
	int column = 0;
	for (h = head->right; h != head; h = h->right) {
		printf("col=%d size=%d\n", column, h->size);
		column++;
	}
}

int main()
{
	int **matrix = sudoku_sparse_create(NULL);
	struct links *head = links_exact_cover(MAX_WIDTH);
	struct plist *o = partial_new();
	links_add_nodes(head, MAX_WIDTH, MAX_HEIGHT, matrix);
}
