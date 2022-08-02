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
        let mut i = 0;
        while i < candidates.len() && target >= candidates[i] {
            curr.push(candidates[i]);
            let mut v = Self::combination(&candidates[i+1..], target-candidates[i], curr.clone());
            ret.append(&mut v);
            curr.pop();
            i += 1;
            while i < candidates.len() && candidates[i] == candidates[i-1] {
                i += 1;
            }
        }
        ret
    }
}