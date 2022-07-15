struct Solution;
fn main() {}

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut curr = vec![];
        let mut subsets = vec![vec![]];
        let mut nums = nums;
        nums.sort();
        Self::traverse(&mut curr, &mut subsets, &nums);
        subsets
    }

    fn traverse(curr: &mut Vec<i32>, subsets: &mut Vec<Vec<i32>>, nums: &[i32]) {
        let mut i = 0;
        while i < nums.len() {
            curr.push(nums[i]);
            subsets.push(curr.clone());
            Self::traverse(curr, subsets, &nums[i+1..]);
            curr.pop();
            i += 1;
            while i < nums.len() && nums[i] == nums[i-1] {
                i += 1;
            }
        }
    }
}