fn main() {}
struct Solution;


impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        Self::combination(k, n, 1, vec![])
    }

    fn combination(k: i32, n: i32, first: i32, curr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let mut curr = curr;
        if k == 1 {
            if first <= n && n <= 9 {
                curr.push(n);
                ret.push(curr);
            }
            return ret;
        }

        for i in first..10 {
            curr.push(i);
            let mut v = Self::combination(k - 1, n - i, i + 1, curr.clone());
            ret.append(&mut v);
            curr.pop();
        }
        ret
    }
}