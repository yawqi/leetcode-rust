struct Solution;

impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        let len = nums.len() / 2;
        nums.sort();
        nums[len]
    }
}
