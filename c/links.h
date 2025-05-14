#ifndef __SUDOKU_LINKS__
#define __SUDOKU_LINKS__

#include <stdbool.h>

struct links {
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
};

struct slist {
    int *grid;
    struct slist *next;
};

struct plist {
    int size;
    int solutions;
    struct slist *s;
    struct links *p[81];
};

struct links *links_exact_cover(int width);
struct links **links_add_nodes(struct links *head, int width, int height, int **matrix);
void links_free(struct links *head);
void links_destroy(struct links *head, struct links **rows, struct links **columns);
void links_dancing(struct links *head, struct plist *o, int k, int limit,
                   int deterministic);
void links_check(struct links *head);
void links_debug(struct links *head);
void links_cover(struct links *column);
void links_cover_free(struct links *column);
void links_uncover(struct links *column);
struct links *links_select_row(struct links *head, int index);
struct links **links_columns_save(struct links *head);

struct plist *partial_new();
void partial_destroy(struct plist *o);


#endif
