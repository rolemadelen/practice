#include <iostream>

using namespace std;

class Node {
    public:
        int data;
        Node* next;
        Node* prev;

        Node(int val) : data(val), next(nullptr), prev(nullptr) {}
};

class DoublyLinkedList {
    private:
        Node *head;
        Node *tail;
        int size;

    public:
        DoublyLinkedList() : head(nullptr), tail(nullptr), size(0) {}

        void pushBack(int data) {
            Node *newNode = new Node(data);

            if (size == 0) {
                head = tail = newNode;
                head->next = tail;
                tail->prev = head;
                tail->next = nullptr;
            } else {   
                newNode->prev = tail;
                tail->next = newNode;
                tail = tail->next;
            }

            ++size;
        }

        void pushFront(int data) {
            Node *newNode = new Node(data);

            if (size == 0) {
                head = tail = newNode;
                head->next = tail;
                tail->prev = head;
                tail->next = nullptr;
            } else {   
                newNode->next = head;
                head->prev = newNode;
                head = newNode;
            }

            ++size;
        }
        void popBack() {
            if (size == 0) {
                cerr << "List is empty" << endl;
                return;
            }

            if (size == 1) {
                delete tail;
                tail = head = nullptr;
                size = 0;
            } else {
                tail = tail->prev;
                delete tail->next;
                tail->next = nullptr;
                --size;
            }
        }

        void popFront() {
            if (size == 0) {
                cerr << "List is empty" << endl;
                return;
            }

            if (size == 1) {
                delete head;
                head = tail = nullptr;
                size = 0;
            } else {
                head = head->next;
                delete head->prev;
                head->prev = nullptr;
                --size;
            }
        }

        Node* getNthFromHead(int n) {
            if (n > size) {
                cerr << "invalid value of N";
                return nullptr;
            }

            Node *curr = head;

            for (int i=1; i<n; ++i) {
                curr = curr->next;
            }

            return curr;
        }

        Node* getNthFromTail(int n) {
            if (n > size) {
                cerr << "invalid value of N";
                return nullptr;
            }

            Node *curr = tail;

            for (int i=1; i<n; ++i) {
                curr = curr->prev;
            }

            return curr;
        }

        void insert(int data, int pos) {
            if (pos < 1 || pos > size+1) {
                cerr << "invalid value" << endl;
                return;
            }

            if (pos == 1) {
                pushFront(data);
            } else if (pos == size+1) {
                pushBack(data);
            } else {
                Node *curr = getNthFromHead(pos);
                Node *newNode = new Node(data);
                curr->prev->next = newNode;
                newNode->prev = curr->prev;
                curr->prev = newNode;
                newNode->next = curr;
                ++size;
            }
        }

        void deleteAt(int pos) {
            if (pos < 1 || pos > size+1) {
                cerr << "invalid value" << endl;
                return;
            }

            if (pos == 1) {
                popFront();
            } else if (pos == size+1) {
                popBack();
            } else {
                Node *curr = getNthFromHead(pos);
                curr->prev->next = curr->next;
                curr->next->prev = curr->prev;
                delete curr;
                --size;
            }
        }
        
        void deleteKey(int data) {
            if (!isExist(data)) {
                cerr << "Key not found" << endl;
                return;
            }

            int pos = 1;
            Node *curr = head;
            while(curr) {
                if (curr->data == data) {
                    deleteAt(pos);
                    --size;
                    return;
                }
                curr = curr->next;
                ++pos;
            }
        }

        bool isExist(int data) {
            Node *curr = head;

            while (curr) {
                if (curr->data == data) {
                    return true;
                }
                curr = curr->next;
            }

            return false;
        }

        bool isEmpty() {
            return size == 0;
        }

        int getSize() {
            return size;
        }

        void display() {
            Node *curr = head;

            while (curr->next) {
                cout << curr->data << ' ';
                curr = curr->next;
            }

            cout << curr->data << endl;
        }

        void displayReverse() {
            Node *curr = tail;

            while (curr->prev) {
                cout << curr->data << ' ';
                curr = curr->prev;
            }

            cout << curr->data << endl;
        }
};

int main() {
    cout << "Process running." << endl;
    
    DoublyLinkedList list;

    list.pushBack(1);
    list.pushBack(3);
    list.pushBack(5);
    list.pushBack(7);

    list.display(); // 1 3 5 7

    // 1 2 3 5 7
    list.insert(2, 2);
    list.display();
    
    // 1 2 3 4 5 7
    list.insert(4, 4);
    list.display();

    // 1 2 3 4 5 6 7
    list.insert(6, 6);
    list.display();
    
    list.insert(0, 1);
    list.display();

    list.insert(-1, 0);
    list.insert(10, 20);

    list.deleteKey(5);
    list.display();

    list.deleteKey(0);
    list.display();

    list.deleteKey(10);
    list.display();

    list.deleteAt(2);
    list.display();

    list.displayReverse();

    cout << "Process finished." << endl;

    return 0;
}