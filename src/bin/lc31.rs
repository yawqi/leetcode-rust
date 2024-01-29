use std::usize;

struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i = nums.len() - 1;
        while i > 0 {
            if nums[i - 1] < nums[i] {
                let mut j = i;
                while j < nums.len() && nums[j] > nums[i - 1] {
                    j += 1;
                }
                nums.swap(i - 1, j - 1);
                break;
            }
            i -= 1;
        }

        nums[i..].reverse()
    }
}

fn main() {
    let mut nums = vec![2, 3, 1, 3, 3];
    Solution::next_permutation(&mut nums);
    println!("{:?}", nums);
}
