#include <vector>
#include <deque>
using namespace std;
struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};
// @lc code=start
class Solution {
public:
    int maxDepth(TreeNode* root) {
        deque<TreeNode *> deq;
        int depth = 0;
        if (root) {
            deq.push_back(root);
        }

        while (!deq.empty()) {
            depth += 1;
            int s = deq.size();
            for (int i = 0; i < s; i++) {
                TreeNode *node = deq.front();
                deq.pop_front();
                if (node->left) {
                    deq.push_back(node->left);
                }
                if (node->right) {
                    deq.push_back(node->right);
                }
            }
        }
        return depth;
    }
};
// @lc code=end
