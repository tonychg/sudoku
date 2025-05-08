#include "random.h"
#include "grid.h"
#include <time.h>

int random_index(void) { return rand() % SIZE; }

int random_seed(void) { return time(NULL); }
