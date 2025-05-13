#include "links.h"
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

// Initialize head of torus linked list as described in
// Knuth paper https://arxiv.org/pdf/cs/0011047
struct links *links_create_torus() {
  struct links *head = (struct links *)malloc(sizeof(struct links));
  head->left = head;
  head->right = head;
  head->up = head;
  head->down = head;
  head->size = 0;
  head->col = 0;
  head->row = 0;
  return head;
}

void links_destroy(struct links *head) {
  struct links *tmp;
  struct links *elem;
  for (tmp = head->right; tmp != head; tmp = tmp->right) {
    for (elem = tmp->up; elem->up != tmp; elem = elem->up) {
      free(elem);
    }
  }
  for (tmp = head->right; tmp != head; tmp = tmp->right) {
    free(tmp);
  }
}

int count_columns(struct links *head) {
  struct links *column;
  int counter = 0;
  for (column = head->right; column != head; column = column->right) {
    counter++;
  }
  return counter;
}

void debug_matrix(struct links *head) {
  struct links *header;
  struct links *tmp;
  for (header = head->right; header != head; header = header->right) {
    for (tmp = header->down; tmp != header; tmp = tmp->down) {
      printf("col=%d row=%d\n", tmp->col + 1, tmp->row + 1);
    }
  }
}

void links_check(struct links *head) {
  struct links *column, *row, *node;
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

// Add all column headers
void links_add_header(struct links *head, int width) {
  if (!head) {
    head = links_create_torus();
  }
  for (int x = 0; x < width; x++) {
    struct links *header = (struct links *)malloc(sizeof(struct links));
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
struct links *links_exact_cover(int width) {
  struct links *head = links_create_torus();
  links_add_header(head, width);
  return head;
}

void links_add_nodes(struct links *head, int width, int height, int **matrix) {
  int x, y;
  struct links *header;
  struct links *first;
  struct links *tmp;
  for (y = 0; y < height; y++) {
    for (x = 0, header = head->right, first = NULL; x < width;
         x++, header = header->right) {
      if (matrix[x][y] != 0) {
        struct links *new = (struct links *)malloc(sizeof(struct links));
        if (!header->size) {
          header->up = new;
          header->down = new;
          new->up = header;
          new->down = header;
        } else {
          struct links *last = header->up;
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
        new->column = header;
        header->size++;
      }
    }
  }
}

void links_cover(struct links *column) {
  struct links *i, *j;
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

void links_uncover(struct links *column) {
  struct links *i, *j;
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

struct links *links_select_column(struct links *head) {
  struct links *header, *selected;
  int min = 1000000000;
  for (header = head->right; header != head; header = header->right) {
    if (header->size < min) {
      selected = header;
      min = header->size;
    }
  }
  return selected;
}

struct links *links_random_column(struct links *head) {
  struct links *header;
  int random_index = rand() % head->size - 1;
  header = head->right;
  while (random_index) {
    header = header->right;
    random_index--;
  }
  return header;
}

struct links *links_select_row(struct links *head, int index) {
  struct links *column, *row, *node, *tmp;
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

void add_solution(struct plist *o) {
  struct links *tmp;
  struct slist *new = (struct slist *)malloc(sizeof(struct slist *));
  int *grid = (int *)calloc(81, sizeof(int));
  for (int i = 0; i < o->size; i++) {
    tmp = o->p[i];
    grid[tmp->row / 9] = tmp->row % 9 + 1;
  }
  new->grid = grid;
  new->next = NULL;
  if (!o->s) {
    o->s = new;
  } else {
    struct slist *p = o->s;
    while (p->next != NULL)
      p = p->next;
    p->next = new;
  }
  o->solutions++;
}

void links_dancing(struct links *head, struct plist *o, int k, int limit) {
  struct links *column, *row, *j, *ok, *r;
  o->i++;
  if (head->right == head) {
    add_solution(o);
    return;
  }
  if (o->i == 1) {
  }
  column = links_select_column(head);
  links_cover(column);
  for (row = column->down; row != column; row = row->down) {
    o->p[k] = row;
    o->size++;
    for (j = row->right; j != row; j = j->right) {
      links_cover(j->column);
    }
    links_dancing(head, o, k + 1, limit);
    row = o->p[k];
    o->p[k] = NULL;
    o->size--;
    column = row->column;
    for (j = row->left; j != row; j = j->left) {
      links_uncover(j->column);
    }
    if (limit != -1 && o->solutions == limit) {
      break;
    }
  }
  links_uncover(column);
  return;
}

void links_dancing_non_deterministic(struct links *head, struct plist *o, int k,
                                     int limit) {
  struct links *column, *row, *j, *ok, *r;
  o->i++;
  if (head->right == head) {
    add_solution(o);
    return;
  }
  column = links_random_column(head);
  links_cover(column);
  for (row = column->down; row != column; row = row->down) {
    int random_index = rand() % column->size;
    while (random_index) {
      row = row->down;
      random_index--;
    }
    if (row == column) {
      row = row->down;
    }
    o->p[k] = row;
    o->size++;
    for (j = row->right; j != row; j = j->right) {
      links_cover(j->column);
    }
    links_dancing(head, o, k + 1, limit);
    row = o->p[k];
    o->p[k] = NULL;
    o->size--;
    column = row->column;
    for (j = row->left; j != row; j = j->left) {
      links_uncover(j->column);
    }
    if (limit != -1 && o->solutions == limit) {
      break;
    }
  }
  links_uncover(column);
  return;
}

struct plist *partial_new() {
  struct plist *p = (struct plist *)malloc(sizeof(struct plist));
  p->solutions = 0;
  p->s = NULL;
  p->i = 0;
  return p;
}

void partial_destroy(struct plist *o) {
  struct slist *s;
  while (o->s != NULL) {
    s = o->s;
    o->s = o->s->next;
    free(s->grid);
    free(s);
  }
  free(o);
}
