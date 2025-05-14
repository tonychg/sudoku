#include "random.h"
#include "board.h"
#include <time.h>

int random_index(void)
{
	return rand() % SIZE;
}

int random_seed(void)
{
	srand(time(NULL));
	return rand();
}
