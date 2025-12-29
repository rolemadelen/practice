#include <iostream>
using namespace std;

class Node {
    public:
        int data;
        Node *next;
        Node(int data) : data(data), next(nullptr) {}
};

class CircularLinkedList {
    private:
        Node *head;
        Node *tail;
        int size;

    public:
        CircularLinkedList() : head(nullptr), tail(nullptr), size(0) {}

        void pushBack(int data) {
            Node *newNode = new Node(data);

            if (size == 0) {
                head = newNode;
                tail = newNode;
                head->next = tail;
            } else {
                tail->next = newNode;
                newNode->next = head;
                tail = newNode;
            }
            ++size;
        }

        void pushFront(int data) {
            Node *newNode = new Node(data);

            if (size == 0) {
                head = newNode;
                tail = newNode;
                head->next = tail;
            } else {
                newNode->next = head;
                tail->next = newNode;
                head = newNode;
            }
            ++size;
        }

        void popBack() {
            if (size == 1) {
                delete head;
                head = nullptr;
                tail = nullptr;
            } else {
                Node *curr = getNthNode(size-2);
                curr->next = head;
                delete tail;
                tail = curr;
            }
            --size;
        }

        void popFront() {
            if (size == 1) {
                delete head;
                head = nullptr;
                tail = nullptr;
            } else {
                Node *delNode = head;
                head = head->next;
                tail->next = head;
                
                delete delNode;
            }
            --size;
        }

        inline int getSize() const { return size; }
        
        void display() {
            Node *curr = head;
            for (int i=0; i<size-1; ++i) {
                cout << curr->data << ' ';
                curr = curr->next;
            }
            if (curr)
               cout << curr->data << " (" << size << ")" << endl;
        }

        Node* getNthNode(int n) {
            if (size == 0) {
                cerr << "List is empty" << endl;
                return nullptr;
            }

            Node *curr = head;
            for (int i=0; i<n; ++i) {
                curr = curr->next;
            }

            return curr;
        }

        bool isEmpty() {
            return size == 0;
        }
};

int main() {
    CircularLinkedList list;
    list.pushBack(3);
    list.pushBack(4);
    list.pushBack(5);
    list.pushBack(6);
    list.display();

    list.pushFront(2);
    list.pushFront(1);
    list.display();

    Node *node = list.getNthNode(100);
    if (node) {
        cout << "node at '100 mod " << list.getSize() << "' = " << node->data << endl;
    }

    // while(!list.isEmpty()) {
    //     list.popBack();
    //     list.display();
    // }

    // Node *data = list.getNthNode(20);
    // if (data) cout << list.getNthNode(20) << endl;
    
    // list.pushBack(10);
    // data = list.getNthNode(20);
    // if (data) cout << data->data << endl;
}