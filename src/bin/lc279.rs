struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut num = 1;
        let mut counts = vec![0; n as usize + 1];
        let mut ns = vec![];
        for i in 1..=n {
            if (num * num == i) {
                counts[i as usize] = 1;
                ns.push(i);
                num += 1;
            } else {
                let mut min_count = i32::MAX;
                for n2 in &ns {
                    min_count = min_count.min(counts[i as usize - *n2 as usize] + 1);
                }
                counts[i as usize] = min_count;
            }
        }

        counts[n as usize]
    }
}
