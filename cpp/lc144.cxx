#include <vector>
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
    vector<int> preorderTraversal(TreeNode* root) {
        vector<int> v;
        if (root) {
            v.push_back(root->val);
            vector<int> r_vec = preorderTraversal(root->right);
            vector<int> l_vec = preorderTraversal(root->left);
            v.reserve(v.size() + r_vec.size() + l_vec.size());

            move(l_vec.begin(), l_vec.end(), inserter(v, v.end()));
            move(r_vec.begin(), r_vec.end(), inserter(v, v.end()));
        }
        return v;
    }
};