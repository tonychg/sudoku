#ifndef __HASHTABLE__
#define __HASHTABLE__

#include "list.h"
#include <stdint.h>

#define GOLDEN_RATIO_64 0x61C8864680B583EBull
#define DEFAULT_CAPACITY 1001

typedef struct hashtable {
	int capacity;
	list_T **buckets;
} hashtable_T;

uint64_t hash(hashtable_T *hashtab, char *key);
hashtable_T *hashtable_create();
hashtable_T *hashtable_create_with_capacity(int capacity);
list_T *hashtable_lookup(hashtable_T *hashtab, char *key);
void hashtable_insert(hashtable_T *hashtab, char *key);
void hashtable_destroy(hashtable_T *hashtab);

#endif
