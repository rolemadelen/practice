// https://www.acmicpc.net/problem/4621

#include <iostream>
using namespace std;

class Node {
    public:
        int id;
        int width;
        Node *next;
        Node *prev;
        Node(int id, int width) : id(id), width(width), next(nullptr), prev(nullptr) {}
};

class LinkedList {
    private:
        Node *head;
        Node *tail;
        int size;
        
    public:
        int total_width;

        LinkedList() : head(nullptr), size(0), total_width(0) {}

        void push_front(int id, int w) {
            Node *new_node = new Node(id, w);

            if(head == nullptr) {
                head = new_node;
                tail = new_node;
            } else {
                head->prev = new_node;
                new_node->next = head;
                head = new_node;
            }

            total_width += w;
            ++size;
        }

        void pop_back() {
            if(size == 0) return;

            int w = tail->width;

            if(size == 1) {
                delete tail;
                head = nullptr;
                tail = nullptr;
            } else {
                tail = tail->prev;
                delete tail->next;
                tail->next = nullptr;
            }

            --size;
            total_width -= w;
        }

        void remove(int id) {
            int pos = find(id);
            if(pos == -1) {
                return;
            }
            if(pos == 0) {
                Node *del_node = head;
                head = head->next;

                total_width -= del_node->width;
                delete del_node;
            } else {
                Node *del_node = get_nth_node(pos);
                del_node->prev->next = del_node->next;
                del_node->next->prev = del_node->prev;
                total_width -= del_node->width;
                delete del_node;
            }
            --size;
        }

        void clear_shelf(int shelf_width) {
            while(total_width > shelf_width) {
                pop_back();
            }
        }

        int find(int id) {
            int index = 0;
            Node *curr = head;
            while(curr) {
                if(curr->id == id) {
                    return index;
                }
                curr = curr->next;
                ++index;
            }

            return -1;
        }

        Node* get_nth_node(int n) {
            Node *curr = head;

            for(int i=0; i<n; ++i) {
                curr = curr->next;
            }

            return curr;
        }

        void display(int tc) {
            cout << "PROBLEM " << tc++ << ": ";
            Node *curr = head;
            for(int i=0; i<size; ++i) {
                cout << curr->id << " ";
                curr = curr->next;
            }
            
            cout << endl;
        }
};

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);
    int tc = 1;

    while(true) {
        int shelf_width;
        cin >> shelf_width;

        if(shelf_width == -1) break;

        LinkedList list;

        while(true) {
            string str;
            cin >> str;
            int id;
            int width;

            char act = str[0];

            if(act == 'E') {
                break;
            } else if (act == 'R') {
                cin >> id;
                list.remove(id);
            } else if (act == 'A') {
                cin >> id;
                cin >> width;

                list.push_front(id, width);
                list.clear_shelf(shelf_width);
            }
        }

        list.display(tc++);
    }
}