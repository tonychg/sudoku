#include "random.h"
#include "board.h"

int random_index(void) { return rand() % SIZE; }

int random_seed(void) { return rand() % 2147483647; }
