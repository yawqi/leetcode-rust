struct Solution;

impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        let mut counts = 0;
        counts += (((n - 1) / 2) as i64 * ((m + 1) / 2) as i64);
        counts += (((n + 1) / 2) as i64 * ((m - 1) / 2) as i64);
        counts
    }
}
