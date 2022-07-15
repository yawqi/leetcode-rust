#include <vector>
using namespace std;
class Solution {
public:
    int fib(int n) {
        vector<int> v{0, 1};
        if (n < 2) return v[n];
        for (int i = 2; i <= n; i++) {
            int nxt = v[0] + v[1];
            v[0] = v[1];
            v[1] = nxt;
        }
        return v[1];
    }
};