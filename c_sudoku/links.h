#ifndef __SUDOKU_LINKS__
#define __SUDOKU_LINKS__

struct links {
    struct links *left;
    struct links *right;
    struct links *up;
    struct links *down;
    struct links *column;
    int row;
    int col;
    int size;
};

struct slist {
    int *grid;
    struct slist *next;
};

struct plist {
    int size;
    int solutions;
    int i;
    struct slist *s;
    struct links *p[81];
};

struct links *links_exact_cover(int width);
void links_add_nodes(struct links *head, int width, int height, int **matrix);
void links_destroy(struct links *head);
void links_dancing(struct links *head, struct plist *o, int k, int limit);
void links_dancing_non_deterministic(struct links *head, struct plist *o, int k,
                                     int limit);
void links_check(struct links *head);
void links_cover(struct links *column);
void links_uncover(struct links *column);
struct links *links_select_row(struct links *head, int index);

struct plist *partial_new();
void partial_destroy(struct plist *o);


#endif
