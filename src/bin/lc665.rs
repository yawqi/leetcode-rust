use std::cmp::Reverse;

struct Solution;
impl Solution {
    //    pub fn check_possibility(mut nums: Vec<i32>) -> bool {
    //        if nums.len() < 2 {
    //            return true;
    //        }
    //
    //        let mut reverse_pair = false;
    //        let mut prev = nums[0];
    //        let mut found = true;
    //        for idx in 1..nums.len() {
    //            println!("{} {}", prev, nums[idx]);
    //            if nums[idx] < prev {
    //                if reverse_pair {
    //                    found = false;
    //                    break;
    //                }
    //                reverse_pair = true;
    //            } else {
    //                prev = nums[idx];
    //            }
    //        }
    //
    //        if !found {
    //            found = true;
    //            reverse_pair = false;
    //            let mut prev_min = i32::MIN;
    //            for idx in 0..nums.len() - 1 {
    //                println!("{} {}", nums[idx], nums[idx + 1]);
    //                if nums[idx] > nums[idx + 1] {
    //                    if reverse_pair || nums[idx + 1] < prev_min {
    //                        found = false;
    //                        break;
    //                    }
    //                    reverse_pair = true;
    //                    prev_min = nums[idx + 1];
    //                } else {
    //                    prev_min = nums[idx];
    //                }
    //            }
    //        }
    //
    //        found
    //    }
    pub fn check_possibility(mut nums: Vec<i32>) -> bool {
        let mut rev_pair = false;
        for idx in 0..nums.len() - 1 {
            if nums[idx] > nums[idx + 1] {
                if rev_pair {
                    return false;
                }
                rev_pair = true;
                if idx > 0 && nums[idx - 1] > nums[idx + 1] {
                    nums[idx + 1] = nums[idx];
                }
            }
        }

        true
    }
}

fn main() {
    let mut nums = vec![4, 2, 1];
    println!("{}", Solution::check_possibility(nums));
}
