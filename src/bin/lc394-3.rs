struct Solution;
impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut counts = vec![vec![0; 10]; m];

        for c in 0..m {
            for num in 0usize..10usize {
                for r in 0..n {
                    if grid[r][c] as usize != num {
                        counts[c][num] += 1;
                    }
                }
            }
        }

        let mut ans = i32::MAX;
        let mut dp = vec![vec![i32::MAX; 10]; m];
        for v in 0usize..10usize {
            dp[0][v] = counts[0][v];
        }

        for c in 1..m {
            for curr_v in 0usize..10usize {
                for prev_v in 0usize..10usize {
                    if curr_v != prev_v {
                        dp[c][curr_v] = dp[c][curr_v].min(dp[c - 1][prev_v] + counts[c][curr_v]);
                    }
                }
            }
        }

        for v in 0..10 {
            ans = ans.min(dp[m - 1][v]);
        }
        ans
    }
}

fn main() {}
