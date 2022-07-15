struct Solution;
fn main() {}
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut combines = vec![];
        let mut combine = vec![];
        let nums = (1..=n).into_iter().collect::<Vec<_>>();
        Self::traverse(&mut combine, &mut combines, &nums, k);
        combines
    }
    fn traverse(combine: &mut Vec<i32>, combines: &mut Vec<Vec<i32>>, nums: &[i32], k: i32) {
        if k == 0 {
            combines.push(combine.clone());
            return;
        }

        for i in 0..nums.len() {
            combine.push(nums[i]);
            Self::traverse(combine, combines, &nums[i+1..], k-1);
            combine.pop();
        }
    }
}