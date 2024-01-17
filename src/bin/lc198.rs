struct Solution;
/*
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 2];
        let mut ret = nums[0];
        dp[2] = nums[0];
        for i in 1..nums.len() {
            dp[i + 2] = std::cmp::max(dp[i] + nums[i], dp[i - 1] + nums[i]);
            ret = ret.max(dp[i + 2]);
        }
        ret
    }
}
*/

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut len = nums.len();
        let mut dp = vec![0; len + 2];
        for i in 0..len {
            dp[i + 2] = std::cmp::max(dp[i + 1], dp[i] + nums[i]);
        }

        dp[len + 1]
    }
}
