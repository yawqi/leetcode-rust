use std::usize;

struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut jump_counts = vec![usize::MAX; len];
        jump_counts[0] = 0;

        for i in 0..len {
            let count = nums[i] as usize;
            for j in 1..=count {
                if i + j >= len {
                    break;
                }
                jump_counts[i + j] = jump_counts[i + j].min(jump_counts[i] + 1);
            }
        }

        jump_counts[len - 1] as i32
    }
}
