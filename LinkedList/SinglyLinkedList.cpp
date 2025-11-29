#include <iostream>
using namespace std;

class Node {
public:
    int data;
    Node* next;

    Node(int val) : data(val), next(nullptr) {}
};

class SinglyLinkedList {
public:
    Node *head;
    int size;

    SinglyLinkedList() : head(nullptr), size(0) {}

    ~SinglyLinkedList() {
        while(head) popFront();
        head = nullptr;
        size = 0;
    }

    void pushBack(int val) {
        Node *newNode = new Node(val);

        if (!head) {
            head = newNode;
        } else {
            Node *temp = getNthFromTail(1);
            temp->next = newNode;
        }

        ++size;
    }

    void pushFront(int val) {
        Node *newNode = new Node(val);
        newNode->next = head;
        head = newNode;
        ++size;
    }

    // 0-based indexing
    void insert(int pos, int val) {
        if (pos < 0 || pos > size) {
            cout << "Position out of bounds" << endl;
            return;
        }

        if (pos == 0) {
            pushFront(val);
            return;
        }

        if (pos == size) {
            pushBack(val);
            return;
        }

        Node *newNode = new Node(val);
        Node *temp = getNthFromHead(pos);

        newNode->next = temp->next;
        temp->next = newNode;
        ++size;
    }

    void popFront() {
        if (head == nullptr) return;
        
        Node *temp = head;
        head = head->next;

        temp->next = nullptr;
        delete temp;
        --size;
    }

    void popBack() {
        if (head == nullptr) return;
        
        if (size == 1) {
            delete head;
            head = nullptr;
            size = 0;
        } else {
            Node *temp = getNthFromTail(2);
            delete temp->next;
            temp->next = nullptr;
            --size;
        }
    }

    // 0-based indexing
    void deleteAt(int pos) {
        if (pos < 0 || pos >= size) {
            cout << "Position out of bounds" << endl;
            return;
        }

        if (pos == 0) {
            popFront();
            return;
        }

        if (pos == size-1) {
            popBack();
            return;
        }

        Node *temp = getNthFromHead(pos);
        Node *deleteNode = temp->next;
        temp->next = temp->next->next;
        delete deleteNode;
        --size;
    }

    bool deleteKey(int val) {
        if(!isExist(val)) return false;   

        Node *temp = head;
        int pos = 1;
        while(temp) {
            if (temp->data == val) {
                deleteAt(pos);
                return true;
            }
            temp = temp->next;
            ++pos;
        }

        return false;
    }

    bool isExist(int val) {
        if (isEmpty()) return false;

        Node *temp = head;
        while(temp) {
            if (temp->data == val) {
                return true;
            }
            temp = temp->next;
        }

        return false;
    }

    bool isEmpty() const {
        return size == 0;
    }

    // 1-based indexing
    Node* getNthFromHead(int n) const {
        if (!head) return nullptr;
        if (n < 0 || n > size) return nullptr;

        Node *temp = head;
        for (int i=1; i<n; ++i) {
           temp = temp->next;
        }

        return temp;
    }

    // 1-based indexing
    Node* getNthFromTail(int n) const {
        if (!head) return nullptr;
        if (n < 0 || n > size) return nullptr;

        Node *temp = head;
        for (int i=size-n; i>0; --i) {
           temp = temp->next;
        }

        return temp;
    }

    int getSize() const {
        return size;
    }

    int getSizeIterative() const {
        int count = 0;
        Node *temp = head;
        while (temp) {
            ++count;
            temp = temp->next;
        }
        return count;
    }

    int getSizeRecursive(Node *node) const {
        if (!node) {
            return 0;
        }
        return 1 + getSizeRecursive(node->next);
    }

    void display() const {
        Node *temp = head;
        while (temp) {
            cout << temp->data << " -> ";
            temp = temp->next;
        }
        cout << "NULL" << endl;
    }

    void displayReverse() {
        reverse();
        display();
        reverse();
    }

    void reverse() {
        Node *prev = nullptr;
        Node *curr = head;
        
        while(curr) {
            Node *temp = curr->next;
            curr->next = prev;
            prev = curr;
            curr = temp;
        }

        head = prev;
    }
};

int main() {
    SinglyLinkedList list;
    list.pushBack(10);
    list.pushBack(20);
    list.pushBack(30);
    list.pushBack(40);
    list.pushBack(50);

    list.display();

    list.insert(2, 25);
    list.display();

    list.deleteAt(4);
    list.display();


    return 0;
}
