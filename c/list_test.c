#include "list.h"
#include <stdio.h>

typedef struct tile {
	int x;
	int y;
} tile_T;

list_T *create_grid(int size)
{
	int i, length = size * size;
	list_T *head = list_create();
	for (i = 0; i < length; i++) {
		tile_T *p = (tile_T *)malloc(sizeof(tile_T));
		p->x = i % size;
		p->y = i / size;
		list_push(head, p);
	}
	return head;
}

void print_tile_callback(int i, void *data)
{
	tile_T *p = (tile_T *)data;
	printf("%2d (%d,%d)\n", i, p->x, p->y);
}

void print_tile(void *data)
{
	tile_T *p = (tile_T *)data;
	printf("(%d,%d)\n", p->x, p->y);
}

int main()
{
	list_T *head = create_grid(9);

	printf("Print positions in order\n");
	list_iter(head, print_tile_callback);

	printf("Print positions in reverse order\n");
	list_iter_reverse(head, print_tile_callback);

	list_iterator_T *it = list_iter_new(head);
	list_T *elem;

	while ((elem = list_iter_next(it)))
		print_tile(elem->data);
	free(it);

	printf("Pop positions from the list\n");
	while (head->next != head) {
		list_T *tmp = list_pop(head);
		print_tile(tmp->data);
		free(tmp->data);
		free(tmp);
	}

	printf("Freeing uneeded resources\n");
	list_free(head);
}
