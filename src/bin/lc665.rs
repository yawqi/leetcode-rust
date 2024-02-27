struct Solution;
impl Solution {
    pub fn check_possibility(mut nums: Vec<i32>) -> bool {
        let mut reverse_pair = false;
        let mut prev_max = i32::MIN;
        for idx in 0..nums.len() - 1 {
            println!("{} {} {}", prev_max, nums[idx], nums[idx + 1]);
            if nums[idx] > nums[idx + 1] {
                if reverse_pair || prev_max > nums[idx + 1] {
                    return false;
                }
                nums[idx] = prev_max;
                reverse_pair = true;
            }
            prev_max = nums[idx];
        }
        true
    }
}

fn main() {
    let mut nums = vec![5, 7, 1, 8];
    Solution::check_possibility(nums);
}
