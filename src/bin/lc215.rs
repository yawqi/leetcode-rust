use std::collections::BinaryHeap;
use std::collections::HashMap;

struct Solution;
impl Solution {
    //    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
    //        let len = nums.len();
    //        Self::quick_select(&mut nums, 0, len - 1, (len - k as usize))
    //    }
    //
    //    fn partition(
    //        nums: &mut Vec<i32>,
    //        mut left: usize,
    //        mut right: usize,
    //        mut pivot_index: usize,
    //    ) -> usize {
    //        let num = nums[pivot_index];
    //        nums.swap(right, pivot_index);
    //        pivot_index = left;
    //        for i in left..right {
    //            if nums[i] < num {
    //                nums.swap(pivot_index, i);
    //                pivot_index += 1;
    //            }
    //        }
    //        nums.swap(pivot_index, right);
    //        pivot_index
    //    }
    //
    //    fn quick_select(nums: &mut Vec<i32>, left: usize, right: usize, k: usize) -> i32 {
    //        if right == left {
    //            return nums[left];
    //        }
    //
    //        let pivot_index = (left + right) / 2;
    //        let order_index = Self::partition(nums, left, right, pivot_index);
    //        if order_index == k {
    //            return nums[order_index];
    //        } else if order_index > k {
    //            return Self::quick_select(nums, left, order_index - 1, k);
    //        } else {
    //            return Self::quick_select(nums, order_index + 1, right, k);
    //        }
    //    }
    //
}
