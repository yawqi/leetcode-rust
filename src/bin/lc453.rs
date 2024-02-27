struct Solution;
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let min = nums.iter().min().unwrap();
        let mut acc = 0;
        for nxt in &nums[..] {
            acc += *nxt - *min;
        }
        acc
    }
}
