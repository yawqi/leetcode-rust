struct Solution;

impl Solution {
    /*
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut results = vec![1; nums.len() + 1];
        let mut max_prod = i32::MIN;
        for len in 1..=nums.len() {
            for idx in (len..=nums.len()).rev() {
                results[idx] = results[idx - 1] * nums[idx - 1];
                max_prod = max_prod.max(results[idx]);
            }
        }
        max_prod
    }
    */

    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut maxs = nums.clone();
        let mut mins = nums.clone();

        let mut ret = i32::MIN;
        for i in 1..nums.len() {
            maxs[i] = maxs[i]
                .max(maxs[i - 1] * nums[i])
                .max(mins[i - 1] * nums[i]);
            mins[i] = mins[i]
                .min(maxs[i - 1] * nums[i])
                .min(mins[i - 1] * nums[i]);

            ret.max(maxs[i]);
        }
        ret
    }
}
