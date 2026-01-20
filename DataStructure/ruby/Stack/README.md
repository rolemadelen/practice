## Stack

**Stack** is a linear data structure where elements can only be added or removed from one end. Because of this structure, the last item you insert is the first one removed.

Think of a stack of plates to wash. Naturally, you will grab the top one and clean it, one by one. This is Stack and we say it follows _LIFO_ (Last-In, First-Out).

스택의 기본 함수와 각 함수의 시간복잡도는 아래와 같다.
- push -> O(1)
- pop -> O(n) if implemented in array
	- O(1) in linked list implementation
- top -> O(1)
- random access -> O(n)
