#include <vector>
#include <set>
using namespace std;
class Solution {
public:
    vector<vector<int>> permute(vector<int>& nums) {
        vector<vector<int>> paths;
        vector<int> path;
        set<int> used;
        traverse(path, paths, used, nums);
        return paths;
    }

    void traverse(vector<int>& path, vector<vector<int>>& paths, set<int>& used, vector<int>& nums) {
        if (nums.size() == path.size()) {
            paths.push_back(vector<int>(path));
            return;
        }

        for (auto n: nums) {
            if (used.find(n) != used.end()) continue;
            used.insert(n);
            path.push_back(n);
            traverse(path, paths, used, nums);
            path.pop_back();
            used.erase(n);
        }
    }
};