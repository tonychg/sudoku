#include "list.h"

list_T *list_create(void)
{
	list_T *node = LIST_NODE_NEW();
	node->next = node;
	node->prev = node;
	node->data = NULL;
	return node;
}

void list_push(list_T *head, void *data)
{
	list_T *node = LIST_NODE_NEW();
	node->data = data;
	node->next = head->next;
	node->prev = head;
	head->next->prev = node;
	head->next = node;
}

void list_push_tail(list_T *head, void *data)
{
	list_T *node = LIST_NODE_NEW();
	node->data = data;
	node->prev = head->prev;
	node->next = head;
	node->prev->next = node;
	head->prev = node;
}

list_T *list_pop(list_T *head)
{
	list_T *node = head->next;
	head->next = node->next;
	node->next->prev = head;
	return node;
}

list_T *list_pop_tail(list_T *head)
{
	list_T *node = head->prev;
	head->prev = node->prev;
	node->prev->next = head;
	return node;
}

bool list_empty(list_T *head)
{
	return head->next == head;
}

void list_iter(list_T *head, void (*callback)(int, void *))
{
	int i = 0;
	list_T *tmp = head->next;
	while (tmp != head) {
		callback(i, tmp->data);
		tmp = tmp->next;
		i++;
	}
}

void list_iter_reverse(list_T *head, void (*callback)(int, void *))
{
	int i = 0;
	list_T *tmp = head->prev;
	while (tmp != head) {
		callback(i, tmp->data);
		tmp = tmp->prev;
		i++;
	}
}

list_iterator_T *list_iter_new(list_T *head)
{
	list_iterator_T *it;
	it = (list_iterator_T *)malloc(sizeof(list_iterator_T));
	it->index = 0;
	it->current = head;
	it->head = head;
	return it;
}

list_T *list_iter_next(list_iterator_T *it)
{
	it->current = it->current->next;
	it->index++;
	if (it->current == it->head)
		return NULL;
	return it->current;
}

void list_free(list_T *head)
{
	list_T *next;
	list_T *tmp = head->next;
	while (tmp != head) {
		next = tmp->next;
		free(tmp);
		tmp = next;
	}
	free(head);
}
