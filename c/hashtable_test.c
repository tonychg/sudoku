#include "hashtable.h"
#include <assert.h>
#include <stdio.h>

int main()
{
	hashtable_T *ht = hashtable_create();
	hashtable_insert(ht, "alice");
	hashtable_insert(ht, "bob");
	assert(hashtable_lookup(ht, "alice"));
	assert(hashtable_lookup(ht, "bob"));
	assert(hashtable_lookup(ht, "john") == NULL);
	hashtable_destroy(ht);
}
