#ifndef __LIB_LIST__
#define __LIB_LIST__

#include <stdlib.h>

typedef struct list {
	void *data;
	struct list *next;
	struct list *prev;
} list_T;

#define LIST_NODE_NEW(void) (list_T *)malloc(sizeof(list_T));

list_T *list_create(void);
void list_push(list_T *head, void *data);
void list_push_tail(list_T *head, void *data);
list_T *list_pop(list_T *head);
list_T *list_pop_tail(list_T *head);
void list_iter(list_T *head, void (*callback)(int, void *));
void list_iter_reverse(list_T *head, void (*callback)(int, void *));
void list_free(list_T *head);

#endif
