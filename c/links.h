#ifndef __SUDOKU_LINKS__
#define __SUDOKU_LINKS__

#include "list.h"
#include <stdbool.h>

typedef struct links {
    struct links *left;
    struct links *right;
    struct links *up;
    struct links *down;
    struct links *column;
    int row;
    int col;
    int size;
    int indice;
    int n;
} links_T;

typedef struct plist {
    int size;
    int solutions;
    list_T *grids;
    links_T *p[81];
} plist_T;

links_T *links_exact_cover(int width);
links_T **links_add_nodes(links_T *head, int width, int height, int **matrix);
void links_free(links_T *head);
void links_destroy(links_T *head, links_T **rows, links_T **columns);
void links_dancing(links_T *head, plist_T *o, int k, int limit, int deterministic);
void links_check(links_T *head);
void links_debug(links_T *head);
void links_cover(links_T *column);
void links_uncover(links_T *column);
links_T *links_select_row(links_T *head, int index);
links_T **links_columns_save(links_T *head);

plist_T *partial_new();
void partial_destroy(plist_T *o);


#endif
