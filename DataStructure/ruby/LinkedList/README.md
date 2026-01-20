## Linked List

Linked list is a linear data structure where each element, called node, is connected via pointers. In other words, it's a structure composed of group of nodes.

A typical node in a linked list looks like this.

```c
struct Node {
	T data;
	Node *next;
}
```

This structure may change depending on the type of a linked list. The above structure is for Singly Linked List　which contains a single pointer called `next`. <br />
This pointer is used to store the address of a next node.

There's an additional pointer called `prev` (previous) in Doubly linked List, because it supports backward traversal.

```c
struct Node {
	T data;
	Node *next;
	Node *prev;
}
```

If backward traversal is necessary for your solution, use doubly linked lists. Note that it will consume more memory than a singly linked list due to extra pointer in the node.

Linked lists are generally better off if lots of insertions and deletions are required because it can be done in constant time.

If look-up operations are used more frequently, then it'd be better to use other data structures.