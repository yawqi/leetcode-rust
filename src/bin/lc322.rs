struct Solution;
fn main() {}
// @lc code=start
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let money = amount as usize;
        let mut counts = vec![i32::MAX; money + 1];
        counts[0] = 0;
        for curr in (1..=money) {
            for coin in &coins {
                if *coin as usize > curr {
                    continue;
                }
                if counts[curr - *coin as usize] != i32::MAX {
                    counts[curr] = std::cmp::min(counts[curr - *coin as usize] + 1, counts[curr]);
                }
            }
        }
        if counts[money] != i32::MAX {
            counts[money]
        } else {
            -1
        }
    }
}
// @lc code=end