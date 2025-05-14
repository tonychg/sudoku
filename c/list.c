#include "list.h"
#include <stdio.h>
#include <stdlib.h>

/*
 * Print the list
 */
void list_print(list_t *list)
{
	int i = 0;
	list_node_t *elem = list->first;
	if (!elem) {
		return;
	}
	while (1) {
		printf("i=%d data=%p\n", i, &elem->data);
		elem = elem->next;
		i++;
		if (elem == list->first)
			break;
	}
}

void list_print_reverse(list_t *list)
{
	int i = 0;
	list_node_t *elem = list->first->prev;
	if (!elem) {
		return;
	}
	while (1) {
		printf("i=%d data=%p\n", i, &elem->data);
		elem = elem->prev;
		i++;
		if (elem == list->first->prev)
			break;
	}
}

/*
 * Initialize the list
 */
list_t *list_new(void)
{
	list_t *node = (list_t *)malloc(sizeof(list_t));
	node->size = 0;
	node->first = NULL;
	return node;
}

/*
 * Create the list and push the first element
 */
void list_init(list_t *list, void *data)
{
	list->first = (list_node_t *)malloc(sizeof(list_node_t));
	list->first->prev = list->first;
	list->first->next = list->first;
	list->first->data = (size_t)data;
	list->size++;
}

/*
 * Push an element at the beginning of the list
 */
void list_push(list_t *list, void *data)
{
	if (!list->first) {
		list_init(list, data);
	} else {
		list_node_t *node = (list_node_t *)malloc(sizeof(list_node_t));
		node->prev = list->first->prev;
		node->next = list->first;
		node->data = (size_t)data;
		if (list->size > 1) {
			list->first->prev->next = node;
		} else {
			list->first->next = node;
		}
		list->first->prev = node;
		list->first = node;
		list->size++;
	}
}

/*
 * Insert an element at the end of the list
 */
void list_insert(list_t *list, void *data)
{
	if (!list->first) {
		list_init(list, data);
	} else {
		list_node_t *node = (list_node_t *)malloc(sizeof(list_node_t));
		list_node_t *last = list->first->prev;
		last->next = node;
		node->prev = last;
		node->next = list->first;
		list->first->prev = node;
		node->data = (size_t)data;
		list->size++;
	}
}

/*
 * Pop the first element of the list
 */
size_t list_pop(list_t *list)
{
	if (!list->first)
		return -1;
	list_node_t *head = list->first;
	size_t data = head->data;
	if (list->size > 1) {
		list_node_t *last = list->first->prev;
		list_node_t *next = list->first->next;
		next->prev = last;
		last->next = next;
		list->first = next;
		list->size--;
		free(head);
	} else {
		list->first = NULL;
		list->size--;
		free(head);
	}
	return data;
}

/*
 * Remove a specific element in the list
 */
void list_del(list_t *list, void *data)
{
	if (!list->first) {
		return;
	}
	list_node_t *elem = list->first;
	while (elem->next != list->first) {
		if (elem->data == (size_t)data) {
			break;
		}
		elem = elem->next;
	}
	elem->prev->next = elem->next;
	elem->next->prev = elem->prev;
	list->size--;
	free(elem);
}
