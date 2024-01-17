use std::simd::usizex1;

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut p = 0;
        let mut q = 0;
        let mut r = 1;
        for n in 1..=n {
            p = q;
            q = r;
            r = p + q;
        }
        r
    }
}
