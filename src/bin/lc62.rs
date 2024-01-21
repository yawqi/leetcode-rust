struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut paths = vec![vec![0; n]; m];
        for i in 0..m {
            paths[i][0] = 1;
        }

        for i in 1..n {
            paths[0][i] = 1;
        }

        for r in 1..m {
            for c in 1..n {
                paths[r][c] = paths[r - 1][c] + paths[r][c - 1];
            }
        }

        paths[m - 1][n - 1] as i32
    }
}
