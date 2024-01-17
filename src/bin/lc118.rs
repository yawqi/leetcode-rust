struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = vec![];
        for nrow in 0..num_rows as usize {
            let mut row = vec![];
            row.resize(nrow + 1, 1);
            for i in 1..nrow {
                row[i] = ret[nrow - 1][i - 1] + ret[nrow - 1][i];
            }
            ret.push(row);
        }
        ret
    }
}
