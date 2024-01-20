use std::usize;

struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().fold(0, |sum, i| sum + *i);
        if sum % 2 == 1 || nums.len() < 2 {
            return false;
        }

        let target = sum / 2;
        if nums.iter().any(|v| *v > target) {
            return false;
        }

        let mut dp = vec![false; target as usize + 1];
        dp[0] = true;
        dp[nums[0] as usize] = true;
        for idx in 1..nums.len() {
            let num = nums[idx];
            for curr_sum in (1..=target).rev() {
                if num <= curr_sum {
                    dp[curr_sum as usize] = dp[curr_sum as usize] | dp[(curr_sum - num) as usize];
                }
            }
        }
        dp[target as usize]
    }
}
