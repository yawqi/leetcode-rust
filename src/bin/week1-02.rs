struct Solution;

impl Solution {
    pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i32> {
        let n = n as usize;
        let x = x as usize;
        let y = y as usize;
        let mut counts = vec![0; n];
        for len in 1..n {
            counts[len - 1] = 2 * (n - len);
        }
        let circle_len = (x.max(y) - x.min(y)) / 2;

        for i in 0..circle_len {
            counts[i] += 2;
            counts[circle_len] -= 2;
        }

        for i in n - circle_len..n {}
        counts.into_iter().map(|v| v as i32).collect()
    }
}
