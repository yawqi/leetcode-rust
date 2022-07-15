using namespace std;
#include <set>
#include <vector>
#include <deque>
#include <string>
class Solution {
public:
    int openLock(vector<string>& deadends, string target) {
        set<string> deads;
        set<string> used;
        deque<string> deq;
        string init = "0000";
        int step = 0;
        for (auto it = deadends.begin(); it != deadends.end(); it++) {
            if (*it == init) return -1;
            deads.insert(*it);
        }
        deq.push_back(init);
        used.insert(init);
        while (!deq.empty()) {
            int len = deq.size();
            for (int i = 0; i < len; i++) {
                string curr = deq.front();
                deq.pop_front();
                if (curr == target) return step;
                for (int j = 0; j < 4; j++) {
                    string plus_one = curr;
                    string minus_one = curr;

                    plus_one[j] = ((curr[j] - '0') + 1) % 10 + '0';
                    minus_one[j] = (((curr[j] - '0') - 1) + 10) % 10 + '0';
                    if (deads.find(plus_one) == deads.end() && used.find(plus_one) == used.end()) {
                        deq.push_back(plus_one);
                        used.insert(plus_one);
                    }
                    if (deads.find(minus_one) == deads.end() && used.find(minus_one) == used.end()) {
                        deq.push_back(minus_one);
                        used.insert(minus_one);
                    }
                }
            }
            step += 1;
        }
        
        return -1;
    }
};