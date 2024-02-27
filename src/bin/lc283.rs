struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut l = 0;
        let mut r = 0;
        let len = nums.len();
        while r < len {
            while r < len && nums[r] == 0 {
                r += 1;
            }
            if r >= len {
                break;
            }
            nums.swap(l, r);
            l += 1;
            r += 1;
        }
    }
}

fn main() {}
