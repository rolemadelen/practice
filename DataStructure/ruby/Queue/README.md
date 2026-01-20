## Queue

**Queue** is another linear data structure where insertion happens at one end, but deletion happens in the other end.

Think of people lining up waiting to pay for their groceries in a market. If you're the first one in the line, you'll pay for your groceries first. That's Queue; first come, first served. But instead, we say _FIFO_ (First-In, First-Out).

큐의 기본 함수와 시간복잡도는 아래와 같다.
- enqueue -> O(1)
- dequeue -> O(n) in array implementation
	- linked list implementation -> O(1)
- front -> O(1)
- rear -> O(1)
- random access -> O(n)