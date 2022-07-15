using namespace std;
struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};
class Solution {
public:
    int m_dep;
    int diameterOfBinaryTree(TreeNode* root) {
        if (root) {
            int r_max = max_depth(root->right);
            int l_max = max_depth(root->left);
            return max(r_max+l_max, m_dep);
        } else {
            return 0;
        }
    }

    int max_depth(TreeNode* root) {
        if (root) {
            int r_max = max_depth(root->right);
            int l_max = max_depth(root->left);
            m_dep = max(m_dep, r_max + l_max);
            return 1 + max(r_max, l_max);
        } else {
            return 0;
        }
    }
};