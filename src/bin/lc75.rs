struct Solution;
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut p1 = 0;
        let mut p2 = 0;
        let len = nums.len();

        for idx in 0..len {
            match nums[idx] {
                0 => {
                    nums.swap(p1, idx);
                    if nums[idx] == 1 {
                        nums.swap(p2, idx);
                    }
                    p1 += 1;
                    p2 += 1;
                }
                1 => {
                    nums.swap(p2, idx);
                    p2 += 1;
                }
                _ => {}
            }
            println!("{:?}", nums);
        }
    }
}

fn main() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    Solution::sort_colors(&mut nums);
}
