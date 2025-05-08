#ifndef __SUDOKU_LIST__
#define __SUDOKU_LIST__

#include <stddef.h>

typedef struct list_node {
    struct list_node *prev;
    struct list_node *next;
    size_t data;
} list_node_t;

typedef struct list {
    struct list_node *first;
    unsigned long size;
} list_t;

list_t *list_new(void);
void list_push(list_t *list, void *data);
void list_insert(list_t *list, void *data);
void list_del(list_t *list, void *data);
void list_print(list_t *list);
void list_print_reverse(list_t *list);
size_t *list_pop(list_t *list);

#endif
