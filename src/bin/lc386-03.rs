struct Solution;

impl Solution {
    fn check(sum: i32, nums: &Vec<i32>, change_indices: &Vec<i32>, len: usize) -> bool {
        let n = nums.len();
        let mut marked = 0;
        let mut is_marked = vec![false; n];
        let mut res = 0;
        if len < n + sum as usize {
            return false;
        }

        for indice in change_indices[0..len].iter().rev() {
            let indice = *indice as usize;
            if !is_marked[indice - 1] {
                is_marked[indice - 1] = true;
                res += nums[indice - 1];
                marked += 1;
            } else {
                res = (res - 1).max(0);
            }
        }

        marked == n && res == 0
    }

    pub fn earliest_second_to_mark_indices(mut nums: Vec<i32>, change_indices: Vec<i32>) -> i32 {
        let mut min = -1;
        let mut l = 1;
        let mut r = change_indices.len();
        let sum = nums.iter().fold(0, |sum, v| sum + *v);
        while l <= r {
            let mid = (l + r) / 2;
            if Self::check(sum, &nums, &change_indices, mid) {
                min = mid as i32;
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        min
    }
}

fn main() {
    let mut nums = vec![2, 2, 0];
    let mut change_indices = vec![2, 2, 2, 2, 3, 2, 2, 1];
    Solution::earliest_second_to_mark_indices(nums, change_indices);
}
