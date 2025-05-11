#include "links.h"
// #include "string.h"
#include "random.h"
#include "sudoku.h"
#include <stdio.h>
#include <stdlib.h>

void debug_column(struct links *head, int x) {
  int i;
  struct links *header;
  struct links *elem;
  for (i = 0, header = head->right; i < x; i++, header = header->right)
    ;
  for (elem = header->down; elem != header; elem = elem->down)
    printf("row=%d col=%d\n", elem->row, elem->col);
}

void debug_matrix_y(struct links *head) {
  struct links *header;
  struct links *tmp;
  for (header = head->right; header != head; header = header->right) {
    for (tmp = header->down; tmp != header; tmp = tmp->down) {
      printf("col=%d row=%d\n", tmp->col + 1, tmp->row + 1);
    }
  }
}

void debug_matrix_x(struct links *head) {
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

void debug_headers(struct links *head) {
  struct links *h;
  int column = 0;
  for (h = head->right; h != head; h = h->right) {
    printf("col=%d size=%d\n", column, h->size);
    column++;
  }
}

int **create_test_matrix_1() {
  // Solution must be rows=1,3,5
  int **matrix = (int **)malloc(sizeof(int *) * 7);
  for (int i = 0; i < 7; i++) {
    matrix[i] = (int *)calloc(6, sizeof(int));
  }
  matrix[0][0] = 1;
  matrix[0][1] = 1;
  matrix[1][4] = 1;
  matrix[1][5] = 1;
  matrix[2][3] = 1;
  matrix[2][4] = 1;
  matrix[3][0] = 1;
  matrix[3][1] = 1;
  matrix[3][2] = 1;
  matrix[4][2] = 1;
  matrix[4][3] = 1;
  matrix[5][3] = 1;
  matrix[5][4] = 1;
  matrix[6][0] = 1;
  matrix[6][2] = 1;
  matrix[6][4] = 1;
  matrix[6][5] = 1;
  for (int j = 0; j < 6; j++) {
    for (int i = 0; i < 7; i++) {
      if (!i && !j) {
        printf("  ");
        for (int x = 0; x < 7; x++) {
          printf("%d", x);
        }
        printf("\n");
      }
      if (!i) {
        printf("%d ", j);
      }
      printf("%d", matrix[i][j]);
    }
    printf("\n");
  }
  printf("Solution must be rows=1,3,5\n");
  return matrix;
}

int **create_test_matrix_2() {
  // Solution must be rows=0,3,4
  int **matrix = (int **)malloc(sizeof(int *) * 7);
  for (int i = 0; i < 7; i++) {
    matrix[i] = (int *)calloc(6, sizeof(int));
  }
  matrix[0][1] = 1;
  matrix[0][3] = 1;
  matrix[1][2] = 1;
  matrix[1][4] = 1;
  matrix[2][0] = 1;
  matrix[2][2] = 1;
  matrix[3][1] = 1;
  matrix[3][3] = 1;
  matrix[3][5] = 1;
  matrix[4][0] = 1;
  matrix[4][5] = 1;
  matrix[5][0] = 1;
  matrix[5][2] = 1;
  // matrix[5][5] = 1;
  matrix[6][1] = 1;
  matrix[6][4] = 1;
  matrix[6][5] = 1;
  for (int j = 0; j < 6; j++) {
    for (int i = 0; i < 7; i++) {
      if (!i && !j) {
        printf("  ");
        for (int x = 0; x < 7; x++) {
          printf("%d", x);
        }
        printf("\n");
      }
      if (!i) {
        printf("%d ", j);
      }
      printf("%d", matrix[i][j]);
    }
    printf("\n");
  }
  printf("Solution must be rows=0,3,4\n");
  return matrix;
}

int main() {
  int **matrix = sudoku_sparse_create();
  int seed = random_seed();
  srand(seed);
  // int **matrix = create_test_matrix_1();
  struct links *head = links_exact_cover(MAX_WIDTH);
  struct plist *o = partial_new();
  int *solutions = (int *)malloc(sizeof(int));
  (*solutions) = 0;
  printf("Add nodes\n");
  links_add_nodes(head, MAX_WIDTH, MAX_HEIGHT, matrix);
  // links_check(head);
  printf("##### Start dancing #####\n");
  links_dancing(head, o, 0, 100000);
  links_destroy(head);
}
