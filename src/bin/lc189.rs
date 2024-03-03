struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let mut len = nums.len();
        let k = k as usize % len;

        let nums_backup = nums.clone();
        let mut part_2 = nums.drain(..k).collect();
        nums.append(&mut part_2);
    }
}
