struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |ret, v| ret ^ v)
    }
}
