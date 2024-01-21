struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut sums = vec![vec![0; cols]; rows];

        let mut sum = 0;
        for r in 0..rows {
            sum += grid[r][0];
            sums[r][0] = sum;
        }
        sum = 0;
        for c in 0..cols {
            sum += grid[0][c];
            sums[0][c] = sum;
        }

        for r in 1..rows {
            for c in 1..cols {
                sums[r][c] = grid[r][c] + std::cmp::min(sums[r - 1][c], sums[r][c - 1]);
            }
        }

        sums[rows - 1][cols - 1]
    }
}
