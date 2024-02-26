use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        nums.into_iter().all(|v| {
            let e = map.entry(v).or_insert(0);
            *e += 1;
            *e <= 2
        })
    }
}
fn main() {}
