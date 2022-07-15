struct Solution;
fn main() {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut subsets = vec![vec![]];
        let mut subset = vec![];
        Self::traverse(&mut subset, &nums[..], &mut subsets);
        subsets
    }
    fn traverse(subset: &mut Vec<i32>, nums: &[i32], subsets: &mut Vec<Vec<i32>>) {
        for i in 0..nums.len() {
            subset.push(nums[i]);
            subsets.push(subset.clone());
            Self::traverse(subset, &nums[i+1..], subsets);
            subset.pop();
        }
    }
}