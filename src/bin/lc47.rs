use std::collections::HashMap;

fn main() {}
struct Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut map = HashMap::new();
        nums.sort();
        nums.iter()
            .for_each(|n| {
                let e = map.entry(*n).or_insert(0);
                *e += 1;
            });
        let mut curr = vec![];
        Self::traverse(&nums, &mut map, &mut curr) 
    }

    fn traverse(nums: &[i32], visited: &mut HashMap<i32, i32>, curr: &mut Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        if nums.len() == curr.len() {
            ret.push(curr.clone());
            return ret;
        }

        let mut i = 0;
        while i < nums.len() {
            let e = visited.entry(nums[i]).or_default();
            if  *e != 0 {
                *e -= 1;
                curr.push(nums[i]);
                let mut v = Self::traverse(nums, visited, curr);
                ret.append(&mut v);
                curr.pop();
                let e = visited.entry(nums[i]).or_default();
                *e += 1;
            }
            i += 1;
            while i < nums.len() && nums[i] == nums[i-1] {
                i += 1;
            }
        }
        ret
    }
}