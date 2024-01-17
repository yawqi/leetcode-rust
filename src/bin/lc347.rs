struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
impl Solution {
    //     pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    //         let mut count_map = HashMap::new();
    //         for num in nums {
    //             count_map.entry(num).and_modify(|c| *c += 1).or_insert(1);
    //         }
    //
    //         let mut min_heap = BinaryHeap::new();
    //         let mut count = &mut 0;
    //         count_map.into_iter().for_each(|(key, v)| {
    //             if *count < k {
    //                 min_heap.push(Reverse((v, key)));
    //             } else {
    //                 let top = min_heap.pop().unwrap();
    //                 if top.0 .0 > v {
    //                     min_heap.push(top);
    //                 } else {
    //                     min_heap.push(Reverse((v, key)));
    //                 }
    //             }
    //             *count += 1;
    //         });
    //         min_heap.into_iter().map(|Reverse((c, v))| v).collect()
    //     }
    //
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count_map = HashMap::new();
        nums.into_iter().for_each(|v| {
            count_map.entry(v).and_modify(|c| *c += 1).or_insert(1);
        });

        let mut nums = count_map.into_iter().collect::<Vec<_>>();
        let len = nums.len();
        Self::quick_sort(&mut nums, 0, len - 1, (k - 1) as usize);
        nums.into_iter().take(k as usize).map(|(k, v)| k).collect()
    }

    fn partition(
        nums: &mut Vec<(i32, i32)>,
        left: usize,
        right: usize,
        pivot_index: usize,
    ) -> usize {
        let pivot_value = nums[pivot_index];
        let mut count_index = left;
        nums.swap(pivot_index, right);
        for i in left..right {
            if nums[i].1 > pivot_value.1 {
                nums.swap(count_index, i);
                count_index += 1;
            }
        }

        nums.swap(count_index, right);
        count_index
    }

    fn quick_sort(nums: &mut Vec<(i32, i32)>, left: usize, right: usize, k: usize) {
        if left >= right {
            return;
        }
        let pivot_index = (left + right) / 2;
        let partition_index = Self::partition(nums, left, right, pivot_index);
        if partition_index >= k {
            Self::quick_sort(nums, left, partition_index - 1, k);
        } else {
            if partition_index > 0 {
                Self::quick_sort(nums, left, partition_index - 1, k);
            }
            Self::quick_sort(nums, partition_index + 1, right, k);
        }
    }
}

fn main() {
    let nums = vec![1];
    let ret = Solution::top_k_frequent(nums, 1);
    println!("{:?}", ret);
}
