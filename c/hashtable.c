#include "hashtable.h"
#include <stdlib.h>
#include <string.h>

hashtable_T *hashtable_init(int capacity)
{
	hashtable_T *hashtab;
	hashtab = (hashtable_T *)malloc(sizeof(hashtable_T));
	hashtab->capacity = capacity;
	hashtab->buckets =
		(list_T **)malloc(sizeof(list_T *) * hashtab->capacity);
	for (int i = 0; i < hashtab->capacity; i++)
		hashtab->buckets[i] = list_create();
	return hashtab;
}

uint64_t hash(hashtable_T *hashtab, char *key)
{
	uint64_t value;
	for (value = 0; *key; key++)
		value = *key + GOLDEN_RATIO_64 * value;
	return value % hashtab->capacity;
}

hashtable_T *hashtable_create()
{
	return hashtable_init(DEFAULT_CAPACITY);
}

hashtable_T *hashtable_create_with_capacity(int capacity)
{
	return hashtable_init(capacity);
}

list_T *hashtable_lookup(hashtable_T *hashtab, char *key)
{
	list_T *bucket = hashtab->buckets[hash(hashtab, key)];
	for (list_T *tmp = bucket->next; tmp != bucket; tmp = tmp->next)
		if (strcmp(key, (char *)tmp->data) == 0)
			return tmp;
	return NULL;
}

void hashtable_insert(hashtable_T *hashtab, char *key)
{
	if (!hashtable_lookup(hashtab, key)) {
		list_T *bucket = hashtab->buckets[hash(hashtab, key)];
		list_push(bucket, key);
	}
}

void hashtable_destroy(hashtable_T *hashtab)
{
	for (int i = 0; i < hashtab->capacity; i++)
		list_free(hashtab->buckets[i]);
	free(hashtab->buckets);
	free(hashtab);
}
