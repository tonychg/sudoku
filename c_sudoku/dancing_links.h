#ifndef __SUDOKU_DANCING_LINKS__
#define __SUDOKU_DANCING_LINKS__

typedef struct dl_node {
    struct dl_node *left;
    struct dl_node *right;
    struct dl_node *up;
    struct dl_node *down;
    struct dl_node *column;
    int row;
    int col;
    int size;
} dl_node_t;

typedef struct dancing_links {
    struct dl_node *header;
} dancing_links_t;

#endif
