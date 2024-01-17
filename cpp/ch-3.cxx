#include <unordered_set>
#include <string>
#include <vector>
#include <iostream>
using namespace std;

bool validate_username(string user) {
    if (user.size() )
    for (auto ch : user) {
        if ((ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z'))
            continue;
        else
            return false;
    }
    return true;
}
int main() {
    int n;
    cin >> n;
    vector<string> users;
    unordered_set<string> registerd_users;
    for (int i = 0; i < n; i++) {
        string user;
        cin >> user;
        users.push_back(user);
    }

    for (string user : users) {
        if (user.size() < 6 || user.size() > 12) {
            cout << "illegal length" << endl;
        } else if (validate_username(user)) {
            if (registerd_users.find(user) == registerd_users.end()) {
                cout << "registration complete" << endl;
                registerd_users.insert(user);
            } else {
                cout << "acount existed" << endl;
            }
        } else {
            cout << "illegal charactor" << endl;
        }
    }
}