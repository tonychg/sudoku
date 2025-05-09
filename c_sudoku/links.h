#ifndef __SUDOKU_LINKS__
#define __SUDOKU_LINKS__

typedef struct links {
    struct links *left;
    struct links *right;
    struct links *up;
    struct links *down;
    struct links *column;
    int row;
    int col;
    int size;
} links_t;

typedef struct matrix {
    links_t *head;
} matrix_t;

#endif
