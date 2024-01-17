
#include <iostream>
#include <string>
#include <vector>
using namespace std;

int min_max = -1;
int n, m;

void traverse(vector<int> &weights, vector<vector<int>> &sons,  int idx, int curr, int target, int curr_max) {
    if (curr >= target) {
        if (min_max == -1)
            min_max = curr_max;
        min_max = min(min_max, curr_max);
        return;
    }
    if (idx >= weights.size())
        return/*  */;
    curr += weights[idx];
    curr_max = max(curr_max, weights[idx]);
    for (auto son_idx : sons[idx]) {
        traverse(weights, sons, son_idx, curr, target, curr_max);
    }
}

int main() {
    cin >> n >> m;
    vector<int> weights(n);
    vector<vector<int>> sons(n);
    for (int i = 0; i < n ; i++)
        cin >> weights[i];

    for (int i = 0; i < n; i++) {
        int count = 0;
        cin >> count;
        while (count) {
            int son = 0;
            cin >> son;
            sons[i].push_back((son-1));
            count--;
        }
    }

    traverse(weights, sons, 0, 0, m, weights[0]);
    cout << min_max;
}

