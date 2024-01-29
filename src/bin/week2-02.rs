struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn maximum_length(mut nums: Vec<i32>) -> i32 {
        let mut num_counts = HashMap::new();
        let mut max_count = 1;
        let mut one_count = 0;
        nums.iter().for_each(|n| {
            if *n == 1 {
                one_count += 1;
            }
            num_counts.entry(*n).and_modify(|e| *e += 1).or_insert(1);
        });

        max_count = max_count.max((one_count - 1) / 2 * 2 + 1);
        nums.sort();
        let max_num = *nums.last().unwrap();

        for num in nums {
            if max_num < num * num || num == 1 {
                continue;
            }

            if *num_counts.get(&num).unwrap() == 1 {
                continue;
            }

            let mut nxt = num * num;
            let mut curr_count = 1;
            while let Some(count) = num_counts.get(&nxt) {
                curr_count += 2;
                if *count == 1 {
                    break;
                }
                nxt = nxt * nxt;
            }
            max_count = max_count.max(curr_count);
        }

        max_count
    }
}

fn main() {
    let mut nums = vec![
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024,
    ];
    println!("{}", Solution::maximum_length(nums));
}
