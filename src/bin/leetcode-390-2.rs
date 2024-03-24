struct Solution;
impl Solution {
    pub fn min_operations(k: i32) -> i32 {
        let mut num = 1;
        let mut count = k - 1;
        let mut div = 2;
        while div < k {
            let mut v = k / div;
            while v * div < k {
                v += 1;
            }
            count = count.min(v - 1 + div - 1);
            div += 1;
        }
        count
    }
}

fn main() {
    unimplemented!();
}
