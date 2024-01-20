struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut lens = vec![1; len + 1];
        let mut max_len = 1;

        lens[0] = 0;
        for idx in 2..=len {
            for prev in 1..idx {
                if nums[prev - 1] < nums[idx - 1] {
                    lens[idx] = lens[idx].max(lens[prev] + 1);
                    max_len = max_len.max(lens[idx]);
                }
            }
        }
        max_len
    }
}
