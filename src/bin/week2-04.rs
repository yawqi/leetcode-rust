struct Solution;

impl Solution {
    pub fn min_or_after_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 1 << 31 | 1 << 30;
        let mut mask = 0i32;
        for shift in (0..30).rev() {
            mask = 1 << shift;
            let mut cnt = 0;
            let mut curr = -1i32;
            for num in &nums {
                curr &= num;
                if curr & (mask | ans) == 0 {
                    curr = -1;
                    continue;
                }
                cnt += 1;
            }

            if cnt <= k {
                ans |= mask;
            }
        }

        !ans
    }
}
fn main() {}
