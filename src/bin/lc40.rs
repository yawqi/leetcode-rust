fn main() {}
struct Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
        Self::combination(&candidates, target, vec![])
    }

    fn combination(candidates: &[i32], target: i32, curr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let mut curr = curr;
        if candidates.len() == 0 || target == 0 {
            if target == 0 {
                ret.push(curr);
            }
            return ret;
        }
        
        if candidates[0] <= target {
            let mut v2 = Self::combination(&candidates[1..], target, curr.clone());
            curr.push(candidates[0]);
            let mut v1 = Self::combination(&candidates[1..], target - candidates[0], curr.clone());
            ret.append(&mut v1);
            ret.append(&mut v2);
        }

        ret
    }
}