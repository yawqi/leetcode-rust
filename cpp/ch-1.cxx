#include <iostream>
#include <string>
#include <vector>
#include <deque>
using namespace std;

class Message {
public:
    int type;
    string val;
    Message *next;
    Message *prev;
    Message(int type = 0, string val = ""): type(type), val(val), next(nullptr), prev(nullptr) {};
};

class MessageQueue {
public:
    Message *head;
    Message *tail;
    deque<Message*> queues[101];
    MessageQueue() {
        head = new Message();
        tail = new Message();
        head->next = tail;
        tail->prev = head;
    };
    
    bool is_empty() {
        return head->next == tail;
    }

    Message* remove_message(int type) {
        Message *ret = nullptr;
        if (type == 0) {
            if (is_empty())
                return nullptr;
            ret = head->next;
            remove_node(ret);
            queues[ret->type].pop_front();
        } else {
            if (queues[type].empty())
                return nullptr;
           ret = queues[type].front();
           queues[type].pop_front();
           remove_node(ret);
        }
        return ret;
    }

    void remove_node(Message *mess) {
        mess->prev->next = mess->next;
        mess->next->prev = mess->prev;
    }

    void append_to_tail(Message *mess) {
        mess->prev = tail->prev;
        mess->next = tail;
        tail->prev->next = mess;
        tail->prev = mess;
        queues[mess->type].push_back(mess);
    }
};

int main() {
    int count;
    cin >> count;
    MessageQueue mq;
    vector<string> ans;
    for (int i = 0; i < count; i++) {
        string op;
        int type;
        string val;
        cin >> op >> type;
        if (op == "in") {
            cin >> val;
            Message *message = new Message(type, val);
            mq.append_to_tail(message);
        } else if (op == "out") {
            Message *ret = mq.remove_message(type);
            if (ret)
                ans.push_back(ret->val);
            else
                ans.push_back("-1");
        }
    }

    for (string a : ans)
        cout << a << endl;
}
#include <iostream>
#include <string>
#include <vector>
#include <deque>
using namespace std;

class Message {
public:
    int type;
    string val;
    Message *next;
    Message *prev;
    Message(int type = 0, string val = ""): type(type), val(val), next(nullptr), prev(nullptr) {};
};

class MessageQueue {
public:
    Message *head;
    Message *tail;
    deque<Message*> queues[101];
    MessageQueue() {
        head = new Message();
        tail = new Message();
        head->next = tail;
        tail->prev = head;
    };
    
    bool is_empty() {
        return head->next == tail;
    }

    Message* remove_message(int type) {
        Message *ret = nullptr;
        if (type == 0) {
            if (is_empty())
                return nullptr;
            ret = head->next;
            remove_node(ret);
            queues[ret->type].pop_front();
        } else {
            if (queues[type].empty())
                return nullptr;
           ret = queues[type].front();
           queues[type].pop_front();
           remove_node(ret);
        }
        return ret;
    }

    void remove_node(Message *mess) {
        mess->prev->next = mess->next;
        mess->next->prev = mess->prev;
    }

    void append_to_tail(Message *mess) {
        mess->prev = tail->prev;
        mess->next = tail;
        tail->prev->next = mess;
        tail->prev = mess;
        queues[mess->type].push_back(mess);
    }
};

int main() {
    int count;
    cin >> count;
    MessageQueue mq;
    vector<string> ans;
    for (int i = 0; i < count; i++) {
        string op;
        int type;
        string val;
        cin >> op >> type;
        if (op == "in") {
            cin >> val;
            Message *message = new Message(type, val);
            mq.append_to_tail(message);
        } else if (op == "out") {
            Message *ret = mq.remove_message(type);
            if (ret)
                ans.push_back(ret->val);
            else
                ans.push_back("-1");
        }
    }

    for (string a : ans)
        cout << a << endl;
}