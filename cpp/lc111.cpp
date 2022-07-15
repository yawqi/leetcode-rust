struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

#include <deque>
using namespace std;
class Solution {
public:
    int minDepth(TreeNode* root) {
        deque<TreeNode*> deq;
        int depth = 0;
        if (root) deq.push_back(root);
        while (!deq.empty()) {
            depth += 1;
            int len = deq.size();
            for (int i = 0; i < len; i++) {
                TreeNode* node = deq.front();
                deq.pop_front();
                if (!node->right && !node->left) return depth;
                if (node->right) deq.push_back(node->right);
                if (node->left) deq.push_back(node->left);
            }
        }

        return depth;
    }
};