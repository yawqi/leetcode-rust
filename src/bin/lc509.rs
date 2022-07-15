struct Solution;
fn main() {}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut v = vec![0, 1];
        if n < 2 {
            return v[n as usize];
        }

        for i in 2..=n {
            let nxt = v[0] + v[1];
            v[0] = v[1];
            v[1] = nxt;
        }
        v[1]
    }
}