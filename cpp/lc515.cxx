using namespace std;
struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};
#include <vector>
#include <deque>
class Solution {
public:
    vector<int> largestValues(TreeNode* root) {
        vector<int> ret;
        deque<TreeNode*> deque;
        if (root) {
            deque.push_back(root);
        }

        while (!deque.empty()) {
            int max = INT_MIN;
            int len = deque.size();
            for (int i = 0; i < len; i++) {
                TreeNode *node = deque.front();
                deque.pop_front();
                max = max(max, node->val);
                if (node->left)
                    deque.push_back(node->left);
                if (node->right)
                    deque.push_back(node->right);
            }
            ret.push_back(max);
        }
        return ret;
    }
};